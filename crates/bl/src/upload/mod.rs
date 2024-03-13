#[cfg(feature = "ssr")]
mod exif_ops;
#[cfg(feature = "ssr")]
mod watermark;

#[cfg(feature = "ssr")]
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use color_eyre::eyre::Result;
#[cfg(feature = "ssr")]
use core_types::PhotoRecordId;
use core_types::{PhotoGroupRecordId, PhotoGroupUploadParams};
use leptos::{server, server_fn::codec::Json, ServerFnError};
use strum::{Display, EnumString};

use crate::rmp_sfn::{MessagePack, RmpEncoded};

#[cfg(feature = "ssr")]
fn thumbnail_size(aspect_ratio: f32) -> (u32, u32) {
  if aspect_ratio > 1.0 {
    (200, (200.0 / aspect_ratio) as u32)
  } else {
    ((200.0 * aspect_ratio) as u32, 200)
  }
}

#[derive(Clone, Debug, EnumString, Display)]
pub enum PhotoUploadError {
  InvalidImage,
  Unauthenticated,
  InternalError(String),
}

#[server(
  input = MessagePack,
  custom = RmpEncoded,
  output = Json,
)]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn upload_photo_group(
  params: PhotoGroupUploadParams,
) -> Result<PhotoGroupRecordId, ServerFnError<PhotoUploadError>> {
  use rayon::prelude::*;
  use tokio::sync::mpsc;

  use crate::model_ext::ModelExt;

  let Some(user) =
    leptos::use_context::<core_types::LoggedInUser>().and_then(|u| u.0)
  else {
    Err(PhotoUploadError::Unauthenticated)?
  };

  let (tx, mut rx) = mpsc::channel(params.photos.len());

  let image_count = params.photos.len();
  let images = params
    .photos
    .into_iter()
    // mark with indices
    .enumerate()
    .par_bridge()
    // load original images
    .map(|(i, p)| {
      let exif_reader = exif::Reader::new();
      let meta = exif_reader
        .read_from_container(&mut std::io::Cursor::new(&p.original))
        .ok();
      image::load_from_memory(&p.original).map(|img| (i, (img, meta)))
    })
    // collect into a hashmap & short circuit on error
    .collect::<Result<HashMap<_, _>, _>>()
    .map_err(|e| {
      tracing::error!("Failed to load original image: {:?}", e);
      PhotoUploadError::InvalidImage
    })?;

  // spawn tasks for each image
  for (i, (img, meta)) in images {
    let tx = tx.clone();
    tokio::spawn(async move {
      let result = create_photo(img, meta).await;
      tx.send((i, result)).await.unwrap();
    });
  }
  drop(tx);

  // collect results
  let mut photo_ids = Vec::with_capacity(image_count);
  while let Some((i, result)) = rx.recv().await {
    let photo_id = result.map_err(|e| {
      let error = e.to_string();
      tracing::error!("Failed to upload photo: {}", error);
      PhotoUploadError::InternalError(error)
    })?;
    photo_ids.push((i, photo_id));
  }

  // sort photo ids by original order
  photo_ids.sort_by_key(|(i, _)| *i);
  let photo_ids: Vec<_> = photo_ids.into_iter().map(|(_, id)| id).collect();

  // create photo group
  let group = core_types::PhotoGroup {
    id:           core_types::PhotoGroupRecordId(core_types::Ulid::new()),
    owner:        user.id,
    photographer: user.id,
    photos:       photo_ids,
    status:       params.status,
    meta:         Default::default(),
  };

  // open a surreal client
  let client =
    clients::surreal::SurrealRootClient::new()
      .await
      .map_err(|e| {
        let error = e.to_string();
        tracing::error!("Failed to create surreal client: {}", error);
        PhotoUploadError::InternalError(error)
      })?;

  // create photo group
  group.create(&client).await.map_err(|e| {
    let error = e.to_string();
    tracing::error!("Failed to create photo group: {}", error);
    PhotoUploadError::InternalError(error)
  })?;

  // patch photos with the group id
  for photo_id in group.photos.iter() {
    core_types::Photo::patch(
      *photo_id,
      &client,
      surrealdb::opt::PatchOp::replace("group", group.id),
    )
    .await
    .map_err(|e| {
      let error = e.to_string();
      tracing::error!("Failed to update photo with group: {}", error);
      PhotoUploadError::InternalError(error)
    })?;
  }

  Ok(group.id)
}

#[cfg(feature = "ssr")]
async fn create_photo(
  mut img: image::DynamicImage,
  meta: Option<exif::Exif>,
) -> Result<PhotoRecordId> {
  use artifact::Artifact;
  use color_eyre::eyre::WrapErr;

  use crate::model_ext::ModelExt;

  // rotate image based on exif orientation
  exif_ops::rotate_image_from_exif(&mut img, meta.as_ref());

  // encode original image as jpeg
  let mut original_jpeg_bytes = Vec::new();
  let encoder = image::codecs::jpeg::JpegEncoder::new(&mut original_jpeg_bytes);
  img
    .write_with_encoder(encoder)
    .wrap_err("Failed to encode original image")?;

  // start a task for uploading the original image
  let original_artifact =
    core_types::PrivateArtifact::new(Some(original_jpeg_bytes.into()));
  let original_upload_task = tokio::spawn({
    let original_artifact = original_artifact.clone();
    async move {
      original_artifact
        .upload_and_push()
        .await
        .wrap_err("Failed to create original artifact")
    }
  });

  // calculate thumbnail size
  let aspect_ratio = img.width() as f32 / img.height() as f32;
  let thumbnail_size = thumbnail_size(aspect_ratio);

  // create thumbnail
  let mut thumbnail_image = img.resize_exact(
    thumbnail_size.0,
    thumbnail_size.1,
    image::imageops::FilterType::Lanczos3,
  );
  // apply watermark
  watermark::apply_watermark(&mut thumbnail_image);

  // encode thumbnail image as jpeg
  let mut thumbnail_bytes = Vec::new();
  let encoder = image::codecs::jpeg::JpegEncoder::new(&mut thumbnail_bytes);
  thumbnail_image
    .write_with_encoder(encoder)
    .wrap_err("Failed to encode thumbnail image")?;

  // start a task for uploading the thumbnail image
  let thumbnail_artifact =
    core_types::PublicArtifact::new(Some(thumbnail_bytes.into()));
  let thumbnail_upload_task = tokio::spawn({
    let thumbnail_artifact = thumbnail_artifact.clone();
    async move {
      thumbnail_artifact
        .upload_and_push()
        .await
        .wrap_err("Failed to create thumbnail artifact")
    }
  });

  // wait for both uploads to finish
  let (original_artifact_result, thumbnail_artifact_result) =
    tokio::try_join!(original_upload_task, thumbnail_upload_task)
      .wrap_err("Failed to upload artifacts")?;
  original_artifact_result.wrap_err("Failed to upload original artifact")?;
  thumbnail_artifact_result.wrap_err("Failed to upload thumbnail artifact")?;

  // create a photo and upload it to surreal
  let photo = core_types::Photo {
    id:         core_types::PhotoRecordId(core_types::Ulid::new()),
    group:      core_types::PhotoGroupRecordId(core_types::Ulid::nil()),
    artifacts:  core_types::PhotoArtifacts {
      original:  core_types::PrivateImageArtifact {
        artifact_id: original_artifact.id,
        size:        (img.width(), img.height()),
      },
      thumbnail: core_types::PublicImageArtifact {
        artifact_id: thumbnail_artifact.id,
        size:        (thumbnail_image.width(), thumbnail_image.height()),
      },
    },
    photo_meta: exif_ops::photo_meta_from_exif(meta),
    meta:       Default::default(),
  };

  let client = clients::surreal::SurrealRootClient::new().await?;

  photo
    .create(&client)
    .await
    .wrap_err("Failed to create photo in surreal")?;

  Ok(photo.id)
}

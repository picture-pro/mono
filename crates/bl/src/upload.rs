use artifact::Artifact;
use bytes::Bytes;
use clients::surreal::SurrealRootClient;
use color_eyre::eyre::Result;
use core_types::{
  Photo, PhotoArtifacts, PhotoGroup, PhotoGroupStatus, PrivateArtifact,
  PublicArtifact, Ulid,
};
use serde::{Deserialize, Serialize};
use surrealdb::opt::PatchOp;
use tracing::instrument;

use crate::model_ext::ModelExt;

#[derive(Clone, Debug, Deserialize, Serialize, thiserror::Error)]
pub enum PhotoUploadError {
  #[error("Failed to load original image: {0}")]
  InvalidImage(String),
  #[error("Failed to create artifact: {0}")]
  ArtifactCreationError(String),
  #[error("Surreal error: {0}")]
  DBError(String),
}

fn thumbnail_size(aspect_ratio: f32) -> (u32, u32) {
  if aspect_ratio > 1.0 {
    (200, (200.0 / aspect_ratio) as u32)
  } else {
    ((200.0 * aspect_ratio) as u32, 200)
  }
}

#[cfg(feature = "ssr")]
#[instrument(skip(original_bytes))]
pub async fn upload_single_photo(
  user_id: core_types::UserRecordId,
  original_bytes: Bytes,
  status: PhotoGroupStatus,
) -> Result<PhotoGroup, PhotoUploadError> {
  // load the original and make sure it's valid
  let original_image =
    image::load_from_memory(&original_bytes).map_err(|e| {
      PhotoUploadError::InvalidImage(format!(
        "Failed to parse original image: {e:?}"
      ))
    })?;

  // upload the original as an artifact
  let original_artifact = PrivateArtifact::new(Some(original_bytes));
  original_artifact.upload_and_push().await.map_err(|e| {
    PhotoUploadError::ArtifactCreationError(format!(
      "Failed to create original artifact: {e:?}"
    ))
  })?;

  // create a thumbnail image
  let aspect_ratio =
    original_image.width() as f32 / original_image.height() as f32;
  let thumbnail_size = thumbnail_size(aspect_ratio);

  let thumbnail_image = original_image.resize_exact(
    thumbnail_size.0,
    thumbnail_size.1,
    image::imageops::FilterType::Lanczos3,
  );

  // encode as jpeg to bytes
  let mut thumbnail_bytes = Vec::new();
  let encoder = image::codecs::jpeg::JpegEncoder::new(&mut thumbnail_bytes);
  thumbnail_image.write_with_encoder(encoder).map_err(|e| {
    PhotoUploadError::InvalidImage(format!(
      "Failed to encode thumbnail image: {e:?}"
    ))
  })?;

  let thumbnail_bytes: Bytes = thumbnail_bytes.into();
  let thumbnail_artifact = PublicArtifact::new(Some(thumbnail_bytes));
  thumbnail_artifact.upload_and_push().await.map_err(|e| {
    PhotoUploadError::ArtifactCreationError(format!(
      "Failed to create thumbnail artifact: {e:?}"
    ))
  })?;

  // create a photo and upload it to surreal
  let photo = Photo {
    id:        core_types::PhotoRecordId(Ulid::new()),
    // this is set to nil because we don't have a group yet
    group:     core_types::PhotoGroupRecordId(Ulid::nil()),
    artifacts: PhotoArtifacts {
      original:  core_types::PrivateImageArtifact {
        artifact_id: original_artifact.id,
        size:        (original_image.width(), original_image.height()),
      },
      thumbnail: core_types::PublicImageArtifact {
        artifact_id: thumbnail_artifact.id,
        size:        (thumbnail_image.width(), thumbnail_image.height()),
      },
    },
    meta:      Default::default(),
  };

  let client = SurrealRootClient::new().await.map_err(|_| {
    PhotoUploadError::DBError("Failed to create surreal client".to_string())
  })?;
  client.use_ns("main").use_db("main").await.map_err(|e| {
    PhotoUploadError::DBError(format!(
      "Failed to use surreal namespace/database: {e}"
    ))
  })?;

  photo.create(&client).await.map_err(|e| {
    PhotoUploadError::DBError(format!("Failed to create photo in surreal: {e}"))
  })?;

  // create a photo group and upload it to surreal
  let group = PhotoGroup {
    id: core_types::PhotoGroupRecordId(Ulid::new()),
    owner: user_id,
    photographer: user_id,
    photos: vec![photo.id],
    status,
    meta: Default::default(),
  };

  group.create(&client).await.map_err(|e| {
    PhotoUploadError::DBError(format!(
      "Failed to create photo group in surreal: {e}"
    ))
  })?;

  Photo::patch(photo.id, &client, PatchOp::replace("group", group.id))
    .await
    .map_err(|e| {
      PhotoUploadError::DBError(format!(
        "Failed to update photo with group in surreal: {e}"
      ))
    })?;

  Ok(group.clone())
}

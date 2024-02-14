use artifact::{Artifact, PrivateArtifact, PublicArtifact};
use bytes::Bytes;
use clients::surreal::SurrealRootClient;
use color_eyre::eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Photo {
  pub id:           ulid::Ulid,
  pub photographer: Thing,
  pub owner:        Thing,
  pub artifacts:    PhotoArtifacts,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoArtifacts {
  pub original:  PrivateArtifact,
  pub thumbnail: PublicArtifact,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoGroup {
  pub id:     ulid::Ulid,
  pub photos: Vec<Thing>,
  pub public: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PhotoGroupUploadMeta {
  pub public: bool,
}

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

pub async fn upload_single_photo(
  user_id: Thing,
  original_bytes: Bytes,
  group_meta: PhotoGroupUploadMeta,
) -> Result<PhotoGroup, PhotoUploadError> {
  let original_image =
    image::load_from_memory(&original_bytes).map_err(|e| {
      PhotoUploadError::InvalidImage(format!(
        "Failed to parse original image: {e}"
      ))
    })?;

  let original_artifact = PrivateArtifact::new(Some(original_bytes));
  original_artifact.upload_and_push().await.map_err(|e| {
    PhotoUploadError::ArtifactCreationError(format!(
      "Failed to create original artifact: {e}"
    ))
  })?;

  let aspect_ratio =
    original_image.width() as f32 / original_image.height() as f32;
  let thumbnail_size = thumbnail_size(aspect_ratio);

  let thumbnail_image = original_image.resize_exact(
    thumbnail_size.0,
    thumbnail_size.1,
    image::imageops::FilterType::Lanczos3,
  );
  let thumbnail_bytes: Bytes = thumbnail_image.as_bytes().to_vec().into();
  let thumbnail_artifact = PublicArtifact::new(Some(thumbnail_bytes));
  thumbnail_artifact.upload_and_push().await.map_err(|e| {
    PhotoUploadError::ArtifactCreationError(format!(
      "Failed to create thumbnail artifact: {e}"
    ))
  })?;

  let photo = Photo {
    id:           ulid::Ulid::new(),
    photographer: user_id.clone(),
    owner:        user_id,
    artifacts:    PhotoArtifacts {
      original:  original_artifact,
      thumbnail: thumbnail_artifact,
    },
  };

  let client = SurrealRootClient::new().await.map_err(|_| {
    PhotoUploadError::DBError("Failed to create surreal client".to_string())
  })?;
  client.use_ns("main").use_db("main").await.map_err(|_| {
    PhotoUploadError::DBError("Failed to use surreal namespace".to_string())
  })?;

  let photo_thing: Option<Thing> = client
    .create(("photo", photo.id.to_string()))
    .content(photo)
    .await
    .map_err(|_| {
      PhotoUploadError::DBError("Failed to create photo in surreal".to_string())
    })?;

  let photo_thing = photo_thing.ok_or_else(|| {
    PhotoUploadError::DBError("Failed to create photo in surreal".to_string())
  })?;

  let group = PhotoGroup {
    id:     ulid::Ulid::new(),
    photos: vec![photo_thing],
    public: group_meta.public,
  };

  let group_thing: Option<Thing> = client
    .create(("photo_group", group.id.to_string()))
    .content(group.clone())
    .await
    .map_err(|_| {
      PhotoUploadError::DBError(
        "Failed to create photo group in surreal".to_string(),
      )
    })?;

  let _group_thing = group_thing.ok_or_else(|| {
    PhotoUploadError::DBError(
      "Failed to create photo group in surreal".to_string(),
    )
  })?;

  Ok(group)
}

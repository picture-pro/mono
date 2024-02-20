use artifact::Artifact;
use clients::surreal::SurrealRootClient;
use color_eyre::eyre::{Context, OptionExt, Result};
use core_types::{CoreId, PhotoGroup};
use tracing::instrument;

use crate::model_ext::ModelExt;

#[instrument]
pub async fn fetch_user_owned_photo_groups(
  user_id: core_types::UserRecordId,
) -> Result<Vec<PhotoGroup>> {
  let client = SurrealRootClient::new()
    .await
    .wrap_err("Failed to create surreal client")?;
  client
    .use_ns("main")
    .use_db("main")
    .await
    .wrap_err("Failed to use surreal namespace/database")?;

  let mut result = client
    .query(format!(
      "SELECT * FROM {} WHERE owner = $user_id",
      core_types::PhotoGroupRecordId::TABLE
    ))
    .bind(("user_id", user_id.0.to_string()))
    .await
    .wrap_err("Failed to query photo groups")?;

  let groups: Vec<PhotoGroup> = result
    .take(0)
    .wrap_err("Failed to take result of photo groups query")?;

  Ok(groups)
}

#[instrument]
pub async fn fetch_user(
  user_id: core_types::UserRecordId,
) -> Result<Option<core_types::PublicUser>> {
  let client = SurrealRootClient::new()
    .await
    .wrap_err("Failed to create surreal client")?;
  client
    .use_ns("main")
    .use_db("main")
    .await
    .wrap_err("Failed to use surreal namespace/database")?;

  core_types::User::fetch(user_id, &client)
    .await
    .map(|user| user.map(|user| user.into()))
    .wrap_err("Failed to fetch user")
}

#[instrument]
pub async fn fetch_photo_thumbnail(
  photo_id: core_types::PhotoRecordId,
) -> Result<core_types::PhotoThumbnailDisplayParams> {
  // prep the surreal client
  let client = clients::surreal::SurrealRootClient::new().await?;
  client
    .use_ns("main")
    .use_db("main")
    .await
    .wrap_err("Failed to start surreal client")?;

  let photo = core_types::Photo::fetch(photo_id, &client)
    .await
    .wrap_err("Failed to select photo")?
    .ok_or_eyre("Failed to select photo: photo is missing")?;

  // select the thumbnail artifact
  let mut thumbnail_artifact: core_types::PublicArtifact = (*client)
    .select(photo.artifacts.thumbnail.artifact_id)
    .await
    .wrap_err("Failed to select thumbnail artifact")?
    .ok_or_eyre("Failed to select thumbnail artifact: artifact is missing")?;

  // download the thumbnail artifact and get the content
  thumbnail_artifact
    .download()
    .await
    .wrap_err("Failed to download photo thumbnail artifact")?;
  let thumbnail_artifact_content = thumbnail_artifact
    .contents
    .ok_or_eyre("Thumbnail artifact is missing contents")?;

  use base64::prelude::*;
  let data = BASE64_STANDARD.encode(&thumbnail_artifact_content);

  Ok(core_types::PhotoThumbnailDisplayParams {
    data,
    alt: "Thumbnail".to_string(),
    size: photo.artifacts.thumbnail.size,
  })
}

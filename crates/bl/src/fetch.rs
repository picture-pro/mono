use core_types::{PhotoGroup, PhotoThumbnailDisplayParams};
use leptos::{server, ServerFnError};

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_user_owned_photo_groups(
  user_id: core_types::UserRecordId,
) -> Result<Vec<PhotoGroup>, ServerFnError> {
  use clients::surreal::SurrealRootClient;
  use color_eyre::eyre::{Report, WrapErr};
  use core_types::CoreId;

  async move {
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
  .await
  .map_err(|e: Report| {
    let error = e.to_string();
    tracing::error!("Failed to fetch user owned photo groups: {}", error);
    ServerFnError::new(error)
  })
}

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_photo_group(
  photo_group_id: core_types::PhotoGroupRecordId,
) -> Result<Option<PhotoGroup>, ServerFnError> {
  use clients::surreal::SurrealRootClient;
  use color_eyre::eyre::{Report, WrapErr};

  use crate::model_ext::ModelExt;

  async move {
    let client = SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;
    client
      .use_ns("main")
      .use_db("main")
      .await
      .wrap_err("Failed to use surreal namespace/database")?;

    let group = core_types::PhotoGroup::fetch(photo_group_id, &client)
      .await
      .wrap_err("Failed to fetch photo group")?;

    Ok(group)
  }
  .await
  .map_err(|e: Report| {
    let error = e.to_string();
    tracing::error!("Failed to fetch photo group: {}", error);
    ServerFnError::new(error)
  })
}

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_user(
  user_id: core_types::UserRecordId,
) -> Result<Option<core_types::PublicUser>, ServerFnError> {
  use clients::surreal::SurrealRootClient;
  use color_eyre::eyre::{Report, WrapErr};

  use crate::model_ext::ModelExt;

  async move {
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
  .await
  .map_err(|e: Report| {
    let error = e.to_string();
    tracing::error!("Failed to fetch user: {}", error);
    ServerFnError::new(error)
  })
}

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_photo_thumbnail(
  photo_id: core_types::PhotoRecordId,
) -> Result<Option<PhotoThumbnailDisplayParams>, ServerFnError> {
  use artifact::Artifact;
  use clients::surreal::SurrealRootClient;
  use color_eyre::eyre::{OptionExt, Report, WrapErr};

  use crate::model_ext::ModelExt;

  async move {
    // prep the surreal client
    let client = SurrealRootClient::new().await?;
    client
      .use_ns("main")
      .use_db("main")
      .await
      .wrap_err("Failed to start surreal client")?;

    let photo = core_types::Photo::fetch(photo_id, &client)
      .await
      .wrap_err("Failed to select photo")?;

    let Some(photo) = photo else {
      return Ok(None);
    };

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

    Ok(Some(PhotoThumbnailDisplayParams {
      data,
      alt: "Thumbnail".to_string(),
      size: photo.artifacts.thumbnail.size,
    }))
  }
  .await
  .map_err(|e: Report| {
    let error = e.to_string();
    tracing::error!("Failed to fetch photo thumbnail: {}", error);
    ServerFnError::new(error)
  })
}

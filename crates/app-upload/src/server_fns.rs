use leptos::prelude::*;
use models::{ArtifactRecordId, PhotoGroupConfig, PhotoGroupRecordId};
use serde::{Deserialize, Serialize};

/// The possible errors of [`create_photo_group_from_artifacts`].
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum CreatePhotoGroupFromArtifactsError {
  /// The request was unauthenticated.
  #[error("The request was unauthenticated.")]
  Unauthenticated,
  /// An internal error occurred.
  #[error("An internal error occurred")]
  InternalError,
  /// The artifact didn't exist.
  #[error("Missing artifact: {0}")]
  MissingArtifact(ArtifactRecordId),
}

/// Create a [`PhotoGroup`](models::PhotoGroup) from a list of
/// [`Artifact`](models::Artifact)s and a [`PhotoGroupConfig`].
#[server]
pub async fn create_photo_group_from_artifacts(
  /// The artifact IDs to use.
  artifact_ids: Vec<ArtifactRecordId>,
  /// The photo group config to use.
  config: PhotoGroupConfig,
) -> Result<
  Result<PhotoGroupRecordId, CreatePhotoGroupFromArtifactsError>,
  ServerFnError,
> {
  use models::{
    AuthStatus, PhotoArtifacts, PhotoCreateRequest, PhotoGroupCreateRequest,
  };
  use prime_domain::PrimeDomainService;

  let auth_session: AuthStatus = expect_context();
  let Some(user) = auth_session.0 else {
    return Ok(Err(CreatePhotoGroupFromArtifactsError::Unauthenticated));
  };

  let pd: PrimeDomainService = expect_context();

  let artifacts =
    futures::future::join_all(artifact_ids.into_iter().map(|ar| {
      tokio::spawn({
        let pd = pd.clone();
        async move { (ar, pd.fetch_artifact(ar).await) }
      })
    }))
    .await;

  // we keep the ID with the value the whole way.
  // here we first get rid of the join errors, then the fetch errors, and then
  // we throw if the artifact doesn't exist.
  let artifacts = artifacts
    .into_iter()
    .try_collect::<Vec<_>>()
    .map_err(|e| {
      tracing::error!("failed to join artifact fetching tasks: {e}");
      CreatePhotoGroupFromArtifactsError::InternalError
    })?
    .into_iter()
    .map(|(ar, a)| a.map(|a| (ar, a)))
    .try_collect::<Vec<_>>()
    .map_err(|e| {
      tracing::error!("failed to fetch artifact: {e}");
      CreatePhotoGroupFromArtifactsError::InternalError
    })?
    .into_iter()
    .map(|(ar, a)| {
      a.ok_or(CreatePhotoGroupFromArtifactsError::MissingArtifact(ar))
    })
    .try_collect::<Vec<_>>()?;

  let mut photos = Vec::with_capacity(artifacts.len());
  for artifact in artifacts {
    let photo_artifacts = PhotoArtifacts {
      original:  artifact.id,
      thumbnail: artifact.id,
    };
    let photo_create_request = PhotoCreateRequest {
      artifacts: photo_artifacts,
    };
    let photo = pd.create_photo(photo_create_request).await.map_err(|e| {
      tracing::error!("failed to create photo: {e}");
      CreatePhotoGroupFromArtifactsError::InternalError
    })?;
    photos.push(photo.id);
  }

  let photo_group_create_request = PhotoGroupCreateRequest {
    owner: user.id,
    photos,
    config,
  };
  let photo_group = pd
    .create_photo_group(photo_group_create_request)
    .await
    .map_err(|e| {
      tracing::error!("failed to create photo group: {e}");
      CreatePhotoGroupFromArtifactsError::InternalError
    })?;

  Ok(Ok(photo_group.id))
}

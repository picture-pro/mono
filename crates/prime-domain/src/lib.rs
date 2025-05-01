//! Provides prime-domain Services, the entry points for domain-specific
//! business logic.

#![feature(iterator_try_collect)]

pub use hex;
use hex::health::{self, HealthAware};
use miette::{Context, IntoDiagnostic, Result};
pub use models;
use models::{
  Artifact, ArtifactMimeType, ArtifactRecordId, BaseUrl, Photo, PhotoArtifacts,
  PhotoCreateRequest, PhotoGroup, PhotoGroupConfig, PhotoGroupCreateRequest,
  PhotoGroupRecordId, PhotoRecordId, UserRecordId,
};
use qr::QrCodeGenerator;
pub use repos;
use repos::{
  belt::Belt, ArtifactRepository, CreateArtifactError, CreateModelError,
  FetchModelByIndexError, FetchModelError, ImageRepository,
  PhotoGroupRepository, PhotoRepository, ReadArtifactError,
};
use tracing::instrument;

/// The prime domain service.
#[derive(Debug, Clone)]
pub struct PrimeDomainService {
  artifact_repo:    ArtifactRepository,
  image_repo:       ImageRepository,
  photo_group_repo: PhotoGroupRepository,
  photo_repo:       PhotoRepository,
  qr_generator:     QrCodeGenerator,
}

#[async_trait::async_trait]
impl health::HealthReporter for PrimeDomainService {
  fn name(&self) -> &'static str { stringify!(PrimeDomainService) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![
      self.photo_repo.health_report(),
      self.artifact_repo.health_report(),
    ])
    .await
    .into()
  }
}

/// The possible errors of [`create_photo_group_from_artifacts`].
#[derive(Debug, thiserror::Error)]
pub enum CreatePhotoGroupFromArtifactsError {
  /// Failed to fetch an artifact.
  #[error("failed to fetch artifact: {0}")]
  ArtifactFetchingFailed(FetchModelError),
  /// The artifact didn't exist.
  #[error("missing artifact: {0}")]
  MissingArtifact(ArtifactRecordId),
  /// Failed to create a photo.
  #[error("failed to create a photo: {0}")]
  PhotoCreatingFailed(CreateModelError),
  /// An internal error occurred.
  #[error("an internal error occurred")]
  InternalError,
}

impl PrimeDomainService {
  /// Create a new [`PrimeDomainService`].
  pub fn new(
    artifact_repo: ArtifactRepository,
    image_repo: ImageRepository,
    photo_repo: PhotoRepository,
    photo_group_repo: PhotoGroupRepository,
  ) -> Self {
    Self {
      artifact_repo,
      image_repo,
      photo_repo,
      photo_group_repo,
      qr_generator: QrCodeGenerator::new(),
    }
  }

  /// Create an [`Artifact`].
  #[instrument(skip(self))]
  pub async fn create_artifact(
    &self,
    data: Belt,
    originator: UserRecordId,
    stated_mime_type: Option<ArtifactMimeType>,
  ) -> Result<Artifact, CreateArtifactError> {
    self
      .artifact_repo
      .create_artifact(data, originator, stated_mime_type)
      .await
  }

  /// Create a [`Photo`].
  #[instrument(skip(self))]
  pub async fn create_photo(
    &self,
    input: PhotoCreateRequest,
  ) -> Result<Photo, CreateModelError> {
    self.photo_repo.create_photo(input).await
  }

  /// Create a [`PhotoGroup`].
  #[instrument(skip(self))]
  async fn create_photo_group(
    &self,
    input: PhotoGroupCreateRequest,
  ) -> Result<PhotoGroup, CreateModelError> {
    self.photo_group_repo.create_photo_group(input).await
  }

  /// Create a [`PhotoGroup`] from a set of [`Artifact`]s.
  #[instrument(skip(self))]
  pub async fn create_photo_group_from_artifacts(
    &self,
    artifact_ids: Vec<ArtifactRecordId>,
    config: PhotoGroupConfig,
    user: UserRecordId,
  ) -> Result<PhotoGroupRecordId, CreatePhotoGroupFromArtifactsError> {
    let artifacts =
      futures::future::join_all(artifact_ids.into_iter().map(|ar| {
        tokio::spawn({
          let pd = self.clone();
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
      .map_err(CreatePhotoGroupFromArtifactsError::ArtifactFetchingFailed)?
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
      let photo = self
        .create_photo(photo_create_request)
        .await
        .map_err(CreatePhotoGroupFromArtifactsError::PhotoCreatingFailed)?;
      photos.push(photo.id);
    }

    let photo_group_create_request = PhotoGroupCreateRequest {
      owner: user,
      photos,
      config,
    };
    let photo_group = self
      .create_photo_group(photo_group_create_request)
      .await
      .map_err(|e| {
        tracing::error!("failed to create photo group: {e}");
        CreatePhotoGroupFromArtifactsError::InternalError
      })?;

    Ok(photo_group.id)
  }

  /// Fetch an [`Artifact`].
  #[instrument(skip(self))]
  pub async fn fetch_artifact(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Artifact>, FetchModelError> {
    self.artifact_repo.fetch_artifact_by_id(id).await
  }

  /// Read the data of an [`Artifact`].
  #[instrument(skip(self))]
  pub async fn read_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<(Belt, Option<ArtifactMimeType>)>, ReadArtifactError> {
    self.artifact_repo.read_artifact_by_id(id).await
  }

  /// Fetch a [`Photo`].
  #[instrument(skip(self))]
  pub async fn fetch_photo(
    &self,
    id: PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError> {
    self.photo_repo.fetch_photo_by_id(id).await
  }

  /// Fetch a [`PhotoGroup`].
  #[instrument(skip(self))]
  pub async fn fetch_photo_group(
    &self,
    id: PhotoGroupRecordId,
  ) -> Result<Option<PhotoGroup>, FetchModelError> {
    self.photo_group_repo.fetch_photo_group_by_id(id).await
  }

  /// Fetch [`PhotoGroup`]s by user.
  #[instrument(skip(self))]
  pub async fn fetch_photo_groups_by_user(
    &self,
    owner: UserRecordId,
  ) -> Result<Vec<PhotoGroup>, FetchModelByIndexError> {
    self
      .photo_group_repo
      .fetch_photo_groups_by_user(owner)
      .await
  }

  /// Generate a QR code for a [`PhotoGroup`].
  #[instrument(skip(self))]
  pub fn generate_photo_group_qr(
    &self,
    base_url: &BaseUrl,
    id: PhotoGroupRecordId,
  ) -> Result<String> {
    self
      .qr_generator
      .generate_photo_group_link(base_url, id)
      .into_diagnostic()
      .context("failed to generate photo group qr code")
  }
}

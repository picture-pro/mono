//! Provides prime-domain Services, the entry points for domain-specific
//! business logic.

pub use hex;
use hex::health::{self, HealthAware};
use miette::{Context, IntoDiagnostic, Result};
pub use models;
use models::{
  Artifact, ArtifactMimeType, ArtifactRecordId, BaseUrl, Photo,
  PhotoCreateRequest, PhotoGroup, PhotoGroupCreateRequest, PhotoGroupRecordId,
  PhotoRecordId, UserRecordId,
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
  photo_repo:       PhotoRepository,
  photo_group_repo: PhotoGroupRepository,
  artifact_repo:    ArtifactRepository,
  image_repo:       ImageRepository,
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

impl PrimeDomainService {
  /// Create a new [`PrimeDomainService`].
  pub fn new(
    photo_repo: PhotoRepository,
    photo_group_repo: PhotoGroupRepository,
    image_repo: ImageRepository,
    artifact_repo: ArtifactRepository,
  ) -> Self {
    Self {
      photo_repo,
      photo_group_repo,
      artifact_repo,
      image_repo,
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
  pub async fn create_photo_group(
    &self,
    input: PhotoGroupCreateRequest,
  ) -> Result<PhotoGroup, CreateModelError> {
    self.photo_group_repo.create_photo_group(input).await
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

//! Provides prime-domain Services, the entry points for domain-specific
//! business logic.

pub use hex;
use hex::health::{self, HealthAware};
use miette::Result;
pub use models;
use models::{
  Artifact, ArtifactRecordId, Photo, PhotoCreateRequest, PhotoGroup,
  PhotoGroupCreateRequest, UserRecordId,
};
pub use repos;
use repos::{
  belt::Belt, ArtifactRepository, CreateArtifactError, CreateModelError,
  FetchModelByIndexError, FetchModelError, PhotoGroupRepository,
  PhotoRepository,
};
use tracing::instrument;

/// The prime domain service.
#[derive(Debug, Clone)]
pub struct PrimeDomainService {
  photo_repo:       PhotoRepository,
  photo_group_repo: PhotoGroupRepository,
  artifact_repo:    ArtifactRepository,
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
    artifact_repo: ArtifactRepository,
  ) -> Self {
    Self {
      photo_repo,
      photo_group_repo,
      artifact_repo,
    }
  }

  /// Create an [`Artifact`].
  #[instrument(skip(self))]
  pub async fn create_artifact(
    &self,
    data: Belt,
    originator: UserRecordId,
  ) -> Result<Artifact, CreateArtifactError> {
    self.artifact_repo.create_artifact(data, originator).await
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
}

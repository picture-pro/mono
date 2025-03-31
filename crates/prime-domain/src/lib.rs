//! Provides prime-domain Services, the entry points for domain-specific
//! business logic.

pub use hex;
use hex::health::{self, HealthAware};
use miette::Result;
pub use models;
use models::{Artifact, UserRecordId};
pub use repos;
use repos::{
  belt::Belt, ArtifactRepository, CreateArtifactError, PhotoRepository,
};
use tracing::instrument;

/// The prime domain service.
#[derive(Debug, Clone)]
pub struct PrimeDomainService {
  photo_repo:    PhotoRepository,
  artifact_repo: ArtifactRepository,
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
    artifact_repo: ArtifactRepository,
  ) -> Self {
    Self {
      photo_repo,
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
}

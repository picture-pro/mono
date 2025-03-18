use hex::health;
use miette::Result;
use models::{
  Artifact, Photo, PhotoCreateRequest, PhotoRecordId, UserRecordId,
};
use repos::{belt::Belt, CreateArtifactError, FetchModelError};

use crate::PrimeDomainService;

/// The canonical implementation of [`PrimeDomainService`].
pub struct PrimeDomainServiceCanonical<
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
  AR: repos::ArtifactRepository,
> {
  photo_repo:    PR,
  artifact_repo: AR,
}

impl<PR, AR> PrimeDomainServiceCanonical<PR, AR>
where
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
  AR: repos::ArtifactRepository,
{
  /// Creates a new [`PrimeDomainServiceCanonical`] with the given photo
  /// repository.
  pub fn new(photo_repo: PR, artifact_repo: AR) -> Self {
    Self {
      photo_repo,
      artifact_repo,
    }
  }
}

#[async_trait::async_trait]
impl<PR, AR> PrimeDomainService for PrimeDomainServiceCanonical<PR, AR>
where
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
  AR: repos::ArtifactRepository,
{
  async fn fetch_photo_by_id(
    &self,
    id: PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError> {
    self.photo_repo.fetch_model_by_id(id).await
  }

  async fn enumerate_photos(&self) -> Result<Vec<Photo>> {
    self.photo_repo.enumerate_models().await
  }

  async fn create_artifact(
    &self,
    data: Belt,
    originator: UserRecordId,
  ) -> Result<Artifact, CreateArtifactError> {
    self.artifact_repo.create_artifact(data, originator).await
  }
}

#[async_trait::async_trait]
impl<PR, AR> health::HealthReporter for PrimeDomainServiceCanonical<PR, AR>
where
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
  AR: repos::ArtifactRepository,
{
  fn name(&self) -> &'static str { stringify!(PrimeDomainServiceCanonical) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![
      self.photo_repo.health_report(),
      self.artifact_repo.health_report(),
    ])
    .await
    .into()
  }
}

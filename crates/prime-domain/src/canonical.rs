use hex::health;
use miette::Result;
use models::{Photo, PhotoCreateRequest, PhotoRecordId};
use repos::FetchModelError;

use crate::PrimeDomainService;

/// The canonical implementation of [`PrimeDomainService`].
pub struct PrimeDomainServiceCanonical<
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
> {
  photo_repo: PR,
}

impl<PR> PrimeDomainServiceCanonical<PR>
where
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
{
  /// Creates a new [`PrimeDomainServiceCanonical`] with the given photo
  /// repository.
  pub fn new(photo_repo: PR) -> Self { Self { photo_repo } }
}

#[async_trait::async_trait]
impl<PR> PrimeDomainService for PrimeDomainServiceCanonical<PR>
where
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
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
}

#[async_trait::async_trait]
impl<PR> health::HealthReporter for PrimeDomainServiceCanonical<PR>
where
  PR: repos::ModelRepository<
    Model = Photo,
    ModelCreateRequest = PhotoCreateRequest,
    CreateError = repos::CreateModelError,
  >,
{
  fn name(&self) -> &'static str { stringify!(PrimeDomainServiceCanonical) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .photo_repo
      .health_report()])
    .await
    .into()
  }
}

use core::fmt;
use std::sync::Arc;

use db::{CreateModelError, FetchModelError};
use hex::health;
use miette::Result;
use models::{Photo, PhotoCreateRequest};
use tracing::instrument;

use crate::ModelRepositoryLike;

#[derive(Clone)]
pub struct PhotoRepository {
  model_repo: Arc<
    dyn ModelRepositoryLike<
      Model = Photo,
      ModelCreateRequest = PhotoCreateRequest,
      CreateError = CreateModelError,
    >,
  >,
}

impl fmt::Debug for PhotoRepository {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("PhotoRepository")
      .field(
        "model_repo",
        &stringify!(
          Arc<
            dyn ModelRepositoryLike<
              Model = Photo,
              ModelCreateRequest = PhotoCreateRequest,
              CreateError = CreateModelError,
            >,
          >
        ),
      )
      .finish()
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for PhotoRepository {
  fn name(&self) -> &'static str { stringify!(PhotoRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .model_repo
      .health_report()])
    .await
    .into()
  }
}

impl PhotoRepository {
  #[instrument(skip(self))]
  pub async fn create_photo(
    &self,
    input: models::PhotoCreateRequest,
  ) -> Result<Photo, CreateModelError> {
    self.model_repo.create_model(input).await
  }

  #[instrument(skip(self))]
  pub async fn fetch_photo_by_id(
    &self,
    id: models::PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError> {
    self.model_repo.fetch_model_by_id(id).await
  }

  #[instrument(skip(self))]
  pub async fn enumerate_photos(&self) -> Result<Vec<Photo>> {
    self.model_repo.enumerate_models().await
  }
}

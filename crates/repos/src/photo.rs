use core::fmt;
use std::sync::Arc;

use db::{CreateModelError, Database, FetchModelError};
use hex::health;
use miette::Result;
use models::Photo;
use tracing::instrument;

use crate::{base::BaseRepository, ModelRepositoryLike};

/// Stores and retrieves [`Photo`]s.
#[derive(Clone)]
pub struct PhotoRepository {
  model_repo: Arc<
    dyn ModelRepositoryLike<
      Model = Photo,
      ModelCreateRequest = Photo,
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
              ModelCreateRequest = Photo,
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
  /// Create a new [`PhotoRepository`].
  pub fn new(
    model_repo: Arc<
      dyn ModelRepositoryLike<
        Model = Photo,
        ModelCreateRequest = Photo,
        CreateError = CreateModelError,
      >,
    >,
  ) -> Self {
    Self { model_repo }
  }

  /// Create a new [`PhotoRepository`], backed by `BaseRepository`.
  pub fn new_from_base(db: Database<Photo>) -> Self {
    Self::new(Arc::new(BaseRepository::new(db)))
  }

  /// Create a [`Photo`] model.
  #[instrument(skip(self))]
  pub async fn create_photo(
    &self,
    input: models::PhotoCreateRequest,
  ) -> Result<Photo, CreateModelError> {
    self.model_repo.create_model(input.into()).await
  }

  /// Fetch a [`Photo`] by id.
  #[instrument(skip(self))]
  pub async fn fetch_photo_by_id(
    &self,
    id: models::PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError> {
    self.model_repo.fetch_model_by_id(id).await
  }

  /// Produce a list of all [`Photo`]s.
  #[instrument(skip(self))]
  pub async fn enumerate_photos(&self) -> Result<Vec<Photo>> {
    self.model_repo.enumerate_models().await
  }
}

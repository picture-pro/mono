use core::fmt;
use std::sync::Arc;

use db::{CreateModelError, Database, FetchModelError};
use hex::health;
use miette::Result;
use models::PhotoGroup;
use tracing::instrument;

use crate::{base::BaseRepository, ModelRepositoryLike};

/// Stores and retrieves [`PhotoGroup`]s.
#[derive(Clone)]
pub struct PhotoGroupRepository {
  model_repo: Arc<
    dyn ModelRepositoryLike<
      Model = PhotoGroup,
      ModelCreateRequest = PhotoGroup,
      CreateError = CreateModelError,
    >,
  >,
}

impl fmt::Debug for PhotoGroupRepository {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("PhotoGroupRepository")
      .field(
        "model_repo",
        &stringify!(
          Arc<
            dyn ModelRepositoryLike<
              Model = PhotoGroup,
              ModelCreateRequest = PhotoGroup,
              CreateError = CreateModelError,
            >,
          >
        ),
      )
      .finish()
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for PhotoGroupRepository {
  fn name(&self) -> &'static str { stringify!(PhotoGroupRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .model_repo
      .health_report()])
    .await
    .into()
  }
}

impl PhotoGroupRepository {
  /// Create a new [`PhotoGroupRepository`].
  pub fn new(
    model_repo: Arc<
      dyn ModelRepositoryLike<
        Model = PhotoGroup,
        ModelCreateRequest = PhotoGroup,
        CreateError = CreateModelError,
      >,
    >,
  ) -> Self {
    Self { model_repo }
  }

  /// Create a new [`PhotoGroupRepository`], backed by `BaseRepository`.
  pub fn new_from_base(db: Database<PhotoGroup>) -> Self {
    Self::new(Arc::new(BaseRepository::new(db)))
  }

  /// Create a [`PhotoGroup`] model.
  #[instrument(skip(self))]
  pub async fn create_photo_group(
    &self,
    input: models::PhotoGroupCreateRequest,
  ) -> Result<PhotoGroup, CreateModelError> {
    self.model_repo.create_model(input.into()).await
  }

  /// Fetch a [`PhotoGroup`] by id.
  #[instrument(skip(self))]
  pub async fn fetch_photo_group_by_id(
    &self,
    id: models::PhotoGroupRecordId,
  ) -> Result<Option<PhotoGroup>, FetchModelError> {
    self.model_repo.fetch_model_by_id(id).await
  }

  /// Produce a list of all [`PhotoGroup`]s.
  #[instrument(skip(self))]
  pub async fn enumerate_photo_groups(&self) -> Result<Vec<PhotoGroup>> {
    self.model_repo.enumerate_models().await
  }
}

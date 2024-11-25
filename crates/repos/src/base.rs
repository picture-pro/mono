use std::marker::PhantomData;

pub use db::CreateModelError;
pub(crate) use db::{DatabaseAdapter, FetchModelByIndexError, FetchModelError};
use hex::health;
use miette::Result;
use tracing::instrument;

use crate::ModelRepository;

/// Provides a base repository implementation for any model.
pub struct BaseRepository<M: models::Model, DB: DatabaseAdapter> {
  db_adapter: DB,
  _phantom:   PhantomData<M>,
}

impl<M: models::Model, DB: DatabaseAdapter + Clone> Clone
  for BaseRepository<M, DB>
{
  fn clone(&self) -> Self {
    Self {
      db_adapter: self.db_adapter.clone(),
      _phantom:   PhantomData,
    }
  }
}

impl<M: models::Model, DB: DatabaseAdapter> BaseRepository<M, DB> {
  /// Creates a new `BaseRepository` instance.
  pub fn new(db_adapter: DB) -> Self {
    tracing::info!(
      "creating new `BaseRepository<{:?}>` instance",
      M::TABLE_NAME
    );

    Self {
      db_adapter,
      _phantom: PhantomData,
    }
  }
}

#[async_trait::async_trait]
impl<M: models::Model, DB: DatabaseAdapter> health::HealthReporter
  for BaseRepository<M, DB>
{
  fn name(&self) -> &'static str { stringify!(BaseRepository<M, DB>) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.db_adapter.health_report(),
    ))
    .await
    .into()
  }
}

#[async_trait::async_trait]
impl<M: models::Model, DB: DatabaseAdapter> ModelRepository
  for BaseRepository<M, DB>
{
  type Model = M;
  type ModelCreateRequest = M;
  type CreateError = CreateModelError;

  #[instrument(skip(self))]
  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, CreateModelError> {
    self.db_adapter.create_model::<Self::Model>(input).await
  }

  #[instrument(skip(self))]
  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> Result<Option<Self::Model>, FetchModelError> {
    self.db_adapter.fetch_model_by_id(id).await
  }

  #[instrument(skip(self))]
  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: models::EitherSlug,
  ) -> Result<Option<Self::Model>, FetchModelByIndexError> {
    self
      .db_adapter
      .fetch_model_by_index(index_name, index_value)
      .await
  }

  #[instrument(skip(self))]
  async fn enumerate_models(&self) -> Result<Vec<Self::Model>> {
    self.db_adapter.enumerate_models::<Self::Model>().await
  }
}

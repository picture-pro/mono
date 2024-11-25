use std::{fmt::Debug, marker::PhantomData};

pub use db::CreateModelError;
pub(crate) use db::{DatabaseAdapter, FetchModelByIndexError, FetchModelError};
use hex::health;
use miette::Result;
use tracing::instrument;

use crate::ModelRepository;

/// Provides a base repository implementation for any model.
pub struct BaseRepository<
  M: models::Model,
  MCR: Debug + Into<M> + Send + Sync + 'static,
  DB: DatabaseAdapter,
> {
  db_adapter: DB,
  _phantom:   PhantomData<M>,
  _phantom2:  PhantomData<MCR>,
}

impl<
    M: models::Model,
    MCR: Debug + Into<M> + Send + Sync + 'static,
    DB: DatabaseAdapter + Clone,
  > Clone for BaseRepository<M, MCR, DB>
{
  fn clone(&self) -> Self {
    Self {
      db_adapter: self.db_adapter.clone(),
      _phantom:   PhantomData,
      _phantom2:  PhantomData,
    }
  }
}

impl<
    M: models::Model,
    MCR: Debug + Into<M> + Send + Sync + 'static,
    DB: DatabaseAdapter,
  > BaseRepository<M, MCR, DB>
{
  /// Creates a new `BaseRepository` instance.
  pub fn new(db_adapter: DB) -> Self {
    tracing::info!(
      "creating new `BaseRepository<{:?}>` instance",
      M::TABLE_NAME
    );

    Self {
      db_adapter,
      _phantom: PhantomData,
      _phantom2: PhantomData,
    }
  }
}

#[async_trait::async_trait]
impl<
    M: models::Model,
    MCR: Debug + Into<M> + Send + Sync + 'static,
    DB: DatabaseAdapter,
  > health::HealthReporter for BaseRepository<M, MCR, DB>
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
impl<
    M: models::Model,
    MCR: Debug + Into<M> + Send + Sync + 'static,
    DB: DatabaseAdapter,
  > ModelRepository for BaseRepository<M, MCR, DB>
{
  type Model = M;
  type ModelCreateRequest = MCR;
  type CreateError = CreateModelError;

  #[instrument(skip(self))]
  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, CreateModelError> {
    self
      .db_adapter
      .create_model::<Self::Model>(input.into())
      .await
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

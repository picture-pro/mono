use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use db::{FetchModelByIndexError, FetchModelError};
use hex::health;
use models::EitherSlug;
use tokio::sync::Mutex;

use crate::ModelRepository;

/// A mock model repository for testing purposes.
#[derive(Debug)]
pub struct MockModelRepository<
  M: models::Model,
  MCR: Debug + Into<M> + Send + Sync + 'static,
> {
  models:  Mutex<HashMap<models::RecordId<M>, M>>,
  indices: Mutex<HashMap<String, HashMap<EitherSlug, models::RecordId<M>>>>,
  phantom: PhantomData<MCR>,
}

impl<M: models::Model, MCR: Debug + Into<M> + Send + Sync + 'static> Default
  for MockModelRepository<M, MCR>
{
  fn default() -> Self {
    Self {
      models:  Mutex::new(HashMap::new()),
      indices: Mutex::new(HashMap::new()),
      phantom: PhantomData,
    }
  }
}

impl<M: models::Model, MCR: Debug + Into<M> + Send + Sync + 'static>
  MockModelRepository<M, MCR>
{
  /// Creates a new `MockModelRepository` instance.
  pub fn new() -> Self { Self::default() }
}

#[async_trait::async_trait]
impl<M: models::Model, MCR: Debug + Into<M> + Send + Sync + 'static>
  health::HealthReporter for MockModelRepository<M, MCR>
{
  fn name(&self) -> &'static str { stringify!(MockModelRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::ComponentHealth::IntrensicallyUp
  }
}

#[async_trait::async_trait]
impl<M: models::Model, MCR: Debug + Into<M> + Send + Sync + 'static>
  ModelRepository for MockModelRepository<M, MCR>
{
  type Model = M;
  type ModelCreateRequest = MCR;
  type CreateError = std::convert::Infallible;

  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, Self::CreateError> {
    let model = input.into();
    self.models.lock().await.insert(model.id(), model.clone());
    let mut indices = self.indices.lock().await;
    for (index_name, index_getter) in M::UNIQUE_INDICES.iter() {
      let index = indices.entry(index_name.to_string()).or_default();
      let index_value = index_getter(&model);
      index.insert(index_value, model.id());
    }

    Ok(model)
  }

  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> Result<Option<Self::Model>, FetchModelError> {
    Ok(self.models.lock().await.get(&id).cloned())
  }

  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<<Self as ModelRepository>::Model>, FetchModelByIndexError>
  {
    if !M::UNIQUE_INDICES.iter().any(|i| i.0 == index_name) {
      return Err(FetchModelByIndexError::IndexDoesNotExistOnModel {
        index_name: index_name.clone(),
      });
    }

    let mut indices = self.indices.lock().await;
    let index = indices.entry(index_name.clone()).or_default();

    let id = index.get(&index_value);
    if let Some(id) = id {
      Ok(self.models.lock().await.get(id).cloned())
    } else {
      Ok(None)
    }
  }

  async fn enumerate_models(&self) -> miette::Result<Vec<Self::Model>> {
    Ok(self.models.lock().await.values().cloned().collect())
  }
}

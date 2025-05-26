use db::{CreateModelError, Database, FetchModelError};
use hex::health::{self, HealthAware};
use miette::Result;
use models::Photo;
use tracing::instrument;

/// Stores and retrieves [`Photo`]s.
#[derive(Clone, Debug)]
pub struct PhotoRepository {
  db: Database<Photo>,
}

#[async_trait::async_trait]
impl health::HealthReporter for PhotoRepository {
  fn name(&self) -> &'static str { stringify!(PhotoRepository) }

  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self.db.health_report()])
      .await
      .into()
  }
}

impl PhotoRepository {
  /// Create a new [`PhotoRepository`].
  #[must_use]
  pub fn new(model_repo: Database<Photo>) -> Self { Self { db: model_repo } }

  /// Create a [`Photo`] model.
  #[instrument(skip(self))]
  pub async fn create_photo(
    &self,
    input: models::PhotoCreateRequest,
  ) -> Result<Photo, CreateModelError> {
    self.db.create_model(input.into()).await
  }

  /// Fetch a [`Photo`] by id.
  #[instrument(skip(self))]
  pub async fn fetch_photo_by_id(
    &self,
    id: models::PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError> {
    self.db.fetch_model_by_id(id).await
  }

  /// Produce a list of all [`Photo`]s.
  #[instrument(skip(self))]
  pub async fn enumerate_photos(&self) -> Result<Vec<Photo>> {
    self.db.enumerate_models().await
  }
}

use db::{CreateModelError, Database, FetchModelError};
use hex::health::{self, HealthAware};
use miette::Result;
use models::Image;
use tracing::instrument;

/// Stores and retrieves [`Image`]s.
#[derive(Clone, Debug)]
pub struct ImageRepository {
  db: Database<Image>,
}

#[async_trait::async_trait]
impl health::HealthReporter for ImageRepository {
  fn name(&self) -> &'static str { stringify!(ImageRepository) }

  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self.db.health_report()])
      .await
      .into()
  }
}

impl ImageRepository {
  /// Create a new [`ImageRepository`].
  #[must_use]
  pub fn new(model_repo: Database<Image>) -> Self { Self { db: model_repo } }

  /// Create a [`Image`] model.
  #[instrument(skip(self))]
  pub async fn create_image(
    &self,
    input: models::ImageCreateRequest,
  ) -> Result<Image, CreateModelError> {
    self.db.create_model(input.into()).await
  }

  /// Fetch a [`Image`] by id.
  #[instrument(skip(self))]
  pub async fn fetch_image_by_id(
    &self,
    id: models::ImageRecordId,
  ) -> Result<Option<Image>, FetchModelError> {
    self.db.fetch_model_by_id(id).await
  }

  /// Produce a list of all [`Image`]s.
  #[instrument(skip(self))]
  pub async fn enumerate_images(&self) -> Result<Vec<Image>> {
    self.db.enumerate_models().await
  }
}

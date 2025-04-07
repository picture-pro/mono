use db::{CreateModelError, Database, FetchModelByIndexError, FetchModelError};
use hex::health::{self, HealthAware};
use miette::Result;
use models::{EitherSlug, PhotoGroup, StrictSlug, UserRecordId};
use tracing::instrument;

/// Stores and retrieves [`PhotoGroup`]s.
#[derive(Clone, Debug)]
pub struct PhotoGroupRepository {
  db: Database<PhotoGroup>,
}

#[async_trait::async_trait]
impl health::HealthReporter for PhotoGroupRepository {
  fn name(&self) -> &'static str { stringify!(PhotoGroupRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self.db.health_report()])
      .await
      .into()
  }
}

impl PhotoGroupRepository {
  /// Create a new [`PhotoGroupRepository`].
  pub fn new(model_repo: Database<PhotoGroup>) -> Self {
    Self { db: model_repo }
  }

  /// Create a [`PhotoGroup`] model.
  #[instrument(skip(self))]
  pub async fn create_photo_group(
    &self,
    input: models::PhotoGroupCreateRequest,
  ) -> Result<PhotoGroup, CreateModelError> {
    self.db.create_model(input.into()).await
  }

  /// Fetch a [`PhotoGroup`] by id.
  #[instrument(skip(self))]
  pub async fn fetch_photo_group_by_id(
    &self,
    id: models::PhotoGroupRecordId,
  ) -> Result<Option<PhotoGroup>, FetchModelError> {
    self.db.fetch_model_by_id(id).await
  }

  /// Fetch [`PhotoGroup`]s by user.
  pub async fn fetch_photo_groups_by_user(
    &self,
    owner: UserRecordId,
  ) -> Result<Vec<PhotoGroup>, FetchModelByIndexError> {
    self
      .db
      .fetch_model_by_index(
        "owner".to_owned(),
        EitherSlug::Strict(StrictSlug::new(owner.to_string())),
      )
      .await
  }

  /// Produce a list of all [`PhotoGroup`]s.
  #[instrument(skip(self))]
  pub async fn enumerate_photo_groups(&self) -> Result<Vec<PhotoGroup>> {
    self.db.enumerate_models().await
  }
}

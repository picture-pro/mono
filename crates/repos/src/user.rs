use core::fmt;
use std::sync::Arc;

use db::{CreateModelError, FetchModelByIndexError, FetchModelError};
use hex::health;
use miette::Result;
use models::{EitherSlug, LaxSlug, User, UserCreateRequest};
use tracing::instrument;

use crate::ModelRepositoryLike;

/// Stores and retrieves [`User`]s.
#[derive(Clone)]
pub struct UserRepository {
  model_repo: Arc<
    dyn ModelRepositoryLike<
      Model = User,
      ModelCreateRequest = UserCreateRequest,
      CreateError = CreateModelError,
    >,
  >,
}

impl fmt::Debug for UserRepository {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("UserRepository")
      .field(
        "model_repo",
        &stringify!(
          Arc<
            dyn ModelRepositoryLike<
              Model = User,
              ModelCreateRequest = UserCreateRequest,
              CreateError = CreateModelError,
            >,
          >
        ),
      )
      .finish()
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for UserRepository {
  fn name(&self) -> &'static str { stringify!(UserRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .model_repo
      .health_report()])
    .await
    .into()
  }
}

impl UserRepository {
  /// Create a new [`UserRepository`].
  pub fn new(
    model_repo: Arc<
      dyn ModelRepositoryLike<
        Model = User,
        ModelCreateRequest = UserCreateRequest,
        CreateError = CreateModelError,
      >,
    >,
  ) -> Self {
    Self { model_repo }
  }

  /// Create a [`User`] model.
  #[instrument(skip(self))]
  pub async fn create_user(
    &self,
    input: models::UserCreateRequest,
  ) -> Result<User, CreateModelError> {
    self.model_repo.create_model(input).await
  }

  /// Fetch a [`User`] by id.
  #[instrument(skip(self))]
  pub async fn fetch_user_by_id(
    &self,
    id: models::UserRecordId,
  ) -> Result<Option<User>, FetchModelError> {
    self.model_repo.fetch_model_by_id(id).await
  }

  /// Fetch a [`User`] by email.
  #[instrument(skip(self))]
  pub async fn fetch_user_by_email(
    &self,
    email: models::EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    self
      .model_repo
      .fetch_model_by_index(
        "email".to_string(),
        EitherSlug::Lax(LaxSlug::new(email.as_ref())),
      )
      .await
  }
}

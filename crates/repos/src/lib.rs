//! Repositories for use in services.

mod artifact;
mod base;
mod mock;
mod photo;
mod user;

pub use db::{self, CreateModelError, FetchModelByIndexError, FetchModelError};
use hex::Hexagonal;
use miette::Result;
use models::EitherSlug;
pub use storage::{self, belt};

pub use self::{artifact::*, mock::*, photo::*, user::*};

/// Defines a repository interface for models.
#[async_trait::async_trait]
pub trait ModelRepositoryLike: Hexagonal {
  /// The model type.
  type Model: models::Model;
  /// The request type for creating a model.
  type ModelCreateRequest: std::fmt::Debug + Send + Sync + 'static;
  /// The error type for creating a model.
  type CreateError: std::error::Error + Send + Sync + 'static;

  /// Creates a new model.
  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, Self::CreateError>;

  /// Fetches a model by its ID.
  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> Result<Option<Self::Model>, FetchModelError>;

  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's `INDICES` constant.
  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<Self::Model>, FetchModelByIndexError>;

  /// Produces a list of all model IDs.
  async fn enumerate_models(&self) -> Result<Vec<Self::Model>>;
}

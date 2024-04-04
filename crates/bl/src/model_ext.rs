//! Provides [`ModelExt`], an extension of the [`CoreModel`] trait.

use clients::surreal::SurrealRootClient;
use color_eyre::eyre::{OptionExt, Result};
use core_types::{CoreId, CoreModel};

/// Provides [`fetch`](ModelExt::fetch), [`create`](ModelExt::create), and
/// [`patch`](ModelExt::patch) methods for interfacing with Surreal for a given
/// model.
pub trait ModelExt: CoreModel {
  /// Fetches the model from Surreal given its [`CoreModel::Id`].
  fn fetch(
    id: <Self as CoreModel>::Id,
    client: &SurrealRootClient,
  ) -> impl std::future::Future<Output = Result<Option<Self>>> + Send {
    async move {
      let result = client.select(id).await?;
      Ok(result)
    }
  }

  /// Pushes a model instance to Surreal, and returns the pushed version.
  fn create(
    &self,
    client: &SurrealRootClient,
  ) -> impl std::future::Future<Output = Result<Self>> + Send {
    let model = self.clone();

    async move {
      let result: Vec<Self> = client
        .create(<<Self as CoreModel>::Id as CoreId>::TABLE)
        .content(model)
        .await?;
      result
        .into_iter()
        .next()
        .ok_or_eyre("Failed to create model")
    }
  }

  /// Patches a model in Surreal using its [`CoreModel::Id`] and a
  /// [`PatchOp`](surrealdb::opt::PatchOp).
  fn patch(
    id: <Self as CoreModel>::Id,
    client: &SurrealRootClient,
    patch: surrealdb::opt::PatchOp,
  ) -> impl std::future::Future<Output = Result<Self>> + Send {
    async move {
      let result: Option<Self> = client.update(id).patch(patch).await?;
      result.ok_or_eyre("Failed to patch model")
    }
  }
}

impl<T: CoreModel> ModelExt for T {}

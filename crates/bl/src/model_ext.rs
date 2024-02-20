use clients::surreal::SurrealRootClient;
use color_eyre::eyre::{OptionExt, Result};
use core_types::{CoreId, CoreModel};

pub trait ModelExt: core_types::CoreModel {
  fn fetch(
    id: <Self as core_types::CoreModel>::Id,
    client: &SurrealRootClient,
  ) -> impl std::future::Future<Output = Result<Option<Self>>> + Send {
    async move {
      let result = client.select(id).await?;
      Ok(result)
    }
  }

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

  fn update(
    &self,
    client: &SurrealRootClient,
  ) -> impl std::future::Future<Output = Result<Self>> + Send {
    let model = self.clone();

    async move {
      let result: Vec<Self> = client
        .update(<<Self as CoreModel>::Id as CoreId>::TABLE)
        .content(model)
        .await?;
      result
        .into_iter()
        .next()
        .ok_or_eyre("Failed to update model")
    }
  }
}

impl<T: core_types::CoreModel> ModelExt for T {}

use color_eyre::eyre::{OptionExt, Result, WrapErr};
use core_types::NewId;

pub fn object_store_from_env(
  bucket_name: &str,
) -> Result<Box<dyn object_store::ObjectStore>> {
  let object_store = object_store::aws::AmazonS3Builder::from_env()
    .with_region(
      std::env::var("AWS_DEFAULT_REGION")
        .wrap_err("Failed to get AWS region from environment")?,
    )
    .with_bucket_name(bucket_name)
    .build()
    .wrap_err("Failed to create object store")?;

  Ok(Box::new(object_store))
}

pub async fn download_artifact(
  object_store: &dyn object_store::ObjectStore,
  id: &str,
) -> Result<bytes::Bytes> {
  let path = object_store::path::Path::from(id);

  let contents = object_store
    .get(&path)
    .await
    .wrap_err("Failed to download artifact")?;

  contents
    .bytes()
    .await
    .wrap_err("Failed to read bytes of downloaded artifact")
}

pub async fn upload_artifact(
  object_store: &dyn object_store::ObjectStore,
  id: &str,
  contents: bytes::Bytes,
) -> Result<()> {
  let path = object_store::path::Path::from(id);

  object_store
    .put(&path, contents)
    .await
    .wrap_err("Failed to upload artifact")?;

  Ok(())
}

pub async fn push_to_surreal<Id, T>(artifact: T) -> Result<()>
where
  Id: NewId,
  T: serde::Serialize + for<'a> serde::Deserialize<'a> + Clone,
{
  let client = clients::surreal::SurrealRootClient::new()
    .await
    .wrap_err("Failed to create surreal client")?;

  client.use_ns("main").use_db("main").await?;

  let pushed_artifact: Vec<T> = client
    .create(Id::TABLE)
    .content(artifact)
    .await
    .wrap_err("Failed to create artifact in surreal")?;

  let _pushed_artifact = pushed_artifact
    .first()
    .ok_or_eyre("Failed to create artifact in surreal")?;

  Ok(())
}

pub async fn pull_from_surreal<Id, T>(id: Id) -> Result<Box<T>>
where
  Id: NewId,
  T: serde::Serialize + for<'a> serde::Deserialize<'a> + Clone,
{
  let client = clients::surreal::SurrealRootClient::new()
    .await
    .wrap_err("Failed to create surreal client")?;

  client.use_ns("main").use_db("main").await?;
  let artifact: Option<T> = client
    .select((Id::TABLE, id.id_with_brackets()))
    .await
    .wrap_err("Failed to get artifact from surreal")?;

  let artifact = artifact.ok_or_eyre("Artifact does not exist in surreal")?;

  Ok(Box::new(artifact))
}

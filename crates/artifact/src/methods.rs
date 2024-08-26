use color_eyre::eyre::{OptionExt, Result, WrapErr};
use tracing::instrument;

use crate::ObjectStoreGenerator;

pub fn object_store_from_env(
  bucket_name: &str,
) -> Result<Box<dyn object_store::ObjectStore>> {
  let endpoint = format!(
    "https://{}.r2.cloudflarestorage.com",
    std::env::var("R2_ACCOUNT")
      .wrap_err("Failed to get R2 account from environment")?
  );
  tracing::info!("endpoint: {}", endpoint);

  let object_store = object_store::aws::AmazonS3Builder::new()
    // .with_region(
    //   std::env::var("AWS_DEFAULT_REGION")
    //     .wrap_err("Failed to get AWS region from environment")?,
    // )
    .with_endpoint(endpoint)
    .with_bucket_name(bucket_name)
    .with_access_key_id(
      std::env::var("AWS_ACCESS_KEY_ID")
        .wrap_err("Failed to get AWS access key ID from environment")?,
    )
    .with_secret_access_key(
      std::env::var("AWS_SECRET_ACCESS_KEY")
        .wrap_err("Failed to get AWS secret access key from environment")?,
    )
    .build()
    .wrap_err("Failed to create object store")?;

  Ok(Box::new(object_store))
}

/// Get the cache directory for artifacts.
///
/// By default, this is the system's temporary directory. It can be overridden
/// by setting the `TMPDIR` environment variable. If the directory does not
/// exist, it will be created.
fn get_cache_dir() -> Result<std::path::PathBuf> {
  let cache_dir = std::env::temp_dir();
  if !cache_dir.exists() {
    tracing::debug!("creating cache directory");
    std::fs::create_dir_all(&cache_dir)
      .wrap_err("Failed to create cache directory")?;
  }
  Ok(cache_dir)
}
/// Get the path to the cached artifact.
///
/// The parent directory will be created in [`get_cache_dir`] if it does not
/// exist.
fn cache_path(id: &str) -> Result<std::path::PathBuf> {
  get_cache_dir().map(|d| d.join(id))
}

#[instrument(skip(object_store))]
pub async fn download_artifact(
  object_store: ObjectStoreGenerator,
  id: &str,
) -> Result<bytes::Bytes> {
  let cache_path = cache_path(id)?;
  if cache_path.exists() {
    tracing::debug!("using cached artifact instead of downloading");
    let contents = tokio::fs::read(cache_path)
      .await
      .wrap_err("Failed to read cached artifact")?;
    return Ok(contents.into());
  }

  let object_store = object_store()?;
  tracing::debug!("downloading uncached artifact");
  let contents = inner_download_artifact(&*object_store, id).await?;

  tokio::fs::write(&cache_path, &contents)
    .await
    .wrap_err("Failed to write cached artifact")?;

  Ok(contents)
}

#[instrument(skip(object_store))]
async fn inner_download_artifact(
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

#[instrument(skip(object_store, contents))]
pub async fn upload_artifact(
  object_store: ObjectStoreGenerator,
  id: &str,
  contents: bytes::Bytes,
) -> Result<()> {
  let cache_path = cache_path(id)?;
  tokio::fs::write(&cache_path, &contents)
    .await
    .wrap_err("Failed to write cached artifact")?;

  let object_store = object_store()?;
  inner_upload_artifact(&*object_store, id, contents).await
}

#[instrument(skip(object_store, contents))]
async fn inner_upload_artifact(
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

#[instrument(skip(artifact))]
pub async fn push_to_surreal<Id, T>(artifact: T) -> Result<()>
where
  Id: core_types::CoreId,
  T: serde::Serialize + for<'a> serde::Deserialize<'a> + Clone,
{
  let client = clients::surreal::SurrealRootClient::new().await?;

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

#[instrument(skip(id))]
pub async fn pull_from_surreal<Id, T>(id: Id) -> Result<Box<T>>
where
  Id: core_types::CoreId<Model = T>,
  T: serde::Serialize + for<'a> serde::Deserialize<'a> + Clone,
{
  let client = clients::surreal::SurrealRootClient::new().await?;

  let artifact: Option<T> = client
    .select(id)
    .await
    .wrap_err("Failed to get artifact from surreal")?;

  let artifact = artifact.ok_or_eyre("Artifact does not exist in surreal")?;

  Ok(Box::new(artifact))
}

#![warn(missing_docs)]

//! # Artifact
//!
//! `artifact` is a library for managing artifacts. It uses the
//! [`PrivateArtifact`] and [`PublicArtifact`] types from [`core_types`] and
//! implements its [`Artifact`] trait on them, which has convenient methods for
//! pulling/pushing to and from SurrealDB, and methods for downloading/uploading
//! to and from the object store.
//!
//! This crate is built with vendor-agnosticism in mind, but currently does not
//! implement it. Right now we only use AWS' S3 service. As such, the library
//! requires environment variables to configure access. It expects the
//! following:
//! - `AWS_ACCESS_KEY_ID`: The access key ID for the AWS account
//! - `AWS_SECRET_ACCESS_KEY`: The secret access key for the AWS account
//! - `AWS_DEFAULT_REGION`: The region that the buckets exist in.
//!
//! We use one private bucket and one public bucket. The private bucket is only
//! accessible to the root account, and the public bucket is completely public.
//! We have separate types (and tables) to enforce the publicity state of our
//! artifacts with the type system.

mod methods;

use std::future::Future;

use color_eyre::eyre::{Result, WrapErr};
use core_types::{PrivateArtifact, PublicArtifact};

use self::methods::*;

const ARTIFACT_PRIVATE_LTS_BUCKET: &str = "picturepro-private";
const ARTIFACT_PUBLIC_LTS_BUCKET: &str = "picturepro-public";

type ObjectStoreGenerator =
  Box<dyn Fn() -> Result<Box<dyn object_store::ObjectStore>> + Send + 'static>;

/// The core artifact trait.
pub trait Artifact {
  /// The type of the ID of the artifact.
  type Id: core_types::CoreId<Model = Self>;

  /// Create a new artifact with the given contents.
  fn new(contents: Option<bytes::Bytes>) -> Self;
  /// Create a new artifact with the given ID and contents.
  fn new_with_id(id: Self::Id, contents: Option<bytes::Bytes>) -> Self;

  /// Download the artifact from the object store.
  ///
  /// The data is stored in the `contents` field of the artifact.
  fn download(&mut self) -> impl Future<Output = Result<()>> + Send;
  /// Upload the artifact to the object store.
  ///
  /// The data is taken from the `contents` field of the artifact. The method
  /// fails if the `contents` field is `None`.
  fn upload(&self) -> impl Future<Output = Result<()>> + Send;
  /// Convenience method for uploading and pushing to SurrealDB.
  fn upload_and_push(&self) -> impl Future<Output = Result<()>> + Send
  where
    Self: Sync,
  {
    async move {
      self.upload().await.wrap_err("Failed to upload artifact")?;
      self
        .push_to_surreal()
        .await
        .wrap_err("Failed to push to surreal")?;

      Ok(())
    }
  }

  /// Push the artifact to SurrealDB.
  fn push_to_surreal(&self) -> impl Future<Output = Result<()>> + Send;
  /// Pull an artifact from SurrealDB.
  fn pull_from_surreal(
    id: Self::Id,
  ) -> impl Future<Output = Result<Box<Self>>> + Send;
  /// Get the object store for the artifact.
  fn object_store() -> Result<Box<dyn object_store::ObjectStore>>;
}

impl Artifact for PublicArtifact {
  type Id = core_types::PublicArtifactRecordId;

  fn new(contents: Option<bytes::Bytes>) -> Self {
    let id = core_types::PublicArtifactRecordId(ulid::Ulid::new());
    Self::new_with_id(id, contents)
  }
  fn new_with_id(id: Self::Id, contents: Option<bytes::Bytes>) -> Self {
    let url =
      format!("https://{}.jlewis.sh/{}", ARTIFACT_PUBLIC_LTS_BUCKET, id.0);
    tracing::info!("public artifact url: {}", url);

    Self {
      id,
      url,
      meta: Default::default(),
      contents,
    }
  }
  async fn download(&mut self) -> Result<()> {
    let object_store = Box::new(Self::object_store);
    self.contents =
      Some(download_artifact(object_store, &self.id.0.to_string()).await?);

    Ok(())
  }
  async fn upload(&self) -> Result<()> {
    let object_store = Box::new(Self::object_store);
    upload_artifact(
      object_store,
      &self.id.0.to_string(),
      self.contents.clone().unwrap(),
    )
    .await?;

    Ok(())
  }
  async fn push_to_surreal(&self) -> Result<()> {
    push_to_surreal::<Self::Id, PublicArtifact>(self.clone()).await
  }
  async fn pull_from_surreal(id: Self::Id) -> Result<Box<Self>> {
    pull_from_surreal::<Self::Id, PublicArtifact>(id).await
  }
  fn object_store() -> Result<Box<dyn object_store::ObjectStore>> {
    object_store_from_env(ARTIFACT_PUBLIC_LTS_BUCKET)
  }
}

impl Artifact for PrivateArtifact {
  type Id = core_types::PrivateArtifactRecordId;

  fn new(contents: Option<bytes::Bytes>) -> Self {
    let id = core_types::PrivateArtifactRecordId(ulid::Ulid::new());
    Self::new_with_id(id, contents)
  }
  fn new_with_id(id: Self::Id, contents: Option<bytes::Bytes>) -> Self {
    Self {
      id,
      meta: Default::default(),
      contents,
    }
  }
  async fn download(&mut self) -> Result<()> {
    let object_store = Box::new(Self::object_store);
    self.contents =
      Some(download_artifact(object_store, &self.id.0.to_string()).await?);

    Ok(())
  }
  async fn upload(&self) -> Result<()> {
    let object_store = Box::new(Self::object_store);
    upload_artifact(
      object_store,
      &self.id.0.to_string(),
      self.contents.clone().unwrap(),
    )
    .await?;

    Ok(())
  }
  async fn push_to_surreal(&self) -> Result<()> {
    push_to_surreal::<Self::Id, PrivateArtifact>(self.clone()).await
  }
  async fn pull_from_surreal(id: Self::Id) -> Result<Box<Self>> {
    pull_from_surreal::<Self::Id, PrivateArtifact>(id).await
  }
  fn object_store() -> Result<Box<dyn object_store::ObjectStore>> {
    object_store_from_env(ARTIFACT_PRIVATE_LTS_BUCKET)
  }
}

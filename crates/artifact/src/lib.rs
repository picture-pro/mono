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

use std::future::Future;

use color_eyre::eyre::{OptionExt, Result, WrapErr};
use core_types::{NewId, PrivateArtifact, PublicArtifact};

const ARTIFACT_PRIVATE_LTS_BUCKET: &str = "picturepro-artifact-private-lts";
const ARTIFACT_PUBLIC_LTS_BUCKET: &str = "picturepro-artifact-public-lts";

/// The core artifact trait.
pub trait Artifact {
  /// The type of the ID of the artifact.
  type Id: core_types::NewId;

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
  fn upload_and_push(&self) -> impl Future<Output = Result<()>> + Send;

  /// Push the artifact to SurrealDB.
  fn push_to_surreal(&self) -> impl Future<Output = Result<()>> + Send;
  /// Pull an artifact from SurrealDB.
  fn pull_from_surreal(
    id: Self::Id,
  ) -> impl Future<Output = Result<Box<Self>>> + Send;
  /// Get the object store for the artifact.
  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>>;
}

impl Artifact for PublicArtifact {
  type Id = core_types::PublicArtifactRecordId;

  fn new(contents: Option<bytes::Bytes>) -> Self {
    let id = core_types::PublicArtifactRecordId(ulid::Ulid::new());
    Self::new_with_id(id, contents)
  }

  fn new_with_id(id: Self::Id, contents: Option<bytes::Bytes>) -> Self {
    Self {
      id,
      contents,
      url: format!(
        "https://s3.{}.amazonaws.com/{}/{}",
        std::env::var("AWS_DEFAULT_REGION").unwrap(),
        ARTIFACT_PUBLIC_LTS_BUCKET,
        id.0
      ),
    }
  }

  async fn download(&mut self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.id.0.to_string());

    let contents = object_store
      .get(&path)
      .await
      .wrap_err("Failed to download artifact")?;

    self.contents = Some(
      contents
        .bytes()
        .await
        .wrap_err("Failed to read bytes of downloaded artifact")?,
    );

    Ok(())
  }

  async fn upload(&self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.id.0.to_string());

    object_store
      .put(
        &path,
        self.contents.clone().ok_or_eyre("No contents to upload")?,
      )
      .await
      .wrap_err("Failed to upload artifact")?;

    Ok(())
  }

  async fn upload_and_push(&self) -> Result<()> {
    self.upload().await.wrap_err("Failed to upload artifact")?;
    self
      .push_to_surreal()
      .await
      .wrap_err("Failed to push to surreal")?;

    Ok(())
  }

  async fn push_to_surreal(&self) -> Result<()> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;

    let pushed_artifact: Vec<Self> = client
      .create(Self::Id::TABLE)
      .content(self.clone())
      .await
      .wrap_err("Failed to create artifact in surreal")?;

    let _pushed_artifact = pushed_artifact
      .first()
      .ok_or_eyre("Failed to create artifact in surreal")?;

    Ok(())
  }

  async fn pull_from_surreal(id: Self::Id) -> Result<Box<Self>> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let artifact: Option<PublicArtifact> = client
      .select((Self::Id::TABLE, id.id_with_brackets()))
      .await
      .wrap_err("Failed to get artifact from surreal")?;

    let artifact = artifact.ok_or_eyre("Artifact does not exist in surreal")?;

    Ok(Box::new(artifact))
  }

  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>> {
    let object_store = object_store::aws::AmazonS3Builder::from_env()
      .with_region(
        std::env::var("AWS_DEFAULT_REGION")
          .wrap_err("Failed to get AWS region from environment")
          .unwrap(),
      )
      .with_bucket_name(ARTIFACT_PUBLIC_LTS_BUCKET)
      .build()
      .wrap_err("Failed to create object store")?;

    Ok(Box::new(object_store))
  }
}

impl Artifact for PrivateArtifact {
  type Id = core_types::PrivateArtifactRecordId;

  fn new(contents: Option<bytes::Bytes>) -> Self {
    let id = core_types::PrivateArtifactRecordId(ulid::Ulid::new());
    Self::new_with_id(id, contents)
  }

  fn new_with_id(id: Self::Id, contents: Option<bytes::Bytes>) -> Self {
    Self { id, contents }
  }

  async fn download(&mut self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.id.0.to_string());

    let contents = object_store
      .get(&path)
      .await
      .wrap_err("Failed to download artifact")?;

    self.contents = Some(
      contents
        .bytes()
        .await
        .wrap_err("Failed to read bytes of downloaded artifact")?,
    );

    Ok(())
  }

  async fn upload(&self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.id.0.to_string());

    object_store
      .put(
        &path,
        self.contents.clone().ok_or_eyre("No contents to upload")?,
      )
      .await
      .wrap_err("Failed to upload artifact")?;

    Ok(())
  }

  async fn upload_and_push(&self) -> Result<()> {
    self.upload().await.wrap_err("Failed to upload artifact")?;
    self
      .push_to_surreal()
      .await
      .wrap_err("Failed to push to surreal")?;

    Ok(())
  }

  async fn push_to_surreal(&self) -> Result<()> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;

    let pushed_artifact: Vec<Self> = client
      .create(Self::Id::TABLE)
      .content(self.clone())
      .await
      .wrap_err("Failed to create artifact in surreal")?;

    let _pushed_artifact = pushed_artifact
      .first()
      .ok_or_eyre("Failed to create artifact in surreal")?;

    Ok(())
  }

  async fn pull_from_surreal(id: Self::Id) -> Result<Box<Self>> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let artifact: Option<PrivateArtifact> = client
      .select((Self::Id::TABLE, id.id_with_brackets()))
      .await
      .wrap_err("Failed to get artifact from surreal")?;

    let artifact = artifact.ok_or_eyre("Artifact does not exist in surreal")?;

    Ok(Box::new(artifact))
  }

  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>> {
    let object_store = object_store::aws::AmazonS3Builder::from_env()
      .with_region(
        std::env::var("AWS_DEFAULT_REGION")
          .wrap_err("Failed to get AWS region from environment")
          .unwrap(),
      )
      .with_bucket_name(ARTIFACT_PRIVATE_LTS_BUCKET)
      .build()
      .wrap_err("Failed to create object store")?;

    Ok(Box::new(object_store))
  }
}

use std::future::Future;

use color_eyre::eyre::{OptionExt, Result, WrapErr};
use serde::{Deserialize, Serialize};

const ARTIFACT_PRIVATE_LTS_BUCKET: &str = "artifact-private-lts";
const ARTIFACT_PUBLIC_LTS_BUCKET: &str = "artifact-public-lts";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealPrivateArtifact {
  pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealPublicArtifact {
  pub id: String,
}

#[derive(Clone, Debug)]
pub struct PrivateArtifact {
  id:       ulid::Ulid,
  contents: Option<bytes::Bytes>,
}

#[derive(Debug, Clone)]
pub struct PublicArtifact {
  id:       ulid::Ulid,
  contents: Option<bytes::Bytes>,
}

pub trait Artifact {
  fn new(contents: Option<bytes::Bytes>) -> Self;

  fn id(&self) -> ulid::Ulid;
  fn has_contents(&self) -> bool;
  fn contents(&self) -> Option<&bytes::Bytes>;
  fn contents_mut(&mut self) -> Option<&mut bytes::Bytes>;
  fn set_contents(&mut self, contents: bytes::Bytes);

  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>>;

  /// Downloads the artifact contents from the object store
  async fn download(&mut self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.id().to_string());

    let object = object_store
      .get(&path)
      .await
      .wrap_err("Failed to download artifact")?;

    self.set_contents(
      object
        .bytes()
        .await
        .wrap_err("Failed to read object contents")?,
    );

    Ok(())
  }

  /// Uploads the artifact contents to the object store
  async fn upload(&self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.id().to_string());

    let _object = object_store
      .put(
        &path,
        self
          .contents()
          .ok_or_eyre("Artifact has no contents")?
          .clone(),
      )
      .await
      .wrap_err("Failed to upload artifact")?;

    Ok(())
  }
  fn push_to_surreal(&self) -> impl Future<Output = Result<()>> + Send;
  fn pull_from_surreal(
    id: ulid::Ulid,
  ) -> impl Future<Output = Result<Box<Self>>> + Send;
}

impl Artifact for PublicArtifact {
  fn new(contents: Option<bytes::Bytes>) -> Self {
    Self {
      id: ulid::Ulid::new(),
      contents,
    }
  }

  fn id(&self) -> ulid::Ulid { self.id }
  fn has_contents(&self) -> bool { self.contents.is_some() }
  fn contents(&self) -> Option<&bytes::Bytes> { self.contents.as_ref() }
  fn contents_mut(&mut self) -> Option<&mut bytes::Bytes> {
    self.contents.as_mut()
  }
  fn set_contents(&mut self, contents: bytes::Bytes) {
    self.contents = Some(contents)
  }

  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>> {
    let object_store = object_store::aws::AmazonS3Builder::from_env()
      .with_bucket_name(ARTIFACT_PUBLIC_LTS_BUCKET)
      .build()
      .wrap_err("Failed to create object store")?;

    Ok(Box::new(object_store))
  }

  async fn push_to_surreal(&self) -> Result<()> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let surreal_artifact = SurrealPublicArtifact {
      id: self.id.to_string(),
    };

    let thing: Option<surrealdb::sql::Thing> = client
      .create(("artifacts", &surreal_artifact.id))
      .content(surreal_artifact)
      .await
      .wrap_err("Failed to create artifact in surreal")?;

    let _thing = thing.ok_or_eyre("Failed to create artifact in surreal")?;

    Ok(())
  }

  async fn pull_from_surreal(id: ulid::Ulid) -> Result<Box<Self>> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let surreal_artifact: Option<SurrealPublicArtifact> = client
      .select(("artifacts", &id.to_string()))
      .await
      .wrap_err("Failed to get artifact from surreal")?;

    let surreal_artifact =
      surreal_artifact.ok_or_eyre("Artifact does not exist in surreal")?;
    let artifact = PublicArtifact {
      id:       surreal_artifact.id.parse().wrap_err("Failed to parse id")?,
      contents: None,
    };

    Ok(Box::new(artifact))
  }
}

impl Artifact for PrivateArtifact {
  fn new(contents: Option<bytes::Bytes>) -> Self {
    Self {
      id: ulid::Ulid::new(),
      contents,
    }
  }

  fn id(&self) -> ulid::Ulid { self.id }
  fn has_contents(&self) -> bool { self.contents.is_some() }
  fn contents(&self) -> Option<&bytes::Bytes> { self.contents.as_ref() }
  fn contents_mut(&mut self) -> Option<&mut bytes::Bytes> {
    self.contents.as_mut()
  }
  fn set_contents(&mut self, contents: bytes::Bytes) {
    self.contents = Some(contents)
  }

  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>> {
    let object_store = object_store::aws::AmazonS3Builder::from_env()
      .with_bucket_name(ARTIFACT_PRIVATE_LTS_BUCKET)
      .build()
      .wrap_err("Failed to create object store")?;

    Ok(Box::new(object_store))
  }

  async fn push_to_surreal(&self) -> Result<()> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let surreal_artifact = SurrealPrivateArtifact {
      id: self.id.to_string(),
    };

    let thing: Option<surrealdb::sql::Thing> = client
      .create(("artifacts", &surreal_artifact.id))
      .content(surreal_artifact)
      .await
      .wrap_err("Failed to create artifact in surreal")?;

    let _thing = thing.ok_or_eyre("Failed to create artifact in surreal")?;

    Ok(())
  }

  async fn pull_from_surreal(id: ulid::Ulid) -> Result<Box<Self>> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let surreal_artifact: Option<SurrealPrivateArtifact> = client
      .select(("artifacts", &id.to_string()))
      .await
      .wrap_err("Failed to get artifact from surreal")?;

    let surreal_artifact =
      surreal_artifact.ok_or_eyre("Artifact does not exist in surreal")?;
    let artifact = PrivateArtifact {
      id:       surreal_artifact.id.parse().wrap_err("Failed to parse id")?,
      contents: None,
    };

    Ok(Box::new(artifact))
  }
}

use color_eyre::eyre::{OptionExt, Result, WrapErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealArtifact {
  pub id:       String,
  pub provider: String,
  pub location: String,
}

impl From<&Artifact> for SurrealArtifact {
  fn from(artifact: &Artifact) -> Self {
    SurrealArtifact {
      id:       artifact.metadata.id.to_string(),
      provider: match artifact.metadata.provider {
        Provider::S3 => "s3".to_string(),
      },
      location: artifact.metadata.location.clone(),
    }
  }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ArtifactDeserializationError {
  #[error("Invalid provider: {0}")]
  InvalidProvider(String),
}

impl TryFrom<SurrealArtifact> for Artifact {
  type Error = ArtifactDeserializationError;

  fn try_from(
    artifact: SurrealArtifact,
  ) -> Result<Self, ArtifactDeserializationError> {
    Ok(Artifact {
      metadata: StoreMetadata {
        id:       ulid::Ulid::from_string(&artifact.id).unwrap(),
        provider: match artifact.provider.as_str() {
          "s3" => Provider::S3,
          _ => {
            return Err(ArtifactDeserializationError::InvalidProvider(
              artifact.provider,
            ))
          }
        },
        location: artifact.location,
      },
      contents: None,
    })
  }
}

#[derive(Clone, Debug)]
pub struct Artifact {
  metadata: StoreMetadata,
  contents: Option<bytes::Bytes>,
}

impl Artifact {
  pub fn new(
    provider: Provider,
    location: String,
    contents: Option<bytes::Bytes>,
  ) -> Self {
    Artifact {
      metadata: StoreMetadata {
        id: ulid::Ulid::new(),
        provider,
        location,
      },
      contents,
    }
  }

  pub fn id(&self) -> ulid::Ulid { self.metadata.id }
  pub fn provider(&self) -> Provider { self.metadata.provider.clone() }
  pub fn location(&self) -> String { self.metadata.location.clone() }
  pub fn has_contents(&self) -> bool { self.contents.is_some() }
  pub fn contents(&self) -> Option<bytes::Bytes> { self.contents.clone() }
  pub fn contents_mut(&mut self) -> Option<&mut bytes::Bytes> {
    self.contents.as_mut()
  }

  fn object_store(&self) -> Result<Box<dyn object_store::ObjectStore>> {
    match &self.metadata.provider {
      Provider::S3 => {
        let s3 = object_store::aws::AmazonS3Builder::from_env()
          .with_bucket_name(self.metadata.location.clone())
          .build()
          .wrap_err("Failed to create S3 object store")?;
        Ok(Box::new(s3))
      }
    }
  }

  /// Downloads the artifact contents from the object store
  pub async fn download(&mut self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.metadata.id.to_string());

    let object = object_store
      .get(&path)
      .await
      .wrap_err("Failed to download artifact")?;

    self.contents = Some(
      object
        .bytes()
        .await
        .wrap_err("Failed to read object contents")?,
    );

    Ok(())
  }

  /// Uploads the artifact contents to the object store
  pub async fn upload(&self) -> Result<()> {
    let object_store = self.object_store()?;
    let path = object_store::path::Path::from(self.metadata.id.to_string());

    let _object = object_store
      .put(
        &path,
        self
          .contents
          .clone()
          .ok_or_eyre("Artifact has no contents")?,
      )
      .await
      .wrap_err("Failed to upload artifact")?;

    Ok(())
  }

  /// Pushes the artifact to the Surreal database
  pub async fn push_to_surreal(&self) -> Result<surrealdb::sql::Thing> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let surreal_artifact: SurrealArtifact = self.into();

    let thing: Option<surrealdb::sql::Thing> = client
      .create(("artifacts", &surreal_artifact.id))
      .content(surreal_artifact)
      .await
      .wrap_err("Failed to create artifact in surreal")?;

    let thing = thing.ok_or_eyre("Failed to create artifact in surreal")?;

    Ok(thing)
  }

  /// Pulls artifact info from the Surreal database
  pub async fn pull_from_surreal(id: ulid::Ulid) -> Result<Self> {
    let client = clients::surreal::SurrealRootClient::new()
      .await
      .wrap_err("Failed to create surreal client")?;

    client.use_ns("main").use_db("main").await?;
    let surreal_artifact: Option<SurrealArtifact> = client
      .select(("artifacts", &id.to_string()))
      .await
      .wrap_err("Failed to get artifact from surreal")?;

    let surreal_artifact =
      surreal_artifact.ok_or_eyre("Artifact does not exist in surreal")?;
    let artifact = Artifact::try_from(surreal_artifact)
      .wrap_err("Failed to deserialize artifact")?;

    Ok(artifact)
  }
}

#[derive(Debug, Clone)]
struct StoreMetadata {
  id:       ulid::Ulid,
  provider: Provider,
  location: String,
}

#[derive(Debug, Clone)]
pub enum Provider {
  S3,
}

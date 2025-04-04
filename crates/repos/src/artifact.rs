use std::{fmt, sync::Arc};

use db::{CreateModelError, Database, FetchModelByIndexError, FetchModelError};
use hex::health::{self, HealthAware};
use models::{
  Artifact, ArtifactCreateRequest, ArtifactPath, ArtifactRecordId,
  CompressionStatus, FileSize, StrictSlug, UserRecordId,
};
use storage::{
  belt::Belt, ReadError as StorageReadError, StorageClient,
  WriteError as StorageWriteError,
};

use crate::{base::BaseRepository, ModelRepositoryLike};

/// An error that occurs when reading the data of an [`Artifact`].
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ReadArtifactError {
  /// An error that occurs when fetching an [`Artifact`] model.
  #[error(
    "Failed to fetch Artifact model while attempting to read Artifact data: \
     {0}"
  )]
  FetchModelError(FetchModelError),
  /// An error that occurs when reading the data of an [`Artifact`].
  #[error("Failed to read Artifact data: {0}")]
  StorageReadError(StorageReadError),
}

/// An error that occurs when creating an [`Artifact`].
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CreateArtifactError {
  /// An error that occurs when creating an [`Artifact`] model.
  #[error("Failed to create Artifact model: {0}")]
  CreateModelError(CreateModelError),
  /// An error that occurs when writing the data of an [`Artifact`].
  #[error("Failed to write Artifact data: {0}")]
  StorageWriteError(StorageWriteError),
}

/// Stores and retrieves [`Artifact`]s.
#[derive(Clone)]
pub struct ArtifactRepository {
  storage_repo: StorageClient,
  model_repo: Arc<
    dyn ModelRepositoryLike<
      Model = Artifact,
      ModelCreateRequest = Artifact,
      CreateError = CreateModelError,
    >,
  >,
}

impl fmt::Debug for ArtifactRepository {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ArtifactRepository")
      .field("storage_repo", &self.storage_repo)
      .field(
        "model_repo",
        &stringify!(
          Arc<
            dyn ModelRepositoryLike<
              Model = Artifact,
              ModelCreateRequest = Artifact,
              CreateError = CreateModelError,
            >,
          >
        ),
      )
      .finish()
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for ArtifactRepository {
  fn name(&self) -> &'static str { stringify!(ArtifactRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![
      self.model_repo.health_report(),
      self.storage_repo.health_report(),
    ])
    .await
    .into()
  }
}

impl ArtifactRepository {
  /// Create a new [`ArtifactRepository`].
  pub fn new(
    storage_repo: StorageClient,
    model_repo: Arc<
      dyn ModelRepositoryLike<
        Model = Artifact,
        ModelCreateRequest = Artifact,
        CreateError = CreateModelError,
      >,
    >,
  ) -> Self {
    Self {
      storage_repo,
      model_repo,
    }
  }

  /// Create a new [`ArtifactRepository`], backed by `BaseRepository`.
  pub fn new_from_base_and_storage_client(
    db: Database<Artifact>,
    storage: StorageClient,
  ) -> Self {
    Self::new(storage, Arc::new(BaseRepository::new(db)))
  }

  /// Fetch an [`Artifact`] by id.
  pub async fn fetch_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Artifact>, FetchModelError> {
    self.model_repo.fetch_model_by_id(id).await
  }

  /// Find an [`Artifact`] based on its unique (and indexed) storage path.
  pub async fn fetch_artifact_by_path(
    &self,
    path: ArtifactPath,
  ) -> Result<Option<Artifact>, FetchModelByIndexError> {
    self
      .model_repo
      .fetch_model_by_index(
        "path".into(),
        models::EitherSlug::Strict(StrictSlug::new(path.to_string())),
      )
      .await
  }

  /// Read an [`Artifact`]'s data, identified by its id.
  pub async fn read_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Belt>, ReadArtifactError> {
    let artifact = self
      .model_repo
      .fetch_model_by_id(id)
      .await
      .map_err(ReadArtifactError::FetchModelError)?;
    match artifact {
      Some(artifact) => {
        let data = self
          .storage_repo
          .read(&artifact.path.to_path_buf())
          .await
          .map_err(ReadArtifactError::StorageReadError)?;
        Ok(Some(data))
      }
      None => Ok(None),
    }
  }

  /// Create and write an [`Artifact`] to storage.
  pub async fn create_artifact(
    &self,
    data: Belt,
    originator: UserRecordId,
  ) -> Result<Artifact, CreateArtifactError> {
    let pre_comp_counter = data.counter();
    let data = data.adapt_to_comp(storage::belt::CompressionAlgorithm::Zstd);

    let path = ArtifactPath::new_random();
    let post_comp_size = self
      .storage_repo
      .write(&path.to_path_buf(), data)
      .await
      .map_err(CreateArtifactError::StorageWriteError)?;

    let comp_status = CompressionStatus::Compressed {
      compressed_size:   post_comp_size,
      uncompressed_size: FileSize::new(pre_comp_counter.current()),
      algorithm:         models::CompressionAlgorithm::Zstd,
    };

    let artifact = self
      .model_repo
      .create_model(
        ArtifactCreateRequest {
          path,
          originator,
          comp_status,
        }
        .into(),
      )
      .await
      .map_err(CreateArtifactError::CreateModelError)?;
    Ok(artifact)
  }
}

use db::{CreateModelError, FetchModelByIndexError, FetchModelError};
use hex::health;
use models::{
  Artifact, ArtifactCreateRequest, ArtifactPath, ArtifactRecordId,
  CompressionStatus, FileSize, StrictSlug, UserRecordId,
};
use storage::{belt::Belt, StorageClient};

use super::{ArtifactRepository, CreateArtifactError, ReadArtifactError};
use crate::ModelRepository;

/// The canonical implementation of an [`ArtifactRepository`].
pub struct ArtifactRepositoryCanonical<
  MR: ModelRepository<
    Model = Artifact,
    ModelCreateRequest = ArtifactCreateRequest,
    CreateError = CreateModelError,
  >,
  SR: StorageClient,
> {
  model_repo:   MR,
  storage_repo: SR,
}

impl<
    MR: ModelRepository<
      Model = Artifact,
      ModelCreateRequest = ArtifactCreateRequest,
      CreateError = CreateModelError,
    >,
    SR: StorageClient,
  > ArtifactRepositoryCanonical<MR, SR>
{
  /// Creates a new `ArtifactRepositoryCanonical` instance.
  pub fn new(model_repo: MR, storage_repo: SR) -> Self {
    tracing::info!("creating new `ArtifactRepositoryCanonical` instance",);
    Self {
      model_repo,
      storage_repo,
    }
  }
}

#[async_trait::async_trait]
impl<
    MR: ModelRepository<
      Model = Artifact,
      ModelCreateRequest = ArtifactCreateRequest,
      CreateError = CreateModelError,
    >,
    SR: StorageClient,
  > health::HealthReporter for ArtifactRepositoryCanonical<MR, SR>
{
  fn name(&self) -> &'static str { stringify!(TempStorageRepositoryCanonical) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![
      self.model_repo.health_report(),
      self.storage_repo.health_report(),
    ])
    .await
    .into()
  }
}

#[async_trait::async_trait]
impl<
    MR: ModelRepository<
      Model = Artifact,
      ModelCreateRequest = ArtifactCreateRequest,
      CreateError = CreateModelError,
    >,
    SR: StorageClient,
  > ArtifactRepository for ArtifactRepositoryCanonical<MR, SR>
{
  async fn fetch_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Artifact>, FetchModelError> {
    self.model_repo.fetch_model_by_id(id).await
  }

  async fn fetch_artifact_by_path(
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

  async fn read_artifact_by_id(
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

  async fn create_artifact(
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
      .create_model(ArtifactCreateRequest {
        path,
        originator,
        comp_status,
      })
      .await
      .map_err(CreateArtifactError::CreateModelError)?;
    Ok(artifact)
  }
}

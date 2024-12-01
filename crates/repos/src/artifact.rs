mod canonical;

use db::{CreateModelError, FetchModelByIndexError, FetchModelError};
use hex::Hexagonal;
use models::{Artifact, ArtifactPath, ArtifactRecordId};
use storage::{
  belt::Belt, ReadError as StorageReadError, WriteError as StorageWriteError,
};

pub use self::canonical::*;

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

/// Defines a repository interface for [`Artifact`] models.
#[async_trait::async_trait]
pub trait ArtifactRepository: Hexagonal {
  /// Fetches an [`Artifact`] by its ID.
  async fn fetch_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Artifact>, FetchModelError>;

  /// Fetches an [`Artifact`] by its path.
  async fn fetch_artifact_by_path(
    &self,
    path: ArtifactPath,
  ) -> Result<Option<Artifact>, FetchModelByIndexError>;

  /// Reads the data of an [`Artifact`] by its ID.
  async fn read_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Belt>, ReadArtifactError>;

  /// Creates a new [`Artifact`].
  async fn create_artifact(
    &self,
    data: Belt,
  ) -> Result<Artifact, CreateArtifactError>;
}

#[async_trait::async_trait]
impl<T, I> ArtifactRepository for T
where
  T: std::ops::Deref<Target = I> + Hexagonal + Sized,
  I: ArtifactRepository + ?Sized,
{
  async fn fetch_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Artifact>, FetchModelError> {
    I::fetch_artifact_by_id(self, id).await
  }

  async fn fetch_artifact_by_path(
    &self,
    path: ArtifactPath,
  ) -> Result<Option<Artifact>, FetchModelByIndexError> {
    I::fetch_artifact_by_path(self, path).await
  }

  async fn read_artifact_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Belt>, ReadArtifactError> {
    I::read_artifact_by_id(self, id).await
  }

  async fn create_artifact(
    &self,
    data: Belt,
  ) -> Result<Artifact, CreateArtifactError> {
    I::create_artifact(self, data).await
  }
}

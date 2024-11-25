use std::fmt;

use dvf::slugger::{EitherSlug, StrictSlug};
use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

/// The table name for [`Artifact`] records.
pub const ARTIFACT_TABLE_NAME: &str = "artifact";

/// An alias for [`RecordId<Artifact>`].
pub type ArtifactRecordId = RecordId<Artifact>;

/// The domain model for any stored blob of data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
  /// The artifact's ID.
  pub id:   ArtifactRecordId,
  /// The artifact's path.
  pub path: ArtifactPath,
  /// The artifact's compression status.
  pub size: dvf::CompressionStatus,
}

/// The object storage path for an [`Artifact`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactPath(dvf::Ulid);

impl ArtifactPath {
  /// Creates a new [`ArtifactPath`] from a [`dvf::Ulid`].
  pub fn new(ulid: dvf::Ulid) -> Self { Self(ulid) }

  /// Creates a new random [`ArtifactPath`].
  pub fn new_random() -> Self { Self(dvf::Ulid::new()) }

  /// Converts the [`ArtifactPath`] into a [`dvf::Ulid`].
  pub fn into_inner(self) -> dvf::Ulid { self.0 }
}

impl fmt::Display for ArtifactPath {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Model for Artifact {
  const TABLE_NAME: &'static str = ARTIFACT_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[("path", |artifact| {
    EitherSlug::Strict(StrictSlug::new(artifact.path.to_string()))
  })];

  fn id(&self) -> ArtifactRecordId { self.id }
}

/// A request to create a new [`Artifact`].
#[derive(Debug)]
pub struct ArtifactCreateRequest {
  /// The artifact's path.
  pub path: ArtifactPath,
  /// The artifact's compression status.
  pub size: dvf::CompressionStatus,
}

impl From<ArtifactCreateRequest> for Artifact {
  fn from(input: ArtifactCreateRequest) -> Self {
    Self {
      id:   ArtifactRecordId::default(),
      path: input.path,
      size: input.size,
    }
  }
}

use std::{fmt, path::PathBuf};

use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

use crate::{CompressionStatus, EitherSlug, StrictSlug, Ulid, UserRecordId};

/// The table name for [`Artifact`] records.
pub const ARTIFACT_TABLE_NAME: &str = "artifact";

/// An alias for [`RecordId<Artifact>`].
pub type ArtifactRecordId = RecordId<Artifact>;

/// The domain model for any stored blob of data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
  /// The artifact's ID.
  pub id:               ArtifactRecordId,
  /// The artifact's path.
  pub path:             ArtifactPath,
  /// The artifact's originator.
  pub originator:       UserRecordId,
  /// The artifact's compression status.
  pub comp_status:      CompressionStatus,
  /// The artifact's stated mime-type.
  pub stated_mime_type: Option<ArtifactMimeType>,
}

/// The object storage path for an [`Artifact`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactPath(Ulid);

impl ArtifactPath {
  /// Creates a new [`ArtifactPath`] from a [`Ulid`].
  #[must_use]
  pub fn new(ulid: Ulid) -> Self { Self(ulid) }

  /// Creates a new random [`ArtifactPath`].
  #[must_use]
  pub fn new_random() -> Self { Self(Ulid::new()) }

  /// Converts the [`ArtifactPath`] into a [`Ulid`].
  #[must_use]
  pub fn into_inner(self) -> Ulid { self.0 }

  /// Converts the [`ArtifactPath`] into a [`PathBuf`].
  #[must_use]
  pub fn to_path_buf(&self) -> PathBuf { self.0.to_string().into() }
}

impl fmt::Display for ArtifactPath {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// The object storage path for an [`Artifact`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactMimeType(String);

impl ArtifactMimeType {
  /// Creates a new [`ArtifactMimeType`].
  #[must_use]
  pub fn new(mime_type: &str) -> Self { Self(mime_type.to_owned()) }

  /// Converts the [`ArtifactMimeType`] into a [`String`].
  #[must_use]
  pub fn into_inner(self) -> String { self.0 }
}

impl AsRef<str> for ArtifactMimeType {
  fn as_ref(&self) -> &str { &self.0 }
}

impl Model for Artifact {
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] =
    &[("originator", |artifact| {
      EitherSlug::Strict(StrictSlug::new(artifact.originator.to_string()))
    })];
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
  pub path:             ArtifactPath,
  /// The artifact's originator.
  pub originator:       UserRecordId,
  /// The artifact's compression status.
  pub comp_status:      CompressionStatus,
  /// The artifact's stated mime-type.
  pub stated_mime_type: Option<ArtifactMimeType>,
}

impl From<ArtifactCreateRequest> for Artifact {
  fn from(input: ArtifactCreateRequest) -> Self {
    Self {
      id:               ArtifactRecordId::new(),
      path:             input.path,
      originator:       input.originator,
      comp_status:      input.comp_status,
      stated_mime_type: input.stated_mime_type,
    }
  }
}

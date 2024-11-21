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
  /// The artifact's file size.
  pub size: dvf::FileSize,
}

impl Model for Artifact {
  const TABLE_NAME: &'static str = ARTIFACT_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[];

  fn id(&self) -> ArtifactRecordId { self.id }
}

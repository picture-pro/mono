use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

use crate::ArtifactRecordId;

/// The table name for [`Photo`] records.
pub const PHOTO_TABLE_NAME: &str = "photo";

/// An alias for [`RecordId<Photo>`].
pub type PhotoRecordId = RecordId<Photo>;

/// The domain model for a photo stored on the platform.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Photo {
  /// The photo's ID.
  pub id:        PhotoRecordId,
  /// The photo's artifacts.
  pub artifacts: PhotoArtifacts,
}

/// The artifacts for a [`Photo`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoArtifacts {
  /// The photo's original artifact.
  pub original:  ArtifactRecordId,
  /// The photo's thumbnail artifact.
  pub thumbnail: ArtifactRecordId,
}

impl Model for Photo {
  const TABLE_NAME: &'static str = PHOTO_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[];

  fn id(&self) -> PhotoRecordId { self.id }
}

/// A request to create a new [`Photo`].
pub struct PhotoCreateRequest {
  /// The photo's artifacts.
  pub artifacts: PhotoArtifacts,
}

impl From<PhotoCreateRequest> for Photo {
  fn from(input: PhotoCreateRequest) -> Self {
    Self {
      id:        PhotoRecordId::default(),
      artifacts: input.artifacts,
    }
  }
}

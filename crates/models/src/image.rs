use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

use crate::{ArtifactRecordId, EitherSlug, LaxSlug};

/// The table name for [`Image`] records.
pub const IMAGE_TABLE_NAME: &str = "image";

/// An alias for [`RecordId<Image>`].
pub type ImageRecordId = RecordId<Image>;

/// An artifact-backed Image.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
  /// The image's ID.
  pub id:           ImageRecordId,
  /// The [`Artifact`](crate::Artifact) backing the image.
  pub artifact:     ArtifactRecordId,
  /// The width of the image.
  pub width:        u32,
  /// The height of the image.
  pub height:       u32,
  /// A tiny preview of the image.
  pub tiny_preview: ImageTinyPreview,
}

/// The maximum side length of an [`Image`]'s [`ImageTinyPreview`].
pub const MAX_TINY_PREVIEW_DIMENSION: u32 = 200;

/// A tiny preview of an [`Image`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTinyPreview {
  /// The width of the preview.
  pub width:  u32,
  /// The height of the preview.
  pub height: u32,
  /// The data of the preview.
  pub data:   Vec<u8>,
}

impl Model for Image {
  const TABLE_NAME: &'static str = IMAGE_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[("artifact", |i| {
    EitherSlug::Lax(LaxSlug::new(i.artifact.to_string()))
  })];
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];
  fn id(&self) -> ImageRecordId { self.id }
}

/// A request to create a new [`Image`].
#[derive(Debug)]
pub struct ImageCreateRequest {
  /// The [`Artifact`](crate::Artifact) backing the image.
  pub artifact:     ArtifactRecordId,
  /// The width of the image.
  pub width:        u32,
  /// The height of the image.
  pub height:       u32,
  /// A tiny preview of the image.
  pub tiny_preview: ImageTinyPreview,
}

impl From<ImageCreateRequest> for Image {
  fn from(value: ImageCreateRequest) -> Self {
    Self {
      id:           ImageRecordId::default(),
      artifact:     value.artifact,
      width:        value.width,
      height:       value.height,
      tiny_preview: value.tiny_preview,
    }
  }
}

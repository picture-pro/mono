use serde::{Deserialize, Serialize};

/// The table name for the photo table.
pub const PHOTO_TABLE: &str = "photo";
/// The table name for the photo group table.
pub const PHOTO_GROUP_TABLE: &str = "photo_group";

use crate::{
  auth::UserRecordId, PrivateArtifactRecordId, PublicArtifactRecordId,
};

/// The record ID for a photo.
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PhotoRecordId(pub ulid::Ulid);

/// A photo.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Photo {
  /// The record ID.
  pub id:           PhotoRecordId,
  /// The user who created the photo.
  pub photographer: UserRecordId,
  /// The user who owns the photo.
  pub owner:        UserRecordId,
  /// The photo's artifacts.
  pub artifacts:    PhotoArtifacts,
}

/// The artifacts for a photo. Not a table.
///
/// This is a separate type to make it easier to work with the photo table.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoArtifacts {
  /// The original image.
  ///
  /// This is a private artifact bc honestly it's our whole product :)
  pub original:  PrivateImageArtifact,
  /// The thumbnail, with a max size of 200x200.
  pub thumbnail: PublicImageArtifact,
}

/// A public image artifact. Not a table.
///
/// This is a descriptor of the public artifact type, with some image
/// metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicImageArtifact {
  /// The record ID.
  pub artifact_id: PublicArtifactRecordId,
  /// The size of the image.
  pub size:        (u32, u32),
}

/// A private image artifact. Not a table.
///
/// This is a descriptor of the private artifact type, with some image
/// metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateImageArtifact {
  /// The record ID.
  pub artifact_id: PrivateArtifactRecordId,
  /// The size of the image.
  pub size:        (u32, u32),
}

/// The record ID for a photo group.
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PhotoGroupRecordId(pub ulid::Ulid);

/// A photo group.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoGroup {
  /// The record ID.
  pub id:     PhotoGroupRecordId,
  /// The user who owns the photo group.
  pub owner:  UserRecordId,
  /// The photos in the group.
  pub photos: Vec<PhotoRecordId>,
  /// Whether the group is publicly visible.
  pub public: bool,
}

/// The metadata for uploading a photo group. Not a table.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PhotoGroupUploadMeta {
  /// Whether the group should be publicly visible.
  pub public: bool,
}

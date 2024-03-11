use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The table name for the photo table.
pub const PHOTO_TABLE: &str = "photo";
/// The table name for the photo group table.
pub const PHOTO_GROUP_TABLE: &str = "photo_group";

use crate::{
  auth::UserRecordId, PrivateArtifactRecordId, PublicArtifactRecordId,
};

/// The record ID for a photo.
#[derive(Clone, Debug, Deserialize, Serialize, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PhotoRecordId(pub ulid::Ulid);

/// A photo.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Photo {
  /// The record ID.
  pub id:         PhotoRecordId,
  /// The photo group that contains this photo.
  pub group:      PhotoGroupRecordId,
  /// The photo's artifacts.
  pub artifacts:  PhotoArtifacts,
  /// Data derived from the photo's EXIF data.
  pub photo_meta: PhotoMeta,
  /// Object metadata.
  pub meta:       crate::ObjectMeta,
}

/// Photo metadata derived from EXIF data.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PhotoMeta {
  /// The date and time the photo was taken.
  pub date_time:   Option<chrono::NaiveDateTime>,
  /// The GPS coordinates where the photo was taken.
  pub gps:         Option<(f64, f64)>,
  /// The original orientation of the photo (uses EXIF orientations).
  pub orientation: Option<u32>,
  /// Extra EXIF data.
  pub extra:       HashMap<String, String>,
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
#[derive(Clone, Debug, Deserialize, Serialize, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PhotoGroupRecordId(pub ulid::Ulid);

/// A photo group.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoGroup {
  /// The record ID.
  pub id:           PhotoGroupRecordId,
  /// The user who uploaded the photo group.
  pub photographer: UserRecordId,
  /// The user who owns the photo group.
  pub owner:        UserRecordId,
  /// The photos in the group.
  pub photos:       Vec<PhotoRecordId>,
  /// The status of the photo group.
  pub status:       PhotoGroupStatus,
  /// Object metadata.
  pub meta:         crate::ObjectMeta,
}

/// The status of a photo group. Not a table.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PhotoGroupStatus {
  /// The ownership of the photo group is for sale.
  OwnershipForSale {
    /// The price of the photo group.
    digital_price: crate::Price,
  },
  /// The ownership of the photo group has been purchased, and it cannot be
  /// sold again.
  OwnershipPurchased {
    /// The user who purchased the photo group.
    owner: UserRecordId,
  },
  /// Usage rights to the photos in the group are for sale. Usage rights can be
  /// sold repeatedly.
  UsageRightsForSale {
    /// The price of the photos in the group.
    digital_price: Vec<(PhotoRecordId, crate::Price)>,
  },
}

/// The display parameters for a photo thumbnail. Not a table.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoThumbnailDisplayParams {
  /// The base64-encoded image data, with the `data:image...` prefix.
  pub data: String,
  /// The alt text for the image.
  pub alt:  String,
  /// The image size.
  pub size: (u32, u32),
}

/// The upload parameters for a photo group. Not a table.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoGroupUploadParams {
  /// The photos to upload.
  pub photos: Vec<PhotoUploadParams>,
  /// The status of the photo group.
  pub status: PhotoGroupStatus,
}

/// The upload parameters for a photo. Not a table.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoUploadParams {
  /// The original image data.
  pub original: Vec<u8>,
}

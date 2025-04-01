use model::{Model, RecordId};
use serde::{Deserialize, Serialize};

use crate::{price::UsdPriceNaive, PhotoRecordId, UserRecordId};

/// The table name for [`PhotoGroup`] records.
pub const PHOTO_GROUP_TABLE_NAME: &str = "photo_group";

/// The minimum allowed price for the usage rights to a photo group.
pub const PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE: UsdPriceNaive =
  UsdPriceNaive::new_from_f32(0.1);

/// An alias for [`RecordId<PhotoGroup>`].
pub type PhotoGroupRecordId = RecordId<PhotoGroup>;

/// The domain model for a photo group.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoGroup {
  /// The photo group's id.
  pub id:     PhotoGroupRecordId,
  /// The photo group's owner.
  pub owner:  UserRecordId,
  /// The photos included in the group.
  pub photos: Vec<PhotoRecordId>,
  /// The configuration for the group.
  pub config: PhotoGroupConfig,
}

/// Configuration for a [`PhotoGroup`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoGroupConfig {
  /// The USD price of usage rights for all photos in the group.
  pub usage_rights_price: UsdPriceNaive,
}

impl Model for PhotoGroup {
  const TABLE_NAME: &'static str = PHOTO_GROUP_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[];

  fn id(&self) -> PhotoGroupRecordId { self.id }
}

/// A request to create a new [`PhotoGroup`].
#[derive(Debug)]
pub struct PhotoGroupCreateRequest {
  /// The photo group's owner.
  pub owner:  UserRecordId,
  /// The photos included in the group.
  pub photos: Vec<PhotoRecordId>,
  /// The configuration for the group.
  pub config: PhotoGroupConfig,
}

impl From<PhotoGroupCreateRequest> for PhotoGroup {
  fn from(input: PhotoGroupCreateRequest) -> Self {
    Self {
      id:     PhotoGroupRecordId::default(),
      owner:  input.owner,
      photos: input.photos,
      config: input.config,
    }
  }
}

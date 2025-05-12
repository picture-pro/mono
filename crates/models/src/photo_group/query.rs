use serde::{Deserialize, Serialize};

use super::*;
use crate::PublicUser;

/// A query containing all the data relating to a given `PhotoGroup`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoGroupFullQuery {
  /// The photo group being queried for.
  pub photo_group: PhotoGroup,
  /// The public user data of the photo group vendor.
  pub vendor_data: PublicUser,
}

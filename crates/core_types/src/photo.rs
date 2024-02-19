use serde::{Deserialize, Serialize};

pub const PHOTO_TABLE: &str = "photo";
pub const PHOTO_GROUP_TABLE: &str = "photo_group";

use crate::{
  auth::UserRecordId, PrivateArtifactRecordId, PublicArtifactRecordId,
};

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PhotoRecordId(pub ulid::Ulid);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Photo {
  pub id:           PhotoRecordId,
  pub photographer: UserRecordId,
  pub owner:        UserRecordId,
  pub artifacts:    PhotoArtifacts,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoArtifacts {
  pub original:  PrivateImageArtifact,
  pub thumbnail: PublicImageArtifact,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicImageArtifact {
  pub artifact_id: PublicArtifactRecordId,
  pub size:        (u32, u32),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateImageArtifact {
  pub artifact_id: PrivateArtifactRecordId,
  pub size:        (u32, u32),
}

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PhotoGroupRecordId(pub ulid::Ulid);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoGroup {
  pub id:     PhotoGroupRecordId,
  pub owner:  UserRecordId,
  pub photos: Vec<PhotoRecordId>,
  pub public: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PhotoGroupUploadMeta {
  pub public: bool,
}

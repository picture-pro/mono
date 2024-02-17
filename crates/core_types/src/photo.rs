use serde::{Deserialize, Serialize};

pub const PHOTO_TABLE: &str = "photo";
pub const PHOTO_GROUP_TABLE: &str = "photo_group";

use crate::{
  auth::UserRecordId, PrivateArtifactRecordId, PublicArtifactRecordId,
};

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::conv::UlidOrThing"))]
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
#[cfg_attr(feature = "ssr", serde(from = "crate::conv::UlidOrThing"))]
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

#[cfg(feature = "ssr")]
mod ssr {
  use surreal_id::NewId;
  use surrealdb::sql::Id;

  use super::*;

  impl NewId for PhotoRecordId {
    const TABLE: &'static str = PHOTO_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
      Self(inner_id.into().to_string().parse().unwrap())
    }
    fn get_inner_string(&self) -> String { self.0.to_string() }
  }

  impl NewId for PhotoGroupRecordId {
    const TABLE: &'static str = PHOTO_GROUP_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
      Self(inner_id.into().to_string().parse().unwrap())
    }
    fn get_inner_string(&self) -> String { self.0.to_string() }
  }
}

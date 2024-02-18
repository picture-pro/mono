use serde::{Deserialize, Serialize};

pub const PRIVATE_ARTIFACT_TABLE: &str = "private_artifact";
pub const PUBLIC_ARTIFACT_TABLE: &str = "public_artifact";

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::conv::UlidOrThing"))]
pub struct PrivateArtifactRecordId(pub ulid::Ulid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateArtifact {
  pub id:       PrivateArtifactRecordId,
  #[serde(skip)]
  pub contents: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::conv::UlidOrThing"))]
pub struct PublicArtifactRecordId(pub ulid::Ulid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicArtifact {
  pub id:       PublicArtifactRecordId,
  pub url:      String,
  #[serde(skip)]
  pub contents: Option<bytes::Bytes>,
}

#[cfg(feature = "ssr")]
mod ssr {
  use surreal_id::NewId;
  use surrealdb::sql::Id;

  use super::*;

  impl NewId for PrivateArtifactRecordId {
    const TABLE: &'static str = PRIVATE_ARTIFACT_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
      Self(inner_id.into().to_string().parse().unwrap())
    }
    fn get_inner_string(&self) -> String { self.0.to_string() }
  }

  impl NewId for PublicArtifactRecordId {
    const TABLE: &'static str = PUBLIC_ARTIFACT_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
      Self(inner_id.into().to_string().parse().unwrap())
    }
    fn get_inner_string(&self) -> String { self.0.to_string() }
  }
}

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

use serde::{Deserialize, Serialize};

/// The table name for the private artifact table.
pub const PRIVATE_ARTIFACT_TABLE: &str = "private_artifact";
/// The table name for the public artifact table.
pub const PUBLIC_ARTIFACT_TABLE: &str = "public_artifact";

/// The record ID for a private artifact.
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PrivateArtifactRecordId(pub ulid::Ulid);

/// A private artifact.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateArtifact {
  /// The record ID.
  pub id:       PrivateArtifactRecordId,
  /// The contents of the artifact (skipped by serde)
  #[serde(skip)]
  pub contents: Option<bytes::Bytes>,
}

/// The record ID for a public artifact.
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct PublicArtifactRecordId(pub ulid::Ulid);

/// A public artifact.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicArtifact {
  /// The record ID.
  pub id:       PublicArtifactRecordId,
  /// The public URL to the artifact.
  pub url:      String,
  #[serde(skip)]
  /// The contents of the artifact (skipped by serde)
  pub contents: Option<bytes::Bytes>,
}

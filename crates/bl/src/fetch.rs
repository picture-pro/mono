use artifact::Artifact;
use bytes::Bytes;
use clients::surreal::SurrealRootClient;
use color_eyre::eyre::{Context, Result};
use core_types::{
  NewId, Photo, PhotoArtifacts, PhotoGroup, PhotoGroupUploadMeta,
  PrivateArtifact, PublicArtifact,
};
use serde::{Deserialize, Serialize};
use surrealdb::opt::PatchOp;
use tracing::instrument;

#[instrument]
pub async fn fetch_user_owned_photo_groups(
  user_id: core_types::UserRecordId,
) -> Result<Vec<PhotoGroup>> {
  let client = SurrealRootClient::new()
    .await
    .wrap_err("Failed to create surreal client")?;
  client
    .use_ns("main")
    .use_db("main")
    .await
    .wrap_err("Failed to use surreal namespace/database")?;

  let mut result = client
    .query(format!(
      "SELECT * FROM {} WHERE owner = $user_id",
      core_types::PhotoGroupRecordId::TABLE
    ))
    .bind(("user_id", user_id.0.to_string()))
    .await
    .wrap_err("Failed to query photo groups")?;

  let groups: Vec<PhotoGroup> = result
    .take(0)
    .wrap_err("Failed to take result of photo groups query")?;

  Ok(groups)
}

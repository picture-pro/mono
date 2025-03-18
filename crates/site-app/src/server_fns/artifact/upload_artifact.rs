use std::io;

use auth_domain::AuthSession;
use axum::{body::Body, extract::State, Json};
use belt::Belt;
use futures::TryStreamExt;
use models::ArtifactRecordId;

/// Uploads an artifact from the HTTP stream. Requires authentication.
#[axum::debug_handler]
pub async fn upload_artifact(
  State(prime_domain): State<prime_domain::DynPrimeDomainService>,
  auth_session: AuthSession,
  body: Body,
) -> Result<Json<ArtifactRecordId>, String> {
  let user = auth_session
    .user
    .ok_or("authentication required".to_string())?;

  let data = Belt::from_stream(
    body.into_data_stream().map_err(io::Error::other),
    Some(belt::DEFAULT_CHUNK_SIZE),
  );

  let artifact = prime_domain
    .create_artifact(data, user.id)
    .await
    .map_err(|e| format!("failed to upload artifact: {e}"))?;

  Ok(Json(artifact.id))
}

#![cfg_attr(
  debug_assertions,
  expect(
    clippy::items_after_statements,
    reason = "axum::debug_handler triggers this"
  )
)]

use std::io;

use auth_domain::AuthSession;
use axum::{
  body::Body,
  extract::State,
  http::{header::CONTENT_TYPE, HeaderMap},
  Json,
};
use belt::Belt;
use futures::TryStreamExt;
use models::{ArtifactMimeType, ArtifactRecordId, ImageRecordId};
use prime_domain::CreateImageFromArtifactError;

/// Uploads an artifact from the HTTP stream. Requires authentication.
#[axum::debug_handler]
pub async fn upload_artifact_as_image(
  req_headers: HeaderMap,
  State(prime_domain): State<prime_domain::PrimeDomainService>,
  auth_session: AuthSession,
  body: Body,
) -> Result<Json<ImageRecordId>, String> {
  let user = auth_session
    .user
    .ok_or("authentication required".to_string())?;

  let mime_type = req_headers
    .get(CONTENT_TYPE)
    .and_then(|v| v.to_str().ok())
    .map(ArtifactMimeType::new);

  let data = Belt::from_stream(
    body.into_data_stream().map_err(io::Error::other),
    Some(belt::DEFAULT_CHUNK_SIZE),
  );

  let artifact = prime_domain
    .create_artifact(data, user.id, mime_type)
    .await
    .map_err(|e| format!("failed to upload artifact: {e}"))?;

  let image = prime_domain
    .create_image_from_artifact(artifact.id)
    .await
    .map_err(|e| match e {
      CreateImageFromArtifactError::MissingArtifact(record_id) => {
        format!("missing artifact {record_id}")
      }
      e => {
        tracing::error!("failed to create image from artifact: {e}");
        "Internal Error".to_string()
      }
    })?;

  Ok(Json(image.id))
}

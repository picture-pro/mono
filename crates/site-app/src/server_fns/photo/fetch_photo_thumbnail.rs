#![cfg_attr(
  debug_assertions,
  expect(
    clippy::items_after_statements,
    reason = "axum::debug_handler triggers this"
  )
)]

use std::str::FromStr;

use axum::{
  body::Body,
  extract::{Path, State},
  http::{
    header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE},
    HeaderMap, HeaderValue, Response, StatusCode,
  },
  response::{IntoResponse, IntoResponseParts},
};
use belt::Belt;
use models::{PhotoRecordId, Ulid};
use prime_domain::PrimeDomainService;

const APPLICATION_OCTET_STREAM: HeaderValue =
  HeaderValue::from_static("application/octet-stream");

/// Fetches the bytes of a [`Photo`](models::Photo) thumbnail.
#[axum::debug_handler]
pub async fn fetch_photo_thumbnail(
  Path(id): Path<String>,
  State(pd): State<PrimeDomainService>,
  headers: HeaderMap,
) -> Result<Response<Body>, Response<Body>> {
  let id = PhotoRecordId::from_ulid(Ulid::from_str(&id).map_err(|_| {
    (StatusCode::BAD_REQUEST, "Malformed Photo ID").into_response()
  })?);

  let photo = pd
    .fetch_photo(id)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch photo: {e}");
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?
    .ok_or_else(|| {
      tracing::warn!("photo not found: {id}");
      (StatusCode::NOT_FOUND, "Photo Not Found").into_response()
    })?;

  let image_id = photo.artifacts.thumbnail;

  let image = pd
    .fetch_image(image_id)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch image: {e}");
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?
    .ok_or_else(|| {
      tracing::warn!("image {image_id} missing (referenced by photo {id})",);
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?;

  let artifact_id = image.artifact;

  let (artifact_data, artifact_mime_type) = pd
    .read_artifact_by_id(artifact_id)
    .await
    .map_err(|e| {
      tracing::error!("failed to read artifact data: {e}");
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?
    .ok_or_else(|| {
      tracing::error!(
        "artifact {artifact_id} missing (referenced by image {image_id})",
      );
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?;

  let content_type = artifact_mime_type
    .and_then(|mt| HeaderValue::from_str(mt.as_ref()).ok())
    .unwrap_or(APPLICATION_OCTET_STREAM);

  Ok(efficiently_compressed_belt_http_response(
    &headers,
    artifact_data,
    HeaderMap::from_iter([
      (
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=31536000, immutable"),
      ),
      (CONTENT_TYPE, content_type),
    ]),
  ))
}

/// Matches the compression of a [`Belt`] to the best available option indicated
/// by request headers, and sets the appropriate headers in the response.
fn efficiently_compressed_belt_http_response(
  req_headers: &HeaderMap,
  belt: Belt,
  parts: impl IntoResponseParts,
) -> Response<Body> {
  let current_comp_http_name = belt.comp().map(|a| match a {
    belt::CompressionAlgorithm::Zstd => "zstd",
  });
  let req_accept_comp = req_headers
    .get("Accept-Encoding")
    .map(|v| {
      v.to_str()
        .unwrap()
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();

  let mut out_headers = HeaderMap::with_capacity(1);
  match current_comp_http_name {
    // current compression is directly usable, so send as-is
    Some(current_comp_http_name)
      if req_accept_comp.contains(&current_comp_http_name) =>
    {
      out_headers.insert(
        CONTENT_ENCODING,
        current_comp_http_name.parse().expect(
          "failed to convert current compression name into header value",
        ),
      );
      (parts, out_headers, Body::from_stream(belt)).into_response()
    }
    // current compression isn't allowed, so decompress
    Some(_) => (
      parts,
      out_headers,
      Body::from_stream(belt.adapt_to_no_comp()),
    )
      .into_response(),
    // currently uncompressed, so don't attempt to compress (for now)
    None => (parts, out_headers, Body::from_stream(belt)).into_response(),
  }
}

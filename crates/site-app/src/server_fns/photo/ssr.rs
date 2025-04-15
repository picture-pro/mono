use std::str::FromStr;

use axum::{
  body::Body,
  extract::{Path, State},
  http::{
    header::{CONTENT_ENCODING, CONTENT_TYPE},
    HeaderMap, HeaderValue, Response, StatusCode,
  },
  response::{IntoResponse, IntoResponseParts},
};
use belt::Belt;
use leptos::prelude::*;
use models::{PhotoRecordId, Ulid};
use prime_domain::PrimeDomainService;

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

  let artifact_id = photo.artifacts.thumbnail;

  let artifact_data = pd
    .read_artifact_by_id(artifact_id)
    .await
    .map_err(|e| {
      tracing::error!("failed to read artifact data: {e}");
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?
    .ok_or_else(|| {
      tracing::error!(
        "artifact {artifact_id} missing (referenced by photo {photo_id})",
        photo_id = photo.id
      );
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
    })?;

  Ok(efficiently_compressed_belt_http_response(
    &headers,
    artifact_data,
    (),
  ))
}

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
        .map(|s| s.trim())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();

  const APPLICATION_OCTET_STREAM: HeaderValue =
    HeaderValue::from_static("application/octet-stream");

  let mut out_headers = HeaderMap::with_capacity(2);
  out_headers.insert(CONTENT_TYPE, APPLICATION_OCTET_STREAM);

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

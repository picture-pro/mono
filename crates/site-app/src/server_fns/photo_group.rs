use leptos::prelude::*;
use models::{PhotoGroup, PhotoGroupRecordId};
#[cfg(feature = "ssr")]
pub use ssr::*;

/// Fetches all [`PhotoGroup`]s for the current user.
#[server]
pub async fn fetch_photo_groups_for_user(
) -> Result<Vec<PhotoGroup>, ServerFnError> {
  use models::AuthStatus;
  use prime_domain::PrimeDomainService;

  let auth_session: AuthStatus = expect_context();
  let Some(user) = auth_session.0 else {
    return Ok(Vec::new());
  };

  let pd: PrimeDomainService = expect_context();

  let photo_groups =
    pd.fetch_photo_groups_by_user(user.id).await.map_err(|e| {
      tracing::error!("failed to fetch photo groups: {e}");
      ServerFnError::new("Internal Error")
    })?;

  Ok(photo_groups)
}

/// Fetches a [`PhotoGroup`].
#[server]
pub async fn fetch_photo_group(
  /// The ID of the [`PhotoGroup`] to fetch.
  id: PhotoGroupRecordId,
) -> Result<Option<PhotoGroup>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  let pd: PrimeDomainService = expect_context();

  let photo_group = pd.fetch_photo_group(id).await.map_err(|e| {
    tracing::error!("failed to fetch photo group: {e}");
    ServerFnError::new("Internal Error")
  })?;

  Ok(photo_group)
}

#[cfg(feature = "ssr")]
mod ssr {
  use std::str::FromStr;

  use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
  };
  use models::{BaseUrl, PhotoGroupRecordId};
  use prime_domain::PrimeDomainService;

  /// Generate a QR code for a [`PhotoGroup`](models::PhotoGroup).
  pub async fn photo_group_qr_code(
    Path(id): Path<String>,
    State(pd): State<PrimeDomainService>,
    State(base_url): State<BaseUrl>,
  ) -> Response<Body> {
    let Ok(id) = PhotoGroupRecordId::from_str(&id) else {
      return (StatusCode::BAD_REQUEST, "Invalid ID").into_response();
    };

    let qr_code = match pd.generate_photo_group_qr(&base_url, id) {
      Ok(qr_code) => qr_code,
      Err(e) => {
        tracing::error!("failed to generate qr_code for photo group {id}: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error")
          .into_response();
      }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
      header::CONTENT_TYPE,
      HeaderValue::from_static("image/svg+xml"),
    );

    (headers, qr_code).into_response()
  }
}

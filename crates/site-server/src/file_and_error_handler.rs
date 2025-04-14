use axum::{
  body::Body,
  extract::{FromRef, Request, State},
  http::{HeaderMap, HeaderValue, StatusCode, Uri},
  response::{IntoResponse, Response},
};
use leptos::config::LeptosOptions;
use tower::ServiceExt;

use crate::{app_state::AppState, leptos_routes_handler, AuthSession};

pub async fn fallback_handler(
  uri: Uri,
  auth_session: AuthSession,
  State(state): State<AppState>,
  req: Request<Body>,
) -> Response<Body> {
  let options = LeptosOptions::from_ref(&state);
  let app_state = AppState::from_ref(&state);
  let res = get_static_file(uri, &options.site_root, req.headers());
  let mut res = res.await.unwrap();

  if res.status() == StatusCode::OK {
    res.headers_mut().insert(
      "Cache-Control",
      HeaderValue::from_static("max-age=31536000, immutable"),
    );
    res.into_response()
  } else {
    let mut res =
      leptos_routes_handler(auth_session, State(app_state), req).await;
    *res.status_mut() = StatusCode::NOT_FOUND;
    res
  }
}

async fn get_static_file(
  uri: Uri,
  root: &str,
  headers: &HeaderMap<HeaderValue>,
) -> Result<Response<Body>, (StatusCode, String)> {
  use axum::http::header::ACCEPT_ENCODING;

  let req = Request::builder().uri(uri);

  let req = match headers.get(ACCEPT_ENCODING) {
    Some(value) => req.header(ACCEPT_ENCODING, value),
    None => req,
  };

  let req = req.body(Body::empty()).unwrap();
  // `ServeDir` implements `tower::Service` so we can call it with
  // `tower::ServiceExt::oneshot` This path is relative to the cargo root
  match tower_http::services::ServeDir::new(root)
    .precompressed_gzip()
    .precompressed_br()
    .oneshot(req)
    .await
  {
    Ok(res) => Ok(res.into_response()),
    Err(err) => Err((
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Something went wrong: {err}"),
    )),
  }
}

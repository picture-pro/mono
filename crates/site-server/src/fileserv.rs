use axum::{
  body::Body,
  extract::State,
  http::{Request, Response, StatusCode, Uri},
  response::{IntoResponse, Response as AxumResponse},
};
use leptos::*;
use site_app::App;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::AppState;

pub async fn file_and_error_handler(
  uri: Uri,
  auth_session: auth::AuthSession,
  State(app_state): State<AppState>,
  req: Request<Body>,
) -> AxumResponse {
  let root = app_state.leptos_options.site_root.clone();
  let res = get_static_file(
    uri.clone(),
    &root,
    matches!(app_state.leptos_options.env, leptos_config::Env::PROD),
  )
  .await
  .unwrap();

  if res.status() == StatusCode::OK {
    res.into_response()
  } else {
    let handler = leptos_axum::render_app_to_stream_with_context(
      app_state.leptos_options.to_owned(),
      move || {
        provide_context(core_types::LoggedInUser(
          auth_session.user.clone().map(core_types::PublicUser::from),
        ))
      },
      move || view! { <App/> },
    );
    handler(req).await.into_response()
  }
}

async fn get_static_file(
  uri: Uri,
  root: &str,
  cache: bool,
) -> Result<Response<Body>, (StatusCode, String)> {
  let req = Request::builder()
    .uri(uri.clone())
    .body(Body::empty())
    .unwrap();
  // `ServeDir` implements `tower::Service` so we can call it with
  // `tower::ServiceExt::oneshot` This path is relative to the cargo root
  match ServeDir::new(root).oneshot(req).await {
    Ok(res) => {
      let mut response = res.into_response();
      if cache {
        response.headers_mut().insert(
          "Cache-Control",
          "public, max-age=31536000, immutable".parse().unwrap(),
        );
      }
      Ok(response)
    }
    Err(err) => Err((
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Something went wrong: {err}"),
    )),
  }
}

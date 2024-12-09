mod app_state;

use std::sync::Arc;

use auth_domain::DynAuthDomainService;
use axum::{
  body::Body,
  extract::{Request, State},
  response::IntoResponse,
  routing::get,
  Router,
};
use axum_login::AuthManagerLayerBuilder;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use prime_domain::repos::db::kv;
use site_app::*;

use self::app_state::AppState;

type TowerSessionsBackend = Arc<
  prime_domain::hex::retryable::Retryable<kv::tikv::TikvClient, miette::Report>,
>;

type AuthSession = axum_login::AuthSession<DynAuthDomainService>;

#[axum::debug_handler]
async fn leptos_routes_handler(
  auth_session: AuthSession,
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> axum::response::Response {
  let handler = leptos_axum::render_app_async_with_context(
    {
      let app_state = app_state.clone();
      move || {
        provide_context(app_state.prime_domain_service.clone());
        provide_context(app_state.auth_domain_service.clone());
        provide_context(auth_session.clone());
      }
    },
    {
      let leptos_options = app_state.leptos_options.clone();
      move || shell(leptos_options.clone())
    },
  );

  handler(request).await.into_response()
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(
      tracing_subscriber::EnvFilter::builder()
        .with_default_directive(
          tracing::level_filters::LevelFilter::INFO.into(),
        )
        .from_env_lossy(),
    )
    .init();

  tracing::info!("starting picturepro site server");

  let conf = get_configuration(None).unwrap();
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  // Generate the list of routes in your Leptos App
  let routes = generate_route_list(App);

  tracing::info!("initializing app state");
  let app_state = AppState::new(leptos_options).await.unwrap();
  tracing::info!("app state initialized");

  let session_layer =
    tower_sessions::SessionManagerLayer::new(app_state.session_store.clone());
  let auth_layer = AuthManagerLayerBuilder::new(
    app_state.auth_domain_service.clone(),
    session_layer,
  )
  .build();

  let app = Router::new()
    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
    .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
    .with_state(app_state)
    .layer(auth_layer);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  tracing::info!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

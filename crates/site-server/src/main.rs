mod app_state;
mod file_and_error_handler;

use std::sync::Arc;

use auth_domain::AuthSession;
use axum::{
  body::Body,
  extract::{Request, State},
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use axum_login::AuthManagerLayerBuilder;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use prime_domain::repos::db::kv;
use site_app::*;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use self::app_state::AppState;

type TowerSessionsBackend = Arc<
  prime_domain::hex::retryable::Retryable<kv::tikv::TikvClient, miette::Report>,
>;

#[axum::debug_handler]
async fn leptos_routes_handler(
  auth_session: AuthSession,
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> axum::response::Response {
  let handler = leptos_axum::render_app_to_stream_with_context(
    {
      let app_state = app_state.clone();
      move || {
        provide_context(app_state.prime_domain_service.clone());
        provide_context(app_state.auth_domain_service.clone());
        provide_context(site_app::AuthStatus(auth_session.user.clone()));
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

  // fallback service with compression
  // this nastiness is to serve an "unrouted" axum handler with state
  let fallback_service = ServiceBuilder::new()
    .layer(CompressionLayer::new())
    .service(
      Router::new()
        .route("/", get(self::file_and_error_handler::fallback_handler))
        .route("/*a", get(self::file_and_error_handler::fallback_handler))
        .with_state(app_state.clone())
        .layer(auth_layer.clone()),
    );

  let server_fn_handler = {
    let app_state = app_state.clone();
    move |req: Request| {
      leptos_axum::handle_server_fns_with_context(
        move || {
          provide_context(app_state.prime_domain_service.clone());
          provide_context(app_state.auth_domain_service.clone());
        },
        req,
      )
    }
  };

  let app = Router::new()
    .leptos_routes_with_handler(routes.clone(), leptos_routes_handler)
    .route(
      "/api/upload_artifact",
      post(site_app::server_fns::upload_artifact),
    )
    .route("/api/*fn_name", post(server_fn_handler))
    .fallback_service(fallback_service)
    .with_state(app_state)
    .layer(auth_layer)
    .layer(TraceLayer::new_for_http());

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  tracing::info!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

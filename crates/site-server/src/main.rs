mod app_state;
mod file_and_error_handler;

use auth_domain::AuthSession;
use axum::{
  body::Body,
  extract::{Request, State},
  handler::Handler,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use axum_login::AuthManagerLayerBuilder;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use miette::{Context, IntoDiagnostic};
use site_app::*;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use self::app_state::AppState;

fn context_provider(
  app_state: AppState,
  auth_session: AuthSession,
) -> impl Fn() + Clone {
  move || {
    provide_context(app_state.prime_domain_service.clone());
    provide_context(app_state.auth_domain_service.clone());
    provide_context(site_app::AuthStatus(auth_session.user.clone()));
  }
}

#[axum::debug_handler]
async fn leptos_routes_handler(
  auth_session: AuthSession,
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> axum::response::Response {
  let leptos_options = app_state.leptos_options.clone();
  leptos_axum::render_app_to_stream_with_context(
    context_provider(app_state.clone(), auth_session),
    move || shell(leptos_options.clone()),
  )(request)
  .await
  .into_response()
}

#[axum::debug_handler]
async fn server_fn_handler(
  auth_session: AuthSession,
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> axum::response::Response {
  leptos_axum::handle_server_fns_with_context(
    context_provider(app_state, auth_session),
    request,
  )
  .await
  .into_response()
}

#[tokio::main]
async fn main() -> miette::Result<()> {
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

  let conf = get_configuration(None)
    .into_diagnostic()
    .context("failed to get leptos configuration")?;
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  let routes = generate_route_list(App);

  tracing::info!("initializing app state");
  let app_state = AppState::new(leptos_options)
    .await
    .context("failed to initialize app state")?;
  tracing::info!("app state initialized");

  let session_layer =
    tower_sessions::SessionManagerLayer::new(app_state.session_store.clone());
  let auth_layer = AuthManagerLayerBuilder::new(
    app_state.auth_domain_service.clone(),
    session_layer,
  )
  .build();

  // fallback service with compression
  let static_service = ServiceBuilder::new()
    .layer(CompressionLayer::new())
    .layer(auth_layer.clone())
    .service(
      self::file_and_error_handler::fallback_handler
        .with_state(app_state.clone()),
    );

  // serve server fns with context from axum

  let api_router = Router::new()
    .route(
      "/api/upload_artifact",
      post(site_app::server_fns::upload_artifact),
    )
    .route(
      "/api/photo_thumbnail/{id}",
      get(site_app::server_fns::fetch_photo_thumbnail),
    )
    .route("/api/{*fn_name}", post(server_fn_handler))
    .with_state(app_state.clone());
  let app = Router::new()
    .leptos_routes_with_handler(routes, leptos_routes_handler)
    .merge(api_router)
    .route_service("/{*path}", static_service)
    .with_state(app_state)
    .layer(TraceLayer::new_for_http())
    .layer(auth_layer);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .into_diagnostic()
    .with_context(|| format!("failed to bind listener to `{addr}`"))?;
  tracing::info!("listening on http://{}", &addr);
  axum::serve(listener, app)
    .await
    .into_diagnostic()
    .context("failed to serve app")?;

  Ok(())
}

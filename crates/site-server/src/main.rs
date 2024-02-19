use axum::{
  body::Body,
  extract::{FromRef, State},
  http::Request,
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
use color_eyre::eyre::Result;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{
  generate_route_list, handle_server_fns_with_context, LeptosRoutes,
};
use leptos_router::RouteListing;
use site_app::*;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

pub mod fileserv;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
  pub leptos_options: LeptosOptions,
  pub routes:         Vec<RouteListing>,
}

async fn server_fn_handler(
  auth_session: auth::AuthSession,
  request: Request<Body>,
) -> impl IntoResponse {
  handle_server_fns_with_context(
    move || {
      provide_context(auth_session.clone());
      provide_context(core_types::LoggedInUser(
        auth_session.user.clone().map(core_types::PublicUser::from),
      ))
    },
    request,
  )
  .await
}

async fn leptos_routes_handler(
  auth_session: auth::AuthSession,
  State(app_state): State<AppState>,
  req: Request<Body>,
) -> Response {
  let handler = leptos_axum::render_route_with_context(
    app_state.leptos_options.clone(),
    app_state.routes.clone(),
    move || {
      // provide_context(auth_session.clone());
      provide_context(core_types::LoggedInUser(
        auth_session.user.clone().map(core_types::PublicUser::from),
      ))
    },
    site_app::App,
  );
  handler(req).await.into_response()
}

fn init_logging() {
  color_eyre::install().expect("Failed to install color_eyre");

  let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
    tracing_subscriber::EnvFilter::new("info,site_server=debug,site_app=debug"),
  );

  #[cfg(not(feature = "chrome-tracing"))]
  {
    tracing_subscriber::fmt().with_env_filter(filter).init();
  }
  #[cfg(feature = "chrome-tracing")]
  {
    use tracing_subscriber::prelude::*;

    let (chrome_layer, _guard) =
      tracing_chrome::ChromeLayerBuilder::new().build();
    tracing_subscriber::registry()
      .with(tracing_subscriber::fmt::layer())
      .with(filter)
      .with(chrome_layer)
      .init();
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  init_logging();

  // Setting get_configuration(None) means we'll be using cargo-leptos's env
  // values For deployment these variables are:
  // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
  // Alternately a file can be specified such as Some("Cargo.toml")
  // The file would need to be included with the executable when moved to
  // deployment
  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  let state = AppState {
    leptos_options: leptos_options.clone(),
    routes:         generate_route_list(App),
  };
  let routes = generate_route_list(App);

  // build our application with a route
  let app = Router::new()
    .route(
      "/api/*fn_name",
      get(server_fn_handler).post(server_fn_handler),
    )
    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
    .fallback(file_and_error_handler)
    .layer(
      ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(auth::build_auth_layer().await?),
    )
    .with_state(state);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log::info!("listening on http://{}", &addr);
  axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
    .await
    .unwrap();

  Ok(())
}

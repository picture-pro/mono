use axum::{
  body::Body,
  extract::{FromRef, Path, State},
  http::Request,
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
use color_eyre::eyre::Result;
use fileserv::file_and_error_handler;
use leptos::{logging::log, *};
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
  path: Path<String>,
  request: Request<Body>,
) -> impl IntoResponse {
  handle_server_fns_with_context(
    move || {
      provide_context(auth_session.clone());
      provide_context(auth_types::LoggedInUser(
        auth_session.user.clone().map(auth_types::User::from),
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
      provide_context(auth_types::LoggedInUser(
        auth_session.user.clone().map(auth_types::User::from),
      ))
    },
    site_app::App,
  );
  handler(req).await.into_response()
}

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;

  simple_logger::init_with_level(log::Level::Debug)
    .expect("couldn't initialize logging");

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

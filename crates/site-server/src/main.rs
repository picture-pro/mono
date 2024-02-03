use axum::{routing::post, Router};
use color_eyre::eyre::Result;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use site_app::*;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

pub mod fileserv;

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
  let routes = generate_route_list(App);

  // build our application with a route
  let app = Router::new()
    .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
    .leptos_routes(&leptos_options, routes, App)
    .fallback(file_and_error_handler)
    .layer(
      ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(auth::build_auth_layer().await?),
    )
    .with_state(leptos_options);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log::info!("listening on http://{}", &addr);
  axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
    .await
    .unwrap();

  Ok(())
}

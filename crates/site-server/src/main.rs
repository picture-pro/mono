use std::{sync::Arc, time::Duration};

use axum::{extract::FromRef, Router};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use miette::Result;
use prime_domain::{
  hex::retryable::Retryable, DynPrimeDomainService, PrimeDomainService,
  PrimeDomainServiceCanonical,
};
use site_app::*;

#[derive(Clone, FromRef)]
struct AppState {
  prime_domain_service: DynPrimeDomainService,
}

impl AppState {
  async fn new() -> Result<Self> {
    let tikv_store_init = move || async move {
      prime_domain::repos::db::kv::tikv::TikvClient::new_from_env().await
    };
    let retryable_tikv_store =
      Retryable::init(5, Duration::from_secs(2), tikv_store_init).await;
    let kv_db_adapter = Arc::new(
      prime_domain::repos::db::KvDatabaseAdapter::new(retryable_tikv_store),
    );

    let photo_repo =
      prime_domain::repos::BaseRepository::new(kv_db_adapter.clone());

    let prime_domain_service: Arc<Box<dyn PrimeDomainService>> =
      Arc::new(Box::new(PrimeDomainServiceCanonical::new(photo_repo)));

    Ok(Self {
      prime_domain_service,
    })
  }
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
  let app_state = AppState::new().await.unwrap();
  tracing::info!("app state initialized");

  let app = Router::new()
    .leptos_routes_with_context(
      &leptos_options,
      routes,
      {
        let app_state = app_state.clone();
        move || {
          provide_context(app_state.prime_domain_service.clone());
        }
      },
      {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
      },
    )
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(leptos_options);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  tracing::info!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

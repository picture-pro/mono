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

  let app = Router::new()
    .leptos_routes_with_handler(routes, leptos_routes_handler)
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

mod file_and_error_handler {
  use std::{future::Future, pin::Pin};

  use axum::{
    body::Body,
    extract::{FromRef, Request, State},
    http::Uri,
  };
  use leptos::{config::LeptosOptions, IntoView};

  pub fn file_and_error_handler<S, IV>(
    shell: fn(LeptosOptions) -> IV,
  ) -> impl Fn(
    Uri,
    State<S>,
    Request<Body>,
  ) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + 'static>>
       + Clone
       + Send
       + 'static
  where
    IV: IntoView + 'static,
    S: Send + 'static,
    LeptosOptions: FromRef<S>,
  {
    move |uri: Uri, State(options): State<S>, req: Request<Body>| {
      Box::pin(async move {
        let options = LeptosOptions::from_ref(&options);
        let res = get_static_file(uri, &options.site_root, req.headers());
        let res = res.await.unwrap();

        if res.status() == StatusCode::OK {
          res.into_response()
        } else {
          let mut res = handle_response_inner(
            || {},
            move || shell(options),
            req,
            |app, chunks| {
              Box::pin(async move {
                let app =
                  app.to_html_stream_in_order().collect::<String>().await;
                let chunks = chunks();
                Box::pin(once(async move { app }).chain(chunks))
                  as PinnedStream<String>
              })
            },
          )
          .await;
          *res.status_mut() = StatusCode::NOT_FOUND;
          res
        }
      })
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
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {err}"),
      )),
    }
  }
}

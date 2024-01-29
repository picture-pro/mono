use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;
  let subscriber = tracing_subscriber::fmt().finish();
  tracing::subscriber::set_global_default(subscriber)?;

  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

  // run our app with hyper, listening globally on port 3000
  let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
  let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
  let address = format!("{}:{}", host, port);
  let listener = tokio::net::TcpListener::bind(&address).await?;

  info!("Listening on {}", address);
  axum::serve(listener, app).await?;

  Ok(())
}

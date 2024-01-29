use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use tracing::info;

pub struct App;

impl App {
  pub fn new() -> Self { Self }

  pub async fn serve(&self) -> Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let address = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    info!("listening on {}", address);
    axum::serve(listener, app).await?;

    Ok(())
  }
}

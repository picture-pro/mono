use axum::{routing::get, Router};
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;

  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  axum::serve(listener, app).await?;

  Ok(())
}

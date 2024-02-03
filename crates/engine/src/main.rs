pub mod app;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;
  let subscriber = tracing_subscriber::fmt().finish();
  tracing::subscriber::set_global_default(subscriber)?;

  let app = app::App::new();
  app.serve().await?;

  Ok(())
}

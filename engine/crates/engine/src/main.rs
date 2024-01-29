use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;

  println!("Hello, world!");

  Ok(())
}

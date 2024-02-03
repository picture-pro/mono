//! Clients for the SurrealDB database.

use std::ops::Deref;

use color_eyre::eyre::{Result, WrapErr};
use surrealdb::{
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
  Surreal,
};

/// A root-level client for the SurrealDB database.
#[derive(Clone, Debug)]
pub struct SurrealRootClient {
  client: surrealdb::Surreal<Client>,
}

impl SurrealRootClient {
  /// Creates a new client.
  pub async fn new() -> Result<Self> {
    let client = Surreal::new::<Ws>(
      std::env::var("SURREALDB_WS_URL")
        .wrap_err("Could not find env var \"SURREALDB_WS_URL\"")?,
    )
    .await
    .wrap_err("Could not connect to SurrealDB endpoint")?;

    let client = Self { client };
    client.sign_in_as_root().await?;

    Ok(client)
  }

  /// Signs in as root.
  pub async fn sign_in_as_root(&self) -> Result<()> {
    self
      .client
      .signin(Root {
        username: &std::env::var("SURREALDB_ROOT_USER")
          .wrap_err("Could not find env var \"SURREALDB_ROOT_USER\"")?,
        password: &std::env::var("SURREALDB_ROOT_PASS")
          .wrap_err("Could not find env var \"SURREALDB_ROOT_PASS\"")?,
      })
      .await
      .wrap_err("Could not sign in to SurrealDB as root")
      .map(|_| ())
  }

  /// Consumes the client and returns the inner client.
  pub fn into_inner(self) -> surrealdb::Surreal<Client> { self.client }
}

impl Deref for SurrealRootClient {
  type Target = surrealdb::Surreal<Client>;

  fn deref(&self) -> &Self::Target { &self.client }
}

use axum_login::{
  AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
};
use color_eyre::eyre::{eyre, Context, OptionExt, Result};
use core_types::NewId;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
  pub email:    String,
  pub password: String,
}

#[derive(Clone, Debug)]
pub struct Backend {
  surreal_client: clients::surreal::SurrealRootClient,
}

impl Backend {
  pub async fn new() -> color_eyre::Result<Self> {
    Ok(Self {
      surreal_client: clients::surreal::SurrealRootClient::new().await?,
    })
  }

  pub async fn signup(
    &self,
    name: String,
    email: String,
    password: String,
  ) -> Result<core_types::User> {
    (*self.surreal_client).use_ns("main").use_db("main").await?;

    // check whether a user with the given email already exists
    let user: Option<core_types::User> = (*self.surreal_client)
      .query("SELECT * FROM users WHERE email = $email")
      .bind(("email", &email))
      .await?
      .take(0)
      .wrap_err("Failed to query SurrealDB for existing user")?;

    if user.is_some() {
      return Err(eyre!("User with email {} already exists", email));
    }

    // create a new user
    let user: Option<core_types::User> = (*self.surreal_client)
      .query(
        "CREATE user SET name = $name, email = $email, pw_hash = \
         crypto::argon2::generate($password), id = $id",
      )
      .bind(("name", &name))
      .bind(("email", &email))
      .bind(("password", &password))
      .bind((
        "id",
        core_types::UserRecordId(core_types::Ulid::new()).id_without_brackets(),
      ))
      .await
      .wrap_err("Failed to create user in SurrealDB")?
      .take(0)
      .wrap_err("Failed to insert user into SurrealDB")?;

    user.ok_or_eyre("Failed to create user")
  }
}

#[async_trait::async_trait]
impl AuthnBackend for Backend {
  type User = core_types::User;
  type Credentials = Credentials;
  type Error = surrealdb::Error;

  async fn authenticate(
    &self,
    credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    (*self.surreal_client).use_ns("main").use_db("main").await?;

    let user: Option<core_types::User> = (*self.surreal_client)
      .query(
        "SELECT * FROM user WHERE email = $email AND \
         crypto::argon2::compare(pw_hash, $password)",
      )
      .bind(("email", &credentials.email))
      .bind(("password", &credentials.password))
      .await?
      .take(0)?;

    Ok(user)
  }

  async fn get_user(
    &self,
    user_id: &UserId<Self>,
  ) -> Result<Option<Self::User>, Self::Error> {
    (*self.surreal_client).use_ns("main").use_db("main").await?;

    let user: Option<core_types::User> = (*self.surreal_client)
      .select((
        core_types::UserRecordId::TABLE,
        core_types::UserRecordId::new(user_id.to_string())
          .unwrap()
          .id_without_brackets(),
      ))
      .await?;
    Ok(user)
  }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

/// Builds an authentication layer for use with an Axum router.
pub async fn build_auth_layer() -> Result<
  AuthManagerLayer<
    Backend,
    tower_sessions_surrealdb_store::SurrealSessionStore<Client>,
  >,
> {
  let session_store_surreal_client =
    clients::surreal::SurrealRootClient::new().await?;
  session_store_surreal_client
    .use_ns("main")
    .use_db("main")
    .await?;
  let session_store = tower_sessions_surrealdb_store::SurrealSessionStore::new(
    session_store_surreal_client.into_inner(),
    "user_session".to_string(),
  );
  let session_manager_layer =
    tower_sessions::SessionManagerLayer::new(session_store);

  Ok(
    AuthManagerLayerBuilder::new(Backend::new().await?, session_manager_layer)
      .build(),
  )
}

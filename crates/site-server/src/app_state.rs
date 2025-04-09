use std::str::FromStr;

use auth_domain::AuthDomainService;
use axum::extract::FromRef;
use leptos::prelude::*;
use miette::Result;
use prime_domain::{
  repos::{
    db::{kv, Database},
    storage::StorageClient,
  },
  PrimeDomainService,
};

#[derive(Clone, FromRef)]
pub struct AppState {
  pub prime_domain_service: PrimeDomainService,
  pub auth_domain_service:  AuthDomainService,
  pub session_store:        tower_sessions_kv_store::TowerSessionsKvStore,
  pub leptos_options:       LeptosOptions,
}

impl AppState {
  pub async fn new(l_opts: LeptosOptions) -> Result<Self> {
    let kv_store_location = std::path::PathBuf::from(
      std::env::var("REDB_STORE_PATH")
        .unwrap_or("/tmp/picturepro-db".to_owned()),
    );
    let storage_location = std::path::PathBuf::from(
      std::env::var("STORAGE_PATH")
        .unwrap_or("/tmp/picturepro-storage".to_owned()),
    );

    let kv_store = kv::KeyValueStore::new_redb(&kv_store_location)?;

    let session_store =
      tower_sessions_kv_store::TowerSessionsKvStore::new(kv_store.clone());

    let photo_repo = prime_domain::repos::PhotoRepository::new(
      Database::new_from_kv(kv_store.clone()),
    );
    let photo_group_repo = prime_domain::repos::PhotoGroupRepository::new(
      Database::new_from_kv(kv_store.clone()),
    );
    let user_repo = prime_domain::repos::UserRepository::new(
      Database::new_from_kv(kv_store.clone()),
    );

    let storage_credentials = prime_domain::models::StorageCredentials::Local(
      prime_domain::models::LocalStorageCredentials(storage_location),
    );
    let artifact_storage_client =
      StorageClient::new_from_storage_creds(storage_credentials).await?;
    let artifact_repo = prime_domain::repos::ArtifactRepository::new(
      artifact_storage_client,
      Database::new_from_kv(kv_store),
    );

    let prime_domain_service =
      PrimeDomainService::new(photo_repo, photo_group_repo, artifact_repo);
    let auth_domain_service = AuthDomainService::new(user_repo);

    Ok(Self {
      prime_domain_service,
      auth_domain_service,
      session_store,
      leptos_options: l_opts,
    })
  }
}

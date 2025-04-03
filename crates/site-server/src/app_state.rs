use std::{str::FromStr, time::Duration};

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
    let retryable_kv_store =
      kv::KeyValueStore::new_retryable_tikv_from_env(5, Duration::from_secs(2))
        .await;

    let session_store = tower_sessions_kv_store::TowerSessionsKvStore::new(
      retryable_kv_store.clone(),
    );

    let photo_repo = prime_domain::repos::PhotoRepository::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );
    let photo_group_repo = prime_domain::repos::PhotoGroupRepository::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );
    let user_repo = prime_domain::repos::UserRepository::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );

    let storage_credentials = prime_domain::models::StorageCredentials::Local(
      prime_domain::models::LocalStorageCredentials(
        std::path::PathBuf::from_str("/tmp/picturepro-store").unwrap(),
      ),
    );
    let artifact_storage_client =
      StorageClient::new_from_storage_creds(storage_credentials).await?;
    let artifact_repo = prime_domain::repos::ArtifactRepository::new(
      artifact_storage_client,
      Database::new_from_kv(retryable_kv_store),
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

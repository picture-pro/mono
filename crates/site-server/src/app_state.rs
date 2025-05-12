use auth_domain::AuthDomainService;
use axum::extract::FromRef;
use leptos::prelude::*;
use miette::{Context, IntoDiagnostic, Result};
use prime_domain::{
  repos::{
    db::{kv, Database},
    storage::StorageClient,
  },
  PrimeDomainService,
};
use site_app::models::BaseUrl;
use tower_sessions_kv_store::TowerSessionsKvStore;

#[derive(Clone, FromRef)]
pub struct AppState {
  pub prime_domain_service: PrimeDomainService,
  pub auth_domain_service:  AuthDomainService,
  pub session_store:        TowerSessionsKvStore,
  pub leptos_options:       LeptosOptions,
  pub base_url:             BaseUrl,
}

impl AppState {
  pub async fn new(l_opts: LeptosOptions) -> Result<Self> {
    let base_url = std::env::var("BASE_URL")
      .into_diagnostic()
      .context("failed to read `BASE_URL` environment variable")?;
    let base_url = base_url
      .parse::<axum::http::Uri>()
      .into_diagnostic()
      .context("failed to parse `BASE_URL` as a URI")?;
    let base_url = BaseUrl(match base_url.to_string().strip_suffix("/") {
      Some(new_url) => new_url.to_string(),
      None => base_url.to_string(),
    });

    let kv_store_location = std::path::PathBuf::from(
      std::env::var("REDB_STORE_PATH")
        .unwrap_or("/tmp/picturepro-db".to_owned()),
    );
    let storage_location = std::path::PathBuf::from(
      std::env::var("STORAGE_PATH")
        .unwrap_or("/tmp/picturepro-storage".to_owned()),
    );

    let kv_store = kv::KeyValueStore::new_redb(&kv_store_location)?;

    let session_store = TowerSessionsKvStore::new(kv_store.clone());

    let image_repo = prime_domain::repos::ImageRepository::new(
      Database::new_from_kv(kv_store.clone()),
    );
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

    let image_processor = prime_domain::imaging::ImageProcessor::new();

    let prime_domain_service = PrimeDomainService::new(
      artifact_repo,
      image_processor,
      image_repo,
      photo_repo,
      photo_group_repo,
      user_repo.clone(),
    );
    let auth_domain_service = AuthDomainService::new(user_repo);

    Ok(Self {
      prime_domain_service,
      auth_domain_service,
      session_store,
      leptos_options: l_opts,
      base_url,
    })
  }
}

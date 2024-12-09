use std::{sync::Arc, time::Duration};

use auth_domain::{AuthDomainServiceCanonical, DynAuthDomainService};
use axum::extract::FromRef;
use leptos::prelude::*;
use miette::Result;
use prime_domain::{
  hex::retryable::Retryable,
  models::{User, UserCreateRequest},
  repos::{db::kv, CreateModelError},
  DynPrimeDomainService, PrimeDomainService, PrimeDomainServiceCanonical,
};

#[derive(Clone, FromRef)]
pub struct AppState {
  pub prime_domain_service: DynPrimeDomainService,
  pub auth_domain_service:  DynAuthDomainService,
  pub session_store:
    tower_sessions_kv_store::TowerSessionsKvStore<super::TowerSessionsBackend>,
  pub leptos_options:       LeptosOptions,
}

impl AppState {
  pub async fn new(l_opts: LeptosOptions) -> Result<Self> {
    let tikv_store_init =
      move || async move { kv::tikv::TikvClient::new_from_env().await };
    let retryable_tikv_store = Arc::new(
      Retryable::init(5, Duration::from_secs(2), tikv_store_init).await,
    );
    let session_store = tower_sessions_kv_store::TowerSessionsKvStore::new(
      retryable_tikv_store.clone(),
    );
    let kv_db_adapter = Arc::new(
      prime_domain::repos::db::KvDatabaseAdapter::new(retryable_tikv_store),
    );

    let photo_repo =
      prime_domain::repos::BaseModelRepository::new(kv_db_adapter.clone());
    let user_repo: Arc<
      Box<
        dyn prime_domain::repos::ModelRepository<
          Model = User,
          ModelCreateRequest = UserCreateRequest,
          CreateError = CreateModelError,
        >,
      >,
    > = Arc::new(Box::new(prime_domain::repos::BaseModelRepository::new(
      kv_db_adapter.clone(),
    )));

    let prime_domain_service: Arc<Box<dyn PrimeDomainService>> =
      Arc::new(Box::new(PrimeDomainServiceCanonical::new(photo_repo)));
    let auth_domain_service: DynAuthDomainService = DynAuthDomainService::new(
      Arc::new(Box::new(AuthDomainServiceCanonical::new(user_repo))),
    );

    Ok(Self {
      prime_domain_service,
      auth_domain_service,
      session_store,
      leptos_options: l_opts,
    })
  }
}

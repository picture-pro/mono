use leptos::prelude::*;
use models::PhotoGroup;

/// Fetches all [`PhotoGroup`]s for the current user.
#[server]
pub async fn fetch_photo_groups_for_user(
) -> Result<Vec<PhotoGroup>, ServerFnError> {
  use base_components::bridge_types::AuthStatus;
  use prime_domain::PrimeDomainService;

  let auth_session: AuthStatus = expect_context();
  let Some(user) = auth_session.0 else {
    return Ok(Vec::new());
  };

  let pd: PrimeDomainService = expect_context();

  let photo_groups =
    pd.fetch_photo_groups_by_user(user.id).await.map_err(|e| {
      tracing::error!("failed to fetch photo groups: {e}");
      ServerFnError::new("Internal Error")
    })?;

  Ok(photo_groups)
}

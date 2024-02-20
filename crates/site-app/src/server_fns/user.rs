use core_types::UserRecordId;
use leptos::*;

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn fetch_user(
  user_id: UserRecordId,
) -> Result<Option<core_types::PublicUser>, ServerFnError> {
  bl::fetch::fetch_user(user_id).await.map_err(|e| {
    let error = format!("Failed to fetch user: {:?}", e);
    tracing::error!("{error}");
    ServerFnError::new(error)
  })
}

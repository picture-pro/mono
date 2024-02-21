use leptos::*;

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn fetch_photo_group(
  photo_group_id: core_types::PhotoGroupRecordId,
) -> Result<Option<core_types::PhotoGroup>, ServerFnError> {
  bl::fetch::fetch_photo_group(photo_group_id)
    .await
    .map_err(|e| {
      let error = format!("Failed to fetch photo group: {:?}", e);
      tracing::error!("{error}");
      ServerFnError::new(error)
    })
}

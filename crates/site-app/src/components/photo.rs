use leptos::*;

#[component]
pub fn Photo(photo_id: core_types::PhotoRecordId) -> impl IntoView {
  let photo =
    create_resource(move || (), move |_| fetch_photo_thumbnail(photo_id));

  view! {
    <div class="bg-base-300 h-32 w-32 rounded-box">
      <Suspense fallback=|| view!{ }>
        { move || match photo() {
          Some(Ok(photo)) => {
            Some(view! {
              <img
                src={format!("data:image/png;base64,{}", photo.data)}
                alt={photo.alt} width={photo.size.0} height={photo.size.1} />
            }
            .into_view())
          }
          Some(Err(e)) => {
            Some(view! {
              <p>{ format!("Failed to load photo: {e}") }</p>
            }
            .into_view())
          }
          None => None,
        } }
      </Suspense>
    </div>
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn fetch_photo_thumbnail(
  photo_id: core_types::PhotoRecordId,
) -> Result<core_types::PhotoThumbnailDisplayParams, ServerFnError> {
  bl::fetch::fetch_photo_thumbnail(photo_id)
    .await
    .map_err(|e| {
      let error = format!("Failed to fetch photo thumbnail: {:?}", e);
      tracing::error!("{error}");
      ServerFnError::new(error)
    })
}

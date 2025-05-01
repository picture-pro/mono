use base64::Engine;
use base_components::{ImageStyle, SmallImageWithFallback};
use leptos::prelude::*;
use models::{Image, PhotoRecordId};

use crate::server_fns::fetch_thumbnail_image_for_photo;

#[component]
pub fn PhotoPreview(id: PhotoRecordId) -> impl IntoView {
  let resource = Resource::new(move || id, fetch_thumbnail_image_for_photo);

  let render_fn = move |i: Image| {
    let url = format!("/api/photo_thumbnail/{id}");
    let fallback_data = format!(
      "data:image/avif;charset=utf-8;base64,{}",
      base64::prelude::BASE64_STANDARD.encode(i.meta.tiny_preview.data)
    );
    view! {
      <SmallImageWithFallback
        url=url fallback_data=fallback_data
        style=ImageStyle::Border
      />
    }
  };

  move || {
    Suspend::new(async move {
      match resource.await {
        Ok(Some(i)) => render_fn(i).into_any(),
        Ok(None) => view! { "image not found" }.into_any(),
        Err(e) => {
          let e = e.to_string();
          view! { "failed to fetch image: " {e} }.into_any()
        }
      }
    })
  }
}

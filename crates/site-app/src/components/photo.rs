use leptos::*;
use serde::{Deserialize, Serialize};

/// Photo display size descriptor.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum PhotoSize {
  /// Factor to scale the photo by.
  Factor(f32),
  /// Fit within a 150x150 square.
  Regular,
  /// Largest size that fits within a square of the given size.
  FitsWithinSquare(u32),
  /// Do not change the photo size.
  #[default]
  Unchanged,
}

impl PhotoSize {
  pub fn physical(&self, input: (u32, u32)) -> (u32, u32) {
    let logical = (input.0 as f32, input.1 as f32);
    let physical = match self {
      PhotoSize::Factor(factor) => (logical.0 * factor, logical.1 * factor),
      PhotoSize::FitsWithinSquare(size) => {
        let factor = *size as f32 / logical.0.max(logical.1);
        (logical.0 * factor, logical.1 * factor)
      }
      PhotoSize::Regular => {
        let size = PhotoSize::FitsWithinSquare(150).physical(input);
        (size.0 as f32, size.1 as f32)
      }
      PhotoSize::Unchanged => logical,
    };
    (physical.0 as u32, physical.1 as u32)
  }
}

#[component]
pub fn Photo(
  photo_id: core_types::PhotoRecordId,
  #[prop(default = PhotoSize::Regular)] size: PhotoSize,
  #[prop(default = "rounded-box")] rounded: &'static str,
  #[prop(default = "")] extra_class: &'static str,
) -> impl IntoView {
  let photo =
    create_resource(move || (), move |_| fetch_photo_thumbnail(photo_id));

  view! {
    <Suspense fallback=|| view!{ }>
      { move || match photo() {
        Some(Ok(photo)) => {
          Some(view! {
            <img
              src={format!("data:image/png;base64,{}", photo.data)} alt={photo.alt}
              width={size.physical(photo.size).0} height={size.physical(photo.size).1}
              class={format!("{rounded} {extra_class}")}
            />
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

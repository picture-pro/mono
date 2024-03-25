use leptos::*;

use crate::components::photo::PhotoSize;

#[component]
pub fn PhotoDeck(
  ids: Vec<core_types::PhotoRecordId>,
  #[prop(default = PhotoSize::Regular)] size: PhotoSize,
) -> impl IntoView {
  view! {
    { ids.into_iter().map(|photo_id| {
      view! {
        <crate::components::photo::Photo photo_id=photo_id size=size />
      }
      .into_view()
    }).collect::<Vec<_>>() }
  }
}

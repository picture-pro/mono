use leptos::*;

use crate::components::photo::PhotoSize;

#[component]
pub fn PhotoDeck(
  ids: Vec<core_types::PhotoRecordId>,
  #[prop(default = PhotoSize::Regular)] size: PhotoSize,
) -> impl IntoView {
  view! {
    <div class="relative">
      { ids.into_iter().map(|photo_id| {
        view! {
          <crate::components::photo::Photo photo_id=photo_id size=size />
        }
        .into_view()
      }).collect::<Vec<_>>() }
      <div class="absolute top-4 left-4 bg-base-100 p-1 rounded-full">
        <LeftArrow class="size-8" />
      </div>
      <div class="absolute top-4 right-4 bg-base-100 p-1 rounded-full">
        <RightArrow class="size-8" />
      </div>
    </div>
  }
}

#[component]
pub fn LeftArrow(#[prop(default = "")] class: &'static str) -> impl IntoView {
  view! {
    <svg class=class xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" />
    </svg>
  }
}

#[component]
pub fn RightArrow(#[prop(default = "")] class: &'static str) -> impl IntoView {
  view! {
    <svg class=class xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" />
    </svg>
  }
}

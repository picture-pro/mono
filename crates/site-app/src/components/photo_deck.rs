use leptos::*;

#[component]
pub fn PhotoDeck(ids: Vec<core_types::PhotoRecordId>) -> impl IntoView {
  view! {
    { ids.into_iter().map(|photo_id| {
      view! {
        <crate::components::photo::Photo photo_id=photo_id />
      }
      .into_view()
    }).collect::<Vec<_>>() }
  }
}

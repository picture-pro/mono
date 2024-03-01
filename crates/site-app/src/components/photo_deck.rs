use leptos::*;

use crate::components::photo::Photo;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PhotoDeckDisplayMode {
  Flat,
  #[default]
  Stacked,
}

#[component]
pub fn PhotoDeck(
  ids: Vec<core_types::PhotoRecordId>,
  #[prop(optional)] display_mode: PhotoDeckDisplayMode,
) -> impl IntoView {
  logging::log!(
    "Rendering PhotoDeck with {} photos in {:?}",
    ids.len(),
    display_mode
  );
  match display_mode {
    PhotoDeckDisplayMode::Flat => view! {
      <div class="flex flex-wrap gap-2 items-center">
        { ids.into_iter().map(|id| view! { <Photo photo_id={id} /> }).collect::<Vec<_>>() }
      </div>
    }
    .into_view(),
    PhotoDeckDisplayMode::Stacked => {
      let count = ids.len();
      view! {
        <div class="grid">
          { ids.into_iter().enumerate().map(|(i, id)| view! {
            <Photo
              photo_id={id}
              rotation={i as f32 * 16.0}
              scale={1.0 - (i as f32 * 0.05)}
              z_index={count as i32 - i as i32}
              opacity={1.0 - (i as f32 * 0.1)}
              extra_class="col-start-1 row-start-1"
            />
          }).collect::<Vec<_>>() }
        </div>
      }
      .into_view()
    }
  }
}

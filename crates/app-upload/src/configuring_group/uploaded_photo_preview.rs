use base_components::SmallImage;
use leptos::prelude::*;
use models::Ulid;
use reactive_stores::Store;

use super::ConfiguringGroupStateStoreFields;
use crate::UploadStateStoreFields;

#[island]
pub(super) fn UploadedPhotoPreviewer() -> impl IntoView {
  let context: Store<super::super::UploadState> = expect_context();
  let state = context
    .configuring_group_0()
    .expect("`UploadContext` not in state `ConfiguringGroup`");
  let photos = state.photos();

  let photo_id_iter = move || {
    let mut keys = photos.read().keys().copied().collect::<Vec<_>>();
    keys.sort_unstable();
    keys.into_iter()
  };

  let class = "flex flex-row flex-wrap gap-4";

  view! {
    <div class=class>
      <For
        each=photo_id_iter
        key=move |id| *id
        children=move |id| view! {
          <UploadedPhotoPreview id=id />
        }
      />
    </div>
  }
}

#[component]
fn UploadedPhotoPreview(id: Ulid) -> impl IntoView {
  let context: Store<super::super::UploadState> = expect_context();
  let state = context
    .configuring_group_0()
    .expect("`UploadContext` not in state `ConfiguringGroup`");
  let photos = state.photos();

  let url = move || photos.read().get(&id).map(|f| f.url().to_string());

  let image_fn = move |url| {
    view! {
      <SmallImage url=url />
    }
  };

  move || url().map(image_fn)
}

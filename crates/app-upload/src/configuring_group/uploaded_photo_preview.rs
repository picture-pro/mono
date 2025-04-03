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

  let grid_class =
    "grid xl:grid-cols-6 lg:grid-cols-5 md:grid-cols-4 grid-cols-3 gap-4";

  view! {
    <div class=grid_class>
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

  let image_class = "w-full sm:max-h-48 max-h-40 border-2 border-base-8 \
                     dark:border-basedark-8 group-hover:border-primary-8 \
                     group-hover:dark:border-primarydark-8 ring-2 \
                     ring-transparent group-hover:ring-primary-8 \
                     group-hover:dark:ring-primarydark-8 transition-colors \
                     rounded-lg";

  move || {
    url().map(|url| {
      view! {
        <div class="flex flex-col justify-center items-center group">
          <div class="relative">
            <img src={url} class=image_class />
          </div>
        </div>
      }
    })
  }
}

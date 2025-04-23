use base_components::SmallImage;
use leptos::{either::EitherOf3, prelude::*};
use models::{FileSize, Ulid};
use reactive_stores::Store;

use crate::{
  MAX_UPLOAD_SIZE, UploadStateStoreFields, photo::PhotoUploadStatus,
  selecting_photos::SelectingPhotosStateStoreFields,
};

#[component]
pub(super) fn PhotoPreviewer() -> impl IntoView {
  let context: Store<super::super::UploadState> = expect_context();

  let photo_id_iter = move || {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    let photos = state.photos();
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
          <PhotoPreview id=id />
        }
      />
    </div>
  }
}

#[component]
fn PhotoPreview(id: Ulid) -> impl IntoView {
  let context: Store<super::super::UploadState> = expect_context();

  let url = move || {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    let photos = state.photos();
    photos.read().get(&id).map(|f| f.url().to_string())
  };

  let delete_handler = move |_| {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    let photos = state.photos();
    photos.write().remove(&id);
  };

  let status_overlay_element = move || {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    let photos = state.photos();
    match photos.read().get(&id).map(|f| f.upload_status()()) {
      Some(PhotoUploadStatus::UploadInProgress) => EitherOf3::A(view! {
        <ProgressOverlay />
      }),
      Some(PhotoUploadStatus::Oversized(file_size)) => EitherOf3::B(view! {
        <OversizedAlertOverlay size=file_size />
      }),
      _ => EitherOf3::C(()),
    }
  };

  move || {
    url().map(|url| {
      view! {
        <div class="flex flex-col justify-center items-center group">
          <div class="relative">
            <SmallImage url=url />
            { status_overlay_element }
            <DeleteButtonOverlay {..} on:click=delete_handler />
          </div>
        </div>
      }
    })
  }
}

#[component]
fn DeleteButtonOverlay() -> impl IntoView {
  use lsc::icons::*;

  let delete_button_class =
    "absolute top-0 right-0 size-8 flex flex-col justify-center items-center \
     bg-base-2 dark:bg-basedark-2 border-2 border-base-8 \
     dark:border-basedark-8 hover:border-danger-8 \
     hover:dark:border-dangerdark-8 rounded-bl-lg rounded-tr-lg \
     cursor-pointer text-base-dim hover:text-danger-dim transition-colors";

  view! {
    <div class=delete_button_class>
      <TrashIcon {..} class="size-6" />
    </div>
  }
}

#[component]
fn SpinnerVector() -> impl IntoView {
  view! {
    <svg class="mr-3 -ml-1 size-5 animate-spin text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
  }
}

#[component]
fn ProgressOverlay() -> impl IntoView {
  let progress_overlay_class = "absolute inset-0 flex flex-col items-center \
                                justify-center bg-base-1/[.8] \
                                dark:bg-basedark-1/[.8] text-center";

  view! {
    <div class=progress_overlay_class>
      <SpinnerVector {..} class="size-12 animate-spin" />
    </div>
  }
}

#[component]
fn OversizedAlertOverlay(size: FileSize) -> impl IntoView {
  use lsc::icons::*;

  let oversized_overlay_class =
    "absolute inset-0 flex flex-col sm:gap-2 gap-1 p-2 items-center \
     justify-center bg-base-1/[.8] dark:bg-basedark-1/[.8] rounded-lg \
     border-2 border-warning-8 dark:border-warningdark-8 text-center \
     text-warning-12 dark:text-warningdark-12 backdrop-blur-sm";

  view! {
    <div class=oversized_overlay_class>
      <ExclamationTriangleIcon {..} class="sm:size-10 size-8" />
      <div>
        <p class="sm:text-lg font-bold">"Oversized"</p>
        <p class="sm:text-sm text-xs text-warning-dim">
          "Image is too large to upload. File: "
          { size.to_string() }
          ", max: "
          { FileSize::new(MAX_UPLOAD_SIZE).to_string() }
        </p>
      </div>
    </div>
  }
}

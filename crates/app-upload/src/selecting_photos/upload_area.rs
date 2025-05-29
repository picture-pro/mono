use gloo::file::FileList;
use leptos::{logging::debug_warn, prelude::*};
use reactive_stores::Store;
use web_sys::Event;

use super::{Photo, SelectingPhotosStateStoreFields};
use crate::UploadStateStoreFields;

#[component]
pub(super) fn UploadArea() -> impl IntoView {
  let class = "w-full bg-base-2 dark:bg-basedark-2 text-base-dim border-2 \
               border-dashed border-base-8 dark:border-basedark-8 rounded-xl \
               flex flex-row items-stretch relative";

  let context: Store<super::super::UploadState> = expect_context();

  let disabled = Signal::derive(move || {
    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");
    #[cfg(feature = "single-photo-upload")]
    return !state.photos().read().is_empty();
    #[cfg(not(feature = "single-photo-upload"))]
    false
  });

  let handler = move |ev: Event| {
    if disabled() {
      return;
    }

    let state = context
      .selecting_photos_0()
      .expect("`UploadContext` not in state `SelectingPhotos`");

    // get file list
    let element: web_sys::HtmlInputElement = event_target(&ev);
    let Some(file_list) = element.files() else {
      debug_warn!("failed to get file list of event target");
      return;
    };

    // extract each image in file list
    for file in FileList::from(file_list).iter() {
      let photo = Photo::new(file.clone());
      let photos_subfield = state.photos();
      photos_subfield.write().insert(photo.id(), photo);
    }

    // reset input
    element.set_value("");
  };

  view! {
    <div class=class>
      <UploadAreaInner handler=handler disabled=disabled />
      { move || {
        disabled().then_some(view! { <UploadAreaOverlay /> })
      }}
    </div>
  }
}

#[component]
fn UploadAreaOverlay() -> impl IntoView {
  let class = "absolute inset-0 bg-base-2 dark:bg-basedark-2 text-base-dim \
               flex flex-col justify-center items-center";
  view! {
    <div class=class>
      <p class="text-2xl">"Photo Already Selected"</p>
    </div>
  }
}

#[component]
fn UploadAreaInner(
  handler: impl Fn(Event) + Copy + 'static,
  disabled: Signal<bool>,
) -> impl IntoView {
  use lsc::icons::*;

  let icon_class = "size-24 text-basea-11 dark:text-basedarka-11";

  #[cfg(feature = "single-photo-upload")]
  let camera_input_el = view! {
    <input
      type="file" class="hidden" id="camera-input" accept="image/*"
      capture="environment" on:change=handler disabled=disabled
    />
  };
  #[cfg(not(feature = "single-photo-upload"))]
  let camera_input_el = view! {
    <input
      type="file" class="hidden" id="camera-input" accept="image/*"
      capture="environment" multiple="multiple" on:change=handler disabled=disabled
    />
  };
  #[cfg(feature = "single-photo-upload")]
  let file_input_el = view! {
    <input
      type="file" class="hidden" id="file-input" accept="image/*"
      on:change=handler disabled=disabled
    />
  };
  #[cfg(not(feature = "single-photo-upload"))]
  let file_input_el = view! {
    <input
      type="file" class="hidden" id="file-input" accept="image/*"
      multiple="multiple" on:change=handler disabled=disabled
    />
  };

  view! {
    <label
      class="flex-1 flex flex-row justify-center items-center p-8 cursor-pointer"
      for="camera-input"
    >
      <div class="flex flex-col items-center gap-4">
        <CameraIcon {..} class=icon_class />
        <div class="flex flex-col items-center gap-1 text-base-dim text-sm">
          <p>"Take a photo with your camera."</p>
          { camera_input_el }
        </div>
      </div>
    </label>

    <div class="my-4 w-[1px] border-l-2 border-dashed border-base-8 dark:border-basedark-8" />

    <label
      class="flex-1 flex flex-row justify-center items-center p-8 cursor-pointer"
      for="file-input"
    >
      <div class="flex flex-col items-center gap-4">
        <UploadIcon {..} class=icon_class />
        <div class="flex flex-col items-center gap-1 text-base-dim text-sm">
          <p>"Upload a photo from your device."</p>
          { file_input_el }
        </div>
      </div>
    </label>
  }
}

use std::{collections::HashMap, ops::Deref};

use gloo::file::{Blob, File, FileList, ObjectUrl};
use leptos::{logging::debug_warn, prelude::*};
use reactive_stores::Store;
use send_wrapper::SendWrapper;
use web_sys::Event;

use crate::components::Section;

#[component]
pub fn UploadPhotoPage() -> impl IntoView {
  let desc_text = "Upload your photos here to put them on the PicturePro \
                   platform. Here you can adjust pricing and other options.";

  view! {
    <UploadContextProvider>
      <Section>
        <p class="text-6xl font-serif font-light tracking-tight mb-4">
          "Upload Photos"
        </p>
        <p class="text-base-dim">
          { desc_text }
        </p>
      </Section>

      <Section>
        <UploadArea />
      </Section>

      <Section>
        <ImagePreviewer />
      </Section>
    </UploadContextProvider>
  }
}

#[derive(Clone)]
struct QueuedUploadFile {
  name: String,
  blob: SendWrapper<Blob>,
  url:  SendWrapper<ObjectUrl>,
}

impl QueuedUploadFile {
  fn new(file: File) -> Self {
    let name = file.name();
    let blob = Blob::from(file);
    let url = ObjectUrl::from(blob.clone());
    Self {
      name,
      blob: SendWrapper::new(blob),
      url: SendWrapper::new(url),
    }
  }
}

#[derive(Clone, Store, Default)]
struct UploadContext {
  last_index: usize,
  /// Map from filename to file
  files:      HashMap<usize, QueuedUploadFile>,
}

#[island]
fn UploadContextProvider(children: Children) -> impl IntoView {
  let context = Store::new(UploadContext::default());
  provide_context(context);

  children()
}

#[island]
fn ImagePreviewer() -> impl IntoView {
  let context: Store<UploadContext> = expect_context();

  let sorted_indices_iter = move || {
    let files_lock = context.files().read();
    let mut indices = files_lock.keys().copied().collect::<Vec<_>>();
    indices.sort_unstable();
    indices.into_iter()
  };

  let grid_class = "grid sm:grid-cols-[repeat(auto-fit,12rem)] \
                    grid-cols-[repeat(auto-fit,8rem)] gap-4";

  view! {
    <div class=grid_class>
      <For
        each=sorted_indices_iter
        key=move |index| *index
        children=move |index| view! {
          <ImagePreview index=index />
        }
      />
    </div>
  }
}

#[component]
fn ImagePreview(index: usize) -> impl IntoView {
  use lsc::icons::*;

  let context: Store<UploadContext> = expect_context();

  let url = move || {
    let files_lock = context.files().read();
    files_lock
      .get(&index)
      .map(|f| f.url.clone().take().to_string())
  };

  let image_class = "w-auto sm:max-h-[12rem] max-h-[8rem] border-2 \
                     border-base-8 dark:border-basedark-8 rounded-lg";
  let delete_button_class =
    "absolute top-0 right-0 size-8 flex flex-col justify-center items-center \
     bg-base-2 dark:bg-basedark-2 border-2 border-base-8 \
     dark:border-basedark-8 rounded-bl-lg rounded-tr-lg cursor-pointer";

  let delete_handler = move |_| {
    context.files().update(|files| {
      files.remove(&index);
    })
  };

  move || {
    url().map(|url| {
      view! {
        <div class="sm:size-48 size-32 flex flex-col justify-center items-center">
          <div class="relative">
            <img src={url} class=image_class />
            <div class=delete_button_class on:click=delete_handler>
              <TrashIcon {..} class="size-6 text-base-dim" />
            </div>
          </div>
        </div>
      }
    })
  }
}

fn accept_image_from_input(ev: Event) {
  let context: Store<UploadContext> = expect_context();

  // get file list
  let element: web_sys::HtmlInputElement = event_target(&ev);
  let Some(file_list) = element.files() else {
    debug_warn!("failed to get file list of event target");
    return;
  };

  // extract each image in file list
  for file in FileList::from(file_list).iter() {
    let last_index = context.last_index().get();

    context.files().update(|files| {
      files.insert(last_index, QueuedUploadFile::new(file.clone()));
    });

    context.last_index().update(|last_index| {
      *last_index += 1;
    });
  }

  // reset input
  element.set_value("");
}

#[island]
fn UploadArea() -> impl IntoView {
  use lsc::icons::*;

  let class = "w-full bg-base-2 dark:bg-basedark-2 border-2 border-dashed \
               border-base-8 dark:border-basedark-8 rounded-xl flex flex-row \
               items-stretch";

  let icon_class = "size-24 text-basea-11 dark:text-basedarka-11";

  view! {
    <div class=class>

      <label
        class="flex-1 flex flex-row justify-center items-center p-8 cursor-pointer"
        for="camera-input"
      >
        <div class="flex flex-col items-center gap-4">
          <CameraIcon {..} class=icon_class />
          <div class="flex flex-col items-center gap-1 text-base-dim text-sm">
            <p>"Take a photo with your camera."</p>
            <input
              type="file" class="hidden" id="camera-input" accept="image/*"
              capture="environment" multiple="multiple" on:change=accept_image_from_input
            />
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
            <input
              type="file" class="hidden" id="file-input" accept="image/*"
              multiple="multiple" on:change=accept_image_from_input
            />
          </div>
        </div>
      </label>

    </div>
  }
}

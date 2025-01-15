use std::{collections::HashMap, ops::Deref};

use gloo::file::{Blob, File, ObjectUrl};
use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;

use crate::components::Section;

#[component]
pub fn UploadPhotoPage() -> impl IntoView {
  let desc_text = "Upload your photos here to put them on the PicturePro \
                   platform. Here you can adjust pricing and other options.";

  view! {
    <UploadContextProvider>
      <Section>
        <p class="text-6xl font-serif font-light tracking-tight mb-4">
          "Upload Photo"
        </p>
      </Section>

      <div class="flex flex-row justify-between gap-4">
        <div>
          <p class="text-base-dim max-w-prose">
            { desc_text }
          </p>
        </div>
        <UploadArea />
      </div>

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

  view! {
    <For
      each=move || context.files().get().into_iter()
      key=move |entry| entry.0
      children=move |entry| view! {
        <ImagePreview name={entry.1.name.clone()} object_url={entry.1.url.clone().take()} />
      }
    />
  }
}

#[component]
fn ImagePreview(name: String, object_url: ObjectUrl) -> impl IntoView {
  let url = object_url.to_string();
  view! {
    <p>{ name }</p>
    <img src={url} />
  }
}

#[island]
fn UploadArea() -> impl IntoView {
  use lsc::icons::*;

  let class = "aspect-square justify-self-center w-[24rem] bg-base-2 \
               dark:bg-basedark-2 border-2 border-dashed border-base-8 \
               dark:border-basedark-8 rounded-xl flex flex-col justify-center \
               items-center gap-4 cursor-pointer";
  let icon_class = "size-24 text-basea-11 dark:text-basedarka-11";
  let input_class = "hidden";

  let input_callback = move |ev| {
    let input_element: web_sys::HtmlInputElement = event_target(&ev);
    let Some(file_list) = input_element.files() else {
      return;
    };
    let file_list = gloo::file::FileList::from(file_list);

    let context: Store<UploadContext> = expect_context();

    for file in file_list.iter() {
      let last_index_lock = context.last_index();
      let last_index = last_index_lock.get();

      let files_lock = context.files();
      files_lock.update(|files| {
        files.insert(last_index, QueuedUploadFile::new(file.clone()));
      });

      let last_index_lock = context.last_index();
      last_index_lock.update(|last_index| {
        *last_index += 1;
      });
    }
  };

  view! {
    <label class=class for="file-input">
      <UploadIcon {..} class=icon_class />
      <div class="flex flex-col items-center gap-1 text-base-dim text-sm">
        <p>"Click here to upload."</p>
        <input
          type="file" class=input_class id="file-input" accept="image/*"
          capture="camera" multiple="multiple" on:change=input_callback
        />
      </div>
    </label>
  }
}

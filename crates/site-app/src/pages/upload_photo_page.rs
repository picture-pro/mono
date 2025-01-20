mod context;

use gloo::file::FileList;
use leptos::{logging::debug_warn, prelude::*};
use web_sys::Event;

use self::context::{QueuedUploadFile, UploadContext, UploadContextProvider};
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

#[island]
fn ImagePreviewer() -> impl IntoView {
  let context: UploadContext = expect_context();

  let indices_iter = move || context.iter_file_indices();

  let grid_class = "grid sm:grid-cols-[repeat(auto-fit,12rem)] \
                    grid-cols-[repeat(auto-fit,8rem)] gap-4";

  view! {
    <div class=grid_class>
      <For
        each=indices_iter
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

  let context: UploadContext = expect_context();

  let url = move || context.get_file(index).map(|f| f.url().to_string());

  let image_class = "w-auto sm:max-h-[12rem] max-h-[8rem] border-2 \
                     border-base-8 dark:border-basedark-8 rounded-lg";
  let delete_button_class =
    "absolute top-0 right-0 size-8 flex flex-col justify-center items-center \
     bg-base-2 dark:bg-basedark-2 border-2 border-base-8 \
     dark:border-basedark-8 rounded-bl-lg rounded-tr-lg cursor-pointer";

  let delete_handler = move |_| {
    context.delete_file(index);
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
  let context: UploadContext = expect_context();

  // get file list
  let element: web_sys::HtmlInputElement = event_target(&ev);
  let Some(file_list) = element.files() else {
    debug_warn!("failed to get file list of event target");
    return;
  };

  // extract each image in file list
  for file in FileList::from(file_list).iter() {
    context.add_file(QueuedUploadFile::new(file.clone()));
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

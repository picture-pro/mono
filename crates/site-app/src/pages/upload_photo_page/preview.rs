use leptos::prelude::*;
use models::FileSize;

use super::{context::UploadContext, MAX_UPLOAD_SIZE};

#[island]
pub(super) fn ImagePreviewer() -> impl IntoView {
  let context: UploadContext = expect_context();

  let indices_iter = move || context.iter_file_indices();

  let grid_class =
    "grid xl:grid-cols-6 lg:grid-cols-5 md:grid-cols-4 grid-cols-3 gap-4";

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
  let is_oversized = move || {
    context
      .get_file(index)
      .map(|f| f.oversized())
      .is_some_and(|v| v)
  };

  let image_class = "w-full sm:max-h-48 max-h-40 border-2 border-base-8 \
                     dark:border-basedark-8 group-hover:border-primary-8 \
                     group-hover:dark:border-primarydark-8 ring-2 \
                     ring-transparent group-hover:ring-primary-8 \
                     group-hover:dark:ring-primarydark-8 transition-colors \
                     rounded-lg";

  let oversized_overlay_class =
    "absolute inset-0 flex flex-col sm:gap-2 gap-1 p-2 items-center \
     justify-center bg-base-1/[.8] dark:bg-basedark-1/[.8] rounded-lg \
     border-2 border-warning-8 dark:border-warningdark-8 text-center \
     text-warning-12 dark:text-warningdark-12 backdrop-blur-sm";
  let file_size = move || {
    context
      .get_file(index)
      .map(|f| FileSize::new(f.blob().size()))
  };
  let oversized_overlay_element = move || {
    is_oversized().then_some(view! {
      <div class=oversized_overlay_class>
        <ExclamationTriangleIcon {..} class="sm:size-10 size-8" />
        <div>
          <p class="sm:text-lg font-bold">"Oversized"</p>
          <p class="sm:text-sm text-xs text-warning-dim">
            "Image is too large to upload. File: "
            { move || file_size().map(|fs| fs.to_string()) }
            ", max: "
            { FileSize::new(MAX_UPLOAD_SIZE).to_string() }
          </p>
        </div>
      </div>
    })
  };

  let delete_button_class =
    "absolute top-0 right-0 size-8 flex flex-col justify-center items-center \
     bg-base-2 dark:bg-basedark-2 border-2 border-base-8 \
     dark:border-basedark-8 hover:border-danger-8 \
     hover:dark:border-dangerdark-8 rounded-bl-lg rounded-tr-lg \
     cursor-pointer text-base-dim hover:text-danger-dim transition-colors";
  let delete_handler = move |_| {
    context.delete_file(index);
  };
  let delete_button_element = move || {
    view! {
      <div class=delete_button_class on:click=delete_handler>
        <TrashIcon {..} class="size-6" />
      </div>
    }
  };

  move || {
    url().map(|url| {
      view! {
        <div class="flex flex-col justify-center items-center group animate-fade-in">
          <div class="relative">
            <img src={url} class=image_class />
            { oversized_overlay_element }
            { delete_button_element }
          </div>
        </div>
      }
    })
  }
}

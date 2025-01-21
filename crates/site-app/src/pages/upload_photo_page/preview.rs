use leptos::prelude::*;
use models::FileSize;

use super::{context::UploadContext, MAX_UPLOAD_SIZE};

#[island]
pub(super) fn ImagePreviewer() -> impl IntoView {
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
  let is_oversized = move || {
    context
      .get_file(index)
      .map(|f| f.oversized())
      .is_some_and(|v| v)
  };

  let image_class = "w-auto sm:max-h-[12rem] max-h-[8rem] border-2 \
                     border-base-8 dark:border-basedark-8 rounded-lg";

  let oversized_overlay_class =
    "absolute inset-0 bg-base-1 dark:bg-basedark-1 opacity-85";
  let oversized_overlay_inner_class =
    "absolute inset-0 flex flex-col justify-center gap-2 items-center \
     text-center text-base-12 dark:text-basedark-12";
  let file_size = move || {
    context
      .get_file(index)
      .map(|f| FileSize::new(f.blob().size()))
  };
  let oversized_overlay_element = move || {
    is_oversized().then_some(view! {
      <div class=oversized_overlay_class>
        <div class=oversized_overlay_inner_class>
          <ExclamationTriangleIcon {..} class="size-10" />
          <div>
            <p class="text-xl">"Oversized"</p>
            <p class="text-sm">
              "This image is too large to upload. File: "
              { move || file_size().map(|fs| fs.to_string()) }
              ", max: "
              { FileSize::new(MAX_UPLOAD_SIZE).to_string() }
            </p>
          </div>
        </div>
      </div>
    })
  };

  let delete_button_class =
    "absolute top-0 right-0 size-8 flex flex-col justify-center items-center \
     bg-base-2 dark:bg-basedark-2 border-2 border-base-8 \
     dark:border-basedark-8 rounded-bl-lg rounded-tr-lg cursor-pointer";
  let delete_handler = move |_| {
    context.delete_file(index);
  };
  let delete_button_element = move || {
    view! {
      <div class=delete_button_class on:click=delete_handler>
        <TrashIcon {..} class="size-6 text-base-dim" />
      </div>
    }
  };

  move || {
    url().map(|url| {
  view! {
    <div class="sm:size-48 size-32 flex flex-col justify-center items-center">
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

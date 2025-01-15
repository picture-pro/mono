use leptos::prelude::*;

use crate::components::Section;

#[component]
pub fn UploadPhotoPage() -> impl IntoView {
  view! {
    <Section>
      <p class="text-6xl font-serif font-light tracking-tight mb-4">
        "Upload Photo"
      </p>
    </Section>

    <div class="grid grid-cols-[1fr_2fr_1fr]">
      <div>
        <p class="text-base-dim">"Lorem ipsum blah blah blah"</p>
      </div>
      <UploadArea />
    </div>
  }
}

#[component]
fn UploadArea() -> impl IntoView {
  use lsc::icons::*;

  let class = "aspect-square justify-self-center max-w-md bg-base-2 \
               dark:bg-basedark-2 border-2 border-dashed border-base-8 \
               dark:border-basedark-8 rounded-xl flex flex-col justify-center \
               items-center gap-4";
  let icon_class = "size-24 text-basea-11 dark:text-basedarka-11";

  view! {
    <div class=class>
      <UploadIcon {..} class=icon_class />
      <div class="flex flex-col items-center gap-1 text-base-dim text-sm">
        <p class="text-base-dim">"This is the upload area"</p>
        <p class="text-base-dim">"Stuff gets uploaded here"</p>
      </div>
    </div>
  }
}

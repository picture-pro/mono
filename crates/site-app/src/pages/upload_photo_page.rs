mod context;
mod preview;
mod upload_area;

use leptos::prelude::*;

use self::{
  context::UploadContextProvider, preview::ImagePreviewer,
  upload_area::UploadArea,
};
use crate::components::Section;

pub const MAX_UPLOAD_SIZE: u64 = 50 * 1000 * 1000; // 50MB

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

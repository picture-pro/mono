mod context;
mod preview;
mod upload_area;

use context::{UploadContext, UploadStage};
use leptos::prelude::*;

use self::{
  context::ContextProvider, preview::ImagePreviewer, upload_area::UploadArea,
};
use crate::components::Section;

pub const MAX_UPLOAD_SIZE: u64 = 50 * 1000 * 1000; // 50MB

#[component]
pub fn UploadPhotoPage() -> impl IntoView {
  let desc_text = "Upload your photos here to put them on the PicturePro \
                   platform. Here you can adjust pricing and other options.";

  view! {
    <ContextProvider>
      <Section>
        <p class="text-6xl font-serif font-light tracking-tight mb-4">
          "Upload Photos"
        </p>
        <p class="text-base-dim">
          { desc_text }
        </p>
      </Section>

      <StageGuard target_stage={UploadStage::PhotosStage}>
        <Section>
          <UploadArea />
        </Section>
        <Section>
          <ImagePreviewer />
        </Section>
      </StageGuard>

      <Section>
        <StageGuard target_stage={UploadStage::PhotosStage}>
          <PhotosStageFooter />
        </StageGuard>
        <StageGuard target_stage={UploadStage::SettingsStage}>
          <SettingsStageFooter />
        </StageGuard>
      </Section>
    </ContextProvider>
  }
}

#[island]
fn StageGuard(target_stage: UploadStage, children: Children) -> impl IntoView {
  let context = expect_context::<UploadContext>();
  let current_stage = context.stage();
  let correct_stage = move || current_stage() == target_stage;
  let class = move || if correct_stage() { "" } else { "hidden" };

  view! {
    <div class=class>
      { children() }
    </div>
  }
}

#[island]
fn PhotosStageFooter() -> impl IntoView {
  use lsc::{button::*, icons::*};

  let context = expect_context::<UploadContext>();

  view! {
    <div class="w-full flex flex-row px-4">
      <div class="flex-1" />
      <Button size={ButtonSize::Large} {..}
        on:click={move |_| context.set_stage(UploadStage::SettingsStage)}
      >
        "Next"
        <ArrowRightIcon {..} class="size-6" />
      </Button>
    </div>
  }
}

#[island]
fn SettingsStageFooter() -> impl IntoView {
  use lsc::{button::*, icons::*};

  view! {
    <div class="w-full flex flex-row px-4">
      <div class="flex-1" />
      <Button color=ButtonColor::Success size=ButtonSize::Large>
        "Upload"
        <UploadIcon {..} class="size-6" />
      </Button>
    </div>
  }
}

mod next_step_button;
mod photo_preview;
mod upload_area;

use std::collections::HashMap;

use base_components::Section;
use leptos::prelude::*;
use models::Ulid;
use reactive_stores::Store;

use self::{
  next_step_button::NextStepButton, photo_preview::*, upload_area::*,
};
use super::photo::Photo;

#[component]
pub(super) fn SelectingPhotosStep() -> impl IntoView {
  view! {
    <Section>
      <p>"Selecting Photos"</p>
    </Section>

    <Section>
      <UploadArea />
    </Section>

    <Section>
      <div class="w-full flex flex-row justify-end">
        <NextStepButton />
      </div>
    </Section>

    <Section>
      <PhotoPreviewer />
    </Section>
  }
}

#[derive(Debug, Store)]
pub(super) struct SelectingPhotosState {
  pub photos: HashMap<Ulid, Photo>,
}

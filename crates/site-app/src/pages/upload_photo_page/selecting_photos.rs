mod photo_preview;
mod upload_area;

use std::collections::HashMap;

use leptos::prelude::*;
use models::Ulid;
use reactive_stores::Store;

use self::{photo_preview::*, upload_area::*};
use super::photo::Photo;
use crate::components::Section;

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
      <PhotoPreviewer />
    </Section>
  }
}

#[derive(Debug, Store)]
pub(super) struct SelectingPhotosState {
  pub photos: HashMap<Ulid, Photo>,
}

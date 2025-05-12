mod group_configurator;
mod next_step_button;
mod uploaded_photo;
mod uploaded_photo_preview;

use std::collections::HashMap;

use base_components::Section;
use leptos::prelude::*;
use models::{Ulid, UsdPriceNaive};
use next_step_button::NextStepButton;
use reactive_stores::Store;

pub use self::uploaded_photo::UploadedPhoto;
use self::{
  group_configurator::GroupConfigurator,
  uploaded_photo_preview::UploadedPhotoPreviewer,
};

#[component]
pub(super) fn ConfiguringGroupStep() -> impl IntoView {
  view! {
    <Section>
      <p>"Configuring Group"</p>
    </Section>

    <Section>
      <div class="flex flex-col gap-4 sm:flex-row sm:items-end">
        <GroupConfigurator />
        <div class="flex-1" />
        <NextStepButton />
      </div>
    </Section>

    <Section>
      <UploadedPhotoPreviewer />
    </Section>
  }
}

#[derive(Debug, Store)]
pub(super) struct ConfiguringGroupState {
  pub photos:             HashMap<Ulid, UploadedPhoto>,
  pub usage_rights_price: Option<UsdPriceNaive>,
}

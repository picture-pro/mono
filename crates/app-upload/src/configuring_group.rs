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
      <GroupConfigurator />
    </Section>

    <Section>
      <UploadedPhotoPreviewer />
    </Section>

    <Section>
      <NextStepButton />
    </Section>
  }
}

#[derive(Debug, Store)]
pub(super) struct ConfiguringGroupState {
  pub photos:             HashMap<Ulid, UploadedPhoto>,
  pub usage_rights_price: Option<UsdPriceNaive>,
}

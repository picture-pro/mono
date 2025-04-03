mod next_step_button;
mod uploaded_photo;
mod uploaded_photo_preview;

mod group_configurator {

  use leptos::prelude::*;
  use models::{UsdPriceNaive, PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE};
  use reactive_stores::Store;

  use crate::{
    pages::upload_photo_page::{
      configuring_group::ConfiguringGroupStateStoreFields,
      UploadStateStoreFields,
    },
    utils::inputs::touched_input_bindings,
  };

  #[island]
  pub(super) fn GroupConfigurator() -> impl IntoView {
    use lsc::field::*;

    let context: Store<super::super::UploadState> = expect_context();
    let state = context
      .configuring_group_0()
      .expect("`UploadContext` not in state `ConfiguringGroup`");

    let price = RwSignal::new(None::<String>);
    let (read_price, write_price) = touched_input_bindings(price);
    let validated_price = Memo::new(move |_| {
      let validated_price = price().as_ref().map(|v| {
        match v.parse::<f32>() {
          Ok(p) if p < 0.0 => Err("Price cannot be negative".to_owned()),
          Ok(p) => Ok(UsdPriceNaive::new_from_f32(p)),
          Err(_) if v.is_empty() => Err("Price is required.".to_owned()),
          Err(_) => Err("Price must be a number.".to_owned()),
        }
        .and_then(|p| match p {
          p if p < PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE => Err(format!(
            "Minimum price is {PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE}."
          )),
          p => Ok(p),
        })
      });

      validated_price
    });

    let field_error_text = move || {
      let Some(Err(error_text)) = validated_price() else {
        return None;
      };
      Some(view! {
        <p class="text-sm text-dangera-11 dark:text-dangerdarka-11">
          { error_text }
        </p>
      })
    };

    // I know this is a sin to update a signal from an effect, but I need to
    // downstream the validated_price into the store. We're explicitly
    // specifying the watched signals, so no infinite loops.
    Effect::watch(
      move || validated_price.get(),
      move |vp, _, _| {
        state
          .usage_rights_price()
          .set(vp.as_ref().cloned().and_then(Result::ok));
      },
      false,
    );

    view! {
      <div class="flex flex-col gap-1">
        <label class="" for="price">"Price"</label>
        <Field size={FieldSize::Large} {..} class=("max-w-md", true)
          placeholder="1.00" id="price" type="text"
          on:input=write_price prop:value=read_price
        />
        { field_error_text }
      </div>
    }
  }
}

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

use base_components::utils::inputs::touched_input_bindings;
use leptos::prelude::*;
use models::{PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE, UsdPriceNaive};
use reactive_stores::Store;

use super::ConfiguringGroupStateStoreFields;
use crate::UploadStateStoreFields;

fn validate_price_input(input: &str) -> Result<UsdPriceNaive, String> {
  match input.parse::<f32>() {
    Ok(p) if p < 0.0 => Err("Price cannot be negative".to_owned()),
    Ok(p) => Ok(UsdPriceNaive::new_from_f32(p)),
    Err(_) if input.is_empty() => Err("Price is required.".to_owned()),
    Err(_) => Err("Price must be a number.".to_owned()),
  }
  .and_then(|p| match p {
    p if p < PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE => Err(format!(
      "Minimum price is {PHOTO_GROUP_USAGE_RIGHTS_MINIMUM_PRICE}."
    )),
    p => Ok(p),
  })
}

#[island]
pub(super) fn GroupConfigurator() -> impl IntoView {
  use lsc::field::*;

  let context: Store<super::super::UploadState> = expect_context();

  let price = RwSignal::new(None::<String>);
  let (read_price, write_price) = touched_input_bindings(price);
  let validated_price =
    Memo::new(move |_| price().as_ref().map(|v| validate_price_input(v)));

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
      let state = context
        .configuring_group_0()
        .expect("`UploadContext` not in state `ConfiguringGroup`");
      state
        .usage_rights_price()
        .set(vp.clone().and_then(Result::ok));
    },
    false,
  );

  view! {
    <div class="flex flex-col gap-1">
      <label class="text-base-dim" for="price">"Price"</label>
      <Field size={FieldSize::Large} {..}
        placeholder="1.00" id="price" type="text"
        on:input=write_price prop:value=read_price
      />
      { field_error_text }
    </div>
  }
}

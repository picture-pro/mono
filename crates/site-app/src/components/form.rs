use std::any::type_name;

use leptos::*;
use validation::{NewType, NewTypeError};
use web_sys::Event;

pub struct ActiveFormElement<P: NewType> {
  pub field_read_signal:      ReadSignal<P::Inner>,
  pub field_write_signal:     WriteSignal<P::Inner>,
  pub display_name:           &'static str,
  pub html_form_input_type:   Option<&'static str>,
  pub skip_validate:          bool,
  pub skip_validate_on_empty: bool,
}

impl<P: NewType> IntoView for ActiveFormElement<P> {
  fn into_view(self) -> View {
    let ActiveFormElement {
      field_read_signal,
      field_write_signal,
      display_name,
      html_form_input_type,
      skip_validate,
      skip_validate_on_empty,
    } = self;

    let write_callback = move |ev: Event| {
      // attempt to parse the input value to the inner validation type
      let Ok(value) = event_target_value(&ev).parse() else {
        panic!(
          "Failed to parse input value to inner validation type `{}` for \
           field `{}`",
          type_name::<<P as NewType>::Inner>(),
          display_name
        );
      };
      field_write_signal(value)
    };
    let read_callback = move || field_read_signal().to_string();
    let validate_callback = move || {
      let value = field_read_signal();
      if skip_validate
        || (skip_validate_on_empty && value.to_string().is_empty())
      {
        return None;
      }
      let result = P::new(value);
      match result {
        Ok(_) => None,
        Err(err) => Some(err.to_string()),
      }
    };

    view! {
      <div class="d-form-control">
        <label class="d-label">{ display_name }</label>
        <input
          class="d-input d-input-bordered w-full max-w-xs"
          placeholder={ display_name } type=html_form_input_type.unwrap_or("text")
          on:input=write_callback value=read_callback
        />
        <p class="text-error/80 text-sm py-1">
          { validate_callback }
        </p>
      </div>
    }
    .into_view()
  }
}

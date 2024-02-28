use leptos::*;
use validation::{FieldValidate, Validate};

pub struct ActiveFormElement<
  P: Validate,
  S: Fn() -> P + 'static,
  R: Fn() -> String + 'static,
  W: Fn(String) + 'static,
> {
  params:             S,
  field_read_signal:  R,
  field_write_signal: W,
  display_name:       &'static str,
  field_name:         &'static str,
  input_type:         Option<&'static str>,
}

impl<
    P: Validate,
    S: Fn() -> P + 'static,
    R: Fn() -> String + 'static,
    W: Fn(String) + 'static,
  > ActiveFormElement<P, S, R, W>
{
  pub fn new(
    params: S,
    field_read_signal: R,
    field_write_signal: W,
    display_name: &'static str,
    field_name: &'static str,
    input_type: Option<&'static str>,
  ) -> Self {
    ActiveFormElement {
      params,
      field_read_signal,
      field_write_signal,
      display_name,
      field_name,
      input_type,
    }
  }
}

impl<
    P: Validate,
    S: Fn() -> P + Copy + 'static,
    R: Fn() -> String + Copy + 'static,
    W: Fn(String) + Copy + 'static,
  > IntoView for ActiveFormElement<P, S, R, W>
{
  fn into_view(self) -> View {
    let ActiveFormElement {
      params,
      field_read_signal,
      field_write_signal,
      display_name,
      field_name,
      input_type,
    } = self;
    view! {
      <div class="d-form-control">
        <label class="d-label">{ display_name }</label>
        <input
          class="d-input d-input-bordered w-full max-w-xs" placeholder={ display_name } type=input_type.unwrap_or("text")
          on:input=move |ev| {field_write_signal(event_target_value(&ev))} prop:value=field_read_signal
        />
        <p class="text-error">{ move || Some(params().field_validate(field_name)).filter(|_| !field_read_signal().is_empty()) }</p>
      </div>
    }
    .into_view()
  }
}

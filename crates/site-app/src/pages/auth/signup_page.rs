use leptos::*;
use validation::{FieldValidate, SignupParams, Validate};

use crate::pages::SmallPageWrapper;

struct ValidationMessage<
  P: Validate,
  S: Fn() -> P + 'static,
  F: Fn() -> String + 'static,
> {
  params:       S,
  field_signal: F,
  field_name:   &'static str,
}

impl<P: Validate, S: Fn() -> P + 'static, F: Fn() -> String + 'static>
  ValidationMessage<P, S, F>
{
  fn new(params: S, field_signal: F, field_name: &'static str) -> Self {
    ValidationMessage {
      params,
      field_signal,
      field_name,
    }
  }
}

impl<P: Validate, S: Fn() -> P + 'static, F: Fn() -> String + 'static> IntoView
  for ValidationMessage<P, S, F>
{
  fn into_view(self) -> View {
    let ValidationMessage {
      params,
      field_signal,
      field_name,
    } = self;
    view! {
      <p class="text-error">{ move || params().field_validate(field_name).filter(|_| !field_signal().is_empty()) }</p>
    }
    .into_view()
  }
}

#[island]
pub fn SignupPage() -> impl IntoView {
  let (name, set_name) = create_signal(String::new());
  let (email, set_email) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());
  let (confirm, set_confirm) = create_signal(String::new());

  let params = create_memo(move |_| {
    with!(|name, email, password, confirm| SignupParams {
      name:     name.clone(),
      email:    email.clone(),
      password: password.clone(),
      confirm:  confirm.clone(),
    })
  });

  let signup_action = create_server_action::<Signup>();
  let value = signup_action.value();

  view! {
    <SmallPageWrapper>
      <div class="d-card-body">
        <p class="d-card-title text-2xl">"Sign Up"</p>

        // name
        <div class="d-form-control">
          <label class="d-label">"Name"</label>
          <input
            type="text" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_name(event_target_value(&ev))} prop:value=name
          />
          { Some(ValidationMessage::new(params, name, "name")) }
        </div>

        // email
        <div class="d-form-control">
          <label class="d-label">"Email"</label>
          <input
            type="text" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_email(event_target_value(&ev))} prop:value=email
          />
          { Some(ValidationMessage::new(params, email, "email")) }
        </div>

        // password
        <div class="d-form-control">
          <label class="d-label">"Password"</label>
          <input
            type="password" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_password(event_target_value(&ev))} prop:value=password
          />
          { Some(ValidationMessage::new(params, password, "password")) }
        </div>

        // confirm password
        <div class="d-form-control">
          <label class="d-label">"Confirm Password"</label>
          <input
            type="password" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_confirm(event_target_value(&ev))} prop:value=confirm
          />
          { Some(ValidationMessage::new(params, confirm, "confirm")) }
        </div>

        { move || value().map(|v| match v {
          Err(e) => view! { <p class="text-error">{ e.to_string() }</p> }.into_view(),
          Ok(_) => view! { <p class="text-success">"Signed up!"</p> }.into_view(),
        })}

        // submit button
        <div class="mt-6"></div>
        <div class="d-form-control">
          <button class="d-btn d-btn-primary" on:click=move |_| {
            signup_action.dispatch(Signup {
              params: SignupParams {
                name: name(),
                email: email(),
                password: password(),
                confirm: confirm(),
              }
            });
          }>"Sign Up"</button>
        </div>
      </div>
    </SmallPageWrapper>
  }
}

#[server(Signup)]
pub async fn login(params: SignupParams) -> Result<(), ServerFnError> {
  params.validate().map_err(|e| {
    logging::error!("Invalid signup params: {:?}", e);
    ServerFnError::new(format!("Invalid signup params: {e}"))
  })?;

  let SignupParams {
    name,
    email,
    password,
    ..
  } = params;

  let auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;

  auth_session
    .backend
    .signup(name, email.clone(), password.clone())
    .await
    .map_err(|e| {
      logging::error!("Failed to sign up: {:?}", e);
      ServerFnError::new("Failed to sign up")
    })?;

  let _login_result = crate::pages::auth::login_page::login(email, password)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  Ok(())
}

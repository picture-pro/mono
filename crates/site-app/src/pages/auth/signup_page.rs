use leptos::*;
use validation::{LoginParams, SignupParams, Validate};

use crate::{components::form::FormElement, pages::SmallPageWrapper};

#[component]
pub fn SignupPage() -> impl IntoView {
  view! {
    <SmallPageWrapper>
      <SignupPageInner/>
    </SmallPageWrapper>
  }
}

#[island]
pub fn SignupPageInner() -> impl IntoView {
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
  let pending = signup_action.pending();

  view! {
    <div class="d-card-body">
      <p class="d-card-title text-2xl">"Sign Up to PicturePro"</p>

      { FormElement::new(params, name, set_name, "Name", "name", None).into_view() }
      { FormElement::new(params, email, set_email, "Email", "email", Some("email")).into_view() }
      { FormElement::new(params, password, set_password, "Password", "password", Some("password")).into_view() }
      { FormElement::new(params, confirm, set_confirm, "Confirm Password", "confirm", Some("password")).into_view() }

      { move || value().map(|v| match v {
        Ok(_) => view! {
          <p class="text-success">"Signed up!"</p>
          { crate::components::navigation::ClientNavInner::new(
            move || "/".to_string(),
          ) }
        }.into_view(),
        Err(e) => view! {<p class="text-error">{ e.to_string() }</p> }.into_view(),
      })}

      // submit button
      <div class="mt-6"></div>
      <div class="d-form-control">
        <button class="d-btn d-btn-primary" on:click=move |_| {
          signup_action.dispatch(Signup {
            params: params(),
          });
        }>
          { move || match pending() {
            true => Some(view! { <span class="d-loading d-loading-spinner" /> }),
            false => None,
          } }
          "Sign Up"
        </button>
      </div>
    </div>
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

  let _login_result = crate::pages::auth::login_page::login(LoginParams {
    email:    email.clone(),
    password: password.clone(),
  })
  .await
  .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  Ok(())
}

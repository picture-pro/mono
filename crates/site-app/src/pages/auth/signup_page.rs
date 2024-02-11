use leptos::{logging::log, *};

use crate::pages::SmallPageWrapper;

#[island]
pub fn SignupPage() -> impl IntoView {
  let (name, set_name) = create_signal(String::new());
  let (email, set_email) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());
  let (confirm, set_confirm) = create_signal(String::new());

  let name_validity = move || {
    name.clone().with(|name| {
      if name.is_empty() {
        return None;
      }
      if name.len() < 3 {
        return Some(
          view! { <p class="d-label text-error">"Name must be at least 3 characters long"</p> }
            .into_view(),
        );
      }
      None
    })
  };

  let email_validity = move || {
    email.clone().with(|email| {
      if email.is_empty() {
        return None;
      }
      Some(match validator::validate_email(email) {
        true => view! {}.into_view(),
        false => {
          view! { <p class="d-label text-error">"Invalid email address"</p> }
            .into_view()
        }
      })
    })
  };

  let password_validity = move || {
    password.clone().with(|password| {
      if password.is_empty() {
        return None;
      }
      if password.len() < 8 {
        return Some(
          view! { <p class="d-label text-error">"Password must be at least 8 characters long"</p> }
            .into_view(),
        );
      }
      None
    })
  };

  let confirm_validity = move || {
    confirm.clone().with(|confirm| {
      if confirm.is_empty() {
        return None;
      }
      if *confirm != password() {
        return Some(
          view! { <p class="d-label text-error">"Passwords do not match"</p> }
            .into_view(),
        );
      }
      None
    })
  };

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
          { name_validity }
        </div>

        // email
        <div class="d-form-control">
          <label class="d-label">"Email"</label>
          <input
            type="text" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_email(event_target_value(&ev))} prop:value=email
          />
          { email_validity }
        </div>

        // password
        <div class="d-form-control">
          <label class="d-label">"Password"</label>
          <input
            type="password" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_password(event_target_value(&ev))} prop:value=password
          />
          { password_validity }
        </div>

        // confirm password
        <div class="d-form-control">
          <label class="d-label">"Confirm Password"</label>
          <input
            type="password" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_confirm(event_target_value(&ev))} prop:value=confirm
          />
          { confirm_validity }
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
              name: name(),
              email: email(),
              password: password(),
              confirm: confirm()
            });
          }>"Sign Up"</button>
        </div>
      </div>
    </SmallPageWrapper>
  }
}

#[server(Signup)]
pub async fn login(
  name: String,
  email: String,
  password: String,
  confirm: String,
) -> Result<(), ServerFnError> {
  if name.len() < 3 {
    return Err(ServerFnError::new(
      "Name must be at least 3 characters long",
    ));
  }
  if email.is_empty() {
    return Err(ServerFnError::new("Email cannot be empty"));
  }
  if !validator::validate_email(&email) {
    return Err(ServerFnError::new("Invalid email address"));
  }
  if password.is_empty() {
    return Err(ServerFnError::new("Password cannot be empty"));
  }
  if password != confirm {
    return Err(ServerFnError::new("Passwords do not match"));
  }

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

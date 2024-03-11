use leptos::*;
use validation::{Email, LoginParams, Password};

use crate::{
  components::{
    form::{ActiveFormCheckboxElement, ActiveFormElement},
    navigation::navigate_to,
  },
  pages::SmallPageWrapper,
};

#[component]
pub fn LoginPage() -> impl IntoView {
  view! {
    <SmallPageWrapper>
      <LoginPageInner/>
    </SmallPageWrapper>
  }
}

#[island]
pub fn LoginPageInner() -> impl IntoView {
  // create the signals
  let (email, set_email) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());
  let (remember, set_remember) = create_signal(false);

  // create the params, aborting if validation fails
  let params: Memo<LoginParams> = create_memo(move |_| {
    with!(|email, password, remember| {
      LoginParams {
        email:    email.to_string(),
        password: password.to_string(),
        remember: *remember,
      }
    })
  });
  let disabled =
    move || with!(|email, password| email.is_empty() || password.is_empty());

  // create the form elements
  let email_element = ActiveFormElement::<Email> {
    field_read_signal:      email,
    field_write_signal:     set_email,
    display_name:           "Email",
    html_form_input_type:   Some("email"),
    skip_validate:          true,
    skip_validate_on_empty: false,
  };
  let password_element = ActiveFormElement::<Password> {
    field_read_signal:      password,
    field_write_signal:     set_password,
    display_name:           "Password",
    html_form_input_type:   Some("password"),
    skip_validate:          true,
    skip_validate_on_empty: false,
  };
  let remember_element = ActiveFormCheckboxElement {
    field_read_signal:  remember,
    field_write_signal: set_remember,
    display_name:       "Remember me",
  };

  // create the login action
  let login_action = create_server_action::<Login>();
  let value = login_action.value();
  let pending = login_action.pending();

  let submit_callback = move |_| {
    login_action.dispatch(Login { params: params() });
  };

  let result_message = move || {
    value().map(|v| match v {
      Ok(true) => view! {
        <p class="text-success">"Logged in!"</p>
      }
      .into_view(),
      Ok(false) => {
        view! { <p class="text-error">"Incorrect email or password"</p> }
          .into_view()
      }
      Err(e) => {
        let message = match e {
          ServerFnError::ServerError(e) => e,
          _ => e.to_string(),
        };
        view! {
          <p class="text-error">
            {format!("Error: {message}")}
          </p>
        }
        .into_view()
      }
    })
  };

  create_effect(move |_| {
    if matches!(value(), Some(Ok(true))) {
      navigate_to("/dashboard");
    }
  });

  view! {
    <div class="d-card-body">
      <p class="d-card-title text-2xl">"Login to PicturePro"</p>

      { email_element }
      { password_element }
      { remember_element }

      { result_message }

      // submit button
      <div class="mt-6"></div>
      <div class="d-form-control">
        <button
          class="d-btn d-btn-primary"
          disabled=disabled on:click=submit_callback
        >
          { move || pending().then(|| view! { <span class="d-loading d-loading-spinner" /> })}
          "Login"
        </button>
      </div>
    </div>
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn login(params: LoginParams) -> Result<bool, ServerFnError> {
  // construct the nutype wrappers and fail if validation fails
  let _ = Email::new(params.email.clone())
    .map_err(|e| ServerFnError::new(format!("Invalid email: {e}")))?;
  let _ = Password::new(params.password.clone())
    .map_err(|e| ServerFnError::new(format!("Invalid password: {e}")))?;

  let creds = auth::Credentials {
    email:    params.email,
    password: params.password,
    remember: params.remember,
  };
  let mut auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;
  let session = use_context::<tower_sessions::Session>()
    .ok_or_else(|| ServerFnError::new("Failed to get session"))?;

  let user = match auth_session.authenticate(creds.clone()).await {
    Ok(Some(user)) => user,
    Ok(None) => return Ok(false),
    Err(e) => {
      return Err(ServerFnError::new(format!("Failed to authenticate: {e}")))
    }
  };

  auth_session
    .login(&user)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  if creds.remember {
    session.set_expiry(Some(tower_sessions::Expiry::AtDateTime(
      time::OffsetDateTime::now_utc() + time::Duration::days(30),
    )));
  }

  tracing::info!("logged in user: {} ({})", user.name, user.id.0);
  leptos_axum::redirect("/");

  Ok(true)
}

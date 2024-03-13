use leptos::*;
#[cfg(feature = "ssr")]
use validation::LoginParams;
use validation::{Email, Name, Password, SignupParams};

use crate::{
  components::{
    form::{ActiveFormCheckboxElement, ActiveFormElement},
    navigation::navigate_to,
  },
  pages::SmallPageWrapper,
};

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
  let (remember, set_remember) = create_signal(false);

  let params: Memo<Option<SignupParams>> = create_memo(move |_| {
    with!(|name, email, password, confirm, remember| {
      let _ = Name::new(name.clone()).ok()?;
      let _ = Email::new(email.clone()).ok()?;
      let _ = Password::new(password.clone()).ok()?;
      if password != confirm {
        return None;
      }
      Some(SignupParams {
        name:     name.clone(),
        email:    email.clone(),
        password: password.clone(),
        remember: *remember,
      })
    })
  });
  let disabled = move || with!(|params| params.is_none());

  // create the form elements
  let name_element = ActiveFormElement::<Name> {
    field_read_signal:      name,
    field_write_signal:     set_name,
    display_name:           "Name",
    html_form_input_type:   None,
    skip_validate:          false,
    skip_validate_on_empty: true,
  };
  let email_element = ActiveFormElement::<Email> {
    field_read_signal:      email,
    field_write_signal:     set_email,
    display_name:           "Email",
    html_form_input_type:   Some("email"),
    skip_validate:          false,
    skip_validate_on_empty: true,
  };
  let password_element = ActiveFormElement::<Password> {
    field_read_signal:      password,
    field_write_signal:     set_password,
    display_name:           "Password",
    html_form_input_type:   Some("password"),
    skip_validate:          false,
    skip_validate_on_empty: true,
  };
  let confirm_element = ActiveFormElement::<Password> {
    field_read_signal:      confirm,
    field_write_signal:     set_confirm,
    display_name:           "Confirm Password",
    html_form_input_type:   Some("password"),
    skip_validate:          true,
    skip_validate_on_empty: true,
  };
  let remember_element = ActiveFormCheckboxElement {
    initial_value:      true,
    field_write_signal: set_remember,
    display_name:       "Remember Me",
  };

  let signup_action = create_server_action::<Signup>();
  let value = signup_action.value();
  let pending = signup_action.pending();

  let submit_callback = move |ev: leptos::ev::SubmitEvent| {
    ev.prevent_default();
    signup_action.dispatch(Signup {
      params: params().unwrap(),
    });
  };

  let result_message = move || {
    value().map(|v| match v {
      Ok(_) => view! {
        <p class="text-success">"Signed up!"</p>
      }
      .into_view(),
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
    if matches!(value(), Some(Ok(_))) {
      navigate_to("/dashboard");
    }
  });

  view! {
    <form class="d-card-body" on:submit=submit_callback>
      <p class="d-card-title text-2xl">"Sign Up to PicturePro"</p>

      { name_element }
      { email_element }
      { password_element }
      { confirm_element }
      { remember_element }

      { result_message }

      // submit button
      <div class="mt-6"></div>
      <div class="d-form-control">
        <button
          class="d-btn d-btn-primary"
          disabled=disabled type="submit"
        >
          { move || pending().then(|| view! { <span class="d-loading d-loading-spinner" /> })}
          "Sign Up"
        </button>
      </div>
    </form>
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn signup(params: SignupParams) -> Result<(), ServerFnError> {
  // construct the nutype wrappers and fail if validation fails
  let _ = Name::new(params.name.clone())
    .map_err(|e| ServerFnError::new(format!("Invalid name: {e}")))?;
  let _ = Email::new(params.email.clone())
    .map_err(|e| ServerFnError::new(format!("Invalid email: {e}")))?;
  let _ = Password::new(params.password.clone())
    .map_err(|e| ServerFnError::new(format!("Invalid password: {e}")))?;

  let SignupParams {
    name,
    email,
    password,
    remember,
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
    email: email.clone(),
    password: password.clone(),
    remember,
  })
  .await
  .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  Ok(())
}

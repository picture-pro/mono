use leptos::*;
use validation::{Email, LoginParams, Password};

use crate::{
  components::{form::ActiveFormElement, navigation::navigate_to},
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
  let (email, set_email) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());

  // create the params, aborting if validation fails
  let params: Memo<Option<LoginParams>> = create_memo(move |_| {
    with!(|email, password| {
      Some(LoginParams {
        email:    Email::new(email).ok()?,
        password: Password::new(password).ok()?,
      })
    })
  });
  let disabled = move || params().is_none();

  let login_action = create_server_action::<Login>();
  let value = login_action.value();
  let pending = login_action.pending();

  let submit_callback = move |_| {
    login_action.dispatch(Login {
      params: params().unwrap(),
    });
  };

  create_effect(move |_| {
    if matches!(value(), Some(Ok(true))) {
      navigate_to("/dashboard");
    }
  });

  view! {
    <div class="d-card-body">
      <p class="d-card-title text-2xl">"Login to PicturePro"</p>

      { ActiveFormElement::<Email>::new(email, set_email, "Email", Some("email")).into_view() }
      { ActiveFormElement::<Password>::new(password, set_password, "Password", Some("password")).into_view() }

      // result message
      { move || value().map(|v| match v {
        Ok(true) => view! {
          <p class="text-success">"Logged in!"</p>
        }.into_view(),
        Ok(false) => view! { <p class="text-error">"Incorrect email or password"</p> }.into_view(),
        Err(e) => view! {<p class="text-error">{format!("Error: {}", e)}</p> }.into_view(),
      })}

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
#[server(Login)]
pub async fn login(params: LoginParams) -> Result<bool, ServerFnError> {
  // we don't validate any creds on ingress here because `nutype` keeps us from
  // deserializing invalid ones
  // see `nutype` crate for more details

  let creds = auth::Credentials {
    email:    params.email.into_inner(),
    password: params.password.into_inner(),
  };
  let mut auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;

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

  tracing::info!("logged in user: {} ({})", user.name, user.id.0);
  leptos_axum::redirect("/");

  Ok(true)
}

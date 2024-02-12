use leptos::{logging::log, *};
use validation::{FieldValidate, LoginParams, Validate};

use crate::{components::form::FormElement, pages::SmallPageWrapper};

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

  let params = create_memo(move |_| {
    with!(|email, password| LoginParams {
      email:    email.clone(),
      password: password.clone(),
    })
  });

  let login_action = create_server_action::<Login>();
  let value = login_action.value();

  view! {
    <div class="d-card-body">
      <p class="d-card-title text-2xl">"Login to PicturePro"</p>

      { FormElement::new(params, email, set_email, "Email", "email", Some("email")).into_view() }
      { FormElement::new(params, password, set_password, "Password", "password", Some("password")).into_view() }

      // error message
      { move || value().map(|v| match v {
        Ok(true) => view! { <p class="text-success">"Logged in!"</p> }.into_view(),
        Ok(false) => view! { <p class="text-error">"Incorrect email or password"</p> }.into_view(),
        Err(e) => view! { <p class="text-error">{format!("Error: {}", e)}</p> }.into_view(),
      })}

      // submit button
      <div class="mt-6"></div>
      <div class="d-form-control">
        <button class="d-btn d-btn-primary" on:click=move |_| {
          login_action.dispatch(Login { params: params() });
        }>"Login"</button>
      </div>
    </div>
  }
}

#[server(Login)]
pub async fn login(params: LoginParams) -> Result<bool, ServerFnError> {
  params.validate().map_err(|e| {
    logging::error!("Invalid signup params: {:?}", e);
    ServerFnError::new(format!("Invalid login params: {}", e))
  })?;

  let creds = auth::Credentials {
    email:    params.email,
    password: params.password,
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

  log!("logged in user: {} ({})", user.name, user.id);
  leptos_axum::redirect("/");

  Ok(true)
}

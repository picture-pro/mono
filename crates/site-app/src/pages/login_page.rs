use leptos::{logging::log, *};

#[island]
pub fn LoginPage() -> impl IntoView {
  let (email, set_email) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());

  let login_action = create_server_action::<Login>();
  let value = login_action.value();

  view! {
    <super::SmallPageWrapper>
      <div class="d-card-body">
        <p class="d-card-title text-2xl">"Login"</p>

        // email
        <div class="d-form-control">
          <label class="d-label">"Email"</label>
          <input
            type="text" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_email(event_target_value(&ev))} prop:value=email
          />
        </div>

        // password
        <div class="d-form-control">
          <label class="d-label">"Password"</label>
          <input
            type="password" class="d-input d-input-bordered w-full max-w-xs"
            on:input=move |ev| {set_password(event_target_value(&ev))} prop:value=password
          />
        </div>

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
            login_action.dispatch(Login { email: email(), password: password() });
          }>"Login"</button>
        </div>
      </div>
    </super::SmallPageWrapper>
  }
}

#[server(Login)]
pub async fn login(
  email: String,
  password: String,
) -> Result<bool, ServerFnError> {
  let creds = auth::Credentials { email, password };
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

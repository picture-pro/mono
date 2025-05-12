use base_components::{
  utils::{
    inputs::touched_input_bindings,
    navigation::{navigate_to, sanitize_auth_next_url},
  },
  FloatingBoxSection, Prose,
};
use leptos::prelude::*;
use lsc::{button::*, field::*};

#[component]
pub fn LoginPage() -> impl IntoView {
  let query = leptos_router::hooks::use_query_map();
  let next_url = Signal::derive(move || query().get("next"));

  view! {
    <LoginPageIsland next_url={next_url.get_untracked()} />
  }
}

#[island]
pub fn LoginPageIsland(next_url: Option<String>) -> impl IntoView {
  use lsc::link::*;

  let signup_url = format!(
    "/sign-up{}",
    next_url
      .clone()
      .map(|nu| format!("?next={nu}"))
      .unwrap_or_default()
  );
  let next_url = sanitize_auth_next_url(next_url);

  let email = RwSignal::new(None::<String>);

  let (read_email_callback, write_email_callback) =
    touched_input_bindings(email);

  let action =
    Action::new(move |(): &()| login(email.get().unwrap_or_default()));
  let action_value = action.value();
  let action_value_view = move || {
    action_value.get().map(|v| match v {
      Ok(true) => leptos::either::EitherOf3::A(view! {
        <p class="text-success-11 dark:text-successdark-11">"Logging in..."</p>
      }),
      Ok(false) => leptos::either::EitherOf3::B(view! {
        <p class="text-warning-11 dark:text-warningdark-11">"Invalid credentials."</p>
      }),
      Err(e) => leptos::either::EitherOf3::C(view! {
        <p class="text-danger-11 dark:text-dangerdark-11">{ e.to_string() }</p>
      }),
    })
  };

  Effect::new(move |_| {
    if matches!(action_value.get(), Some(Ok(true))) {
      navigate_to(&next_url);
    }
  });

  view! {
    <FloatingBoxSection>
      <p class="text-3xl font-serif font-semibold tracking-tight">
        "Log into your account"
      </p>

      <Prose>
        "Don't have an account? "
        <Link size=LinkSize::Medium underline={LinkUnderline::Always} {..} href=signup_url>
          "Sign up"
        </Link>
        "."
      </Prose>

      <form class="mt-2 mb-4 flex flex-col gap-4">
        <div class="flex flex-col gap-1">
          <label class="" for="email">"Email"</label>
          <Field size={FieldSize::Large} {..}
            placeholder="Enter your email" type="email" id="email"
            on:input=write_email_callback prop:value=read_email_callback
          />
        </div>
      </form>

      <div class="flex flex-row">
        <div class="flex-1" />
        <Button size={ButtonSize::Large} {..} on:click={move |_| {action.dispatch(());}}>
          "Log in"
          <lsc::icons::ArrowRightIcon {..} class="size-5" />
        </Button>
      </div>

      { move || action_value_view().map(|v| view! {
        <div class="self-center mt-4">{ v }</div>
      })}
    </FloatingBoxSection>
  }
}

#[server(name = LoginActionParams)]
#[tracing::instrument]
async fn login(email: String) -> Result<bool, ServerFnError> {
  use auth_domain::{AuthDomainService, AuthSession};
  use models::{EmailAddress, PublicUser, UserAuthCredentials};

  let auth_service = use_context::<AuthDomainService>().ok_or_else(|| {
    tracing::error!("auth service not found");
    ServerFnError::new("Internal error")
  })?;
  let mut auth_session =
    leptos_axum::extract::<AuthSession>().await.map_err(|_| {
      tracing::error!("auth session not found");
      ServerFnError::new("Internal error")
    })?;

  let email = EmailAddress::try_new(email)
    .map_err(|_| ServerFnError::new("Email address is invalid"))?;

  let creds = UserAuthCredentials::EmailEntryOnly(email.clone());

  let user = auth_service.user_authenticate(creds).await.map_err(|e| {
    tracing::error!("failed to fetch user: {e}");
    ServerFnError::new("Internal error")
  })?;

  let Some(user) = user else {
    return Ok(false);
  };
  let public_user = PublicUser::from(user);

  auth_session.login(&public_user).await.map_err(|e| {
    tracing::error!("failed to fetch user: {e}");
    ServerFnError::new("Internal error")
  })?;

  Ok(true)
}

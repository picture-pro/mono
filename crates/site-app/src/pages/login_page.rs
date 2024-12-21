use leptos::prelude::*;
use lsc::{button::*, field::*};

use crate::{
  components::FloatingBoxSection, utils::inputs::touched_input_bindings,
};

#[island]
pub fn LoginPage() -> impl IntoView {
  let email = RwSignal::new(None::<String>);

  let (read_email_callback, write_email_callback) =
    touched_input_bindings(email);

  let action =
    Action::new(move |_: &()| login(email.get().unwrap_or_default()));
  let action_value = action.value();
  let action_value_view = move || {
    action_value().map(|v| match v {
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
    if matches!(action_value(), Some(Ok(true))) {
      crate::utils::navigation::navigate_to("/profile");
    }
  });

  view! {
    <FloatingBoxSection>
      <p class="text-3xl font-serif font-semibold tracking-tight">
        "Log into your account"
      </p>

      <p class="text-base-dim max-w-prose">
        "This is some placeholder text. More placeholder text will hold the place of text whose place needs to be held."
      </p>

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
async fn login(email: String) -> Result<bool, ServerFnError> {
  use auth_domain::{AuthDomainService, AuthSession, DynAuthDomainService};
  use models::{EmailAddress, PublicUser, UserAuthCredentials};

  let auth_service =
    use_context::<DynAuthDomainService>().ok_or_else(|| {
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

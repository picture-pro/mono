use base_components::FloatingBoxSection;
use leptos::prelude::*;
use lsc::{button::*, icons::*};

#[component]
pub fn LogoutPage() -> impl IntoView {
  view! {
    <FloatingBoxSection>
      <p class="text-3xl font-serif font-semibold tracking-tight">
        "Are you sure?"
      </p>

      <p class="text-base-dim max-w-prose">
        "You can always log back in later. Come back soon!"
      </p>

      <div class="flex flex-row justify-between items-center mt-8">
        <Button color=ButtonColor::Base size=ButtonSize::Large>
          <ArrowLeftIcon {..} class="size-5" />
          "Cancel"
        </Button>
        <LogoutButton />
      </div>
    </FloatingBoxSection>
  }
}

#[island]
fn LogoutButton() -> impl IntoView {
  let action = Action::new(move |_: &()| logout());
  let action_value = action.value();

  Effect::new(move |_| {
    if matches!(action_value(), Some(Ok(()))) {
      crate::utils::navigation::navigate_to("/");
    }
  });

  view! {
    <Button
      color=ButtonColor::Danger variant=ButtonVariant::Solid size={ButtonSize::Large}
      {..}
      on:click={move |_| { action.dispatch(()); }}
    >
      "Log Out"
      <ArrowRightIcon {..} class="size-5" />
    </Button>
  }
}

#[server(name = LogoutActionParams)]
async fn logout() -> Result<(), ServerFnError> {
  use auth_domain::AuthSession;

  let mut auth_session =
    leptos_axum::extract::<AuthSession>().await.map_err(|_| {
      tracing::error!("auth session not found");
      ServerFnError::new("Internal error")
    })?;

  auth_session.logout().await.map_err(|e| {
    tracing::error!("logout failed: {:?}", e);
    ServerFnError::new("Internal error")
  })?;

  leptos_axum::redirect("/");

  Ok(())
}

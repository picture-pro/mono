use base_components::utils::navigation::url_to_full_path;
use leptos::{either::Either, prelude::*};
use leptos_router::location::Url;
use lsc::{button::*, link::*};
use models::{AuthStatus, AuthUser};

#[component]
pub fn Header() -> impl IntoView {
  let class = "sticky top-0 container bg-base-subtle h-12 mx-auto px-3 flex \
               flex-row items-center justify-between gap-2 rounded-b-xl \
               border border-t-0 border-base-7 dark:border-basedark-7";

  let auth_status = use_context::<AuthStatus>().and_then(|as_| as_.0);
  let logo_link_dest = if auth_status.is_some() {
    "/profile"
  } else {
    "/"
  };

  view! {
    <header class=class>
      <span class="font-serif font-semibold tracking-tight">
        <Link
          color=LinkColor::Base size=LinkSize::ExtraLarge underline=LinkUnderline::Always high_contrast=true
          {..}
          href=logo_link_dest
        >
          "PicturePro"
        </Link>
      </span>
      <div class="flex flex-row items-center gap-2">
        <HeaderUserArea />
      </div>
    </header>
  }
}

#[component]
fn HeaderUserArea() -> impl IntoView {
  let auth_status = use_context::<AuthStatus>().and_then(|as_| as_.0);

  match auth_status {
    Some(user) => Either::Left(view! { <LoggedInUserAuthActions user=user /> }),
    None => Either::Right(view! { <LoggedOutUserAuthActions /> }),
  }
}

#[component]
fn LoggedOutUserAuthActions() -> impl IntoView {
  let query = leptos_router::hooks::use_query_map();
  // if we already have a `next` url
  let existing_next_url = Signal::derive(move || query().get("next"));

  // if we need a `next_url`
  let return_url = leptos_router::hooks::use_url();
  let escaped_return_url =
    Signal::derive(move || Url::escape(&url_to_full_path(&return_url())));

  // use the existing `next` url if it exists, rather than setting it to the
  // current page
  let redirect_url = Signal::derive(move || match existing_next_url() {
    Some(existing_next_url) => existing_next_url,
    _ => escaped_return_url(),
  });

  let signup_url =
    Signal::derive(move || format!("/sign-up?next={}", redirect_url()));
  let login_url =
    Signal::derive(move || format!("/log-in?next={}", redirect_url()));

  view! {
    <Button
      element_type=ButtonElementType::Link color={ButtonColor::Primary}
      {..} href=signup_url
    >
      "Sign Up"
    </Button>
    <Button
      element_type=ButtonElementType::Link color={ButtonColor::Base}
      {..} href=login_url
    >
      "Log In"
    </Button>
  }
}

#[component]
fn LoggedInUserAuthActions(user: AuthUser) -> impl IntoView {
  view! {
    <span class="text-sm text-base-dim leading-none text-right">
      "Welcome, "
      <Link size=LinkSize::Small underline={LinkUnderline::Always} {..} href="/profile">
        { user.name.to_string() }
      </Link>
    </span>
    <Button
      element_type=ButtonElementType::Link color={ButtonColor::Base}
      {..} href="/log-out"
    >
      "Log Out"
    </Button>
  }
}

use leptos::{either::Either, prelude::*};
use leptos_router::location::Url;
use lsc::{button::*, link::*};
use models::{AuthStatus, PublicUser};

#[component]
pub fn Header() -> impl IntoView {
  let class = "sticky top-0 container bg-base-subtle h-12 mx-auto px-3 flex \
               flex-row items-center justify-between gap-2 rounded-b-xl \
               border border-t-0 border-base-7 dark:border-basedark-7";

  let auth_status = use_context::<AuthStatus>().and_then(|as_| as_.0);
  let logo_link_dest = match auth_status.is_some() {
    true => "/profile",
    false => "/",
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
  let return_url = leptos_router::hooks::use_url();
  let escaped_return_url =
    Signal::derive(move || Url::escape(&url_to_full_path(&return_url())));
  let signup_url =
    Signal::derive(move || format!("/sign-up?next={}", escaped_return_url()));
  let login_url =
    Signal::derive(move || format!("/log-in?next={}", escaped_return_url()));

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
fn LoggedInUserAuthActions(user: PublicUser) -> impl IntoView {
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

// taken from https://github.com/leptos-rs/leptos/blob/2ee4444bb44310e73e908b98ccd2b353f534da01/router/src/location/mod.rs#L87-L100
fn url_to_full_path(url: &Url) -> String {
  let mut path = url.path().to_string();
  if !url.search().is_empty() {
    path.push('?');
    path.push_str(url.search());
  }
  if !url.hash().is_empty() {
    if !url.hash().starts_with('#') {
      path.push('#');
    }
    path.push_str(url.hash());
  }
  path
}

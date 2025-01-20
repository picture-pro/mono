use leptos::{either::Either, prelude::*};
use lsc::{button::*, link::*};
use models::PublicUser;

use crate::AuthStatus;

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
  view! {
    <Button is_link=true color={ButtonColor::Primary} {..} href="/sign-up">
      "Sign Up"
    </Button>
    <Button is_link=true color={ButtonColor::Base} {..} href="/log-in">
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
    <Button is_link=true color={ButtonColor::Base} {..} href="/log-out">
      "Log Out"
    </Button>
  }
}

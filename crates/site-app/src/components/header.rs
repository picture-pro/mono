use leptos::{either::Either, prelude::*};
use lsc::{button::*, link::*};
use models::PublicUser;

use crate::AuthStatus;

#[component]
pub fn Header() -> impl IntoView {
  let class = "sticky top-0 container bg-base-subtle h-12 mx-auto px-3 flex \
               flex-row items-center rounded-b-xl border border-t-0 \
               border-base-7 dark:border-basedark-7";

  view! {
    <header class=class>
      <span class="font-serif font-semibold tracking-tight">
        <Link href="/" color=LinkColor::Base size=LinkSize::ExtraLarge underline=LinkUnderline::Hover high_contrast=true>
          "PicturePro"
        </Link>
      </span>
      <div class="flex-1" />
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
    <Button href="/sign-up" color=ButtonColor::Primary>
      "Sign Up"
    </Button>
    <Button href="/log-in" color=ButtonColor::Base>
      "Log In"
    </Button>
  }
}

#[component]
fn LoggedInUserAuthActions(user: PublicUser) -> impl IntoView {
  view! {
    <span class="text-sm text-basea-11 dark:text-basedarka-11">
      "Welcome, "
      <span class="text-primarya-11 dark:text-primarydarka-11">
        { user.name.to_string() }
      </span>
    </span>
    <Button href="/log-out" color=ButtonColor::Base>
      "Log Out"
    </Button>
  }
}

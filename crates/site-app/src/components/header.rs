use leptos::prelude::*;
use lsc::*;

use crate::AuthStatus;

#[component]
pub fn Header() -> impl IntoView {
  let class = "sticky top-0 container bg-base-subtle h-12 mx-auto px-3 flex \
               flex-row items-center rounded-b-xl border border-t-0 \
               border-base-7 dark:border-basedark-7";

  let auth_status = use_context::<AuthStatus>();
  let auth_status_text = format!("Status: {:?}", auth_status);

  view! {
    <header class=class>
      <span class="font-serif font-semibold tracking-tight">
        <Link href="/" color=LinkColor::Base size=LinkSize::Large underline=LinkUnderline::Hover high_contrast=true>
          "PicturePro"
        </Link>
      </span>
      <div class="flex-1" />
      <div class="flex flex-row items-center gap-2">
        { auth_status_text }
        <Button href="/sign-up" color=ButtonColor::Primary>
          "Sign Up"
        </Button>
        <Button href="/log-in" color=ButtonColor::Base>
          "Log In"
        </Button>
      </div>
    </header>
  }
}

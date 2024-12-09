use leptos::prelude::*;
use lsc::*;

#[component]
pub fn Header() -> impl IntoView {
  let class = "sticky top-0 container bg-base-subtle h-12 mx-auto px-2 flex \
               flex-row items-center rounded-b-xl border border-t-0 \
               border-base-7 dark:border-basedark-7";

  view! {
    <header class=class>
      <span class="font-serif font-semibold tracking-tight">
        <Link href="/" color=LinkColor::Base size=LinkSize::Large high_contrast=true>
          "PicturePro"
        </Link>
      </span>
      <div class="flex-1" />
      <Button>
        "Sign In"
      </Button>
    </header>
  }
}

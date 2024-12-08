use leptos::prelude::*;
use lsc::*;

#[component]
pub fn Header() -> impl IntoView {
  let class = "sticky top-0 container bg-base-2 dark:bg-basedark-1 h-12 \
               mx-auto text-base-dim px-4 flex flex-row items-center \
               rounded-b-xl border border-t-0 border-base-6 \
               dark:border-basedark-6 shadow";

  view! {
    <header class=class>
      <p class="text-xl font-semibold tracking-tight">"PicturePro"</p>
      <div class="flex-1" />
      <Button>
        "Sign In"
      </Button>
    </header>
  }
}

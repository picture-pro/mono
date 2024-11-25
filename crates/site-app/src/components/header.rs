use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
  let outer_class = "bg-basea-2 dark:bg-basedarka-2 text-base-dim w-full h-12";
  let inner_class = "container mx-auto h-full flex flex-row items-center";

  view! {
    <header class=outer_class>
      <div class=inner_class>
        <p class="text-xl font-semibold tracking-tight">"PicturePro"</p>
      </div>
    </header>
  }
}

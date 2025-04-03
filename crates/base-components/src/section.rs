use leptos::prelude::*;

#[component]
pub fn Section(children: Children) -> impl IntoView {
  view! {
    <section class="my-12 space-y-4">
      { children() }
    </section>
  }
}

#[component]
pub fn FloatingBoxSection(children: Children) -> impl IntoView {
  let class = "md:mt-24 mt-12 mx-auto max-w-md w-full rounded-md \
               bg-base-subtle border border-base-7 dark:border-basedark-7 p-6 \
               flex flex-col gap-4 shadow-xl";

  view! {
    <section class=class>
      { children() }
    </section>
  }
}

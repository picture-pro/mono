use leptos::prelude::*;
use lsc::link::*;

use crate::components::Section;

#[component]
pub fn NotFoundPage() -> impl IntoView {
  view! {
    <Section>
      <p class="text-6xl font-serif font-light tracking-tight mb-4">
        "Page not found."
      </p>
      <p class="max-w-prose text-base-dim">
        "We don't have the page you requested on hand. "
        <Link {..} href="/">
          "Go home."
        </Link>
      </p>
    </Section>
  }
}

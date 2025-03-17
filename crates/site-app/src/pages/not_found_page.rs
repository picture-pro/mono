use leptos::prelude::*;
use lsc::link::*;

use crate::components::{Section, Title};

#[component]
pub fn NotFoundPage() -> impl IntoView {
  view! {
    <Section>
      <Title>"Page not found."</Title>
      <p class="max-w-prose text-base-dim">
        "We don't have the page you requested on hand. "
        <Link {..} href="/">
          "Go home."
        </Link>
      </p>
    </Section>
  }
}

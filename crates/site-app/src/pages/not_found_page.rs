use base_components::{Prose, Section, Title};
use leptos::prelude::*;
use lsc::link::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
  view! {
    <Section>
      <Title>"Page not found."</Title>
      <Prose>
        "We don't have the page you requested on hand. "
        <Link {..} href="/">
          "Go home."
        </Link>
      </Prose>
    </Section>
  }
}

use leptos::prelude::*;

use crate::components::Section;

#[component]
pub fn ProfilePage() -> impl IntoView {
  view! {
    <Section>
      <p class="text-6xl font-serif font-light tracking-tight mb-4">
        "User Profile"
      </p>
    </Section>
    <Section>
      <p class="max-w-prose text-base-dim">"We've got nothing else to display here right now."</p>
      <p class="max-w-prose text-base-dim">"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</p>
    </Section>
  }
}

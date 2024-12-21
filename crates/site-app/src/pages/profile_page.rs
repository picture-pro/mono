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
      <pre><code>
      </code></pre>
    </Section>
  }
}

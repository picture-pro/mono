use leptos::prelude::*;

use crate::components::Section;

pub fn HomePage() -> impl IntoView {
  view! {
    <Section>
      <p class="text-5xl font-serif font-semibold tracking-tight mb-4">
        "Welcome to PicturePro"
      </p>
      <p class="max-w-prose">
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Duo Reges: constructio interrete. Quae cum dixisset, finem ille."
      </p>
    </Section>
  }
}

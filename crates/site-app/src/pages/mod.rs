use leptos::{either::Either, prelude::*};

use crate::components::Section;

#[component]
pub fn HomePage() -> impl IntoView {
  let fallback = move || {
    view! {
      <p>"Loading photos..."</p>
    }
  };
  let photos = Resource::new(|| (), |_| enumerate_photos());
  let photos_suspense_viewer = move || {
    Suspend::new(async move {
      match photos.await {
        Ok(photos) => Either::Left(view! {
          <pre>{ format!("{photos:#?}") }</pre>
        }),
        Err(e) => Either::Right(view! {
          <pre>{ format!("{e}") }</pre>
        }),
      }
    })
  };

  view! {
    <Section>
      <p class="text-5xl font-serif font-semibold tracking-tight mb-4">
        "Welcome to PicturePro"
      </p>
      <p class="max-w-prose">
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nonne merninisti licere mihi ista probare, quae sunt a te dicta? Duo Reges: constructio interrete. Quae cum dixisset, finem ille."
      </p>
      <Suspense fallback>
        { photos_suspense_viewer }
      </Suspense>
    </Section>
  }
}

#[server]
pub async fn enumerate_photos() -> Result<Vec<models::Photo>, ServerFnError> {
  let service: prime_domain::DynPrimeDomainService = expect_context();

  service.enumerate_photos().await.map_err(ServerFnError::new)
}

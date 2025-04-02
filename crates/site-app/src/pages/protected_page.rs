use leptos::{either::Either, prelude::*};

use crate::{
  components::{Section, Title},
  AuthStatus,
};

#[component]
pub fn ProtectedPage(children: Children) -> impl IntoView {
  let auth_status: AuthStatus = expect_context();

  match auth_status.0 {
    Some(_) => Either::Left(view! {
      { children() }
    }),
    None => Either::Right(view! {
      <Section>
        <Title>"You Don't Have Access To This Page"</Title>
      </Section>

      <Section>
        <p class="max-w-prose text-base-dim">
          "You need to be logged in to see this page."
        </p>
      </Section>
    }),
  }
}

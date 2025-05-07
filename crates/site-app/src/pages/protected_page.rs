use base_components::{
  utils::navigation::{url_to_full_path, UnconditionalClientRedirect},
  Prose, Section, Title,
};
use leptos::{either::Either, prelude::*};
use leptos_router::location::Url;
use models::AuthStatus;

#[component]
pub fn ProtectedPage(children: Children) -> impl IntoView {
  let auth_status: AuthStatus = expect_context();

  let return_url = leptos_router::hooks::use_url();
  let escaped_return_url =
    Signal::derive(move || Url::escape(&url_to_full_path(&return_url())));
  let redirect_url =
    Signal::derive(move || format!("/log-in?next={}", escaped_return_url()));

  match auth_status.0 {
    Some(_) => Either::Left(view! {
      { children() }
    }),
    None => Either::Right(view! {
      <Section>
        <Title>"Log In To See This Page"</Title>
      </Section>

      <Section>
        <Prose>
          "You need to be logged in to see this page."
        </Prose>
      </Section>

      { move || view! { <UnconditionalClientRedirect path=redirect_url() /> }}
    }),
  }
}

use core_types::UserRecordId;
use leptos::*;

use crate::components::basic::Link;

#[component]
pub fn UserName(id: UserRecordId) -> impl IntoView {
  let user = create_resource(move || id, crate::server_fns::user::fetch_user);

  view! {
    <Suspense>
      { move || user.map(|user| {
        match user {
          Ok(Some(user)) if Some(user.id) == crate::authenticated_user().map(|u| u.id) => {
            view! {
              <Link>"you"</Link>
            }
            .into_view()
          }
          Ok(Some(user)) => {
            let user_name = user.name.to_owned();
            view! {
              <Link>{ user_name }</Link>
            }
            .into_view()
          }
          _ => {
            view! {
              <p>"Unknown User"</p>
            }
            .into_view()
          }
        }
      }) }
    </Suspense>
  }
}

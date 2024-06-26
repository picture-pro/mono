use leptos::*;

use crate::components::photo_group::PhotoGroup;

#[allow(unused)]
#[component]
pub fn Gallery() -> impl IntoView {
  let Some(user) = crate::authenticated_user() else {
    return view! {
      <p>"Not Authenticated"</p>
    }
    .into_view();
  };

  let photo_groups =
    create_resource(move || user.id, bl::fetch::fetch_user_owned_photo_groups);

  view! {
    <Suspense fallback=|| view!{ }>
      { move || photo_groups.map(|groups| {
        match groups {
          Ok(groups) => {
            view! {
              <PhotoGroupList groups=groups.clone() />
            }
            .into_view()
          }
          Err(e) => {
            view! {
              <p>{ format!("Failed to load photo groups: {e}") }</p>
            }
            .into_view()
          }
        }
      }) }
    </Suspense>
  }
  .into_view()
}

#[component]
fn PhotoGroupList(groups: Vec<core_types::PhotoGroup>) -> impl IntoView {
  if groups.is_empty() {
    return view! {
      <p>"You haven't uploaded any photos."</p>
    }
    .into_view();
  }

  view! {
    <div class="flex-1 flex flex-col gap-4 items-stretch lg:max-w-xl">
      { groups.into_iter().map(|group| {
        view! {
          <PhotoGroup group=group />
        }
        .into_view()
      }).collect::<Vec<_>>() }
    </div>
  }
  .into_view()
}

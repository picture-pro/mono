use leptos::*;

use crate::components::photo_group::PhotoGroup;

#[component]
pub fn Gallery() -> impl IntoView {
  let Some(user) = crate::authenticated_user() else {
    return view! {
      <p>"Not Authenticated"</p>
    }
    .into_view();
  };

  let photo_groups =
    create_resource(move || (), move |_| fetch_user_photo_groups(user.id));

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
    <div class="flex-1 flex flex-col gap-4 items-stretch lg:max-w-lg">
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

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn fetch_user_photo_groups(
  user_id: core_types::UserRecordId,
) -> Result<Vec<core_types::PhotoGroup>, ServerFnError> {
  bl::fetch::fetch_user_owned_photo_groups(user_id)
    .await
    .map_err(|e| {
      let error = format!("Failed to fetch user photo groups: {:?}", e);
      tracing::error!("{error}");
      ServerFnError::new(error)
    })
}

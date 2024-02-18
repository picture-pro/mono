use leptos::*;

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
  view! {
    <div class="flex flex-row flex-wrap gap-4">
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

#[component]
fn PhotoGroup(group: core_types::PhotoGroup) -> impl IntoView {
  view! {
    <div class="flex p-4 bg-base-200 rounded-box border">
      <div class="flex gap-4">
        { group.photos.into_iter().map(|photo_id| {
          view! {
            <crate::components::photo::Photo photo_id=photo_id />
          }
          .into_view()
        }).collect::<Vec<_>>() }
      </div>
    </div>
  }
}

#[server]
pub async fn fetch_user_photo_groups(
  user_id: core_types::UserRecordId,
) -> Result<Vec<core_types::PhotoGroup>, ServerFnError> {
  bl::get_user_photo_groups(user_id).await.map_err(|e| {
    ServerFnError::new(format!("Failed to get user photo groups: {e}"))
  })
}

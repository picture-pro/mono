use leptos::*;

use crate::components::user::UserName;

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
    <div class="flex-1 flex flex-col gap-4 justify-stretch max-w-lg">
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
  let status_element = match group.status {
    core_types::PhotoGroupStatus::OwnershipForSale { digital_price } => view! {
      <p class="text-2xl tracking-tight text-base-content">
        "For Sale: "
        <span class="font-semibold">
          { format!("${:.2}", digital_price.0) }
        </span>
      </p>
    }
    .into_view(),
    _ => view! {
      <p class="text-xl font-semibold tracking-tight text-base-content/80">
        "Not For Sale"
      </p>
    }
    .into_view(),
  };

  view! {
    <div class="flex p-4 gap-4 bg-base-100 rounded-box shadow">
      { group.photos.into_iter().map(|photo_id| {
        view! {
          <crate::components::photo::Photo photo_id=photo_id />
        }
        .into_view()
      }).collect::<Vec<_>>() }
      <div class="flex-1 flex flex-col gap-2">
        { status_element }
        <div class="flex-1" />
        <div class="flex flex-col gap-1">
          <p class="text-xs text-base-content/80">
            "Owned by "<UserName id={group.owner} />
          </p>
          <p class="text-xs text-base-content/80">
            "Photographed by "<UserName id={group.photographer} />", "<crate::components::basic::TimeAgo time={group.meta.created_at} />
          </p>
        </div>
      </div>
      <div class="flex flex-col items-end justify-between">
        // context menu ellipsis
        <button class="
          d-btn d-btn-ghost d-btn-circle d-btn-sm text-xl font-bold
          justify-center items-center text-center
        ">"⋮"</button>
        <button class="d-btn d-btn-primary d-btn-sm text-lg font-semibold tracking-tight">
          "Share"
        </button>
      </div>
    </div>
  }
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

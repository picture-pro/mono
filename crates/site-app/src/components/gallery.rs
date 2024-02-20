use core_types::UserRecordId;
use leptos::*;

#[component]
pub fn Link(
  children: Children,
  #[prop(optional)] href: String,
  #[prop(default = false)] subtle: bool,
  #[prop(default = false)] external: bool,
) -> impl IntoView {
  let target = if external { Some("_blank") } else { None };
  let rel = if external {
    Some("noopener noreferrer")
  } else {
    None
  };
  let class = if !subtle { "hover:underline" } else { "" };

  view! {
    <a href={href} target={target} rel={rel} class={class}>
      { children() }
    </a>
  }
}

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
fn UserName(id: UserRecordId) -> impl IntoView {
  let user = create_resource(move || id, fetch_user);

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

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn fetch_user(
  user_id: UserRecordId,
) -> Result<Option<core_types::PublicUser>, ServerFnError> {
  bl::fetch::fetch_user(user_id).await.map_err(|e| {
    let error = format!("Failed to fetch user: {:?}", e);
    tracing::error!("{error}");
    ServerFnError::new(error)
  })
}

#[component]
fn PhotoGroup(group: core_types::PhotoGroup) -> impl IntoView {
  view! {
    <div class="flex p-4 bg-base-100 rounded-box shadow">
      <div class="flex gap-4">
        { group.photos.into_iter().map(|photo_id| {
          view! {
            <crate::components::photo::Photo photo_id=photo_id />
          }
          .into_view()
        }).collect::<Vec<_>>() }
        <div class="flex flex-col gap-2">
          <div class="flex flex-col gap-1">
            <p class="text-xs text-base-content/80">
              "Owned by "<UserName id={group.owner} />
            </p>
            <p class="text-xs text-base-content/80">
              "Photographed by "<UserName id={group.photographer} />
            </p>
          </div>
        </div>
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

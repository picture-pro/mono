pub mod error;

use leptos::*;
use leptos_router::use_params_map;

use crate::{
  components::photo_group::PhotoGroup, pages::PageWrapper,
  server_fns::photo_group::fetch_photo_group,
};

#[component]
pub fn PurchasePage() -> impl IntoView {
  let params = use_params_map();
  let id = params().get("id").cloned();

  // fail out if there's no id
  let Some(id) = id else {
    return error::PurchasePageNoId().into_view();
  };

  // parse to a `PhotoGroupRecordId`
  let Ok(ulid) = core_types::Ulid::from_string(&id) else {
    return error::PurchasePageInvalidId().into_view();
  };
  let photo_group_id = core_types::PhotoGroupRecordId(ulid);

  // create a resource for fetching the photo group
  let photo_group = create_resource(move || photo_group_id, fetch_photo_group);

  // render the page, using InnerPurchasePage if everything works
  view! {
    <Suspense fallback=|| view!{ }>
      { move || photo_group.map(|group| {
        match group {
          Ok(Some(group)) => {
            view! {
              <InnerPurchasePage group={group.clone()} />
            }
          }
          Ok(None) => {
            view! {
              <error::PurchasePageMissing />
            }
          }
          Err(e) => {
            view! {
              <error::PurchasePageInternalError error={e.to_string()} />
            }
          }
        }
      }) }
    </Suspense>
  }
}

#[component]
fn AboutThePhotographer(user_id: core_types::UserRecordId) -> impl IntoView {
  let user = create_resource(
    move || (),
    move |_| crate::server_fns::user::fetch_user(user_id),
  );

  view! {
    <Suspense fallback=|| view!{ }>
      { move || match user() {
        Some(Ok(Some(user))) => {
          Some(view! {
            <InnerAboutThePhotographer user=user />
          }
          .into_view())
        }
        Some(Ok(None)) => {
          Some(view! {
            <p>{ "User not found" }</p>
          }
          .into_view())
        }
        Some(Err(e)) => {
          Some(view! {
            <p>{ format!("Failed to load user: {e}") }</p>
          }
          .into_view())
        }
        None => None,
      } }
    </Suspense>
  }
}

#[component]
fn InnerAboutThePhotographer(user: core_types::PublicUser) -> impl IntoView {
  let user_initials = user
    .name
    .split_whitespace()
    .filter_map(|s| s.chars().next())
    .collect::<String>()
    .to_uppercase();

  view! {
    <div class="flex flex-col p-6 gap-4 bg-base-100 rounded-box shadow">
      <p class="text-2xl font-semibold tracking-tight">About the Photographer</p>
      <div class="flex flex-row gap-2 items-center">
        <div class="d-avatar d-placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-10">
            <span class="text-lg">{user_initials}</span>
          </div>
        </div>
        <div class="flex flex-col gap-0">
          <p class="text-xl font-semibold tracking-tight leading-tight">{user.name}</p>
          <p class="text-xs text-base-content/80">"58 photos sold on-platform"</p>
        </div>
      </div>
      <p class="text-lg text-base-content/80 max-w-prose text-pretty">
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
      </p>
    </div>
  }
}

#[component]
fn InnerPurchasePage(group: core_types::PhotoGroup) -> impl IntoView {
  let title_element = format!(
    "Photo{} for Sale",
    if group.photos.len() > 1 { "s" } else { "" }
  );

  view! {
    <PageWrapper backed=false>
      <p class="text-4xl font-semibold tracking-tight">
        { title_element }
      </p>
      <div class="flex flex-col gap-4">
        <div class="flex flex-col lg:flex-row lg:justify-between gap-4">
          <PhotoGroup
            group={group.clone()} extra_class="lg:flex-1"
            read_only=true
          />
          <PhotoPurchaseOptions group={group.clone()} />
        </div>
        <AboutThePhotographer user_id={group.photographer} />
      </div>
    </PageWrapper>
  }
}

struct PurchaseOption {
  title: &'static str,
  desc:  &'static str,
  price: Option<f32>,
}

#[component]
fn PhotoPurchaseOptions(group: core_types::PhotoGroup) -> impl IntoView {
  let user = crate::authenticated_user();
  let user_id = user.as_ref().map(|u| u.id);

  let options = match group.status {
    core_types::PhotoGroupStatus::OwnershipForSale { .. }
      if Some(group.owner) == user_id =>
    {
      vec![PurchaseOption {
        title: "You Own This",
        desc:  "You own the digital rights to this photo at this moment, but \
                it is still available for purchase.",
        price: None,
      }]
    }
    core_types::PhotoGroupStatus::OwnershipForSale { digital_price } => {
      vec![
        PurchaseOption {
          title: "Ownership",
          desc:  "Own the digital rights to this photo. You'll recieve an \
                  email with a link to download the full resolution photo.",
          price: Some(digital_price.0),
        },
        PurchaseOption {
          title: "Prints",
          desc:  "Order physical prints of this photo. Includes digital \
                  rights.",
          price: Some(digital_price.0 + 5.0),
        },
      ]
    }
    core_types::PhotoGroupStatus::OwnershipPurchased { owner }
      if Some(owner) == user_id =>
    {
      vec![PurchaseOption {
        title: "You Own This",
        desc:  "You own the digital rights to this photo, and it is no longer \
                available for purchase.",
        price: None,
      }]
    }
    core_types::PhotoGroupStatus::OwnershipPurchased { .. } => {
      vec![PurchaseOption {
        title: "Already Owned",
        desc:  "This photo has already been purchased by someone else",
        price: None,
      }]
    }
    core_types::PhotoGroupStatus::UsageRightsForSale { .. } => {
      todo!()
    }
  };

  view! {
    <div class="grid sm:grid-flow-col sm:auto-cols-fr lg:max-w-lg gap-4">
      { options.into_iter().map(|option| {
        view! {
          <div class="flex flex-col gap-4 p-6 bg-base-100 rounded-box shadow">
            <p class="text-2xl tracking-tight">
              { option.title }
              { option.price.map(|p| view! {
                ": "<span class="font-semibold">{ format!("${p:.2}") }</span>
              })}
            </p>
            <p class="text-base-content/80 max-w-prose">{ option.desc }</p>
            <div class="flex-1" />
            { option.price.map(|_| view! {
              <div class="w-full flex flex-row">
                <div class="flex-1" />
                <div class="max-w-48">
                  <button class="apple-pay-button apple-pay-button-black">Apple Pay</button>
                </div>
                <div class="flex-1" />
              </div>
            })}
          </div>
        }
      }).collect::<Vec<_>>() }
    </div>
  }
}

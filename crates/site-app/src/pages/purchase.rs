pub mod error;

use leptos::*;
use leptos_router::use_params_map;

use crate::{pages::PageWrapper, server_fns::photo_group::fetch_photo_group};

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
  let photo_group =
    create_resource(move || photo_group_id, move |id| fetch_photo_group(id));

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
fn InnerPurchasePage(group: core_types::PhotoGroup) -> impl IntoView {
  view! {
    <PageWrapper>
      <h1 class="text-4xl font-semibold tracking-tight">Purchase</h1>
      <pre><code>{ format!("{:#?}", group) }</code></pre>
    </PageWrapper>
  }
}

use leptos::*;

use crate::components::user::UserName;

#[component]
pub fn EllipsisButton() -> impl IntoView {
  view! {
    <button class="
      d-btn d-btn-ghost d-btn-circle d-btn-sm text-xl font-bold
      justify-center items-center text-center
    ">"⋮"</button>
  }
}

#[component]
pub fn PhotoGroup(
  group: core_types::PhotoGroup,
  #[prop(default = false)] read_only: bool,
  #[prop(default = "")] extra_class: &'static str,
) -> impl IntoView {
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

  let purchase_url = format!("/photo/{}", group.id.0);
  let qr_code_url = format!("/qr/{}", group.id.0);

  let owned_by_element = view! {
    "Owned by "<UserName id={group.owner} />
  }
  .into_view();

  let uploaded_by_element = view! {
    "Uploaded by "<UserName id={group.photographer} />
  }
  .into_view();

  let created_at_element = view! {
    <crate::components::basic::TimeAgo time={group.meta.created_at} />
  }
  .into_view();

  view! {
    <div class={format!(
      "grid grid-cols-[auto_1fr] p-6 gap-4 bg-base-100 rounded-box shadow {extra_class}"
    )}>
      <div class={format!(
        "col-start-1 col-span-1 row-start-1 flex flex-col justify-center xs:px-4 {adjusted_for_action}",
        adjusted_for_action = if !read_only { "row-span-1 sm:row-span-2" } else { "row-span-2" },
      )}>
        <crate::components::photo_deck::PhotoDeck
          ids={group.photos.clone()}
          size=crate::components::photo::PhotoSize::FitsWithinSquare(200)
        />
      </div>
      <div class="col-start-2 col-span-1 row-start-1 row-span-1 flex flex-row justify-between gap-4">
        { status_element }
        <crate::components::photo_group::EllipsisButton />
      </div>
      <div class={format!(
        "row-start-2 row-span-1 flex flex-row items-end justify-between gap-4 {adjusted_for_action}",
        adjusted_for_action = if !read_only { "col-start-1 col-span-2 sm:col-start-2 sm:col-span-1" } else { "col-start-2 col-span-1" },
      )}>
        <div class="flex flex-col gap-1">
          <p class="text-xs text-base-content/80">
            { owned_by_element }
          </p>
          <p class="text-xs text-base-content/80">
            { uploaded_by_element }
            ", "
            { created_at_element }
          </p>
        </div>
        { match read_only {
          false => view! {
            <div class="flex flex-col gap-4">
              <a class="d-btn d-btn-sm" href={ purchase_url }>"Purchase Page"</a>
              <a class="d-btn d-btn-primary d-btn-sm" href={ qr_code_url }>"QR Code"</a>
            </div>
          }.into_view(),
          true => ().into_view()
        } }
      </div>
    </div>
  }
}

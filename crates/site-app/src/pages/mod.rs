pub mod auth;
pub mod dashboard;
pub mod home_page;
pub mod purchase;
pub mod qr_code;

use leptos::*;

use crate::components::basic::Link;

#[component]
pub fn SmallPageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="flex-1 flex flex-col justify-center items-center h-full">
      <div class="d-card w-full max-w-sm bg-base-100 rounded-lg shadow-xl">
        {children()}
      </div>
    </div>
  }
}

/// A wrapper for all pages in the app.
///
/// It acts as a responsive container, and if `backed` is true, it will look
/// like a card with a shadow, a background color, and a border radius.
#[component]
pub fn PageWrapper(
  children: Children,
  /// Whether the wrapper should appear "backed" like a card.
  #[prop(default = true)]
  backed: bool,
) -> impl IntoView {
  view! {
    <div class={format!(
      "flex-1 flex flex-col gap-4 md:container md:mx-auto my-8 {rounded}
      {shadow} p-4 md:px-6 {bg_color} overflow-x-auto",
      rounded = if backed { "md:rounded-xl" } else { "" },
      shadow = if backed { "shadow" } else { "" },
      bg_color = if backed { "bg-base-100" } else { "" },
    )}>
      {children()}
    </div>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="mb-8 md:container md:mx-auto flex justify-center items-center text-xs text-base-content/80">
      <div class="flex flex-row justify-center gap-y-1 gap-x-4 md:gap-x-6 items-center flex-wrap mx-8">
        <p><span inner_html="&copy;" />" 2024 PicturePro"</p>
        <Link href="/terms".to_string()>Terms of Service</Link>
        <Link href="/privacy".to_string()>Privacy</Link>
        <Link href="/security".to_string()>Security</Link>
        <Link href="/contact".to_string()>Contact</Link>
      </div>
    </div>
  }
}

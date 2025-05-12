use base_components::Title;
use leptos::{prelude::*, server::codee::string::FromToStringCodec};

fn use_hide_loader_cookie() -> (Signal<Option<bool>>, WriteSignal<Option<bool>>)
{
  leptos_use::use_cookie::<bool, FromToStringCodec>("hide_loader")
}

#[component]
pub fn PageCover() -> impl IntoView {
  let (hide, _) = use_hide_loader_cookie();

  move || match hide() {
    // if the hide cookie is already set, don't render the cover
    Some(_) => None,
    // otherwise render the cover and let the client set the cookie
    None => Some(view! { <PageCoverInner /> }),
  }
}

#[island]
fn PageCoverInner() -> impl IntoView {
  let (hide, set_hide) = use_hide_loader_cookie();
  let hide = Signal::derive(move || hide.get().is_some_and(|v| v));

  // set hide right after render
  Effect::watch(
    move || (),
    move |(), _, _| {
      set_hide.set(Some(true));
    },
    true,
  );

  let class = Signal::derive(move || {
    format!(
      "absolute inset-0 bg-primary-app flex flex-col items-center \
       justify-center gap-8 pointer-events-none {}",
      if hide() {
        " delay-[500ms] duration-[500ms] ease-in-out opacity-0 \
         transition-opacity"
      } else {
        ""
      }
    )
  });

  view! {
    <div class=class>
      <Title class="font-semibold">"PicturePro"</Title>
      <div class="flex flex-row justify-between gap-8 text-xl text-base-dim">
        <p class="flex-1">"Capture"</p>
        <p class="flex-1">"Share"</p>
        <p class="flex-1">"Profit"</p>
      </div>
    </div>
  }
}

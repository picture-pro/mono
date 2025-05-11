use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::use_cookie;

#[component]
pub fn PageCover() -> impl IntoView {
  let (hide, _) = use_cookie::<bool, FromToStringCodec>("hide_loader");

  move || match hide() {
    // if the hide cookie is already set, don't render the cover
    Some(_) => None,
    // otherwise render the cover and let the client set the cookie
    None => Some(view! { <PageCoverInner /> }),
  }
}

#[island]
fn PageCoverInner() -> impl IntoView {
  let (hide, set_hide) = use_cookie::<bool, FromToStringCodec>("hide_loader");
  let hide = Signal::derive(move || hide.get().is_some_and(|v| v));

  // set hide right after render
  Effect::watch(
    move || (),
    move |_, _, _| {
      set_hide.set(Some(true));
    },
    true,
  );

  let class = Signal::derive(move || {
    let mut class = "absolute inset-0 bg-base-app flex flex-col items-center \
                     justify-center gap-4 pointer-events-none"
      .to_string();
    if hide() {
      class.push_str(
        " delay-[500ms] duration-[500ms] ease-in-out opacity-0 \
         transition-opacity",
      );
    }
    class
  });

  view! {
    <div class=class>
      <base_components::Title>"PicturePro"</base_components::Title>
    </div>
  }
}

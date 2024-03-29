use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <super::PageWrapper backed=false>
      <p class="font-semibold tracking-tight text-4xl">
        "Share / Sell Photos with Anyone!"
      </p>
      <ColoredBoxHero />
    </super::PageWrapper>
  }
}

#[component]
fn ColoredBox(
  border_color: &'static str,
  bg_color: &'static str,
  text_color: &'static str,
  title: &'static str,
  description: &'static str,
  href: Option<&'static str>,
) -> impl IntoView {
  let container_class = format!(
    "flex flex-col gap-2 px-4 py-2 border-2 {border_color} {bg_color} \
     rounded-box {text_color} min-h-32",
  );

  let title_element = view! {
    <p class="text-2xl font-semibold tracking-tight leading-tight">{title}</p>
  };
  let description_element = view! {
    <p class="text-sm">{description}</p>
  };

  match href {
    Some(href) => view! {
      <a href={href} class={container_class}>
        {title_element}
        {description_element}
      </a>
    }
    .into_view(),
    None => view! {
      <div class={container_class}>
        {title_element}
        {description_element}
      </div>
    }
    .into_view(),
  }
}

#[component]
fn ColoredBoxHero() -> impl IntoView {
  let authenticated = crate::utils::authenticated_user().is_some();

  view! {
    <div class="grid gap-4 grid-cols-2 grid-rows-2 w-full">
      <ColoredBox
        border_color="border-orange-700"
        bg_color="bg-orange-500/20"
        text_color="text-orange-200/80"
        title="Private Session"
        description="Separate subjects, separate customers."
        href={if authenticated { Some("/dashboard") } else { Some("/login?next=/dashboard") } }
      />
      <ColoredBox
        border_color="border-green-700"
        bg_color="bg-green-500/20"
        text_color="text-green-200/80"
        title="Public Session"
        description="Share with everyone, sell to everyone."
        href={if authenticated { None } else { Some("/login") } }
      />
      <ColoredBox
        border_color="border-blue-700"
        bg_color="bg-blue-500/20"
        text_color="text-blue-200/80"
        title="Discover"
        description="Discover photos from around the world."
        href={if authenticated { None } else { Some("/login") } }
      />
      <ColoredBox
        border_color="border-purple-700"
        bg_color="bg-purple-500/20"
        text_color="text-purple-200/80"
        title="School Event"
        description="Share photos from school events."
        href={if authenticated { None } else { Some("/login") } }
      />
    </div>
  }
}

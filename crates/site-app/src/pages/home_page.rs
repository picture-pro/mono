use base_components::{Section, Title};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
  use lsc::icons::*;

  view! {
    <Section>
      <Title class="ml-4">
        <span class="block">"Welcome to "</span>
        <span class="block">
          <span class="font-semibold">"PicturePro"</span>
          "."
        </span>
      </Title>
    </Section>

    <Section>
      <div class="grid grid-cols-2 w-full gap-3 xs:gap-4">
        <ColoredBox
          href="/upload-photo"
          extra_outer_class="bg-ruby-3 dark:bg-rubydark-3 border-ruby-normal text-rubya-normal"
          extra_dim_text_class="text-rubya-dim"
          title="Single Subject"
          icon_view={ view! { <CameraIcon {..} class="size-5 shrink-0" /> }.into_any() }
        />

        <ColoredBox
          extra_outer_class="bg-jade-3 dark:bg-jadedark-3 border-jade-normal text-jadea-normal"
          extra_dim_text_class="text-jadea-dim"
          title="Upload to Marketplace"
          icon_view={ view! { <UploadIcon {..} class="size-5 shrink-0" /> }.into_any() }
        />

        <ColoredBox
          extra_outer_class="bg-cyan-3 dark:bg-cyandark-3 border-cyan-normal text-cyana-normal"
          extra_dim_text_class="text-cyana-dim"
          title="Explore Marketplace"
          icon_view={ view! { <MagnifyingGlassIcon {..} class="size-5 shrink-0" /> }.into_any() }
        />

        <ColoredBox
          extra_outer_class="bg-orange-3 dark:bg-orangedark-3 border-orange-normal text-orangea-normal"
          extra_dim_text_class="text-orangea-dim"
          title="Event"
          icon_view={ view! { <LayersIcon {..} class="size-5 shrink-0" /> }.into_any() }
        />
      </div>
    </Section>
  }
}

#[component]
fn ColoredBox(
  #[prop(optional)] href: Option<&'static str>,
  extra_outer_class: &'static str,
  extra_dim_text_class: &'static str,
  title: &'static str,
  icon_view: AnyView,
) -> impl IntoView {
  let outer_class = format!(
    "flex flex-col gap-2 p-3 xs:p-4 border rounded-xl shadow-xl transition \
     {extra_outer_class}"
  );
  let dim_text_class = format!("max-w-prose text-sm {extra_dim_text_class}");

  view! {
    <a href=href class=outer_class>
      <div class="flex flex-row items-center gap-2">
        { icon_view }
        <p class="xs:text-lg leading-tight">
          { title }
        </p>
      </div>
      <p class=dim_text_class>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut auctor magna in lacus egestas tincidunt. Maecenas erat tellus, dapibus feugiat purus eget, tincidunt semper risus."
      </p>
    </a>
  }
}

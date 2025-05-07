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
      <div class="grid sm:grid-cols-2 w-full gap-4">
        <a href="/upload-photo" class="bg-ruby-3 dark:bg-rubydark-3 border border-ruby-normal text-rubya-normal flex flex-col gap-2 p-4 rounded-xl shadow-xl transition">
          <div class="flex flex-row items-center gap-2">
            <CameraIcon {..} class="size-5" />
            <p class="text-lg">
              "Single Subject"
            </p>
          </div>
          <p class="text-rubya-dim max-w-prose text-sm">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut auctor magna in lacus egestas tincidunt. Maecenas erat tellus, dapibus feugiat purus eget, tincidunt semper risus."
          </p>
        </a>

        <div class="bg-jade-3 dark:bg-jadedark-3 border border-jade-normal text-jadea-normal flex flex-col gap-2 p-4 rounded-xl shadow-xl transition">
          <div class="flex flex-row items-center gap-2">
            <UploadIcon {..} class="size-5" />
            <p class="text-lg">
              "Upload to Marketplace"
            </p>
          </div>
          <p class="text-jadea-dim max-w-prose text-sm">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut auctor magna in lacus egestas tincidunt. Maecenas erat tellus, dapibus feugiat purus eget, tincidunt semper risus."
          </p>
        </div>

        <div class="bg-cyan-3 dark:bg-cyandark-3 border border-cyan-normal text-cyana-normal flex flex-col gap-2 p-4 rounded-xl shadow-xl transition">
          <div class="flex flex-row items-center gap-2">
            <MagnifyingGlassIcon {..} class="size-5" />
            <p class="text-lg">
              "Explore Marketplace"
            </p>
          </div>
          <p class="text-cyana-dim max-w-prose text-sm">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut auctor magna in lacus egestas tincidunt. Maecenas erat tellus, dapibus feugiat purus eget, tincidunt semper risus."
          </p>
        </div>

        <div class="bg-orange-3 dark:bg-orangedark-3 border border-orange-normal text-orangea-normal flex flex-col gap-2 p-4 rounded-xl shadow-xl transition">
          <div class="flex flex-row items-center gap-2">
            <LayersIcon {..} class="size-5" />
            <p class="text-lg">
              "Event"
            </p>
          </div>
          <p class="text-orangea-dim max-w-prose text-sm">
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut auctor magna in lacus egestas tincidunt. Maecenas erat tellus, dapibus feugiat purus eget, tincidunt semper risus."
          </p>
        </div>
      </div>
    </Section>
  }
}

use leptos::*;

#[component]
pub fn Gallery() -> impl IntoView {
  let Some(user) = crate::authenticated_user() else {
    return view! {
      <p>"Not Authenticated"</p>
    }
    .into_view();
  };

  let photo_groups = create_resource(
    move || (),
    move |_| get_user_photo_groups(user.id.clone()),
  );

  view! {
    <Suspense fallback=|| view!{<p>"Loading..."</p>}>
      { move || photo_groups.map(|groups| view! {
        <pre class="text-sm leading-6 flex ligatures-none overflow-auto whitespace-pre-wrap">
          <code class="bg-base-200 rounded-box p-2">
            {format!("Photo Groups: {:#?}", groups)}
          </code>
        </pre>
      })}
    </Suspense>
  }
  .into_view()
}

#[server]
pub async fn get_user_photo_groups(
  user_id: core_types::UserRecordId,
) -> Result<Vec<core_types::PhotoGroup>, ServerFnError> {
  bl::get_user_photo_groups(user_id).await.map_err(|e| {
    ServerFnError::new(format!("Failed to get user photo groups: {e}"))
  })
}

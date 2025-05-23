// not useful for component-oriented crates
#![allow(clippy::must_use_candidate)]

//! The photo upload flow for `PicturePro`.

mod configuring_group;
mod photo;
mod selecting_photos;
mod server_fns;
mod upload_finished;

use std::collections::HashMap;

use leptos::{either::EitherOf3, prelude::*};
use reactive_stores::Store;

pub(crate) const MAX_UPLOAD_SIZE: u64 = 50 * 1000 * 1000; // 50MB

use base_components::{Section, Title};

use self::{
  configuring_group::{ConfiguringGroupState, ConfiguringGroupStep},
  selecting_photos::{SelectingPhotosState, SelectingPhotosStep},
  upload_finished::{UploadFinishedState, UploadFinishedStep},
};

/// The upload photo page.
#[component]
pub fn UploadPhotoPage() -> impl IntoView {
  view! {
    <Section>
      <Title>"Upload Photos"</Title>
    </Section>

    <UploadPhotoWizard />
  }
}

#[island]
fn UploadPhotoWizard() -> impl IntoView {
  let overall_context = Store::new(UploadState::default());
  provide_context(overall_context);

  Effect::new(move || {
    let context: Store<UploadState> = expect_context();
    leptos::logging::log!("{:#?}", &*context.read());
  });

  let memoized_top_level_switch = Memo::new(move |_| {
    if overall_context.selecting_photos() {
      EitherOf3::A(())
    } else if overall_context.configuring_group() {
      EitherOf3::B(())
    } else if overall_context.upload_finished() {
      EitherOf3::C(())
    } else {
      unreachable!("UploadContext not in any state")
    }
  });

  move || match memoized_top_level_switch() {
    EitherOf3::A(()) => EitherOf3::A(view! { <SelectingPhotosStep /> }),
    EitherOf3::B(()) => EitherOf3::B(view! { <ConfiguringGroupStep /> }),
    EitherOf3::C(()) => EitherOf3::C(view! { <UploadFinishedStep /> }),
  }
}

#[derive(Store, Debug)]
enum UploadState {
  SelectingPhotos(SelectingPhotosState),
  ConfiguringGroup(ConfiguringGroupState),
  UploadFinished(UploadFinishedState),
}

impl Default for UploadState {
  fn default() -> Self {
    UploadState::SelectingPhotos(SelectingPhotosState {
      photos: HashMap::new(),
    })
  }
}

mod configuring_group;
mod photo;
mod selecting_photos;
mod upload_finished;

use std::{collections::HashMap, ops::Deref};

use leptos::{either::EitherOf3, prelude::*};
use photo::PhotoUploadState;
use reactive_stores::Store;

pub const MAX_UPLOAD_SIZE: u64 = 50 * 1000 * 1000; // 50MB

use self::{
  configuring_group::{
    ConfiguringGroupState, ConfiguringGroupStateStoreFields,
    ConfiguringGroupStep,
  },
  selecting_photos::{
    SelectingPhotosState, SelectingPhotosStateStoreFields, SelectingPhotosStep,
  },
  upload_finished::{UploadFinishedState, UploadFinishedStep},
};
use crate::components::{Section, Title};

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
pub fn UploadPhotoWizard() -> impl IntoView {
  let overall_context = Store::new(UploadState::default());
  provide_context(overall_context);

  Effect::new(move || {
    let guard = overall_context.read();
    leptos::logging::log!("{:#?}", guard.deref());
  });

  move || {
    if overall_context.selecting_photos() {
      EitherOf3::A(view! { <SelectingPhotosStep /> })
    } else if overall_context.configuring_group() {
      EitherOf3::B(view! { <ConfiguringGroupStep /> })
    } else if overall_context.upload_finished() {
      EitherOf3::C(view! { <UploadFinishedStep /> })
    } else {
      unreachable!("UploadContext not in any state")
    }
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

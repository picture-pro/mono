use std::collections::HashMap;

use gloo::file::{Blob, File, ObjectUrl};
use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};

use super::MAX_UPLOAD_SIZE;

#[derive(Clone)]
pub(super) struct QueuedUploadFile {
  name:      String,
  blob:      SendWrapper<Blob>,
  url:       SendWrapper<ObjectUrl>,
  oversized: bool,
}

impl QueuedUploadFile {
  pub(super) fn new(file: File) -> Self {
    let name = file.name();

    let blob = Blob::from(file);
    let blob_size = blob.size();
    let oversized = blob_size > MAX_UPLOAD_SIZE;
    if oversized {
      leptos::logging::log!("File {} is too large", name);
    }
    let url = ObjectUrl::from(blob.clone());

    Self {
      name,
      blob: SendWrapper::new(blob),
      url: SendWrapper::new(url),
      oversized,
    }
  }
  pub(super) fn blob(&self) -> Blob { self.blob.clone().take() }
  pub(super) fn url(&self) -> ObjectUrl { self.url.clone().take() }
  pub(super) fn oversized(&self) -> bool { self.oversized }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub(super) enum UploadStage {
  #[default]
  PhotosStage,
  SettingsStage,
}

#[derive(Clone, Store, Default)]
pub(super) struct UploadContextStore {
  last_index: usize,
  files:      HashMap<usize, QueuedUploadFile>,
  stage:      UploadStage,
}

#[derive(Clone, Copy)]
pub(super) struct UploadContext(Store<UploadContextStore>);

impl UploadContext {
  pub(super) fn new() -> Self {
    UploadContext(Store::new(UploadContextStore::default()))
  }

  pub(super) fn set_stage(&self, stage: UploadStage) {
    self.0.stage().set(stage);
  }
  pub(super) fn stage(&self) -> Signal<UploadStage> {
    let subfield = self.0.stage();
    Signal::derive(move || subfield.get())
  }

  pub(super) fn add_file(&self, file: QueuedUploadFile) {
    let last_index = self.0.last_index().get();

    self.0.files().update(|files| {
      files.insert(last_index, file);
    });
    self.0.last_index().update(|last_index| {
      *last_index += 1;
    });
  }
  pub(super) fn get_file(&self, index: usize) -> Option<QueuedUploadFile> {
    let files_lock = self.0.files().read();
    files_lock.get(&index).cloned()
  }
  pub(super) fn delete_file(&self, index: usize) {
    self.0.files().update(|files| {
      files.remove(&index);
    })
  }

  pub(super) fn iter_file_indices(&self) -> impl Iterator<Item = usize> {
    let files_lock = self.0.files().read();
    let mut indices = files_lock.keys().copied().collect::<Vec<_>>();
    drop(files_lock);
    indices.sort_unstable();
    indices.into_iter()
  }
}

#[island]
pub(super) fn ContextProvider(children: Children) -> impl IntoView {
  provide_context(UploadContext::new());

  children()
}

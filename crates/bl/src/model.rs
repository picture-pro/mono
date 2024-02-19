use std::fmt::Debug;

use clients::surreal::SurrealRootClient;
use color_eyre::eyre::Result;
use core_types::{
  NewId, Photo, PhotoGroup, PhotoGroupRecordId, PhotoRecordId, PrivateArtifact,
  PrivateArtifactRecordId, PublicArtifact, PublicArtifactRecordId, User,
  UserRecordId,
};
use serde::{Deserialize, Serialize};
use surrealdb::opt::IntoResource;

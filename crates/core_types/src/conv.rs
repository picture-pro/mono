use serde::Deserialize;
use surreal_id::NewId;
use surrealdb::{
  opt::{IntoResource, Resource},
  sql::{Id, Thing},
};

use crate::{
  PhotoGroupRecordId, PhotoRecordId, PrivateArtifactRecordId,
  PublicArtifactRecordId, UserRecordId,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum UlidOrThing {
  Ulid(ulid::Ulid),
  Thing(Thing),
}

impl From<UlidOrThing> for ulid::Ulid {
  fn from(u: UlidOrThing) -> ulid::Ulid {
    match u {
      UlidOrThing::Ulid(u) => u,
      UlidOrThing::Thing(t) => t.id.to_string().parse().unwrap(),
    }
  }
}

pub trait AsThing {
  fn as_thing(&self) -> Thing;
}

impl<T: NewId> AsThing for T {
  fn as_thing(&self) -> Thing {
    Thing {
      tb: T::TABLE.to_string(),
      id: Id::String(self.get_inner_string()),
    }
  }
}

macro_rules! impl_record_id {
  ($type:ident) => {
    impl From<UlidOrThing> for $type {
      fn from(u: UlidOrThing) -> $type { $type(ulid::Ulid::from(u)) }
    }

    impl<R> IntoResource<Option<R>> for $type {
      fn into_resource(self) -> Result<Resource, surrealdb::Error> {
        Ok(Resource::RecordId(self.as_thing()))
      }
    }
  };
}

impl_record_id!(UserRecordId);
impl_record_id!(PhotoRecordId);
impl_record_id!(PhotoGroupRecordId);
impl_record_id!(PrivateArtifactRecordId);
impl_record_id!(PublicArtifactRecordId);

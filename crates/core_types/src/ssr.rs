use serde::Deserialize;
use surreal_id::NewId;
use surrealdb::{
  opt::{IntoResource, Resource},
  sql::{Id, Thing},
};

use crate::{
  PhotoGroupRecordId, PhotoRecordId, PrivateArtifactRecordId,
  PublicArtifactRecordId, UserRecordId, PHOTO_GROUP_TABLE, PHOTO_TABLE,
  PRIVATE_ARTIFACT_TABLE, PUBLIC_ARTIFACT_TABLE, USER_TABLE,
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
  ($type:ident, $table:ident) => {
    impl NewId for $type {
      const TABLE: &'static str = $table;

      fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        Self(inner_id.into().to_string().parse().unwrap())
      }
      fn get_inner_string(&self) -> String { self.0.to_string() }
    }

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

impl_record_id!(UserRecordId, USER_TABLE);
impl_record_id!(PhotoRecordId, PHOTO_TABLE);
impl_record_id!(PhotoGroupRecordId, PHOTO_GROUP_TABLE);
impl_record_id!(PrivateArtifactRecordId, PRIVATE_ARTIFACT_TABLE);
impl_record_id!(PublicArtifactRecordId, PUBLIC_ARTIFACT_TABLE);

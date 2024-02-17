use serde::Deserialize;
use surreal_id::NewId;
use surrealdb::sql::{Id, Thing};

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

impl From<UlidOrThing> for UserRecordId {
  fn from(u: UlidOrThing) -> UserRecordId { UserRecordId(ulid::Ulid::from(u)) }
}

impl From<UlidOrThing> for PhotoRecordId {
  fn from(u: UlidOrThing) -> PhotoRecordId {
    PhotoRecordId(ulid::Ulid::from(u))
  }
}

impl From<UlidOrThing> for PhotoGroupRecordId {
  fn from(u: UlidOrThing) -> PhotoGroupRecordId {
    PhotoGroupRecordId(ulid::Ulid::from(u))
  }
}

impl From<UlidOrThing> for PrivateArtifactRecordId {
  fn from(u: UlidOrThing) -> PrivateArtifactRecordId {
    PrivateArtifactRecordId(ulid::Ulid::from(u))
  }
}

impl From<UlidOrThing> for PublicArtifactRecordId {
  fn from(u: UlidOrThing) -> PublicArtifactRecordId {
    PublicArtifactRecordId(ulid::Ulid::from(u))
  }
}

pub trait AsThing {
  fn as_thing(&self) -> Thing;
}

impl<T: NewId> AsThing for T {
  fn as_thing(&self) -> Thing {
    println!("converting to thing: {:?}", self.get_inner_string());
    Thing {
      tb: T::TABLE.to_string(),
      id: Id::String(self.get_inner_string()),
    }
  }
}

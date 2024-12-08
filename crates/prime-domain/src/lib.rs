//! Provides prime-domain Services, the entry points for domain-specific
//! business logic.

mod canonical;

use std::sync::Arc;

pub use hex;
use hex::Hexagonal;
use miette::Result;
pub use models;
use models::{Photo, PhotoRecordId};
pub use repos;
use repos::FetchModelError;

pub use self::canonical::*;

/// A dynamic [`PrimeDomainService`] trait object.
pub type DynPrimeDomainService = Arc<Box<dyn PrimeDomainService>>;

/// The prime domain service trait.
#[async_trait::async_trait]
pub trait PrimeDomainService: Hexagonal {
  /// Fetch a [`Photo`] by its ID.
  async fn fetch_photo_by_id(
    &self,
    id: PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError>;
  /// Produce a list of all [`Photo`]s.
  async fn enumerate_photos(&self) -> Result<Vec<Photo>>;
}

#[async_trait::async_trait]
impl<T, I> PrimeDomainService for T
where
  T: std::ops::Deref<Target = I> + Hexagonal + Sized,
  I: PrimeDomainService + ?Sized,
{
  async fn fetch_photo_by_id(
    &self,
    id: PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError> {
    self.deref().fetch_photo_by_id(id).await
  }

  async fn enumerate_photos(&self) -> Result<Vec<Photo>> {
    self.deref().enumerate_photos().await
  }
}

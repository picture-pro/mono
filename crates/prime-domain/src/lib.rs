//! Provides prime-domain Services, the entry points for domain-specific
//! business logic.

mod canonical;

use std::sync::Arc;

pub use hex;
use hex::Hexagonal;
use models::{Artifact, ArtifactRecordId, Photo, PhotoRecordId};
use repos::db::FetchModelError;

/// A dynamic [`PrimeDomainService`] trait object.
pub type DynPrimeDomainService = Arc<Box<dyn PrimeDomainService>>;

/// The prime domain service trait.
#[async_trait::async_trait]
pub trait PrimeDomainService: Hexagonal {
  /// Fetch an [`Artifact`] by its ID.
  async fn fetch_cache_by_id(
    &self,
    id: ArtifactRecordId,
  ) -> Result<Option<Artifact>, FetchModelError>;
  /// Fetch a [`Photo`] by its ID.
  async fn fetch_photo_by_id(
    &self,
    id: PhotoRecordId,
  ) -> Result<Option<Photo>, FetchModelError>;
}

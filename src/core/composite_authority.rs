//! Sealed generation authority shared by producers and replay without reversing AD-004.
use super::artifact::Artifact;
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
use std::sync::Arc;

pub(crate) mod sealed {
    pub trait Sealed {}
}
/// An immutable generator-owned snapshot, never constructible from caller fields.
#[derive(Debug)]
pub struct CompositeReplayOrigin {
    pub(crate) original: Arc<CheckedPackageV2>,
    pub(crate) artifacts: Arc<Vec<Artifact>>,
    pub(crate) entries: Vec<CompositeReplayEntry>,
}
#[derive(Debug)]
pub(crate) struct CompositeReplayEntry {
    pub(crate) node: CheckedNodeId,
    pub(crate) equal: bool,
    pub(crate) left: CheckedNodeId,
    pub(crate) right: CheckedNodeId,
    pub(crate) oracle: String,
    pub(crate) environment: String,
}
/// Refusal to recover unchanged producer authority.
#[derive(Debug)]
pub enum CompositeOriginRefusal {
    /// No original admitted package was retained.
    Unavailable,
    /// Published generation data changed after generation.
    Changed,
}
/// Sealed producer access to the retained original generation.
pub trait CompositeReplayAuthority: sealed::Sealed {
    /// Validate the complete public generation result before releasing its original snapshot.
    /// # Errors
    /// Refuses missing or changed generation authority.
    fn replay_origin(&self) -> Result<&Arc<CompositeReplayOrigin>, CompositeOriginRefusal>;
}

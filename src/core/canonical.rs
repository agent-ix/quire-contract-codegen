//! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
//!
//! A digest preimage is encoded here and nowhere else, never by `serde_json`, whose key order is
//! struct field order and which does not canonicalise integers above 2^53.

use std::fmt;

use quire_canonical::{Encode, Limits};

/// The canonical byte ceiling of one digest preimage.
const PREIMAGE_LIMITS: Limits = Limits::new(1 << 20);

/// Why a value has no canonical bytes or digest: the encoder refused it.
#[derive(Debug)]
pub struct DigestError(quire_canonical::Error);

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the value is not RFC 8785 encodable: {}", self.0)
    }
}

impl std::error::Error for DigestError {}

/// The SHA-256 digest of the RFC 8785 encoding of `preimage`, with no domain label. This is
/// interim: ADR-013 O-09 records that the obligation identity's digest domain is not in the closed
/// FR-201 set today, and QC-4 / TK-07 may add one, which would change every identity.
///
/// # Errors
///
/// [`DigestError`] when `preimage` has no RFC 8785 encoding within the ceilings.
pub(crate) fn content_digest<T: Encode + ?Sized>(preimage: &T) -> Result<[u8; 32], DigestError> {
    quire_canonical::sha256(preimage, PREIMAGE_LIMITS)
        .map(|digest| *digest.as_bytes())
        .map_err(DigestError)
}

/// The RFC 8785 canonical bytes of `value`: the bytes [`content_digest`] hashes, for a document
/// that is provided as well as addressed by its `sha256-jcs` digest.
///
/// # Errors
///
/// [`DigestError`] when `value` has no RFC 8785 encoding within the ceilings.
pub(crate) fn content_bytes<T: Encode + ?Sized>(value: &T) -> Result<Vec<u8>, DigestError> {
    quire_canonical::to_vec(value, PREIMAGE_LIMITS).map_err(DigestError)
}

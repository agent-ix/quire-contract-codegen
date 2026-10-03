//! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
//!
//! A digest preimage is encoded here and nowhere else, never by `serde_json`, whose key order is
//! struct field order and which does not canonicalise integers above 2^53.

use std::fmt;

use quire_canonical::{Encode, Limits};

/// The canonical byte ceiling of one digest preimage.
const PREIMAGE_LIMITS: Limits = Limits::new(1 << 20);

/// Why a preimage has no digest: the encoder refused it.
#[derive(Debug)]
pub struct DigestError(quire_canonical::Error);

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the digest preimage is not encodable: {}", self.0)
    }
}

impl std::error::Error for DigestError {}

/// The SHA-256 digest of the RFC 8785 encoding of `preimage`, with no domain label: ADR-013 O-09
/// says the obligation identity's digest domain is not in the closed FR-201 set.
///
/// # Errors
///
/// [`DigestError`] when `preimage` has no RFC 8785 encoding within the ceilings.
pub(crate) fn content_digest<T: Encode + ?Sized>(preimage: &T) -> Result<[u8; 32], DigestError> {
    quire_canonical::sha256(preimage, PREIMAGE_LIMITS)
        .map(|digest| *digest.as_bytes())
        .map_err(DigestError)
}

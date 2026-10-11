//! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
//!
//! A digest preimage is encoded here and nowhere else, never by `serde_json`, whose key order is
//! struct field order and which does not canonicalise integers above 2^53.

use std::{fmt, io};

use quire_canonical::{Encode, Limits};
use serde::Serialize;

use crate::core::artifact::MAX_ARTIFACT_BYTES;

/// The canonical byte ceiling of one digest preimage.
const PREIMAGE_LIMITS: Limits = Limits::new(1 << 20);
// Reserve one byte for the required trailing newline in the artifact itself.
const ARTIFACT_LIMITS: Limits = Limits::new((MAX_ARTIFACT_BYTES - 1) as u64);

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

/// A typed generated JSON artifact could not be encoded within the artifact ceiling.
#[derive(Debug)]
pub(crate) enum ArtifactJsonError {
    /// The typed record could not be projected into JSON values.
    Projection(serde_json::Error),
    /// The canonical encoder refused the projected value.
    Encode(quire_canonical::Error),
    /// Canonical JSON bytes could not be represented as Rust text.
    Utf8(std::string::FromUtf8Error),
}

/// The finite generated record shapes allowed through the JSON artifact projection. Each
/// implementation must be a derived, bounded-depth `Serialize` record with no user-defined
/// serializer that can emit unbounded nested values.
pub(crate) trait ArtifactJsonRecord: Serialize {}

/// Discards a serde projection while bounding the bytes it can ask us to write. The preflight
/// runs before `to_value`, so an oversized record never materializes an unbounded JSON tree.
struct ProjectionCounter {
    remaining: usize,
}

impl io::Write for ProjectionCounter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.remaining {
            return Err(io::Error::other(
                "generated JSON projection exceeds the artifact ceiling",
            ));
        }
        self.remaining -= bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Display for ArtifactJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Projection(error) => error.fmt(f),
            Self::Encode(error) => error.fmt(f),
            Self::Utf8(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ArtifactJsonError {}

/// RFC 8785 text for a typed generated artifact, followed by one newline. The record's own
/// `Serialize` implementation defines its schema; `serde_json::Value` is only an intermediate
/// tree. A counted serde preflight limits its size before allocation. The sole emitted byte
/// encoder is `quire-canonical`, which walks that tree with an explicit stack.
///
/// # Errors
///
/// [`ArtifactJsonError`] when projection or canonical encoding fails, including the byte ceiling.
pub(crate) fn artifact_json<T: ArtifactJsonRecord + ?Sized>(
    value: &T,
) -> Result<String, ArtifactJsonError> {
    serde_json::to_writer(
        ProjectionCounter {
            remaining: MAX_ARTIFACT_BYTES - 1,
        },
        value,
    )
    .map_err(ArtifactJsonError::Projection)?;
    let tree = serde_json::to_value(value).map_err(ArtifactJsonError::Projection)?;
    let mut bytes =
        quire_canonical::to_vec(&tree, ARTIFACT_LIMITS).map_err(ArtifactJsonError::Encode)?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(ArtifactJsonError::Utf8)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::artifact_json;

    #[derive(Serialize)]
    struct Unsorted<'a> {
        zeta: &'a str,
        alpha: u8,
    }

    impl super::ArtifactJsonRecord for Unsorted<'_> {}

    /// Trace: AD-004 L-9, TC-023. The emitted file has canonical key order and one newline;
    /// an ordinary serde_json serializer would preserve this struct's opposite field order.
    #[test]
    fn generated_json_uses_canonical_bytes() {
        let text = artifact_json(&Unsorted {
            zeta: "z",
            alpha: 1,
        })
        .expect("bounded record");
        assert_eq!(text, "{\"alpha\":1,\"zeta\":\"z\"}\n");
    }

    /// Trace: AD-004 L-9, TC-023. A typed artifact above its byte ceiling refuses rather than
    /// returning a truncated or partially encoded JSON file.
    #[test]
    fn generated_json_refuses_an_over_bound_record() {
        let large = "x".repeat(super::MAX_ARTIFACT_BYTES);
        let error = artifact_json(&Unsorted {
            zeta: &large,
            alpha: 1,
        })
        .expect_err("the counted projection must refuse before materializing a JSON tree");
        assert!(matches!(
            error,
            super::ArtifactJsonError::Projection(ref source)
                if source.is_io()
                    && source.to_string().contains("exceeds the artifact ceiling")
        ));
    }
}

//! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
//!
//! A digest preimage is encoded here and nowhere else. Serde may project an owning record into a
//! bounded JSON value; only `quire-canonical` orders and hashes its bytes.

use std::{fmt, io};

use quire_canonical::{Encode, Limits};
use serde::Serialize;

use crate::core::artifact::MAX_ARTIFACT_BYTES;

/// The canonical byte ceiling of one digest preimage.
const PREIMAGE_BYTES: usize = 1 << 20;
const PREIMAGE_LIMITS: Limits = Limits::new(PREIMAGE_BYTES as u64);
// FiniteInput's owning serializer writes each object/reference as a three-field map. The corpus
// identity replaces those maps with three-string tuples before hashing. Each map is 35 bytes
// longer than its tuple, and validated fields are nonempty: a map is less than four times its
// tuple's bytes. A final preimage within 1 MiB therefore has an owning input projection within
// 4 MiB. Keep this intermediate allocation bounded without refusing a valid compact identity.
const OWNER_INPUT_PROJECTION_BYTES: usize = 4 * PREIMAGE_BYTES;
// Reserve one byte for the required trailing newline in the artifact itself.
const ARTIFACT_LIMITS: Limits = Limits::new((MAX_ARTIFACT_BYTES - 1) as u64);

/// Why a value has no canonical bytes or digest.
#[derive(Debug)]
pub struct DigestError(DigestErrorKind);

#[derive(Debug)]
enum DigestErrorKind {
    Projection(serde_json::Error),
    Shape(&'static str),
    Encode(quire_canonical::Error),
}

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            DigestErrorKind::Projection(error) => {
                write!(f, "JSON preimage projection failed: {error}")
            }
            DigestErrorKind::Shape(message) => write!(f, "JSON preimage shape changed: {message}"),
            DigestErrorKind::Encode(error) => {
                write!(f, "the value is not RFC 8785 encodable: {error}")
            }
        }
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
        .map_err(|error| DigestError(DigestErrorKind::Encode(error)))
}

/// The RFC 8785 canonical bytes of `value`: the bytes [`content_digest`] hashes, for a document
/// that is provided as well as addressed by its `sha256-jcs` digest.
///
/// # Errors
///
/// [`DigestError`] when `value` has no RFC 8785 encoding within the ceilings.
pub(crate) fn content_bytes<T: Encode + ?Sized>(value: &T) -> Result<Vec<u8>, DigestError> {
    quire_canonical::to_vec(value, PREIMAGE_LIMITS)
        .map_err(|error| DigestError(DigestErrorKind::Encode(error)))
}

/// Projects an owning serializer into a JSON value under the digest-preimage ceiling. The
/// counted first pass refuses oversized input before `to_value` allocates its tree. The returned
/// value must still pass through `content_digest` or `content_bytes`; serde does not emit the
/// canonical digest bytes.
///
/// # Errors
///
/// [`DigestError`] when the owning record cannot be projected within the ceiling.
pub(crate) fn preimage_value<T: Serialize + ?Sized>(
    value: &T,
) -> Result<serde_json::Value, DigestError> {
    projected_value(
        value,
        PREIMAGE_BYTES,
        "canonical preimage exceeds the byte ceiling",
    )
}

/// Projects the owning `FiniteInput` before its object/reference maps become compact tuples.
/// The final normalized preimage is still checked against [`PREIMAGE_LIMITS`].
///
/// # Errors
///
/// [`DigestError`] when the owning record exceeds its derived intermediate ceiling.
pub(crate) fn owner_input_value<T: Serialize + ?Sized>(
    value: &T,
) -> Result<serde_json::Value, DigestError> {
    projected_value(
        value,
        OWNER_INPUT_PROJECTION_BYTES,
        "finite input projection exceeds the byte ceiling",
    )
}

fn projected_value<T: Serialize + ?Sized>(
    value: &T,
    byte_limit: usize,
    error: &'static str,
) -> Result<serde_json::Value, DigestError> {
    counted_projection(value, byte_limit, error)
        .map_err(|error| DigestError(DigestErrorKind::Projection(error)))?;
    serde_json::to_value(value).map_err(|error| DigestError(DigestErrorKind::Projection(error)))
}

/// Refuses a change to the owning serializer's required identity shape.
pub(crate) fn preimage_shape_changed(message: &'static str) -> DigestError {
    DigestError(DigestErrorKind::Shape(message))
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
    error: &'static str,
}

impl io::Write for ProjectionCounter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.remaining {
            return Err(io::Error::other(self.error));
        }
        self.remaining -= bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn counted_projection<T: Serialize + ?Sized>(
    value: &T,
    limit: usize,
    message: &'static str,
) -> Result<(), serde_json::Error> {
    serde_json::to_writer(
        ProjectionCounter {
            remaining: limit,
            error: message,
        },
        value,
    )
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
    counted_projection(
        value,
        MAX_ARTIFACT_BYTES - 1,
        "generated JSON projection exceeds the artifact ceiling",
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

//! The one place CG calls `quire-canonical` (AD-004 `core/canonical`, step 1a; AD-003 E-1).
//!
//! `quire-canonical` is the RFC 8785 encoder and its digest. CG writes no encoding or hash routine
//! of its own: a JSON artifact CG emits and a content digest CG mints are both made here, over the
//! same encoding, so one value never has two byte forms. Member names are sorted and numbers are
//! ES6 text, so the bytes do not depend on a struct's field order. An integer is carried as an
//! IEEE 754 double and an integer whose magnitude exceeds 2^53 is refused rather than rounded.
//!
//! This module becomes `core/canonical.rs`.

use std::fmt;

use quire_canonical::{Limits, Sha256Digest};
use serde::Serialize;

use crate::{artifact::MAX_ARTIFACT_BYTES, identity::ContentDigest};

/// The deepest nesting any encoding CG makes may open. The values encoded here are flat records
/// and short lists of them, far below this.
const MAX_DEPTH: u32 = 64;

/// A value could not be encoded as RFC 8785 canonical JSON.
#[derive(Debug)]
pub(crate) enum CanonicalError {
    /// The encoder refused the value: an integer above 2^53, a non-finite float, a key that is not
    /// a string, or a limit reached.
    Encode(quire_canonical::Error),
    /// The encoder's output was not UTF-8, which RFC 8785 text always is.
    NotUtf8(std::string::FromUtf8Error),
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encode(error) => write!(formatter, "{error}"),
            Self::NotUtf8(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for CanonicalError {}

/// The limits every encoding runs under: an artifact's byte ceiling and [`MAX_DEPTH`].
const LIMITS: Limits = match Limits::new(MAX_ARTIFACT_BYTES as u64, MAX_DEPTH) {
    Ok(limits) => limits,
    Err(_) => panic!("MAX_DEPTH is within quire_canonical::Limits::MAX_DEPTH"),
};

/// `value` as the text of a JSON artifact file: its RFC 8785 canonical bytes and one newline.
pub(crate) fn json_file(value: &(impl Serialize + ?Sized)) -> Result<String, CanonicalError> {
    let mut bytes = quire_canonical::to_vec(value, LIMITS).map_err(CanonicalError::Encode)?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(CanonicalError::NotUtf8)
}

/// The SHA-256 content digest of `value`'s RFC 8785 canonical bytes, with no newline.
pub(crate) fn content_digest(
    value: &(impl Serialize + ?Sized),
) -> Result<ContentDigest, CanonicalError> {
    let digest: Sha256Digest =
        quire_canonical::sha256(value, LIMITS).map_err(CanonicalError::Encode)?;
    Ok(ContentDigest::new(digest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Unsorted {
        zeta: u8,
        alpha: &'static str,
    }

    /// RFC 8785 sorts member names where `serde_json` keeps field order, and the file form adds
    /// one newline the digest preimage does not have.
    #[test]
    fn a_file_is_sorted_canonical_json_and_a_newline() {
        let text = json_file(&Unsorted {
            zeta: 1,
            alpha: "a",
        })
        .expect("a flat record encodes");
        assert_eq!(text, "{\"alpha\":\"a\",\"zeta\":1}\n");
    }

    #[test]
    fn a_digest_is_over_the_canonical_bytes_without_the_newline() {
        let value = Unsorted {
            zeta: 1,
            alpha: "a",
        };
        let digest = content_digest(&value).expect("a flat record digests");
        let reordered = serde_json::json!({"zeta": 1, "alpha": "a"});
        assert_eq!(
            digest,
            content_digest(&reordered).expect("the same members")
        );
        // `printf '{"alpha":"a","zeta":1}' | sha256sum`: the canonical bytes, no newline.
        assert_eq!(
            digest.to_string(),
            "e9e33096132dc36eacd1663db82f4d0a06b6c4f059c493fb60eed8a1f08d781e"
        );
    }

    #[test]
    fn an_integer_above_two_to_the_53_is_refused() {
        let beyond = (1_i128 << 53) + 1;
        assert!(content_digest(&beyond).is_err());
        assert!(json_file(&beyond).is_err());
        assert!(content_digest(&(1_i128 << 53)).is_ok());
    }
}

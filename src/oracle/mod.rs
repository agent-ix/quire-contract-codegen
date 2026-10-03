//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).
//!
//! The shared generation-result and claim vocabulary, the V1 Boolean and bound oracles, and the
//! exact scalar, composite equality and exact function oracle generators live here.

// V1 Boolean oracle; retired with V1.
pub(crate) mod boolean_v1;
// V1 bound oracle generation; retired with V1.
pub(crate) mod bound_v1;
// Shared generation-result and claim vocabulary (FR-014, FR-018, FR-021).
pub(crate) mod claim;
// Implements: FR-018
pub(crate) mod equality;
// Implements: FR-021
pub(crate) mod function;
// Implements: FR-014
pub(crate) mod scalar;

use quire_contract_model::{CheckedPackageLimit, CompleteLoweringRecordV2};

/// How one `failed` lowering record is refused (FR-014-AC-40 to FR-014-AC-42).
///
/// Each generator carries these three outcomes as variants of its own refusal enum, with the
/// same fields, and converts through `From`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoweringFailure {
    /// The lowering work ceiling was reached.
    WorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
        consumed: u64,
    },
    /// The package's byte ceiling was exceeded.
    ByteLimitExceeded {
        /// The ceiling the package was read under.
        limit: u64,
        /// The canonical byte count the encoder needed.
        consumed: u64,
    },
    /// The record names a limit kind Contract IR documents no `failed` record for.
    LimitUnrecognised {
        /// The snake_case name of the `CheckedPackageLimit` variant.
        limit_kind: &'static str,
        /// The record's ceiling.
        limit: u64,
        /// The record's counter.
        consumed: u64,
    },
}

/// The one place a `failed` lowering record's `limit_kind` is read (FR-014-AC-42): `work` is
/// work exhaustion, `bytes` is the byte ceiling, and every other kind is unrecognised and keeps
/// its own name. The match names every kind, so a kind Contract IR adds fails to compile here
/// instead of reading as work exhaustion.
///
/// Returns `None` for a record that is not a `failed` one.
pub(crate) fn classify_lowering_failure(
    record: &CompleteLoweringRecordV2,
) -> Option<LoweringFailure> {
    let CompleteLoweringRecordV2::Failed {
        limit_kind,
        limit,
        consumed,
        ..
    } = record
    else {
        return None;
    };
    let (limit, consumed) = (*limit, *consumed);
    let unrecognised = |limit_kind| LoweringFailure::LimitUnrecognised {
        limit_kind,
        limit,
        consumed,
    };
    Some(match limit_kind {
        CheckedPackageLimit::Work => LoweringFailure::WorkExhausted { limit, consumed },
        CheckedPackageLimit::Bytes => LoweringFailure::ByteLimitExceeded { limit, consumed },
        CheckedPackageLimit::Depth => unrecognised("depth"),
        CheckedPackageLimit::Nodes => unrecognised("nodes"),
        CheckedPackageLimit::Edges => unrecognised("edges"),
        CheckedPackageLimit::Occurrences => unrecognised("occurrences"),
        CheckedPackageLimit::Diagnostics => unrecognised("diagnostics"),
    })
}

/// Hand-built `failed` records for the three generators' own `Failed` arms: a package read
/// through CG's public API fails every requested record for bytes at once, so the per-node
/// case and the kinds Contract IR never emits are reachable only this way.
#[cfg(test)]
pub(crate) mod failed_records {
    use quire_contract_model::{CheckedNodeId, CheckedPackageLimit, CompleteLoweringRecordV2};

    /// The limit kinds that are neither `work` nor `bytes`, each with the snake_case name
    /// FR-014 gives it.
    pub(crate) const UNRECOGNISED_KINDS: [(CheckedPackageLimit, &str); 5] = [
        (CheckedPackageLimit::Depth, "depth"),
        (CheckedPackageLimit::Nodes, "nodes"),
        (CheckedPackageLimit::Edges, "edges"),
        (CheckedPackageLimit::Occurrences, "occurrences"),
        (CheckedPackageLimit::Diagnostics, "diagnostics"),
    ];

    /// A `failed` record of `limit_kind` with the given ceiling and counter.
    pub(crate) fn failed_record(
        limit_kind: CheckedPackageLimit,
        limit: u64,
        consumed: u64,
    ) -> CompleteLoweringRecordV2 {
        CompleteLoweringRecordV2::Failed {
            node_id: CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: "a".repeat(64).into(),
            },
            limit_kind,
            limit,
            consumed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::failed_records::{failed_record, UNRECOGNISED_KINDS};
    use super::*;

    /// The classifier gives `work` its own outcome, `bytes` its own, and every other kind the
    /// unrecognised outcome under its snake_case name, with the record's `limit` and `consumed`
    /// unchanged; a record that did not fail is none of them.
    ///
    /// Trace: FR-014-AC-41, TC-024.
    #[test]
    fn tc_024_the_classifier_reads_each_limit_kind_once() {
        assert_eq!(
            classify_lowering_failure(&failed_record(CheckedPackageLimit::Work, 5, 6)),
            Some(LoweringFailure::WorkExhausted {
                limit: 5,
                consumed: 6
            })
        );
        assert_eq!(
            classify_lowering_failure(&failed_record(CheckedPackageLimit::Bytes, 7, 8)),
            Some(LoweringFailure::ByteLimitExceeded {
                limit: 7,
                consumed: 8
            })
        );
        for (kind, name) in UNRECOGNISED_KINDS {
            assert_eq!(
                classify_lowering_failure(&failed_record(kind, 3, u64::MAX)),
                Some(LoweringFailure::LimitUnrecognised {
                    limit_kind: name,
                    limit: 3,
                    consumed: u64::MAX
                }),
                "{name}"
            );
        }
    }
}

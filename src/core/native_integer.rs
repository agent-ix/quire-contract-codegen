// SPDX-License-Identifier: AGPL-3.0-or-later
//! The selected native-integer contract and actual checked interval decisions (AD-006).

use quire_contract_model::CheckedNodeId;
use quire_exact::NativeIntegerInterval;
use serde::Serialize;

/// CG's selected operator-capacity contract, independently of Git provenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum NativeIntegerCapacityPolicy {
    /// Full admitted i128 Add/Subtract/Negate/ordering; signed32 Multiply
    /// operands with their exact signed64 image retained in i128.
    #[serde(rename = "i128-arithmetic-signed32-multiply/v1")]
    Signed32MultiplyV1,
}

/// Inclusive endpoints represented without losing precision in canonical JSON.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeIntegerIntervalDecision {
    /// Inclusive lower endpoint, canonical decimal.
    pub lower: String,
    /// Inclusive upper endpoint, canonical decimal.
    pub upper: String,
}

impl From<NativeIntegerInterval> for NativeIntegerIntervalDecision {
    fn from(interval: NativeIntegerInterval) -> Self {
        Self {
            lower: interval.lower().to_string(),
            upper: interval.upper().to_string(),
        }
    }
}

/// One operation's complete executed operand populations and checked kernel image.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeIntegerOperationDecision {
    /// Authentic source claim or clause owning this evaluation step.
    pub node_id: CheckedNodeId,
    /// Evaluation-plan index; zero for a single scalar operation.
    pub step: usize,
    /// Original admitted operator identity.
    pub operation_identity: String,
    /// Complete operand intervals supplied to the checked kernel constructor.
    pub operands: Vec<NativeIntegerIntervalDecision>,
    /// Actual kernel plan capacity; absent for Boolean-valued ordering.
    pub capacity: Option<NativeIntegerIntervalDecision>,
    /// Original semantic result domain, separately from arithmetic capacity.
    pub semantic_domain: Option<NativeIntegerIntervalDecision>,
}

/// One selected native ABI's versioned policy and actual interval decisions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeIntegerSelection {
    /// Selected capacity policy and revision; not a kernel-owned certificate.
    pub policy: NativeIntegerCapacityPolicy,
    /// Numeric operations in the original evaluation order.
    pub operations: Vec<NativeIntegerOperationDecision>,
}

impl NativeIntegerSelection {
    pub(crate) fn new() -> Self {
        Self {
            policy: NativeIntegerCapacityPolicy::Signed32MultiplyV1,
            operations: Vec::new(),
        }
    }

    /// The same selected metadata supplies both emitted source and proof identity.
    pub(crate) fn annotation(&self) -> Option<String> {
        let preimage = super::canonical::preimage_value(self).ok()?;
        let bytes = super::canonical::content_bytes(&preimage).ok()?;
        let text = String::from_utf8(bytes).ok()?;
        Some(format!("// Native integer selection: {text}\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    #[trace("FR-015-AC-125")]
    #[test]
    fn native_policy_revision_changes_canonical_identity_without_changing_domains() {
        let selected = NativeIntegerSelection::new();
        let current = crate::core::canonical::preimage_value(&selected).unwrap();
        assert_eq!(current["policy"], "i128-arithmetic-signed32-multiply/v1");
        let mut next_revision = current.clone();
        next_revision["policy"] = serde_json::json!("i128-arithmetic-signed32-multiply/v2");
        assert_ne!(
            crate::core::canonical::content_digest(&current).unwrap(),
            crate::core::canonical::content_digest(&next_revision).unwrap(),
        );
        let annotation = selected.annotation().unwrap();
        let emitted: serde_json::Value = serde_json::from_str(
            annotation
                .trim()
                .strip_prefix("// Native integer selection: ")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(emitted, current);
    }

    #[trace("FR-015-AC-125")]
    #[test]
    fn native_interval_identity_retains_full_i128_endpoints_as_exact_decimal() {
        let interval = NativeIntegerInterval::new(i128::MIN, i128::MAX).unwrap();
        let decision = NativeIntegerIntervalDecision::from(interval);
        let preimage = crate::core::canonical::preimage_value(&decision).unwrap();
        crate::core::canonical::content_bytes(&preimage).unwrap();
        assert_eq!(decision.lower.parse::<i128>().unwrap(), i128::MIN);
        assert_eq!(decision.upper.parse::<i128>().unwrap(), i128::MAX);
    }
}

//! Codegen-side admission for Contract IR's public bounded-Kani profile.
//!
//! This boundary consumes Contract IR's matrix without re-describing semantic families and
//! returns typed non-Boolean outcomes before a generator can render an artifact.

use quire_contract_ir::kani::{
    CapabilityDisposition, CapabilityEntry, KaniOutcome, KaniOutcomeKind, KaniProfile,
};

/// Selected public Contract IR profile as consumed by code generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedKaniProfile {
    profile: KaniProfile,
}

impl BoundedKaniProfile {
    /// Retains a Contract IR profile whose constructor validated identity and matrix shape.
    #[must_use]
    pub const fn new(profile: KaniProfile) -> Self {
        Self { profile }
    }

    /// Selects supported constructs or preserves the exact typed non-success disposition.
    pub fn classify(
        &self,
        constructs: &[String],
        source_id: &str,
    ) -> Result<Vec<CapabilityEntry>, KaniOutcome> {
        let entries = self.profile.classify(constructs, source_id)?;
        for entry in &entries {
            match &entry.disposition {
                CapabilityDisposition::Supported { .. } => {}
                CapabilityDisposition::Refused { code } => {
                    return Err(KaniOutcome::non_success(
                        KaniOutcomeKind::Refused,
                        code.clone(),
                        source_id,
                        self.profile.selection.revision.clone(),
                    ));
                }
                CapabilityDisposition::Inconclusive { code } => {
                    return Err(KaniOutcome::non_success(
                        KaniOutcomeKind::Inconclusive,
                        code.clone(),
                        source_id,
                        self.profile.selection.revision.clone(),
                    ));
                }
            }
        }
        Ok(entries)
    }
}

/// Classifies one request through the public profile with no reverse Contract IR dependency.
pub fn classify_bounded_kani_profile(
    profile: KaniProfile,
    constructs: &[String],
    source_id: &str,
) -> Result<Vec<CapabilityEntry>, KaniOutcome> {
    BoundedKaniProfile::new(profile).classify(constructs, source_id)
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, KaniProfile, ProfileSelection,
    };

    use super::classify_bounded_kani_profile;

    fn profile(disposition: CapabilityDisposition) -> KaniProfile {
        KaniProfile::new(
            ProfileSelection {
                profile: "kani-bounded/1".to_owned(),
                revision: "profile-r1".to_owned(),
                executable_digest: "exe".to_owned(),
                options_digest: "options".to_owned(),
                abi_revision: "abi-r1".to_owned(),
            },
            vec![CapabilityEntry {
                construct: "arithmetic.add".to_owned(),
                disposition,
            }],
        )
        .expect("fixture profile is valid")
    }

    /// Trace: FR-007-AC-1, FR-007-AC-3, TC-023.
    #[test]
    fn tc_023_profile_refusal_and_inconclusive_remain_non_boolean() {
        let request = vec!["arithmetic.add".to_owned()];
        for disposition in [
            CapabilityDisposition::Refused {
                code: "unsupported".to_owned(),
            },
            CapabilityDisposition::Inconclusive {
                code: "unavailable".to_owned(),
            },
        ] {
            let outcome = classify_bounded_kani_profile(profile(disposition), &request, "source")
                .expect_err("non-supported profile entry must not lower");
            assert_eq!(outcome.boolean_claim(), None);
        }
    }
}

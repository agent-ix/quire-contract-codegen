//! Codegen-side admission for Contract IR's public bounded-Kani profile.
//!
//! This boundary consumes Contract IR's matrix without re-describing semantic families and
//! returns the complete per-construct disposition census for a request, so a generator can
//! render an artifact for every construct rather than stopping at the first non-supported one.

use quire_contract_ir::kani::{CapabilityEntry, KaniOutcome, KaniProfile};

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

    /// Returns the complete ordered disposition census for every encountered construct.
    ///
    /// Every requested construct receives exactly one disposition -- `Supported`, `Refused`, or
    /// `Inconclusive` -- as its own [`CapabilityEntry`] (FR-007-AC-1). The upstream profile
    /// matrix already builds that complete per-construct census; this method returns it as-is
    /// rather than walking it and bailing out with a single [`KaniOutcome`] on the first
    /// `Refused` or `Inconclusive` entry, which discarded every disposition after it. A caller
    /// that needs to know whether every entry was supported inspects the returned census
    /// (FR-007-AC-3's typed non-Boolean dispositions are the `Refused`/`Inconclusive` entries
    /// inside it, not an early-exit error).
    ///
    /// `Err` is reserved for a request the upstream matrix cannot classify at all: an empty or
    /// duplicated construct name (`kani_capability_request_invalid`), or a construct absent from
    /// the profile's matrix entirely (`kani_capability_missing`). Both arrive with
    /// `KaniOutcomeKind::Refused` -- the same `kind` a refused *construct* disposition used to
    /// produce before this method stopped early-exiting on one. A caller cannot tell these two
    /// `Err` cases apart by `kind` alone and must discriminate on `code`. Either way this is a
    /// malformed request, not a disposition, so it is not part of the census.
    pub fn classify(
        &self,
        constructs: &[String],
        source_id: &str,
    ) -> Result<Vec<CapabilityEntry>, KaniOutcome> {
        self.profile.classify(constructs, source_id)
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

    fn profile(capabilities: Vec<CapabilityEntry>) -> KaniProfile {
        KaniProfile::new(
            ProfileSelection {
                profile: "kani-bounded/1".to_owned(),
                revision: "profile-r1".to_owned(),
                executable_digest: "exe".to_owned(),
                options_digest: "options".to_owned(),
                abi_revision: "abi-r1".to_owned(),
            },
            capabilities,
        )
        .expect("fixture profile is valid")
    }

    fn entry(construct: &str, disposition: CapabilityDisposition) -> CapabilityEntry {
        CapabilityEntry {
            construct: construct.to_owned(),
            disposition,
        }
    }

    /// Builds a matrix that is a strict superset of any three-construct request used below,
    /// listed in an order different from every request, so that a mutation which returns the
    /// whole matrix verbatim (ignoring the request's membership and order) is distinguishable
    /// from the correct behavior by both length and by per-index construct/disposition.
    fn superset_matrix() -> Vec<CapabilityEntry> {
        vec![
            entry(
                "collection.contains",
                CapabilityDisposition::Inconclusive {
                    code: "unavailable".to_owned(),
                },
            ),
            entry(
                "extra.unrequested",
                CapabilityDisposition::Supported {
                    module: "never_requested".to_owned(),
                },
            ),
            entry(
                "arithmetic.add",
                CapabilityDisposition::Refused {
                    code: "unsupported".to_owned(),
                },
            ),
            entry(
                "graph.reachable",
                CapabilityDisposition::Supported {
                    module: "finite_reference_graphs".to_owned(),
                },
            ),
        ]
    }

    /// Trace: FR-007-AC-1, FR-007-AC-3, TC-023.
    ///
    /// An early `Refused` disposition must not discard the dispositions of constructs
    /// encountered after it, and a later `Inconclusive` disposition must not discard a
    /// `Supported` one between it and the refusal: FR-007-AC-1 requires one exact disposition
    /// for every selected construct, not just the ones before the first non-supported entry.
    ///
    /// The matrix (`superset_matrix`) contains a fourth construct, `extra.unrequested`, that
    /// this test's request never names, and lists its three requested constructs in a different
    /// order than the request below. That makes this test catch a mutation that returns the
    /// whole matrix verbatim (`Ok(self.profile.capabilities.clone())`), ignoring the request
    /// entirely: such a mutation would return 4 entries in matrix order, not 3 in request order,
    /// so it fails both the length assertion and the per-index assertions here. This was verified
    /// by hand against that exact mutation.
    #[test]
    fn tc_023_census_reports_every_construct_including_ones_after_an_early_refusal() {
        let request = vec![
            "arithmetic.add".to_owned(),
            "graph.reachable".to_owned(),
            "collection.contains".to_owned(),
        ];

        let census = classify_bounded_kani_profile(profile(superset_matrix()), &request, "source")
            .expect(
            "a structurally valid request yields the complete census, never an early-exit error",
        );

        assert_eq!(
            census.len(),
            request.len(),
            "the census must follow the request's membership, not the larger matrix's"
        );
        assert_eq!(
            census
                .iter()
                .map(|e| e.construct.as_str())
                .collect::<Vec<_>>(),
            vec!["arithmetic.add", "graph.reachable", "collection.contains"],
            "the census must follow the request's order, not the matrix's order"
        );
        assert!(
            census.iter().all(|e| e.construct != "extra.unrequested"),
            "a construct absent from the request must not appear in the census"
        );
        assert_eq!(
            census[0].disposition,
            CapabilityDisposition::Refused {
                code: "unsupported".to_owned(),
            },
            "the refused construct's disposition must still appear in the census"
        );
        assert_eq!(
            census[1].disposition,
            CapabilityDisposition::Supported {
                module: "finite_reference_graphs".to_owned(),
            },
            "a supported construct after a refusal must not be discarded"
        );
        assert_eq!(
            census[2].disposition,
            CapabilityDisposition::Inconclusive {
                code: "unavailable".to_owned(),
            },
            "an inconclusive construct after a refusal must not be discarded"
        );
    }

    /// Trace: FR-007-AC-1, TC-023.
    ///
    /// A request naming a construct absent from the profile's matrix is a malformed request, not
    /// a disposition: `classify` must reject it with `Err` carrying the `kani_capability_missing`
    /// code, rather than folding it into the returned census or reusing the
    /// `kani_capability_request_invalid` code reserved for an empty/duplicate construct name.
    #[test]
    fn tc_023_missing_construct_is_rejected_with_kani_capability_missing() {
        let request = vec!["arithmetic.add".to_owned(), "no.such.construct".to_owned()];

        let outcome = classify_bounded_kani_profile(profile(superset_matrix()), &request, "source")
            .expect_err("a construct absent from the matrix must not be classified");

        assert_eq!(
            outcome.kind,
            quire_contract_ir::kani::KaniOutcomeKind::Refused
        );
        assert_eq!(outcome.code, "kani_capability_missing");
    }
}

//! The one Kani generator: the request and result vocabulary, the negotiation entry and the
//! family lowerers.

// Census validation; interim, see the file header.
pub(crate) mod census_validation;
// The V1 clause lowering the precondition and contract families share.
pub(crate) mod clause;
// Bounded collection claims over admitted V2 model fields.
pub(crate) mod collection;
// The contract family.
pub(crate) mod contract;
// The bounded-Kani corpus family.
pub(crate) mod corpus;
// The state-frame family.
// Implements: FR-015 (IR-412: state-clause operation contract and frame effects).
pub(crate) mod frame;
// The Kani family lowerings over Contract IR.
pub(crate) mod lower;
// `negotiate_kani_obligations`, the one public generation entry.
pub(crate) mod negotiate;
// The request and result vocabulary every family speaks.
pub(crate) mod outcome;
// Physical component bindings for prepared Rational predicates.
pub(crate) mod rational_predicate;
// The precondition family.
pub(crate) mod precondition;
// The validated harness path and the persisted per-harness record.
pub(crate) mod record;
#[cfg(test)]
mod role_parity_tests;
// The scalar family.
pub(crate) mod scalar;
// The V1 bundle generator; interim, see the file header.
pub(crate) mod v1_bundle;

// Validated private descriptions and their pure renderer.
mod render;
mod spec;

// The private libtest fixture returns the existing sealed output, never a HarnessSpec.
#[cfg(test)]
pub(crate) fn unsatisfiable_harness_for_test() -> crate::kani::identity::KaniScalarObligationHarness
{
    render::tests::unsatisfiable_harness()
}

// A false-bound paired-run control uses the same renderer and origin sealing as production.
#[cfg(test)]
pub(crate) fn false_bound_harness_for_test(
    original: &crate::kani::identity::KaniScalarObligationHarness,
    artifacts: &[crate::core::artifact::Artifact],
) -> crate::kani::identity::KaniScalarObligationHarness {
    scalar::false_bound_harness_for_test(original, artifacts)
}

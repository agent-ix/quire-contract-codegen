//! FR-018-AC-2, AC-8 (the `check_type` guard half), AC-9 and AC-11 (the
//! complementary-outcomes half): generated composite-equality oracles agree
//! with an independently assembled direct Contract Runtime call, across
//! record, tuple, option, collection and recursive shapes; a malformed
//! environment is refused with no charge; a denied charge surfaces as
//! `Outcome::Incomplete`, never a completed Boolean; and the two operator
//! variants over one expression node produce complementary outcomes.
//!
//! Nothing generated is committed. This module generates the corpus crate now
//! and runs `tests/composite_equality_support/agreement_cases.rs` against it
//! through `exact_scalar_agreement`'s scratch-crate runner.

use super::exact_scalar_agreement::run_agreement_cases;

/// Trace: FR-018-AC-2, FR-018-AC-8, FR-018-AC-9, FR-018-AC-10, FR-018-AC-11, TC-029.
///
/// The crate the generator emits now for the corpus request compiles and every
/// agreement case passes against it. Each case drives the generated oracle and
/// a direct runtime call assembled from the request's own descriptor, so an
/// emitted operator, operand order or conversion that disagrees with the
/// descriptor fails here.
#[test]
fn tc_029_generated_composite_equality_crate_agrees_with_direct_runtime() {
    let oracles = super::composite_equality_generation::corpus_oracles();
    let names = super::composite_equality_generation::agreement_names(&oracles);
    run_agreement_cases(
        "composite-equality-agreement",
        oracles
            .artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/composite_equality_support/agreement_cases.rs",
    );
}

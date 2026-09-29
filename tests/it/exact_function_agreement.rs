//! FR-021-AC-2, AC-4, AC-5, AC-7, AC-9, AC-17: generated function-application
//! oracles agree with an independently assembled direct Contract Runtime
//! call; `InputRefusal` surfaces unchanged; the shared `MAX_CALL_DEPTH`
//! budget bounds a nested-call chain; a denial injected at `function.call`
//! yields `Outcome::Incomplete` with no charge applied; and the static
//! location map's `origin` half is confirmed against the runtime's own
//! `CheckRefusal`.
//!
//! Nothing generated is committed. The execution cases (AC-2, AC-4, AC-5,
//! AC-7, AC-9) live in `tests/exact_function_support/agreement_cases.rs` and
//! run against the crate the generator emits now, through
//! `exact_scalar_agreement`'s scratch-crate runner. AC-17 needs the generator
//! itself and runs here.
//!
//! FR-021-AC-18 (three-way agreement with the QSL authority) is not
//! implemented here: the spec records it "🚧 Planned, pending the
//! quire-spec-language re-pin named in Dependencies". IR-254 repointed this
//! repository's QSL pin, but onto `qsl-replay`'s public API rather than the
//! `quire_spec_language::value::expression` API AC-18 was written against
//! (see `src/exact_function.rs`'s module doc). The agreement legs are exactly
//! two: the generated oracle and a direct Contract Runtime call.

// Duplicated per consumer (also `exact_function_generation.rs`) for structural consistency with
// the exact_scalar/composite_equality families (IR-237). Unlike those two, this package.rs holds
// no process-global state, so duplication here isn't load-bearing the way it is for them -- it's
// kept for uniformity across the tests/it/*_support/package.rs pattern, not to avoid a hazard.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_function_support/package.rs"]
mod package;

use package::*;
use quire_contract_runtime::exact as rt;

use super::exact_scalar_agreement::run_agreement_cases;

/// Trace: FR-021-AC-2, FR-021-AC-4, FR-021-AC-5, FR-021-AC-7, FR-021-AC-9, TC-031.
///
/// The main-corpus crate the generator emits now, with the chain corpus's
/// generated source beside it, compiles and passes every agreement case: each
/// generated oracle equals a direct `CheckedPackage::call` on an independently
/// assembled package, input refusals surface unchanged and before any charge,
/// arity is decided before value kind, a denied `function.call` is incomplete
/// with no charge applied, and the generated 140-link chain is refused once
/// `MAX_CALL_DEPTH` is exceeded.
#[test]
fn tc_031_generated_function_crate_agrees_with_direct_runtime() {
    let main = super::exact_function_generation::main_oracles();
    let chain = super::exact_function_generation::chain_oracles();
    let chain_source = chain
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("the chain corpus generates src/lib.rs");
    run_agreement_cases(
        "exact-function-agreement",
        main.artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
        &[("chain/lib.rs", chain_source.contents.as_str())],
        "tests/exact_function_support/agreement_cases.rs",
    );
}

/// Trace: FR-021-AC-17, TC-031. Each location map entry's `origin` field
/// equals the `Origin::Body{function, index}` the runtime itself reports
/// for that function: re-submit the same assembled package to
/// `PackageDeclarations::check` with one function's measure left
/// undischarged, read the `Origin::Body` off the returned `CheckRefusal`,
/// and assert it equals the location map's recorded origin for that
/// function.
#[test]
fn tc_031_ac17_location_map_origin_confirmed_against_runtime_check_refusal() {
    let package = ext_corpus_package().admit();
    let functions = main_functions_for_ac17();
    let items = vec![item(ITEM_CALL_ADD, "add_fn")];
    let oracles =
        quire_contract_codegen::generate_exact_function_oracles(&package, &functions, &items)
            .expect("generation succeeds");

    let add_entry = oracles
        .location_map
        .iter()
        .find(|entry| entry.function == "add_fn")
        .expect("add_fn has a location map entry");

    // Rebuild the SAME assembled package the generator itself would admit
    // (same functions, same order -- by declaring node id, digest domain
    // then digest, matching the generator's own Behavior), but with
    // `add_fn`'s own measure left undischarged, so `check` refuses at
    // exactly that function's own Origin::Body.
    let mut ordered = functions.clone();
    ordered.sort_by(|a, b| a.node_id.cmp(&b.node_id));
    let declarations: Vec<rt::FunctionDeclaration> = ordered
        .iter()
        .map(|declaration| rt::FunctionDeclaration {
            name: declaration.name.clone(),
            parameters: vec![
                ("a".to_owned(), rt::ValueType::Integer),
                ("b".to_owned(), rt::ValueType::Integer),
            ],
            result: rt::ValueType::Integer,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: declaration.name != "add_fn",
            body: Box::new(|_frame, _args| rt::Outcome::Refused(rt::Refusal::CheckedInvariant)),
        })
        .collect();
    // `CheckedPackage` carries no `Debug` impl, so `expect_err` (which
    // requires the `Ok` side to be `Debug`) cannot be used here.
    let check_result = rt::PackageDeclarations {
        types: rt::TypeEnvironment::new(
            Vec::new(),
            core::iter::empty::<rt::ObjectTypeDeclaration>(),
        )
        .expect("empty declaration closure admits"),
        functions: declarations,
    }
    .check(rt::CheckMode::Linked, rt::CheckingLimits::default());
    let refusals = match check_result {
        Ok(_) => panic!("expected an undischarged-measure refusal, got an admitted package"),
        Err(refusals) => refusals,
    };

    let refusal = refusals
        .iter()
        .find(|refusal| matches!(refusal.cause, rt::CheckCause::UndischargedMeasure))
        .expect("an UndischargedMeasure refusal exists");
    match &refusal.location.origin {
        rt::Origin::Body { function, index } => {
            assert_eq!(function, "add_fn");
            match &add_entry.location.origin {
                quire_contract_codegen::RecordedOrigin::Body {
                    function: recorded_function,
                    index: recorded_index,
                } => {
                    assert_eq!(recorded_function, function);
                    assert_eq!(recorded_index, index);
                }
            }
        }
        other => panic!("expected Origin::Body, got {other:?}"),
    }
}

/// `main_functions`-equivalent restricted to what AC-17's test needs
/// (`add_fn` plus one sibling, so the function ordering is non-trivial).
fn main_functions_for_ac17() -> Vec<quire_contract_codegen::ExactFunctionDeclaration> {
    vec![function_add("add_fn"), function_unrelated("unrelated_fn")]
}

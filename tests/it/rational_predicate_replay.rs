//! Native-injected witnesses exercise the public adapter; none is solver playback evidence.
use std::collections::BTreeMap;

use ix_trace_rs::trace;
use qsl_replay::{
    Category, ReplayLimits, ReplayRequestWire, ReplaySource, ScalarLimits, WitnessArmResult,
    WitnessSettlement, WitnessValue,
};
use quire_contract_codegen::{
    replay_falsification, FunctionPredicateItem, LockedSource, ObligationIdentityError,
    ReplayInputs, ReplayPackage, SpineReplayError,
};
use quire_contract_model::{CheckedNodeId, ContractPackageV1};
use quire_exact::{Integer, Rational};

use crate::rational_predicate_support::{admit, HEALTHY, MUTANT};

struct Fixture {
    package: ReplayPackage,
    contract: ContractPackageV1,
    function: CheckedNodeId,
}
impl Fixture {
    fn new(source: &str, function: &str) -> Self {
        let (contract, selected) = admit(source);
        let package = ReplayPackage::new(
            ReplayInputs {
                decode_limits: crate::unbounded_decode_limits(),
                source: LockedSource {
                    authority: "a".into(),
                    identity: "u".into(),
                    namespace: "git".into(),
                    revision: "1".into(),
                    bytes: source.as_bytes().to_vec(),
                },
                dependencies: Vec::new(),
                accounting_limits: ScalarLimits {
                    integer_bits: u64::MAX,
                    decimal_digits: u64::MAX,
                    scale_expansion: u64::MAX,
                    text_input_bytes: u64::MAX,
                    text_scalars: u64::MAX,
                    normalized_scalars: u64::MAX,
                    unit_edges: u64::MAX,
                    value_occurrences: u64::MAX,
                    work_units: u64::MAX,
                    result_units: u64::MAX,
                },
                stage_limits: BTreeMap::new(),
                declared_domains: Vec::new(),
                replay_limits: ReplayLimits::default(),
            },
            function,
        )
        .expect("the genuine selected source admits");
        Self {
            package,
            contract,
            function: selected,
        }
    }
    fn item(&self) -> FunctionPredicateItem<'_> {
        FunctionPredicateItem {
            package: &self.contract,
            function: &self.function,
        }
    }
    fn request(&self, source: ReplaySource) -> ReplayRequestWire {
        self.package
            .rational_predicate_request(self.item(), source)
            .unwrap()
    }
    fn replay(
        &self,
        values: &[(String, WitnessValue)],
    ) -> Result<WitnessArmResult, SpineReplayError> {
        replay_falsification(
            "native_injected_rational",
            "predicate",
            values,
            &self.package.parameters(),
            |source| self.request(source),
            ReplayLimits::default(),
        )
    }
}
fn value(n: i64, d: i64) -> WitnessValue {
    let rational = Rational::new(Integer::from(n), Integer::from(d)).unwrap();
    WitnessValue::Rational {
        numerator: rational.numerator().clone(),
        denominator: rational.denominator().clone(),
    }
}
fn arguments(a: WitnessValue, b: WitnessValue) -> Vec<(String, WitnessValue)> {
    vec![("a".into(), a), ("b".into(), b)]
}
fn assert_evaluation(result: WitnessArmResult, expected: bool) {
    if expected {
        assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
        assert_eq!(
            result.disagreement().unwrap().replayed().category(),
            Category::Success
        );
    } else {
        assert_eq!(
            result.settlement(),
            WitnessSettlement::ReproducedWithEvaluatedWitness
        );
        assert_eq!(result.category(), Category::Violation);
        assert!(result.disagreement().is_none());
    }
    assert!(result.charges().work_units > 0);
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn native_injected_rational_witnesses_reach_actual_whole_predicate_replay() {
    let healthy = Fixture::new(HEALTHY, "rational_commutative");
    let mutant = Fixture::new(MUTANT, "rational_commutative");
    for (a, b) in [
        ((1, 2), (2, 3)),
        ((-1, 2), (1, 3)),
        ((0, 1), (0, 1)),
        ((-9, 1), (9, 1)),
        ((1, 9), (-1, 9)),
    ] {
        let input = arguments(value(a.0, a.1), value(b.0, b.1));
        assert_evaluation(
            healthy
                .replay(&input)
                .expect("the public adapter must support Rational"),
            true,
        );
        assert_evaluation(
            mutant
                .replay(&input)
                .expect("the original mutated predicate must replay"),
            a == b,
        );
        let mut permuted = input;
        permuted.reverse();
        assert_evaluation(healthy.replay(&permuted).unwrap(), true);
        assert_evaluation(mutant.replay(&permuted).unwrap(), a == b);
    }
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn logical_domains_and_real_selected_site_bind_the_identity() {
    let original = Fixture::new(HEALTHY, "rational_commutative");
    let identity = original
        .package
        .rational_predicate_obligation_identity(original.item())
        .unwrap();
    assert_eq!(
        original
            .request(ReplaySource::Input(Vec::new()))
            .obligation_identity,
        *identity.as_bytes()
    );
    for domain in [
        "Rational[-8, 9; 1, 9]",
        "Rational[-9, 8; 1, 9]",
        "Rational[-9, 9; 2, 9]",
        "Rational[-9, 9; 1, 8]",
    ] {
        let source = HEALTHY.replacen("Rational[-9, 9; 1, 9]", domain, 1);
        let changed = Fixture::new(&source, "rational_commutative");
        assert_ne!(
            identity,
            changed
                .package
                .rational_predicate_obligation_identity(changed.item())
                .unwrap()
        );
        assert!(matches!(
            original
                .package
                .rational_predicate_obligation_identity(changed.item()),
            Err(ObligationIdentityError::SourcePackage)
        ));
    }
    let renamed = HEALTHY.replace("rational_commutative", "rational_renamed");
    let renamed = Fixture::new(&renamed, "rational_renamed");
    assert_ne!(
        identity,
        renamed
            .package
            .rational_predicate_obligation_identity(renamed.item())
            .unwrap()
    );
    assert!(matches!(
        original
            .package
            .rational_predicate_obligation_identity(renamed.item()),
        Err(ObligationIdentityError::SourcePackage)
    ));
    let foreign = original
        .contract
        .wire()
        .semantic_graph
        .nodes
        .iter()
        .find(|n| n.node_id != original.function)
        .unwrap();
    assert!(matches!(
        original
            .package
            .rational_predicate_obligation_identity(FunctionPredicateItem {
                package: &original.contract,
                function: &foreign.node_id
            }),
        Err(ObligationIdentityError::FunctionSelection)
    ));
    // Public source/package/node authorities are immutable; no caller vector of logical domains
    // or physical bindings is accepted by this request constructor.
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn genuine_reversed_declaration_keeps_name_bound_values() {
    let reversed = MUTANT.replace(
        "a: Rational[-9, 9; 1, 9], b: Rational[-9, 9; 1, 9]",
        "b: Rational[-9, 9; 1, 9], a: Rational[-9, 9; 1, 9]",
    );
    assert_ne!(reversed, MUTANT);
    let fixture = Fixture::new(&reversed, "rational_commutative");
    assert_eq!(fixture.package.parameters()[0].argument, "b");
    let mut input = arguments(value(1, 2), value(2, 3));
    assert_evaluation(fixture.replay(&input).unwrap(), false);
    input.reverse();
    assert_evaluation(fixture.replay(&input).unwrap(), false);
}

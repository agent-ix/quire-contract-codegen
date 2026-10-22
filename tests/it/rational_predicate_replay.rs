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
    assert_eq!(
        result.value(),
        Some(qsl_replay::EvaluatedValue::Boolean(expected))
    );
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

fn owner_refusal(result: Result<WitnessArmResult, SpineReplayError>) -> qsl_replay::ReplayRefusal {
    match result {
        Err(SpineReplayError::Refused(refusal)) => *refusal,
        other => panic!("expected the responsible typed owner refusal, got {other:?}"),
    }
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn native_rational_adverse_inputs_retain_responsible_owner_refusals() {
    use qsl_replay::{DecodeRefusal, EntryFault, ReplayRefusal, WireNodeId};
    let fixture = Fixture::new(HEALTHY, "rational_commutative");
    for argument in ["a", "b"] {
        let node = WireNodeId::from_hex(
            fixture
                .package
                .parameters()
                .iter()
                .find(|p| p.argument == argument)
                .unwrap()
                .node_id,
        )
        .unwrap();
        for invalid in [value(-10, 1), value(10, 1), value(1, 10)] {
            let mut input = arguments(value(1, 2), value(2, 3));
            input
                .iter_mut()
                .find(|(name, _)| name == argument)
                .unwrap()
                .1 = invalid;
            let observed = fixture.replay(&input);
            eprintln!(
                "domain candidate {argument} {:?}: {observed:?}",
                input.iter().find(|(name, _)| name == argument).unwrap().1
            );
            let refusal = owner_refusal(observed);
            let declared_position = fixture
                .package
                .parameters()
                .iter()
                .position(|p| p.argument == argument)
                .unwrap();
            assert!(
                matches!(refusal, ReplayRefusal::Supplied(_)),
                "{argument}: {refusal:?}"
            );
            assert!(
                matches!(refusal.input_refusal(), Some(qsl_replay::InputRefusal::WrongValueKind { parameter }) if *parameter == declared_position)
            );
        }
        let mut missing = arguments(value(1, 2), value(2, 3));
        missing.retain(|(name, _)| name != argument);
        assert!(
            matches!(owner_refusal(fixture.replay(&missing)),ReplayRefusal::Witness{refusal:DecodeRefusal::Missing(actual),..} if actual==node)
        );
        let mut duplicate = arguments(value(1, 2), value(2, 3));
        duplicate.push((argument.into(), value(1, 3)));
        assert!(
            matches!(owner_refusal(fixture.replay(&duplicate)),ReplayRefusal::Witness{refusal:DecodeRefusal::Duplicate(actual),..} if actual==node)
        );
        for (n, d) in [(0_i64, 2_i64), (2, 2), (1, 0), (1, -2)] {
            // Deliberately raw typed input, NOT a canonical physical-backend witness.
            // The owner text encoder serializes it; the owner's transcript decoder refuses it.
            let raw = WitnessValue::Rational {
                numerator: Integer::from(n),
                denominator: Integer::from(d),
            };
            assert!(raw.to_value_text().is_ok());
            let mut input = arguments(value(1, 2), value(2, 3));
            input
                .iter_mut()
                .find(|(name, _)| name == argument)
                .unwrap()
                .1 = raw;
            assert!(
                matches!(owner_refusal(fixture.replay(&input)),ReplayRefusal::Witness{refusal:DecodeRefusal::Malformed{parameter,fault:EntryFault::Spelling,..},..} if parameter==node)
            );
        }
    }
    let mut unknown = arguments(value(1, 2), value(2, 3));
    unknown.push(("foreign".into(), value(1, 2)));
    assert!(
        matches!(fixture.replay(&unknown),Err(SpineReplayError::UnboundArgument{argument}) if argument=="foreign")
    );
    // Changing only the function body retains parameter identities. Use an actual changed
    // parameter declaration/domain so this is genuinely a foreign parameter control.
    let foreign_source = HEALTHY.replacen("Rational[-9, 9; 1, 9]", "Rational[-8, 9; 1, 9]", 1);
    let foreign = Fixture::new(&foreign_source, "rational_commutative");
    let mut parameters = fixture.package.parameters();
    let foreign_parameters = foreign.package.parameters();
    let original_node = parameters[0].node_id;
    parameters[0].node_id = foreign_parameters
        .iter()
        .find(|p| p.argument == parameters[0].argument)
        .unwrap()
        .node_id;
    assert_ne!(parameters[0].node_id, original_node);
    let invalid = replay_falsification(
        "native_injected",
        "predicate",
        &arguments(value(1, 2), value(2, 3)),
        &parameters,
        |s| fixture.request(s),
        ReplayLimits::default(),
    );
    assert!(matches!(
        owner_refusal(invalid),
        ReplayRefusal::Witness {
            refusal: DecodeRefusal::Unbound(_),
            ..
        }
    ));
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn genuine_wrong_selection_stale_package_and_limits_are_not_repaired() {
    use qsl_replay::{ReplayRefusal, ReplayRequestRefusal};
    let fixture = Fixture::new(HEALTHY, "rational_commutative");
    let foreign = Fixture::new(MUTANT, "rational_commutative");
    let renamed = Fixture::new(
        &HEALTHY.replace("rational_commutative", "other_function"),
        "other_function",
    );
    let input = arguments(value(1, 2), value(2, 3));
    let wrong = replay_falsification(
        "native_injected",
        "predicate",
        &input,
        &fixture.package.parameters(),
        |s| {
            let mut request = fixture.request(s);
            request.selected_function = renamed
                .request(ReplaySource::Input(Vec::new()))
                .selected_function;
            request
        },
        ReplayLimits::default(),
    );
    assert!(matches!(
        owner_refusal(wrong),
        ReplayRefusal::UnknownFunction { .. }
    ));
    let stale = replay_falsification(
        "native_injected",
        "predicate",
        &input,
        &fixture.package.parameters(),
        |s| {
            let mut request = fixture.request(s);
            request.package_id = foreign.request(ReplaySource::Input(Vec::new())).package_id;
            request
        },
        ReplayLimits::default(),
    );
    assert!(matches!(
        owner_refusal(stale),
        ReplayRefusal::PackageIdMismatch { .. }
    ));
    let limited = replay_falsification(
        "native_injected",
        "predicate",
        &input,
        &fixture.package.parameters(),
        |s| fixture.request(s),
        ReplayLimits::default().with_input_bytes(1),
    );
    assert!(matches!(
        owner_refusal(limited),
        ReplayRefusal::Request(ReplayRequestRefusal::BoundExceeded(_))
    ));
    for argument in [
        WitnessValue::Text("unsupported".into()),
        WitnessValue::ExactInteger(Integer::from(1_i64)),
        WitnessValue::Uuid("550e8400-e29b-41d4-a716-446655440000".into()),
    ] {
        assert!(
            matches!(fixture.replay(&arguments(argument,value(1,2))),Err(SpineReplayError::UnsupportedWitnessValue{argument}) if argument=="a")
        );
    }
    let delimiter = replay_falsification(
        "bad|harness",
        "predicate",
        &input,
        &fixture.package.parameters(),
        |_| panic!("field delimiter must refuse before request construction"),
        ReplayLimits::default(),
    );
    assert!(matches!(delimiter, Err(SpineReplayError::FieldDelimiter)));
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn supplied_order_does_not_change_the_canonical_request_or_value_text() {
    use qsl_replay::{WireNodeId, Witness, WitnessBinding, WitnessValueType};
    let fixture = Fixture::new(HEALTHY, "rational_commutative");
    let mut input = arguments(value(1, 2), value(-2, 3));
    let mut transcripts = Vec::new();
    let mut identities = Vec::new();
    for _ in 0..2 {
        replay_falsification(
            "native_injected",
            "predicate",
            &input,
            &fixture.package.parameters(),
            |s| {
                let ReplaySource::Witness(witness) = &s else {
                    panic!("native adapter must produce witness source")
                };
                transcripts.push(witness.transcript().to_owned());
                let request = fixture.request(s);
                identities.push(request.obligation_identity);
                request
            },
            ReplayLimits::default(),
        )
        .unwrap();
        input.reverse();
    }
    assert_eq!(transcripts[0], transcripts[1]);
    assert_eq!(identities[0], identities[1]);
    let witness = Witness::parse(transcripts.remove(0)).unwrap();
    let parameters = fixture.package.parameters();
    let bindings = parameters
        .iter()
        .map(|p| WitnessBinding {
            parameter: WireNodeId::from_hex(p.node_id).unwrap(),
            value_type: WitnessValueType::Canonical,
        })
        .collect::<Vec<_>>();
    let decoded = witness.decode(&bindings).unwrap();
    for (parameter, decoded) in parameters.iter().zip(decoded) {
        assert_eq!(
            &decoded,
            &input
                .iter()
                .find(|(name, _)| name == parameter.argument)
                .unwrap()
                .1
        );
    }
}

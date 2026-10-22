//! Synthetic decoder inputs ONLY. These strings are not retained Kani/solver evidence.
use std::collections::BTreeMap;

use ix_trace_rs::trace;
use qsl_replay::{ReplayLimits, ScalarLimits, WitnessValue};
use quire_contract_codegen::{
    FunctionPredicateItem, LockedSource, RationalComponent, RationalPredicateCodec,
    RationalPredicateDecodeError, ReplayInputs, ReplayPackage, WitnessDecodeLimits,
};
use quire_exact::{Integer, Rational};

use crate::rational_predicate_support::{admit, source_parameters, HEALTHY};

fn codec(source: &str) -> RationalPredicateCodec {
    let (contract, function) = admit(source);
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
        "rational_commutative",
    )
    .unwrap();
    package
        .rational_predicate_codec(FunctionPredicateItem {
            package: &contract,
            function: &function,
        })
        .unwrap()
}

fn synthetic(values: &[i64]) -> String {
    let entries = values
        .iter()
        .map(|n| format!("// {n}\nvec!{:?},\n", n.to_le_bytes()))
        .collect::<String>();
    format!("/// Test generated for harness `native::synthetic`\n/// Check for `assertion`: \"native predicate at {{}}\", 17\nfn synthetic() {{\nlet concrete_vals: Vec<Vec<u8>> = vec![\n{entries}];\nkani::concrete_playback_run(concrete_vals, harness);\n}}")
}
fn limits(text: &str) -> WitnessDecodeLimits {
    WitnessDecodeLimits {
        bytes: text.len(),
        values: 4,
    }
}
fn rational(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn synthetic_four_physical_values_decode_to_two_real_logical_arguments() {
    let codec = codec(HEALTHY);
    let raw = synthetic(&[1, 2, -2, 3]);
    let decoded = codec
        .decode_inputs(codec.schema(), "synthetic", "native", &raw, limits(&raw))
        .expect("a genuine admitted native schema must decode its four I64 positions");
    assert_eq!(
        decoded.values,
        vec![
            (
                "a".into(),
                WitnessValue::Rational {
                    numerator: Integer::from(1_i64),
                    denominator: Integer::from(2_i64)
                }
            ),
            (
                "b".into(),
                WitnessValue::Rational {
                    numerator: Integer::from(-2_i64),
                    denominator: Integer::from(3_i64)
                }
            )
        ]
    );
    assert_eq!(decoded.check_text, "native predicate at {}");
    assert_eq!(
        decoded.check_clause,
        raw.split_once("/// Check for `assertion`").unwrap().1
    );
    let original = source_parameters(HEALTHY, "rational_commutative");
    let expected = original
        .iter()
        .map(|(name, _)| {
            if name == "a" {
                rational(1, 2)
            } else {
                rational(-2, 3)
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(decoded.oracle_arguments, expected);
    assert_eq!(codec.schema().physical.len(), 4);
    assert_eq!(codec.schema().logical.len(), 2);
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn retained_schema_role_position_domain_and_mapping_mutations_are_not_authority() {
    let codec = codec(HEALTHY);
    let raw = synthetic(&[1, 2, 2, 3]);
    let mut changes = Vec::new();
    let mut s = codec.schema().clone();
    s.physical.pop();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical.push(s.physical[0].clone());
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical[0].component = RationalComponent::Denominator;
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical[0].position = 1;
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical.swap(0, 1);
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical[0].maximum += 1;
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical[0].parameter = s.function.clone();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.logical.pop();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.logical.push(s.logical[0].clone());
    changes.push(s);
    let mut s = codec.schema().clone();
    s.logical[0].oracle_position = 1 - s.logical[0].oracle_position;
    changes.push(s);
    let mut s = codec.schema().clone();
    s.logical[0].numerator_maximum = "8".into();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.declaration.ordinal += 1;
    changes.push(s);
    let mut s = codec.schema().clone();
    s.function = s.logical[0].parameter.clone();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.source.lock.sources[0].identity = "foreign".into();
    changes.push(s);
    for changed in changes {
        assert!(matches!(
            codec.decode_inputs(&changed, "synthetic", "native", &raw, limits(&raw)),
            Err(RationalPredicateDecodeError::SchemaMismatch)
        ));
    }
}

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

fn replay_package(source: &str) -> ReplayPackage {
    ReplayPackage::new(
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
    .unwrap()
}
fn prepare_codec(source: &str) -> RationalPredicateCodec {
    let (contract, function) = admit(source);
    replay_package(source)
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
    let codec = prepare_codec(HEALTHY);
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
    let codec = prepare_codec(HEALTHY);
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
    let mut s = codec.schema().clone();
    s.physical[0].identifier = s.physical[1].identifier.clone();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.physical[0].primitive = quire_contract_codegen::KaniPrimitiveType::Boolean;
    changes.push(s);
    let mut s = codec.schema().clone();
    s.logical[0].identifier = "foreign".into();
    changes.push(s);
    let mut s = codec.schema().clone();
    s.logical[0].parameter = s.logical[1].parameter.clone();
    changes.push(s);
    for endpoint in 0..4 {
        let mut s = codec.schema().clone();
        match endpoint {
            0 => s.logical[0].numerator_minimum = "-8".into(),
            1 => s.logical[0].numerator_maximum = "8".into(),
            2 => s.logical[0].denominator_minimum = "2".into(),
            _ => s.logical[0].denominator_maximum = "8".into(),
        };
        changes.push(s);
    }
    for changed in changes {
        assert!(matches!(
            codec.decode_inputs(&changed, "synthetic", "native", &raw, limits(&raw)),
            Err(RationalPredicateDecodeError::SchemaMismatch)
        ));
    }
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn real_asymmetric_reversed_declarations_map_name_order_back_to_oracle_node_order() {
    let asymmetric = HEALTHY
        .replace("a: Rational[-9, 9; 1, 9]", "a: Rational[-3, 5; 1, 7]")
        .replace("b: Rational[-9, 9; 1, 9]", "b: Rational[-8, 9; 2, 9]");
    let reversed = asymmetric.replace(
        "a: Rational[-3, 5; 1, 7], b: Rational[-8, 9; 2, 9]",
        "b: Rational[-8, 9; 2, 9], a: Rational[-3, 5; 1, 7]",
    );
    assert_ne!(asymmetric, reversed);
    let mut nonidentity_mapping = false;
    for source in [&asymmetric, &reversed] {
        let codec = prepare_codec(source);
        let schema = codec.schema();
        assert_eq!(
            schema
                .logical
                .iter()
                .map(|p| p.identifier.as_str())
                .collect::<Vec<_>>(),
            ["a", "b"]
        );
        assert_eq!(
            (schema.physical[0].minimum, schema.physical[0].maximum),
            (-3, 5)
        );
        assert_eq!(
            (schema.physical[1].minimum, schema.physical[1].maximum),
            (1, 7)
        );
        assert_eq!(
            (schema.physical[2].minimum, schema.physical[2].maximum),
            (-8, 9)
        );
        assert_eq!(
            (schema.physical[3].minimum, schema.physical[3].maximum),
            (2, 9)
        );
        let original = source_parameters(source, "rational_commutative");
        let raw = synthetic(&[1, 2, -2, 3]);
        let decoded = codec
            .decode_inputs(schema, "synthetic", "native", &raw, limits(&raw))
            .unwrap();
        for (position, (name, node)) in original.iter().enumerate() {
            let logical = schema
                .logical
                .iter()
                .find(|p| &p.identifier == name)
                .unwrap();
            assert_eq!(logical.parameter.digest.as_ref(), node);
            assert_eq!(logical.oracle_position, position);
            nonidentity_mapping |= position
                != schema
                    .logical
                    .iter()
                    .position(|p| &p.identifier == name)
                    .unwrap();
            assert_eq!(
                decoded.oracle_arguments[position],
                if name == "a" {
                    rational(1, 2)
                } else {
                    rational(-2, 3)
                }
            );
        }
        let mut corrupted = schema.clone();
        corrupted.logical[0].oracle_position = 1 - corrupted.logical[0].oracle_position;
        assert!(matches!(
            codec.decode_inputs(&corrupted, "synthetic", "native", &raw, limits(&raw)),
            Err(RationalPredicateDecodeError::SchemaMismatch)
        ));
    }
    assert!(
        nonidentity_mapping,
        "the genuine fixture must exercise a different node order, not assume incidental identity"
    );
}

fn decode_code(codec: &RationalPredicateCodec, raw: &str, limits: WitnessDecodeLimits) -> String {
    match codec.decode_inputs(codec.schema(), "synthetic", "native", raw, limits) {
        Err(RationalPredicateDecodeError::Decode(failure)) => failure.code,
        other => panic!("expected the existing primitive decoder's typed refusal: {other:?}"),
    }
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn physical_byte_value_arity_width_comment_and_check_selection_guards_are_reused() {
    let codec = prepare_codec(HEALTHY);
    let raw = synthetic(&[1, 2, 2, 3]);
    assert_eq!(
        decode_code(
            &codec,
            &raw,
            WitnessDecodeLimits {
                bytes: raw.len() - 1,
                values: 4
            }
        ),
        "cg_witness_decode_bytes_limit"
    );
    for count in 0..4 {
        assert_eq!(
            decode_code(
                &codec,
                &raw,
                WitnessDecodeLimits {
                    bytes: raw.len(),
                    values: count
                }
            ),
            "cg_witness_decode_values_limit"
        );
    }
    assert!(codec
        .decode_inputs(
            codec.schema(),
            "synthetic",
            "native",
            &raw,
            WitnessDecodeLimits {
                bytes: raw.len(),
                values: 4
            }
        )
        .is_ok());
    let too_many = synthetic(&[1, 2, 2, 3, 4]);
    assert_eq!(
        decode_code(
            &codec,
            &too_many,
            WitnessDecodeLimits {
                bytes: too_many.len(),
                values: 4
            }
        ),
        "cg_witness_decode_values_limit"
    );
    assert_eq!(
        decode_code(
            &codec,
            &too_many,
            WitnessDecodeLimits {
                bytes: too_many.len(),
                values: 5
            }
        ),
        "kani_witness_arity_mismatch"
    );
    for count in 0..4 {
        let fewer = synthetic(&[1, 2, 2, 3][..count]);
        assert_eq!(
            decode_code(&codec, &fewer, limits(&fewer)),
            "kani_witness_arity_mismatch"
        );
    }
    let first = format!("vec!{:?}", 1_i64.to_le_bytes());
    for (changed, expected) in [
        (
            raw.replacen(&first, "vec![1,0,0,0,0,0,0]", 1),
            "kani_witness_width_mismatch",
        ),
        (
            raw.replacen("// 1\n", "// 999\n", 1),
            "kani_witness_comment_mismatch",
        ),
        (
            raw.replacen(&first, "vec![999]", 1),
            "kani_witness_byte_invalid",
        ),
        (
            raw.replacen("// 1\n", "", 1),
            "kani_witness_comment_missing",
        ),
        (
            raw.replace("`assertion`", "`cover`"),
            "kani_witness_cover_refused",
        ),
        (
            raw.replace("`assertion`", "`unwind`"),
            "kani_witness_check_kind_refused",
        ),
        (
            format!("{raw}\n{raw}"),
            "kani_witness_multiple_assertions_refused",
        ),
        (
            raw.replace("native::synthetic", "native::foreign"),
            "cg_witness_harness_identity_mismatch",
        ),
    ] {
        assert_eq!(decode_code(&codec, &changed, limits(&changed)), expected);
    }
    let cover = raw
        .replace("`assertion`", "`cover`")
        .replace("native predicate at {}", "synthetic cover only");
    let combined = format!("{cover}\n{raw}");
    let decoded = codec
        .decode_inputs(
            codec.schema(),
            "synthetic",
            "native",
            &combined,
            limits(&combined),
        )
        .unwrap();
    let expected = combined.rsplit_once("/// Check for `assertion`").unwrap().1;
    assert_eq!(decoded.check_clause, expected);
    assert!(
        std::ptr::eq(decoded.check_clause.as_ptr(), expected.as_ptr()),
        "the clause is the actual selected raw substring"
    );
    assert_eq!(decoded.check_text, "native predicate at {}");
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn owner_canonical_roundtrip_and_original_component_domains_are_not_repaired() {
    let codec = prepare_codec(HEALTHY);
    for argument in 0..2 {
        let parameter = codec.schema().logical[argument].parameter.clone();
        for pair in [[0, 2], [2, 2], [1, 0], [1, -2], [-2, -3]] {
            let mut values = [1, 2, 2, 3];
            values[argument * 2..argument * 2 + 2].copy_from_slice(&pair);
            let raw = synthetic(&values);
            assert!(
                matches!(codec.decode_inputs(codec.schema(),"synthetic","native",&raw,limits(&raw)),Err(RationalPredicateDecodeError::NonCanonical{parameter:actual}) if actual==parameter)
            );
        }
        for (pair, component) in [
            ([-10, 1], RationalComponent::Numerator),
            ([10, 1], RationalComponent::Numerator),
            ([1, 10], RationalComponent::Denominator),
        ] {
            let mut values = [1, 2, 2, 3];
            values[argument * 2..argument * 2 + 2].copy_from_slice(&pair);
            let raw = synthetic(&values);
            assert!(
                matches!(codec.decode_inputs(codec.schema(),"synthetic","native",&raw,limits(&raw)),Err(RationalPredicateDecodeError::OutsideDomain{parameter:actual,component:role}) if actual==parameter && role==component)
            );
        }
    }
    // Swapping raw components can represent a DIFFERENT valid input: no content provenance
    // claim exists here, and valid in-domain canonical values must not be silently discarded.
    for values in [[0, 1, 0, 1], [-9, 1, 9, 1], [1, 9, -1, 9], [1, 2, 2, 1]] {
        let raw = synthetic(&values);
        let decoded = codec
            .decode_inputs(codec.schema(), "synthetic", "native", &raw, limits(&raw))
            .unwrap();
        assert_eq!(decoded.values.len(), 2);
        for (index, (_, value)) in decoded.values.iter().enumerate() {
            assert_eq!(
                value,
                &WitnessValue::Rational {
                    numerator: Integer::from(values[index * 2]),
                    denominator: Integer::from(values[index * 2 + 1])
                }
            );
        }
    }
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn foreign_source_function_and_loaded_schema_never_bypass_native_metadata_authority() {
    use crate::rational_predicate_support::MUTANT;
    let codec = prepare_codec(HEALTHY);
    let raw = synthetic(&[1, 2, 2, 3]);
    let loaded: quire_contract_codegen::RationalPredicateSchema =
        serde_json::from_str(&serde_json::to_string(codec.schema()).unwrap()).unwrap();
    assert!(codec
        .decode_inputs(&loaded, "synthetic", "native", &raw, limits(&raw))
        .is_ok());
    let foreign = prepare_codec(MUTANT);
    assert_ne!(
        foreign.schema().source.package_id,
        codec.schema().source.package_id
    );
    assert!(matches!(
        codec.decode_inputs(foreign.schema(), "synthetic", "native", &raw, limits(&raw)),
        Err(RationalPredicateDecodeError::SchemaMismatch)
    ));
    let mut nested = serde_json::to_value(codec.schema()).unwrap();
    nested["physical"][0]["unrecognized_role"] = serde_json::json!(true);
    assert!(
        serde_json::from_value::<quire_contract_codegen::RationalPredicateSchema>(nested).is_err()
    );
    let mut unknown = serde_json::to_value(codec.schema()).unwrap();
    unknown["extra"] = serde_json::json!(true);
    assert!(
        serde_json::from_value::<quire_contract_codegen::RationalPredicateSchema>(unknown).is_err()
    );
}

#[trace("TC-060", "FR-046-AC-9", "FR-025-AC-10")]
#[test]
fn canonical_i64_extreme_components_widen_losslessly_in_the_actual_admitted_domain() {
    let source = HEALTHY.replace(
        "Rational[-9, 9; 1, 9]",
        "Rational[-9223372036854775808, 9223372036854775807; 1, 9]",
    );
    let codec = prepare_codec(&source);
    let raw = synthetic(&[i64::MIN, 1, i64::MAX, 1]);
    let decoded = codec
        .decode_inputs(codec.schema(), "synthetic", "native", &raw, limits(&raw))
        .unwrap();
    assert_eq!(
        decoded.values[0].1,
        WitnessValue::Rational {
            numerator: Integer::from(i64::MIN),
            denominator: Integer::from(1_i64)
        }
    );
    assert_eq!(
        decoded.values[1].1,
        WitnessValue::Rational {
            numerator: Integer::from(i64::MAX),
            denominator: Integer::from(1_i64)
        }
    );
}

#[trace("TC-060", "FR-046-AC-9")]
#[test]
fn codec_constructor_uses_the_same_validated_source_function_context_as_requests() {
    use crate::rational_predicate_support::MUTANT;
    use quire_contract_codegen::ObligationIdentityError;
    let package = replay_package(HEALTHY);
    let (foreign, foreign_function) = admit(MUTANT);
    assert!(matches!(
        package.rational_predicate_codec(FunctionPredicateItem {
            package: &foreign,
            function: &foreign_function
        }),
        Err(ObligationIdentityError::SourcePackage)
    ));
    let (contract, function) = admit(HEALTHY);
    let parameter = contract
        .wire()
        .semantic_graph
        .nodes
        .iter()
        .find(|n| n.semantic_form.as_ref() == "parameter")
        .unwrap();
    assert!(matches!(
        package.rational_predicate_codec(FunctionPredicateItem {
            package: &contract,
            function: &parameter.node_id
        }),
        Err(ObligationIdentityError::FunctionSelection)
    ));
    let codec = package
        .rational_predicate_codec(FunctionPredicateItem {
            package: &contract,
            function: &function,
        })
        .unwrap();
    assert_eq!(
        codec.schema().obligation,
        package
            .rational_predicate_obligation_identity(FunctionPredicateItem {
                package: &contract,
                function: &function
            })
            .unwrap()
            .to_string()
    );
}

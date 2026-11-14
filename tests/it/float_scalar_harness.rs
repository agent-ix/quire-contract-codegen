//! FR-053 IEEE float scalar comparison harnesses over a genuine QSL source admitted through
//! Contract IR, routed through `generate_routed`.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    decode_float_falsification, generate_float_scalar_harness, parse_assertion_playback,
    FloatCapability, FloatScalarHarness, FloatScalarOutcome, FloatScalarRefusal,
    FloatScalarRequest, KaniRunOutcome, ObligationDisposition, UnsupportedObligation,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm,
};

use super::routed_generation::{
    admitted_routing_input, generate_routed, kani_context, kani_parts, route,
};

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
profile v = \"quire.value.complete/v1\";\n\
function numeq64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { quire::value::ieee::numericEqual(a, b) }\n\
function total64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { quire::value::ieee::totalOrder(a, b) }\n\
function bitid64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { quire::value::ieee::bitIdentical(a, b) }\n\
function numeq32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Boolean pure { quire::value::ieee::numericEqual(a, b) }\n\
function total32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Boolean pure { quire::value::ieee::totalOrder(a, b) }\n\
function bitid32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Boolean pure { quire::value::ieee::bitIdentical(a, b) }\n\
function eq64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { a = b }\n\
function add64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Float64[nearest-even] pure { a + b }\n\
function sub32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Float32[nearest-even] pure { a - b }\n\
function mul64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Float64[nearest-even] pure { a * b }\n\
function div32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Float32[nearest-even] pure { a / b }\n\
function div64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Float64[nearest-even] pure { a / b }\n\
function same64 using v(a: Float64[nearest-even]): Boolean pure { quire::value::ieee::numericEqual(a, a) }\n\
function guarded64 using v(flag: Boolean, a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { if flag then quire::value::ieee::numericEqual(a, b) else false }\n\
function not_float using v(a: Int[0, 9], b: Int[0, 9]): Boolean pure { a < b }\n";

struct Corpus {
    package: CheckedPackageV2,
}

fn corpus() -> Corpus {
    let compiled = qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: qsl_replay::SourceIdentity::new("test", "float-row.native"),
            package_name: "float-row".to_owned(),
            path: "float-row.native",
            bytes: SOURCE.as_bytes(),
        },
        [],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("QSL admits the float source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        compiled.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("Contract IR admits the QSL float source");
    };
    Corpus { package: *package }
}

impl Corpus {
    /// The `identity` application node inside `function`'s source region.
    fn node(&self, function: &str, identity: &str) -> CheckedNodeId {
        let package = &self.package;
        let declaration = package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration
                    .as_ref()
                    .is_some_and(|decl| decl.qualified_name == [Box::<str>::from(function)])
            })
            .expect("real function declaration");
        let regions = package
            .source_map()
            .iter()
            .filter(|entry| entry.node_id == declaration.node_id)
            .flat_map(|entry| &entry.regions)
            .collect::<Vec<_>>();
        package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                matches!(&node.body, CheckedTerm::Application { operation, .. }
                    if operation.identity() == Some(identity))
                    && package
                        .source_map()
                        .iter()
                        .filter(|entry| entry.node_id == node.node_id)
                        .any(|entry| {
                            entry.regions.iter().any(|region| {
                                regions.iter().any(|outer| {
                                    outer.source == region.source
                                        && outer.start <= region.start
                                        && region.end <= outer.end
                                })
                            })
                        })
            })
            .unwrap_or_else(|| panic!("{function} holds a {identity} node"))
            .node_id
            .clone()
    }

    fn request<'a>(&'a self, node: &'a CheckedNodeId, guarded: bool) -> FloatScalarRequest<'a> {
        FloatScalarRequest {
            source: &self.package,
            node_id: node,
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            unwind: 2,
            requirement_occurrence: None,
            has_path_condition: guarded,
            ordinal: 0,
        }
    }

    fn outcome(&self, function: &str, identity: &str) -> FloatScalarOutcome {
        let node = self.node(function, identity);
        generate_float_scalar_harness(&self.request(&node, false))
    }

    fn harness(&self, function: &str, identity: &str) -> FloatScalarHarness {
        match self.outcome(function, identity) {
            FloatScalarOutcome::Supported(harness) => *harness,
            FloatScalarOutcome::Refused(refusal) => panic!("{function} refused: {refusal:?}"),
        }
    }

    /// Routes each `(function, identity)` through `generate_routed` as one request.
    fn routed(&self, items: &[(&str, &str)]) -> quire_contract_codegen::RoutedGeneration {
        let routed = items
            .iter()
            .enumerate()
            .map(|(index, (function, identity))| route(index, &self.node(function, identity)))
            .collect::<Vec<_>>();
        let input = admitted_routing_input(&self.package, &routed);
        generate_routed(
            input.source,
            &input.package,
            &input.positions,
            &routed,
            &kani_context("crate::subject", 2),
        )
        .expect("the routed call generates")
    }

    /// The routed harness of one comparison and the manifest its crate builds under.
    fn routed_harness(
        &self,
        function: &str,
        identity: &str,
    ) -> (
        quire_contract_codegen::KaniScalarObligationHarness,
        quire_contract_codegen::Artifact,
    ) {
        let generation = self.routed(&[(function, identity)]);
        let manifest = generation
            .oracle_artifacts
            .as_ref()
            .and_then(|artifacts| artifacts.iter().find(|a| a.path == "Cargo.toml"))
            .expect("the oracle crate manifest")
            .clone();
        let (record, harness) = kani_parts(&generation.items[0].output);
        (
            harness
                .unwrap_or_else(|| panic!("a harness, got {:?}", record.disposition))
                .clone(),
            manifest,
        )
    }
}

/// Function, identity, width and the native comparison variant of each supported row.
const ROWS: [(&str, &str, &str, &str); 6] = [
    (
        "numeq64",
        "quire.op.ieee.numeric_equal",
        "float64",
        "NumericEqual",
    ),
    (
        "total64",
        "quire.op.ieee.total_order",
        "float64",
        "TotalOrder",
    ),
    (
        "bitid64",
        "quire.op.ieee.bit_identical",
        "float64",
        "BitIdentical",
    ),
    (
        "numeq32",
        "quire.op.ieee.numeric_equal",
        "float32",
        "NumericEqual",
    ),
    (
        "total32",
        "quire.op.ieee.total_order",
        "float32",
        "TotalOrder",
    ),
    (
        "bitid32",
        "quire.op.ieee.bit_identical",
        "float32",
        "BitIdentical",
    ),
];

fn proof_body(harness: &FloatScalarHarness) -> &str {
    let source = &harness.harness.rust.contents;
    &source[source.find("#[kani::proof]").expect("a proof attribute")..]
}

/// Every comparison at each width, from a genuine QSL source, generates one harness over native
/// carriers that names its operation, width and parameters.
#[trace("TC-067", "FR-053-AC-1")]
#[test]
fn every_comparison_generates_one_native_carrier_harness_from_its_node() {
    let corpus = corpus();
    for (function, identity, width, comparison) in ROWS {
        let generated = corpus.harness(function, identity);
        let id = &generated.harness.identity;
        assert_eq!(id.operation_identity, identity);
        assert_eq!(id.oracle_symbol, "quire_exact::compare_native_float");
        let float = id.float_scalar.as_ref().expect("a float identity");
        assert_eq!(float.width, width);
        assert_eq!(float.operator, identity);
        assert_eq!(
            id.arguments
                .iter()
                .map(|a| a.identifier.as_str())
                .collect::<Vec<_>>(),
            ["a", "b"]
        );
        let body = proof_body(&generated);
        assert!(
            body.contains(&format!("rt::IeeeComparison::{comparison}")),
            "{body}"
        );
        assert!(body.contains("compare_native_float"), "{body}");
        assert!(
            !body.contains("evaluate_ieee") && !body.contains("compare_ieee("),
            "the harness reaches the owner's BigInt path"
        );
    }
}

/// The QSL `=` operator over floats lowers to the same `numeric_equal` row.
#[trace("TC-067", "FR-053-AC-1")]
#[test]
fn the_qsl_equality_operator_generates_the_numeric_equal_row() {
    let corpus = corpus();
    let generated = corpus.harness("eq64", "quire.op.ieee.numeric_equal");
    assert_eq!(
        generated.harness.identity.operation_identity,
        "quire.op.ieee.numeric_equal"
    );
}

/// Each operand is one `i64` bit pattern: every `i64` for `float64`, `0..=0xFFFF_FFFF` for
/// `float32`, read as `u64` or `u32` bits with no other constraint.
#[trace("TC-067", "FR-053-AC-2")]
#[test]
fn the_symbolic_domain_is_every_bit_pattern_of_the_width() {
    let corpus = corpus();
    for (function, identity, width, _) in ROWS {
        let generated = corpus.harness(function, identity);
        let body = proof_body(&generated);
        let (minimum, maximum, bits) = if width == "float64" {
            (i64::MIN, i64::MAX, "u64")
        } else {
            (0, i64::from(u32::MAX), "u32")
        };
        for argument in &generated.harness.identity.arguments {
            assert_eq!((argument.minimum, argument.maximum), (minimum, maximum));
        }
        assert_eq!(body.matches("kani::any()").count(), 2, "{body}");
        for name in ["left", "right"] {
            assert!(
                body.contains(&format!("NativeFloat::{}({name} as {bits})", {
                    if width == "float64" {
                        "binary64"
                    } else {
                        "binary32"
                    }
                })),
                "{body}"
            );
        }
        // Nothing restricts a draw beyond the width's own range, and nothing reads the outcome.
        assert_eq!(body.matches("kani::assume").count(), 2, "{body}");
    }
}

/// The assertion requires `Completed`, the stopped outcomes are asserted by name and no
/// assumption reads the outcome.
#[trace("TC-067", "FR-053-AC-5")]
#[test]
fn the_assertion_requires_completed_and_no_assumption_reads_the_outcome() {
    let corpus = corpus();
    let generated = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    let body = proof_body(&generated);
    for needle in [
        "Ok(rt::Outcome::Incomplete(_))",
        "DistinctIeeeWidths",
        "Ok(rt::Outcome::Completed(value)) => *value == native",
    ] {
        assert!(body.contains(needle), "{needle}: {body}");
    }
    for assumption in body.lines().filter(|line| line.contains("kani::assume")) {
        assert!(!assumption.contains("outcome"), "{assumption}");
    }
}

/// One completion cover, last.
#[trace("TC-067", "FR-053-AC-7")]
#[test]
fn each_harness_ends_with_exactly_one_completion_cover() {
    let corpus = corpus();
    for (function, identity, ..) in ROWS {
        let generated = corpus.harness(function, identity);
        let body = proof_body(&generated);
        assert_eq!(body.matches("kani::cover!").count(), 1, "{body}");
        let cover = body.rfind("kani::cover!").unwrap();
        assert!(
            !body[cover..].contains("kani::assert") && !body[cover..].contains("assert!"),
            "no assertion follows the cover: {body}"
        );
    }
}

/// Every arithmetic identity is refused with its named capability and no harness; a refusal
/// names the measured disposition and the node's rounding spelling.
#[trace("TC-067", "FR-053-AC-6")]
#[test]
fn arithmetic_is_refused_with_a_named_not_tractable_capability() {
    let corpus = corpus();
    for (function, identity, capability) in [
        (
            "add64",
            "quire.op.ieee.float64.add",
            FloatCapability::AddNotTractable,
        ),
        (
            "sub32",
            "quire.op.ieee.float32.sub",
            FloatCapability::SubNotTractable,
        ),
        (
            "mul64",
            "quire.op.ieee.float64.mul",
            FloatCapability::MulNotTractable,
        ),
        (
            "div32",
            "quire.op.ieee.float32.div",
            FloatCapability::DivNotTractable,
        ),
        (
            "div64",
            "quire.op.ieee.float64.div",
            FloatCapability::DivNotTractable,
        ),
    ] {
        let FloatScalarOutcome::Refused(FloatScalarRefusal::Unsupported {
            capability: found,
            operation,
            detail,
        }) = corpus.outcome(function, identity)
        else {
            panic!("{function} must be refused");
        };
        assert_eq!(found, capability);
        assert_eq!(operation, identity);
        assert!(detail.contains("nearest-even"), "{detail}");
        assert!(detail.contains("FR-405 Behavior 15"), "{detail}");
    }
    let FloatScalarOutcome::Refused(FloatScalarRefusal::Unsupported { detail, .. }) =
        corpus.outcome("div32", "quire.op.ieee.float32.div")
    else {
        panic!("refused");
    };
    assert!(
        detail.contains("0x800000A9 / 0x9C873333") && detail.contains("0x1BA00000"),
        "the division finding is retained: {detail}"
    );
    assert_eq!(
        FloatCapability::DivNotTractable.name(),
        "float.div.not_kani_tractable"
    );
}

/// A same-parameter operand and a path condition are refused with named capabilities, and a node
/// of another family is not a float operation.
#[trace("TC-067", "FR-053-AC-1")]
#[test]
fn unsupported_operand_shapes_are_refused_with_named_capabilities() {
    let corpus = corpus();
    let FloatScalarOutcome::Refused(FloatScalarRefusal::Unsupported { capability, .. }) =
        corpus.outcome("same64", "quire.op.ieee.numeric_equal")
    else {
        panic!("a same-parameter operand is refused");
    };
    assert_eq!(capability, FloatCapability::SameParameterOperand);
    let node = corpus.node("guarded64", "quire.op.ieee.numeric_equal");
    let FloatScalarOutcome::Refused(FloatScalarRefusal::Unsupported { capability, .. }) =
        generate_float_scalar_harness(&corpus.request(&node, true))
    else {
        panic!("a path condition is refused");
    };
    assert_eq!(capability, FloatCapability::PathCondition);
    assert_eq!(capability.name(), "float.path_condition");
    let node = corpus.node("not_float", "quire.op.integer.lt");
    assert!(matches!(
        generate_float_scalar_harness(&corpus.request(&node, false)),
        FloatScalarOutcome::Refused(FloatScalarRefusal::NotFloatOperation)
    ));
}

/// One routed request settles three supported comparisons, a refused arithmetic sibling and a
/// node of another family; each supported item is generated exactly as it is alone.
#[trace("TC-067", "FR-053-AC-6")]
#[test]
fn a_routed_request_settles_comparisons_and_refuses_arithmetic_siblings() {
    let corpus = corpus();
    let mixed = corpus.routed(&[
        ("numeq64", "quire.op.ieee.numeric_equal"),
        ("add64", "quire.op.ieee.float64.add"),
        ("total32", "quire.op.ieee.total_order"),
        ("not_float", "quire.op.integer.lt"),
    ]);
    let alone = corpus.routed(&[("numeq64", "quire.op.ieee.numeric_equal")]);
    assert_eq!(mixed.items[0].output, alone.items[0].output);
    let (record, harness) = kani_parts(&mixed.items[0].output);
    assert!(matches!(
        &record.disposition,
        ObligationDisposition::Supported { harness_symbol }
            if *harness_symbol == harness.expect("a harness").identity.harness_symbol
    ));
    let (record, harness) = kani_parts(&mixed.items[1].output);
    assert!(harness.is_none());
    let ObligationDisposition::Unsupported {
        reason:
            UnsupportedObligation::FloatOperationNotRendered {
                cause, operation, ..
            },
    } = &record.disposition
    else {
        panic!("{:?}", record.disposition);
    };
    assert_eq!(cause, "float.add.not_kani_tractable");
    assert_eq!(operation, "quire.op.ieee.float64.add");
    let (record, harness) = kani_parts(&mixed.items[2].output);
    let harness = harness.expect("the float32 total order generates");
    assert_ne!(
        harness.identity.harness_symbol,
        kani_parts(&mixed.items[0].output)
            .1
            .unwrap()
            .identity
            .harness_symbol
    );
    assert!(matches!(
        record.disposition,
        ObligationDisposition::Supported { .. }
    ));
    let (record, harness) = kani_parts(&mixed.items[3].output);
    assert!(harness.is_none());
    assert!(
        !matches!(
            &record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::FloatOperationNotRendered { .. }
            }
        ),
        "a node of another family is not settled by the float row"
    );
}

// ---- witness decode (FR-053-AC-11) ----

fn transcript(harness: &FloatScalarHarness, draws: &[i64]) -> String {
    super::scalar_playback::transcript(&harness.harness.identity, draws)
}

fn decode(
    harness: &FloatScalarHarness,
    transcript: &str,
) -> Result<quire_contract_codegen::FloatWitness, quire_contract_codegen::DecodeFailure> {
    let identity = &harness.harness.identity;
    let playback = parse_assertion_playback(
        transcript,
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .map_err(quire_contract_codegen::DecodeFailure::from)?;
    decode_float_falsification(identity, &playback)
}

/// A decoded `float64` keeps its exact bit pattern: a pattern with the sign bit set is a negative
/// `i64` and decodes to the same `u64`, so a decode through a host float or a checked signed
/// conversion would fail these controls.
#[trace("TC-067", "FR-053-AC-11")]
#[test]
fn float64_witnesses_keep_their_exact_bit_patterns() {
    let corpus = corpus();
    let generated = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    for bits in [
        0x8000_0000_0000_0000_u64,
        0x7FF0_0000_0000_0001,
        0xFFF8_0000_0000_0000,
        0x7FF8_0000_0000_0001,
        0,
        u64::MAX,
    ] {
        let signed = i64::from_ne_bytes(bits.to_ne_bytes());
        let witness = decode(&generated, &transcript(&generated, &[signed, 0]))
            .expect("every i64 is a binary64 pattern");
        assert_eq!(
            witness.operands,
            [("a".to_owned(), bits), ("b".to_owned(), 0)],
            "{bits:#x}"
        );
        assert_eq!(witness.width, "float64");
    }
}

/// A `float32` witness decodes in range and is refused above `u32::MAX` or negative.
#[trace("TC-067", "FR-053-AC-11")]
#[test]
fn float32_witnesses_are_range_checked_and_keep_their_bits() {
    let corpus = corpus();
    let generated = corpus.harness("numeq32", "quire.op.ieee.numeric_equal");
    for bits in [0x8000_0000_u32, 0x7F80_0001, 0xFFC0_0000, 0] {
        let witness = decode(&generated, &transcript(&generated, &[i64::from(bits), 1]))
            .expect("an in-range pattern decodes");
        assert_eq!(witness.operands[0], ("a".to_owned(), u64::from(bits)));
    }
    for bad in [1_i64 << 32, -1, i64::MIN, i64::MAX] {
        let error = decode(&generated, &transcript(&generated, &[bad, 0]))
            .expect_err("an out-of-range float32 value is refused");
        assert_eq!(error.code, "cg_scalar_witness_operand_out_of_domain");
    }
}

/// Missing, extra and foreign entries and a schema that disagrees with the width are refused with
/// typed causes.
#[trace("TC-067", "FR-053-AC-11")]
#[test]
fn malformed_float_bindings_and_schemas_are_refused_with_typed_causes() {
    let corpus = corpus();
    let generated = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    for draws in [&[1_i64][..], &[1, 2, 3][..]] {
        let error = decode(&generated, &transcript(&generated, draws)).expect_err("arity");
        assert_eq!(error.code, "kani_witness_arity_mismatch");
    }
    let mut narrowed = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    narrowed.harness.identity.arguments[0].maximum = i64::from(u32::MAX);
    let error = decode(&narrowed, &transcript(&narrowed, &[1, 2])).expect_err("schema");
    assert_eq!(error.code, "cg_scalar_witness_schema_float_mismatch");
    let mut duplicate = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    duplicate.harness.identity.arguments[1].identifier = "a".to_owned();
    let error = decode(&duplicate, &transcript(&duplicate, &[1, 2])).expect_err("duplicate");
    assert_eq!(error.code, "cg_scalar_witness_schema_duplicate_binding");
    let mut foreign = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    foreign
        .harness
        .identity
        .float_scalar
        .as_mut()
        .unwrap()
        .operator = "quire.op.ieee.total_order".to_owned();
    let error = decode(&foreign, &transcript(&foreign, &[1, 2])).expect_err("foreign");
    assert_eq!(error.code, "cg_scalar_witness_schema_float_mismatch");
    let integer = {
        let mut other = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
        other.harness.identity.float_scalar = None;
        other
    };
    let identity = &integer.harness.identity;
    let text = transcript(&integer, &[1, 2]);
    let playback = parse_assertion_playback(
        &text,
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    let error = decode_float_falsification(identity, &playback).expect_err("not a float identity");
    assert_eq!(error.code, "cg_scalar_witness_float_identity_missing");
}

// ---- owner replay (FR-053-AC-11) ----

fn owner_package(function: &str) -> quire_contract_codegen::ReplayPackage {
    let unlimited = qsl_replay::ScalarLimits {
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
    };
    quire_contract_codegen::ReplayPackage::new(
        quire_contract_codegen::ReplayInputs {
            decode_limits: quire_contract_codegen::WitnessDecodeLimits {
                bytes: 1 << 20,
                values: 16,
            },
            package_name: "float-row".to_owned(),
            source: quire_contract_codegen::LockedSource {
                authority: "test".to_owned(),
                identity: "float-row.native".to_owned(),
                bytes: SOURCE.as_bytes().to_vec(),
            },
            dependencies: Vec::new(),
            accounting_limits: unlimited,
            stage_limits: std::collections::BTreeMap::from([(
                "s1.input_bytes".to_owned(),
                qsl_replay::DEFAULT_REPLAY_INPUT_BYTES,
            )]),
            supplied_limits: None,
            declared_domains: Vec::new(),
            replay_limits: qsl_replay::ReplayLimits::default(),
        },
        function,
    )
    .expect("the owner package recompiles the corpus source")
}

/// The owner's result for `draws` through the selected QSL function: the decoded bit patterns
/// are replayed as exact `WitnessValue::Float64` / `Float32` values.
fn owner_result(function: &str, identity: &str, draws: &[i64]) -> bool {
    let corpus = corpus();
    let generated = corpus.harness(function, identity);
    let witness = decode(&generated, &transcript(&generated, draws)).expect("decodes");
    quire_contract_codegen::replay_float_witness(&owner_package(function), &witness)
        .expect("the owner route evaluates the bit patterns")
}

/// The owner route evaluates exact bit patterns, NaN payloads and signed zeros included, and its
/// answer is the comparison's own.
#[trace("TC-067", "FR-053-AC-11")]
#[test]
fn a_decoded_witness_replays_through_the_owner_route_with_exact_bits() {
    let signed = |bits: u64| i64::from_ne_bytes(bits.to_ne_bytes());
    let (zero, minus_zero) = (0_u64, 0x8000_0000_0000_0000_u64);
    let (qnan_1, qnan_2) = (0x7FF8_0000_0000_0001_u64, 0x7FF8_0000_0000_0002_u64);
    let numeq = |a: u64, b: u64| {
        owner_result(
            "numeq64",
            "quire.op.ieee.numeric_equal",
            &[signed(a), signed(b)],
        )
    };
    let bitid = |a: u64, b: u64| {
        owner_result(
            "bitid64",
            "quire.op.ieee.bit_identical",
            &[signed(a), signed(b)],
        )
    };
    let total = |a: u64, b: u64| {
        owner_result(
            "total64",
            "quire.op.ieee.total_order",
            &[signed(a), signed(b)],
        )
    };
    assert!(
        numeq(zero, minus_zero),
        "numeric equality ignores the sign of zero"
    );
    assert!(!bitid(zero, minus_zero), "bit identity does not");
    assert!(
        !numeq(qnan_1, qnan_1),
        "a NaN is not numerically equal to itself"
    );
    assert!(bitid(qnan_1, qnan_1));
    assert!(!bitid(qnan_1, qnan_2), "NaN payloads are distinguished");
    assert!(total(minus_zero, zero), "-0 precedes +0 in totalOrder");
    assert!(!total(zero, minus_zero));
    let narrow = owner_result("numeq32", "quire.op.ieee.numeric_equal", &[0x8000_0000, 0]);
    assert!(narrow, "binary32 zeros of both signs are numerically equal");
}

// ---- installed Kani lane ----

/// Run the routed harness, with `site` replaced by `with` when a mutation is given. The source
/// text is confirmed to change.
fn run(
    name: &str,
    function: &str,
    identity: &str,
    mutation: Option<(&str, &str)>,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let corpus = corpus();
    let (mut harness, manifest) = corpus.routed_harness(function, identity);
    if let Some((site, with)) = mutation {
        assert!(harness.rust.contents.contains(site), "{site}");
        let mutated = harness.rust.contents.replace(site, with);
        assert_ne!(
            mutated, harness.rust.contents,
            "the mutation changed the source"
        );
        harness.rust.contents = mutated;
    }
    let started = std::time::Instant::now();
    let evidence = crate::kani_obligations::run_scalar_under_real_kani(name, &harness, &manifest);
    eprintln!(
        "FLOATVERDICT {name} outcome={:?} wall_ms={} unwind={} solver={:?} argv={:?}",
        evidence.outcome,
        started.elapsed().as_millis(),
        evidence.unwind,
        evidence.solver,
        evidence.arguments
    );
    evidence
}

fn healthy(function: &str, identity: &str) {
    let evidence = run(&format!("float-{function}"), function, identity, None);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{evidence:?}");
}

fn falsified(function: &str, identity: &str, name: &str, site: &str, with: &str) {
    let evidence = run(name, function, identity, Some((site, with)));
    assert!(
        matches!(evidence.outcome, KaniRunOutcome::Falsified { .. }),
        "{evidence:?}"
    );
}

macro_rules! kani_verifies {
    ($($test:ident => ($function:literal, $identity:literal)),* $(,)?) => {$(
        #[trace("TC-067", "FR-053-AC-4")]
        #[test]
        #[ignore = "kani lane: run serially through `make kani`"]
        fn $test() {
            healthy($function, $identity);
        }
    )*};
}

kani_verifies! {
    kani_numeric_equal_float64_verifies => ("numeq64", "quire.op.ieee.numeric_equal"),
    kani_total_order_float64_verifies => ("total64", "quire.op.ieee.total_order"),
    kani_bit_identical_float64_verifies => ("bitid64", "quire.op.ieee.bit_identical"),
    kani_numeric_equal_float32_verifies => ("numeq32", "quire.op.ieee.numeric_equal"),
    kani_total_order_float32_verifies => ("total32", "quire.op.ieee.total_order"),
    kani_bit_identical_float32_verifies => ("bitid32", "quire.op.ieee.bit_identical"),
}

/// The comparison mutants of FR-053-AC-8: the native comparison asked as another one, so its result
/// disagrees with the expected side on an in-domain input.
#[trace("TC-067", "FR-053-AC-8")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_numeric_equal_asked_as_bit_identical_is_falsified() {
    falsified(
        "numeq64",
        "quire.op.ieee.numeric_equal",
        "float-numeq-as-bitid",
        "rt::IeeeComparison::NumericEqual",
        "rt::IeeeComparison::BitIdentical",
    );
}

#[trace("TC-067", "FR-053-AC-8")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_bit_identical_asked_as_numeric_equal_is_falsified() {
    falsified(
        "bitid64",
        "quire.op.ieee.bit_identical",
        "float-bitid-as-numeq",
        "rt::IeeeComparison::BitIdentical",
        "rt::IeeeComparison::NumericEqual",
    );
}

#[trace("TC-067", "FR-053-AC-8")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_total_order_asked_as_bit_identical_is_falsified() {
    falsified(
        "total32",
        "quire.op.ieee.total_order",
        "float-total-as-bitid",
        "rt::IeeeComparison::TotalOrder",
        "rt::IeeeComparison::BitIdentical",
    );
}

/// The expected side mutated: numeric equality that treats a NaN as equal to itself.
#[trace("TC-067", "FR-053-AC-8")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_nan_reflexive_numeric_equal_expectation_is_falsified() {
    falsified(
        "numeq32",
        "quire.op.ieee.numeric_equal",
        "float-numeq-nan-reflexive",
        "!a_nan && !b_nan && (((a | b) << 1) == 0 || a == b)",
        "(((a | b) << 1) == 0 || a == b)",
    );
}

/// The counterexample of a falsified mutant is decoded to exact bit patterns and replayed through
/// the owner route, which gives the comparison's own answer, so the mutant's disagreement is
/// reproduced against the declared semantics.
#[trace("TC-067", "FR-053-AC-11")]
#[trace("TC-067", "FR-053-AC-8")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_a_falsifying_counterexample_replays_through_the_owner_route() {
    let corpus = corpus();
    let generated = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    let evidence = run(
        "float-replay",
        "numeq64",
        "quire.op.ieee.numeric_equal",
        Some((
            "rt::IeeeComparison::NumericEqual",
            "rt::IeeeComparison::BitIdentical",
        )),
    );
    let KaniRunOutcome::Falsified { counterexample } = evidence.outcome else {
        panic!("the mutant must falsify: {evidence:?}");
    };
    let witness = decode(&generated, &counterexample).expect("the counterexample decodes");
    let owner = quire_contract_codegen::replay_float_witness(&owner_package("numeq64"), &witness)
        .expect("the owner route evaluates the bit patterns");
    let (a, b) = (witness.operands[0].1, witness.operands[1].1);
    let nan = |bits: u64| (bits >> 52) & 0x7FF == 0x7FF && bits & 0xF_FFFF_FFFF_FFFF != 0;
    let expected = !nan(a) && !nan(b) && ((a | b) << 1 == 0 || a == b);
    assert_eq!(
        owner, expected,
        "the owner's answer is numeric equality: {a:#x} {b:#x}"
    );
    assert_ne!(
        owner,
        a == b,
        "the mutant (bit identity) disagrees with the owner on this witness"
    );
}

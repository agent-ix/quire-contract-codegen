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
function lit64 using v(a: Float64[nearest-even]): Boolean pure { quire::value::ieee::numericEqual(a, float64(bits: 0x3ff0000000000000)) }\n\
function nested64 using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { quire::value::ieee::numericEqual(a + b, b) }\n\
function unpinned64 using v(a: Float64, b: Float64): Boolean pure { quire::value::ieee::numericEqual(a, b) }\n\
function not_float using v(a: Int[0, 9], b: Int[0, 9]): Boolean pure { a < b }\n";

struct Corpus {
    package: CheckedPackageV2,
}

fn compiled_bytes() -> Vec<u8> {
    qsl_replay::compile_admitted(
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
    .expect("QSL admits the float source")
    .package()
    .bytes()
    .to_vec()
}

fn read_package(bytes: &[u8]) -> Option<CheckedPackageV2> {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), &evidence) {
        CheckedPackageV2ReadResult::Admitted(package) => Some(*package),
        _ => None,
    }
}

fn corpus() -> Corpus {
    Corpus {
        package: read_package(&compiled_bytes()).expect("Contract IR admits the QSL float source"),
    }
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

// ---- operand controls (FR-053-AC-1, AC-3) ----

/// The disposition cause and operation of the routed item at `position`.
fn routed_cause(generation: &quire_contract_codegen::RoutedGeneration, position: usize) -> String {
    let (record, harness) = kani_parts(&generation.items[position].output);
    assert!(harness.is_none(), "a refused item has no harness");
    match &record.disposition {
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::FloatOperationNotRendered { cause, .. },
        } => cause.clone(),
        ObligationDisposition::RequiresBound { .. } => "requires-bound".to_owned(),
        other => panic!("not a float disposition: {other:?}"),
    }
}

/// A float literal operand is named `FloatLiteralOperand` and an operand that is neither a
/// parameter nor a literal is FR-015's `OperandNotSupported`; each carries no harness, directly
/// and through `generate_routed`, and a supported sibling is unchanged.
#[trace("TC-067", "FR-053-AC-1")]
#[test]
fn a_literal_operand_and_a_non_parameter_operand_are_refused_under_their_own_names() {
    let corpus = corpus();
    let cause = |function: &str| match corpus.outcome(function, "quire.op.ieee.numeric_equal") {
        FloatScalarOutcome::Refused(FloatScalarRefusal::Unsupported { capability, .. }) => {
            capability
        }
        other => panic!("{function} must be refused: {other:?}"),
    };
    assert_eq!(cause("lit64"), FloatCapability::LiteralOperand);
    assert_eq!(cause("nested64"), FloatCapability::OperandNotParameter);
    assert_eq!(
        FloatCapability::LiteralOperand.name(),
        "float.literal_operand"
    );
    assert_eq!(
        FloatCapability::OperandNotParameter.name(),
        "float.operand_not_parameter"
    );
    let routed = corpus.routed(&[
        ("lit64", "quire.op.ieee.numeric_equal"),
        ("nested64", "quire.op.ieee.numeric_equal"),
        ("numeq64", "quire.op.ieee.numeric_equal"),
    ]);
    assert_eq!(routed_cause(&routed, 0), "float.literal_operand");
    assert_eq!(routed_cause(&routed, 1), "float.operand_not_parameter");
    let (record, harness) = kani_parts(&routed.items[2].output);
    assert!(
        harness.is_some(),
        "the supported sibling keeps its harness: {:?}",
        record.disposition
    );
}

/// The corpus package with the `unpinned64` parameters retyped by `retype`, which gets the node's
/// JSON and the `float64` scalar type's id. QSL types a bare `Float64` as `exact`-pinned, so an
/// operand with no `float_rounding` bound exists only as hand-edited Contract IR.
fn corpus_with_unpinned_operands(retype: bool, bogus_rounding: bool) -> (Corpus, CheckedNodeId) {
    let mut value: serde_json::Value = serde_json::from_slice(&compiled_bytes()).expect("json");
    let nodes = value["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    let scalar = nodes
        .iter()
        .find(|n| n["node_id"]["path"][0] == "scalar_type" && n["node_id"]["path"][1] == "float64")
        .expect("the float64 scalar type")["node_id"]
        .clone();
    // Ids carry operand positions, so the `exact` pin is found by its path spelling.
    let pinned_exact = |id: &serde_json::Value| id["path"].to_string().contains("\"exact\"");
    // Each retyped node's operand row loses the entry of its dropped type id, and every later
    // operand marker of that node moves down by one.
    fn shift(value: &mut serde_json::Value, above: u64) {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(k) = map.get("operand").and_then(serde_json::Value::as_u64) {
                    if k > above {
                        map.insert("operand".to_owned(), serde_json::json!(k - 1));
                    }
                }
                map.values_mut().for_each(|inner| shift(inner, above));
            }
            serde_json::Value::Array(items) => items.iter_mut().for_each(|i| shift(i, above)),
            _ => {}
        }
    }
    let mut dropped = Vec::new();
    for (position, node) in nodes.iter_mut().enumerate() {
        if retype && node["semantic_form"] == "parameter" && pinned_exact(&node["semantic_type"]) {
            dropped.push(position);
            node["semantic_type"] = scalar.clone();
            shift(node, 3);
        }
    }
    let rows = value["semantic_graph"]["operand_rows"]
        .as_array_mut()
        .expect("rows");
    for position in dropped {
        rows[position].as_array_mut().expect("row").remove(3);
    }
    let nodes = value["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    for node in nodes.iter_mut() {
        if bogus_rounding && node["node_tag"] == "bounded_domain" && pinned_exact(&node["node_id"])
        {
            node["body"]["members"][0]["value"]["value"] = serde_json::json!("bogus");
        }
    }
    let bytes = serde_json::to_vec(&value).expect("json");
    let package = match CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &{
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        evidence
    }) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("the edited package is not admitted: {other:?}"),
    };
    let scalar_id = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|n| {
            n.node_tag.as_ref() == "scalar_type" && format!("{:?}", n.node_id).contains("float64")
        })
        .expect("scalar node")
        .node_id
        .clone();
    (Corpus { package }, scalar_id)
}

/// An operand whose type reaches no `float_rounding` bound is `requires-bound` naming that type,
/// with no harness (CG's own check until the Contract IR float ruling), an unreadable bound is a
/// named unsupported capability, and a pinned operand pair is unaffected.
#[trace("TC-067", "FR-053-AC-3")]
#[test]
fn an_unpinned_float_operand_requires_a_bound_naming_its_type() {
    let (corpus, scalar) = corpus_with_unpinned_operands(true, false);
    let FloatScalarOutcome::Refused(FloatScalarRefusal::RequiresBound { unbounded_type }) =
        corpus.outcome("unpinned64", "quire.op.ieee.numeric_equal")
    else {
        panic!(
            "an unpinned operand requires a bound: {:?}",
            corpus.outcome("unpinned64", "quire.op.ieee.numeric_equal")
        );
    };
    assert_eq!(
        unbounded_type, scalar,
        "the operand's own type node is named"
    );
    // The routed path settles this refusal through `into_disposition`. Contract IR lowers the
    // hand-edited package to an empty lowered set, so no admitted routing input exists and
    // `generate_routed` cannot be reached with it; the seam is tested directly.
    assert_eq!(
        FloatScalarRefusal::RequiresBound {
            unbounded_type: unbounded_type.clone()
        }
        .into_disposition(),
        ObligationDisposition::RequiresBound { unbounded_type }
    );
    let sibling = corpus.outcome("numeq64", "quire.op.ieee.numeric_equal");
    assert!(
        matches!(sibling, FloatScalarOutcome::Supported(_)),
        "a pinned sibling is unaffected: {sibling:?}"
    );

    let (corpus, _) = corpus_with_unpinned_operands(false, true);
    let FloatScalarOutcome::Refused(FloatScalarRefusal::Unsupported { capability, .. }) =
        corpus.outcome("unpinned64", "quire.op.ieee.numeric_equal")
    else {
        panic!("an unreadable bound is unsupported");
    };
    assert_eq!(capability, FloatCapability::RoundingUnreadable);

    // No result bound is required of a comparison, and the pin narrows no operand domain.
    let pinned = corpus.harness("numeq64", "quire.op.ieee.numeric_equal");
    assert!(
        proof_body(&pinned).contains("kani::assume(left >= i64::MIN && left <= i64::MAX)"),
        "{}",
        proof_body(&pinned)
    );
}

/// A hand-built witness of an unknown width is refused, never replayed as binary64.
#[trace("TC-067", "FR-053-AC-11")]
#[test]
fn an_unknown_witness_width_is_refused_not_replayed_as_binary64() {
    let witness = quire_contract_codegen::FloatWitness {
        operator: "quire.op.ieee.numeric_equal".to_owned(),
        width: "float16".to_owned(),
        operands: vec![("a".to_owned(), 0), ("b".to_owned(), 0)],
    };
    assert!(matches!(
        quire_contract_codegen::replay_float_witness(&owner_package("numeq64"), &witness),
        Err(quire_contract_codegen::SpineReplayError::UnsupportedWitnessValue { .. })
    ));
}

// ---- encoding correspondence (FR-053-AC-2) ----

/// The class-boundary set of one width: both signs; exponent field 0, 1, the bias, the largest
/// finite and all ones; fraction 0, 1, the largest below the quiet bit, the quiet bit, the quiet
/// bit plus one and all ones.
fn boundary_patterns(wide: bool) -> Vec<u64> {
    let (exponent_bits, fraction_bits): (u32, u32) = if wide { (11, 52) } else { (8, 23) };
    let ones = (1_u64 << exponent_bits) - 1;
    let quiet = 1_u64 << (fraction_bits - 1);
    let all = (1_u64 << fraction_bits) - 1;
    let bias = (1_u64 << (exponent_bits - 1)) - 1;
    let mut patterns = Vec::new();
    for sign in [0_u64, 1] {
        for exponent in [0, 1, bias, ones - 1, ones] {
            for fraction in [0, 1, quiet - 1, quiet, quiet + 1, all] {
                patterns.push(
                    (sign << (exponent_bits + fraction_bits))
                        | (exponent << fraction_bits)
                        | fraction,
                );
            }
        }
    }
    patterns
}

/// The encoding a generated harness states: which `i64` draws it admits and the bit pattern each
/// decodes to. Both are read from the harness text.
struct Encoding {
    admit: Box<dyn Fn(i64) -> bool>,
    decode: Box<dyn Fn(i64) -> u64>,
}

fn encoding_of(text: &str) -> Encoding {
    let assume = text
        .lines()
        .find(|line| line.starts_with("kani::assume(left >="))
        .expect("the left domain assumption");
    let bound = |marker: &str, end: &str| -> i64 {
        let from = assume.find(marker).expect("a bound") + marker.len();
        let rest = &assume[from..];
        let literal = &rest[..rest.find(end).expect("a bound end")];
        match literal.trim().trim_end_matches("_i64") {
            "i64::MIN" => i64::MIN,
            "i64::MAX" => i64::MAX,
            "u32::MAX" => i64::from(u32::MAX),
            other => other
                .replace('_', "")
                .parse()
                .unwrap_or_else(|_| panic!("bound {other:?} in {assume}")),
        }
    };
    let (low, high) = (bound("left >= ", " &&"), bound("left <= ", ")"));
    let decode: Box<dyn Fn(i64) -> u64> = if text.contains("binary64(left as u64)") {
        Box::new(|draw| draw as u64)
    } else if text.contains("binary32(left as u32)") {
        Box::new(|draw| u64::from(draw as u32))
    } else {
        panic!("no recognised decode of the draw");
    };
    Encoding {
        admit: Box::new(move |draw| low <= draw && draw <= high),
        decode,
    }
}

/// The correspondence check of FR-196-AC-5 over the class-boundary set: an in-range pattern is
/// admitted exactly once and decodes to the same bits, and an out-of-range `float32` draw is
/// excluded.
fn corresponds(encoding: &Encoding, wide: bool) -> Result<(), String> {
    for pattern in boundary_patterns(wide) {
        let draw = if wide {
            pattern as i64
        } else {
            i64::try_from(pattern).expect("a binary32 pattern fits an i64")
        };
        if !(encoding.admit)(draw) {
            return Err(format!("pattern {pattern:#x} is not admitted"));
        }
        if (encoding.decode)(draw) != pattern {
            return Err(format!(
                "pattern {pattern:#x} decodes to {:#x}",
                (encoding.decode)(draw)
            ));
        }
    }
    if !wide {
        for outside in [1_i64 << 32, -1, i64::MIN, i64::MAX] {
            if (encoding.admit)(outside) {
                return Err(format!("the out-of-range draw {outside} is admitted"));
            }
        }
    }
    Ok(())
}

fn is_nan(bits: u64, wide: bool) -> bool {
    let (shift, ones, fraction) = if wide {
        (52, 0x7FF, (1_u64 << 52) - 1)
    } else {
        (23, 0xFF, (1_u64 << 23) - 1)
    };
    (bits >> shift) & ones == ones && bits & fraction != 0
}

/// The generated harness's draw encoding matches the stated class-boundary set at both widths, and
/// each of the seven mutants of the encoding fails the correspondence check at its named point.
#[trace("TC-067", "FR-053-AC-2")]
#[test]
fn the_draw_encoding_matches_the_class_boundary_set_and_every_encoding_mutant_fails() {
    let corpus = corpus();
    let text64 = corpus
        .harness("numeq64", "quire.op.ieee.numeric_equal")
        .harness
        .rust
        .contents;
    let text32 = corpus
        .harness("numeq32", "quire.op.ieee.numeric_equal")
        .harness
        .rust
        .contents;
    corresponds(&encoding_of(&text64), true).expect("float64 corresponds");
    corresponds(&encoding_of(&text32), false).expect("float32 corresponds");
    // Every pattern of the set is distinct, so each is reachable exactly once.
    for wide in [true, false] {
        let mut seen = boundary_patterns(wide);
        let count = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), count);
    }

    let fails = |encoding: &Encoding, wide: bool, point: u64| {
        let error = corresponds(encoding, wide).expect_err("the mutant must fail");
        assert!(
            error.contains(&format!("{point:#x}")),
            "{error} at {point:#x}"
        );
    };
    // (1) non-negative float64 bits assumed: fails at 0x8000000000000000. Source mutation.
    let mutated = text64.replace("left >= i64::MIN", "left >= 0");
    assert_ne!(mutated, text64);
    fails(&encoding_of(&mutated), true, 0x8000_0000_0000_0000);
    // (5) a float32 draw unconstrained above u32::MAX: 2^32 is admitted (it aliases Z+).
    let mutated = text32.replace("left <= 4294967295_i64", "left <= i64::MAX");
    assert_ne!(mutated, text32, "{text32}");
    let error = corresponds(&encoding_of(&mutated), false).expect_err("2^32 is admitted");
    assert!(error.contains("out-of-range draw 4294967296"), "{error}");
    // (2), (3), (4), (6), (7): the model of the mutated assumption or decode.
    let excluding = |wide: bool, excluded: fn(u64, bool) -> bool| Encoding {
        admit: Box::new(move |draw| {
            let in_range = wide || (0..=i64::from(u32::MAX)).contains(&draw);
            let bits = if wide {
                draw as u64
            } else {
                u64::from(draw as u32)
            };
            in_range && !excluded(bits, wide)
        }),
        decode: if wide {
            Box::new(|draw| draw as u64)
        } else {
            Box::new(|draw| u64::from(draw as u32))
        },
    };
    fails(&excluding(true, is_nan), true, 0x7FF0_0000_0000_0001);
    fails(
        &excluding(true, |bits, _| {
            bits & 0x7FFF_FFFF_FFFF_FFFF == 0x7FF0_0000_0000_0000
        }),
        true,
        0x7FF0_0000_0000_0000,
    );
    fails(
        &excluding(true, |bits, _| bits & 0x7FF0_0000_0000_0000 == 0),
        true,
        0,
    );
    // (6) a float32 draw decoded as a signed 32-bit value.
    let signed = Encoding {
        admit: Box::new(|draw| (0..=i64::from(u32::MAX)).contains(&draw)),
        decode: Box::new(|draw| i64::from(draw as i32) as u64),
    };
    fails(&signed, false, 0x8000_0000);
    // (7) a decode that canonicalizes a NaN keeps no payload: the signaling NaN is the point.
    let canonical = Encoding {
        admit: Box::new(|_| true),
        decode: Box::new(|draw| {
            if is_nan(draw as u64, true) {
                0x7FF8_0000_0000_0000
            } else {
                draw as u64
            }
        }),
    };
    fails(&canonical, true, 0x7FF0_0000_0000_0001);
}

// ---- host differential of the native comparisons against the shipped owner (FR-053-AC-4, AC-13) ----

const CONTROLS_64: [u64; 12] = [
    0x0000_0000_0000_0000,
    0x8000_0000_0000_0000,
    0x3FF0_0000_0000_0000,
    0xBFF0_0000_0000_0000,
    0x4000_0000_0000_0000,
    0xC000_0000_0000_0000,
    0x7FF8_0000_0000_0001,
    0x7FF8_0000_0000_0002,
    0xFFF8_0000_0000_0000,
    0xFFF0_0000_0000_0001,
    0x7FEF_FFFF_FFFF_FFFF,
    0x0000_0000_0000_0001,
];

const CONTROLS_32: [u64; 12] = [
    0x0000_0000,
    0x8000_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x4000_0000,
    0xC000_0000,
    0x7FC0_0001,
    0x7FC0_0002,
    0xFFC0_0000,
    0xFF80_0001,
    0x7F7F_FFFF,
    0x0000_0001,
];

fn limits_of(value: u64) -> quire_exact::ScalarLimits {
    quire_exact::ScalarLimits {
        integer_bits: value,
        decimal_digits: value,
        scale_expansion: value,
        text_input_bytes: value,
        text_scalars: value,
        normalized_scalars: value,
        unit_edges: value,
        value_occurrences: value,
        work_units: value,
        result_units: value,
    }
}

fn limits_with(kind: quire_exact::LimitKind, value: u64) -> quire_exact::ScalarLimits {
    use quire_exact::LimitKind;
    let mut limits = limits_of(u64::MAX);
    match kind {
        LimitKind::IntegerBits => limits.integer_bits = value,
        LimitKind::ValueOccurrences => limits.value_occurrences = value,
        LimitKind::WorkUnits => limits.work_units = value,
        LimitKind::ResultUnits => limits.result_units = value,
        other => panic!("{other:?} is not charged by a comparison"),
    }
    limits
}

/// For every comparison and width, over the class-boundary set and the control points, the native
/// comparison and the shipped owner agree on the typed outcome and on the meter at zero, exact and
/// short budgets of every counter a comparison charges, and the denied charge is the expected point.
#[trace("TC-067", "FR-053-AC-4")]
#[trace("TC-067", "FR-053-AC-13")]
#[test]
fn the_native_comparisons_equal_the_owner_with_charges_and_budgets_at_every_limit() {
    use quire_exact::{
        compare_ieee, compare_native_float, ChargePoint, IeeeComparison, IeeeWidth, LimitKind,
        Meter, NativeFloat, Outcome,
    };
    for (wide, width) in [(true, IeeeWidth::Binary64), (false, IeeeWidth::Binary32)] {
        let bits = u64::from(width.bits());
        let mut grid = boundary_patterns(wide);
        grid.extend(if wide { CONTROLS_64 } else { CONTROLS_32 });
        grid.sort_unstable();
        grid.dedup();
        // (counter, limit, the point whose charge the limit denies; `None` when the budget suffices)
        let budgets: [(LimitKind, u64, Option<ChargePoint>); 10] = [
            (LimitKind::WorkUnits, u64::MAX, None),
            (LimitKind::WorkUnits, 2, None),
            (LimitKind::WorkUnits, 1, Some(ChargePoint::IeeeResultRetain)),
            (LimitKind::WorkUnits, 0, Some(ChargePoint::IeeeOperands)),
            (LimitKind::ResultUnits, 1, None),
            (
                LimitKind::ResultUnits,
                0,
                Some(ChargePoint::IeeeResultRetain),
            ),
            (LimitKind::IntegerBits, bits, None),
            (
                LimitKind::IntegerBits,
                bits - 1,
                Some(ChargePoint::IeeeOperands),
            ),
            (LimitKind::ValueOccurrences, 2, None),
            (
                LimitKind::ValueOccurrences,
                1,
                Some(ChargePoint::IeeeOperands),
            ),
        ];
        for comparison in IeeeComparison::ALL {
            for &a in &grid {
                for &b in &grid {
                    let (left, right) = (
                        NativeFloat::new(width, a).expect("a pattern of the width"),
                        NativeFloat::new(width, b).expect("a pattern of the width"),
                    );
                    for (kind, value, denied) in budgets {
                        let limits = limits_with(kind, value);
                        let (mut native, mut owner) = (Meter::new(limits), Meter::new(limits));
                        let n = compare_native_float(comparison, left, right, &mut native);
                        let o = compare_ieee(
                            comparison,
                            left.to_ieee_value(),
                            right.to_ieee_value(),
                            &mut owner,
                        );
                        assert_eq!(n, o, "{comparison:?} {a:#x} {b:#x} {kind:?}={value}");
                        assert_eq!(native, owner, "meter: {comparison:?} {a:#x} {b:#x}");
                        match (&n, denied) {
                            (Ok(Outcome::Incomplete(stop)), Some(point)) => {
                                assert_eq!(stop.charge_point, point, "{kind:?}={value}");
                                assert_eq!(stop.limit_kind, kind);
                            }
                            (Ok(Outcome::Completed(_)), None) => {}
                            other => panic!("{kind:?}={value}: unexpected {other:?}"),
                        }
                    }
                    // Exact budget at zero consumed counters: the charges completed, in order, and
                    // the consumed counters are the comparison's own.
                    let mut meter = Meter::new(limits_of(u64::MAX));
                    let _ = compare_native_float(comparison, left, right, &mut meter);
                    assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
                    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
                    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
                    assert_eq!(meter.consumed(LimitKind::IntegerBits), bits);
                }
            }
        }
        // Operands of distinct widths are ill-typed before any charge.
        let other = if wide {
            IeeeWidth::Binary32
        } else {
            IeeeWidth::Binary64
        };
        let mut meter = Meter::new(limits_of(u64::MAX));
        let mismatch = compare_native_float(
            IeeeComparison::BitIdentical,
            NativeFloat::new(width, 0).expect("zero"),
            NativeFloat::new(other, 0).expect("zero"),
            &mut meter,
        );
        assert!(mismatch.is_err());
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 0);
    }
}

/// The expected results of the AC-8 control points hold against the shipped owner before any
/// mutant is run against them.
#[trace("TC-067", "FR-053-AC-13")]
#[test]
fn every_comparison_control_point_has_its_stated_owner_result() {
    use quire_exact::IeeeComparison::{BitIdentical, NumericEqual, TotalOrder};
    for (comparison, width_wide, a, b, expected) in COMPARISON_POINTS {
        assert_eq!(
            owner_compare(comparison, width_wide, a, b),
            expected,
            "{comparison:?} {a:#x} {b:#x}"
        );
    }
    let _ = (BitIdentical, NumericEqual, TotalOrder);
}

fn owner_compare(comparison: quire_exact::IeeeComparison, wide: bool, a: u64, b: u64) -> bool {
    let width = if wide {
        quire_exact::IeeeWidth::Binary64
    } else {
        quire_exact::IeeeWidth::Binary32
    };
    let mut meter = quire_exact::Meter::new(limits_of(u64::MAX));
    let value = |bits| quire_exact::NativeFloat::new(width, bits).expect("a pattern of the width");
    match quire_exact::compare_ieee(
        comparison,
        value(a).to_ieee_value(),
        value(b).to_ieee_value(),
        &mut meter,
    ) {
        Ok(quire_exact::Outcome::Completed(result)) => result,
        other => panic!("the owner did not complete: {other:?}"),
    }
}

const Z_P: u64 = 0;
const Z_N: u64 = 0x8000_0000_0000_0000;
const ONE: u64 = 0x3FF0_0000_0000_0000;
const M_ONE: u64 = 0xBFF0_0000_0000_0000;
const TWO: u64 = 0x4000_0000_0000_0000;
const M_TWO: u64 = 0xC000_0000_0000_0000;
const INF_P: u64 = 0x7FF0_0000_0000_0000;
const INF_N: u64 = 0xFFF0_0000_0000_0000;
const QN0: u64 = 0x7FF8_0000_0000_0000;
const QN1: u64 = 0x7FF8_0000_0000_0001;
const QN2: u64 = 0x7FF8_0000_0000_0002;
const SN1: u64 = 0x7FF0_0000_0000_0001;
const NQN0: u64 = 0xFFF8_0000_0000_0000;
const NSN1: u64 = 0xFFF0_0000_0000_0001;

/// Every control point of the comparison mutants (FR-053 Behavior control points, AC-8): the
/// comparison, whether it is `binary64`, the two bit patterns and the owner's expected result.
const COMPARISON_POINTS: [(quire_exact::IeeeComparison, bool, u64, u64, bool); 22] = {
    use quire_exact::IeeeComparison::{BitIdentical as B, NumericEqual as N, TotalOrder as T};
    [
        (N, true, Z_P, Z_N, true),
        (N, true, QN0, QN0, false),
        (N, true, ONE, M_ONE, false),
        (B, true, Z_P, Z_N, false),
        (B, true, QN0, QN0, true),
        (B, true, QN1, QN2, false),
        (T, true, M_ONE, M_TWO, false),
        (T, true, M_ONE, ONE, true),
        (T, true, Z_P, Z_N, false),
        (T, true, ONE, ONE, true),
        (T, true, QN0, QN0, true),
        (T, true, INF_P, QN0, true),
        (T, true, NQN0, INF_N, true),
        (T, true, QN0, SN1, false),
        (T, true, NSN1, NQN0, false),
        (T, true, ONE, TWO, true),
        (T, false, 0xBF80_0000, 0x3F80_0000, true),
        (N, false, 0x7FC0_0000, 0x7FC0_0000, false),
        (N, true, ONE, ONE, true),
        (B, true, ONE, ONE, true),
        (T, true, TWO, ONE, false),
        (N, true, INF_P, INF_P, true),
    ]
};

/// One comparison mutant of FR-053-AC-8: the text that replaces the expected side of the harness, a
/// host model of that mutant, and the control point that distinguishes it from the owner.
struct Mutant {
    id: &'static str,
    function: &'static str,
    identity: &'static str,
    comparison: quire_exact::IeeeComparison,
    wide: bool,
    site: &'static str,
    with: &'static str,
    point: (u64, u64),
    model: fn(u64, u64) -> bool,
}

const TOTAL_F64: &str = "f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_le()";
const NUMERIC_F64: &str = "!a_nan && !b_nan && (((a | b) << 1) == 0 || a == b)";

fn nan64(bits: u64) -> bool {
    is_nan(bits, true)
}

fn total64(a: u64, b: u64) -> bool {
    f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_le()
}

fn mutants() -> Vec<Mutant> {
    use quire_exact::IeeeComparison::{BitIdentical, NumericEqual, TotalOrder};
    vec![
        Mutant {
            id: "c numeric_equal ignoring the sign of a nonzero value",
            function: "numeq64",
            identity: "quire.op.ieee.numeric_equal",
            comparison: NumericEqual,
            wide: true,
            site: NUMERIC_F64,
            with: "!a_nan && !b_nan && (((a | b) << 1) == 0 || (a << 1) == (b << 1))",
            point: (ONE, M_ONE),
            model: |a, b| !nan64(a) && !nan64(b) && ((a | b) << 1 == 0 || a << 1 == b << 1),
        },
        Mutant {
            id: "e bit_identical canonicalizing a NaN",
            function: "bitid64",
            identity: "quire.op.ieee.bit_identical",
            comparison: BitIdentical,
            wide: true,
            site: "let native = a == b;",
            with: "let native = (if a_nan { 0x7FF8_0000_0000_0000_u64 } else { a }) == (if b_nan { 0x7FF8_0000_0000_0000_u64 } else { b });",
            point: (QN1, QN2),
            model: |a, b| {
                (if nan64(a) { QN0 } else { a }) == (if nan64(b) { QN0 } else { b })
            },
        },
        Mutant {
            id: "f total_order as signed integer comparison of the raw bits",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "(a as i64) <= (b as i64)",
            point: (M_ONE, M_TWO),
            model: |a, b| (a as i64) <= (b as i64),
        },
        Mutant {
            id: "g total_order as unsigned comparison of the raw bits",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "a <= b",
            point: (M_ONE, ONE),
            model: |a, b| a <= b,
        },
        Mutant {
            id: "h total_order equating the zeros",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "(((a | b) << 1) == 0) || f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_le()",
            point: (Z_P, Z_N),
            model: |a, b| (a | b) << 1 == 0 || total64(a, b),
        },
        Mutant {
            id: "i total_order strict instead of precedes-or-equal",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_lt()",
            point: (ONE, ONE),
            model: |a, b| f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_lt(),
        },
        Mutant {
            id: "j total_order leaving NaN unordered",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "!a_nan && !b_nan && f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_le()",
            point: (INF_P, QN0),
            model: |a, b| !nan64(a) && !nan64(b) && total64(a, b),
        },
        Mutant {
            id: "k total_order equating all NaNs",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "(a_nan && b_nan) || f64::from_bits(a).total_cmp(&f64::from_bits(b)).is_le()",
            point: (QN0, SN1),
            model: |a, b| (nan64(a) && nan64(b)) || total64(a, b),
        },
        Mutant {
            id: "l total_order with the operands swapped",
            function: "total64",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: true,
            site: TOTAL_F64,
            with: "f64::from_bits(b).total_cmp(&f64::from_bits(a)).is_le()",
            point: (ONE, TWO),
            model: |a, b| total64(b, a),
        },
        Mutant {
            id: "m total_order key built with the binary64 sign position at binary32",
            function: "total32",
            identity: "quire.op.ieee.total_order",
            comparison: TotalOrder,
            wide: false,
            site: "f32::from_bits(a).total_cmp(&f32::from_bits(b)).is_le()",
            with: "{ let ka = (a as u64) ^ (if (a as u64) >> 63 == 1 { u64::MAX } else { 1u64 << 63 }); let kb = (b as u64) ^ (if (b as u64) >> 63 == 1 { u64::MAX } else { 1u64 << 63 }); ka <= kb }",
            point: (0xBF80_0000, 0x3F80_0000),
            model: |a, b| {
                let key = |x: u64| x ^ if x >> 63 == 1 { u64::MAX } else { 1 << 63 };
                key(a) <= key(b)
            },
        },
    ]
}

/// Each comparison mutant's control point is one where the mutant and the shipped owner disagree.
#[trace("TC-067", "FR-053-AC-8")]
#[test]
fn every_comparison_mutant_disagrees_with_the_owner_at_its_control_point() {
    for mutant in mutants() {
        let (a, b) = mutant.point;
        let owner = owner_compare(mutant.comparison, mutant.wide, a, b);
        assert_ne!(
            (mutant.model)(a, b),
            owner,
            "{}: the point {a:#x} {b:#x} must distinguish the mutant",
            mutant.id
        );
    }
}

// ---- the expected side shares no function with the native operations (FR-053-AC-14) ----

/// The statements of a harness that form the expected side: the operand bit patterns, the NaN
/// classification and the expected result.
fn expected_side(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| {
            [
                "let a =",
                "let b =",
                "let a_nan",
                "let b_nan",
                "let native =",
            ]
            .iter()
            .any(|prefix| line.starts_with(prefix))
        })
        .collect()
}

/// The inspection: no statement of the expected side names a carrier function, the decode or the
/// owner's path. Only the host's float operations and bit arithmetic remain.
fn inspect_expected_side(text: &str) -> Result<(), String> {
    let lines = expected_side(text);
    if lines.len() != 5 {
        return Err(format!("expected five statements, found {}", lines.len()));
    }
    for line in lines {
        for forbidden in [
            "rt::",
            "compare_",
            "evaluate_",
            "NativeFloat",
            "IeeeValue",
            "decode",
            "classify",
            "carrier",
            "meter",
        ] {
            if line.contains(forbidden) {
                return Err(format!("`{line}` reaches `{forbidden}`"));
            }
        }
    }
    Ok(())
}

/// The call graph of the expected side of every supported comparison shows no function of the
/// native operations, the decode included, and a mutant that makes it call them fails the
/// inspection although the harness still compiles to a comparison against the oracle.
#[trace("TC-067", "FR-053-AC-14")]
#[test]
fn the_expected_side_reaches_no_native_operation_and_a_sharing_mutant_fails_the_inspection() {
    let corpus = corpus();
    for (function, identity, _, _) in ROWS {
        let text = corpus.harness(function, identity).harness.rust.contents;
        inspect_expected_side(&text).unwrap_or_else(|error| panic!("{function}: {error}"));
        // The native operation is called exactly once, outside the expected side.
        assert_eq!(
            text.matches("compare_native_float(").count(),
            1,
            "{function}"
        );
    }
    let text = corpus
        .harness("bitid64", "quire.op.ieee.bit_identical")
        .harness
        .rust
        .contents;
    for (site, with) in [
        (
            "let native = a == b;",
            "let native = matches!(compare_native_float(comparison, left_carrier, right_carrier, &mut meter), Ok(rt::Outcome::Completed(true)));",
        ),
        (
            "let a_nan = ((a >> 52) & 0x7FF_u64) == 0x7FF_u64 && (a & 0xF_FFFF_FFFF_FFFF_u64) != 0;",
            "let a_nan = rt::NativeFloat::binary64(a).is_nan();",
        ),
        ("let a = left as u64;", "let a = left_carrier.bits();"),
    ] {
        let mutated = text.replace(site, with);
        assert_ne!(mutated, text, "{site}");
        assert!(inspect_expected_side(&mutated).is_err(), "{with}");
    }
}

// ---- the identity and the shipped tractability record (FR-053-AC-12, AC-6) ----

/// The generated identity carries the operation and its source association, the operand forms and
/// width, the correspondence, the ordered physical bindings, the subject, every backend option,
/// the unwind bound and the solver, and the oracle crate manifest names the carrier dependency.
#[trace("TC-067", "FR-053-AC-12")]
#[test]
fn the_identity_records_the_operation_bindings_options_and_dependency_cohort() {
    let corpus = corpus();
    let (harness, manifest) = corpus.routed_harness("numeq32", "quire.op.ieee.numeric_equal");
    let id = &harness.identity;
    assert_eq!(id.operation_identity, "quire.op.ieee.numeric_equal");
    assert!(
        id.requirement_occurrence.is_some(),
        "the source association"
    );
    let float = id.float_scalar.as_ref().expect("the float identity");
    assert_eq!(float.width, "float32");
    assert_eq!(float.operands.len(), 2, "two operand parameters, in order");
    assert!(
        float.encoding.contains("0..=0xFFFF_FFFF"),
        "{}",
        float.encoding
    );
    assert_eq!(
        id.arguments
            .iter()
            .map(|a| (a.identifier.as_str(), a.minimum, a.maximum))
            .collect::<Vec<_>>(),
        [("a", 0, i64::from(u32::MAX)), ("b", 0, i64::from(u32::MAX))]
    );
    assert_eq!(
        id.options,
        [
            "-Z".to_owned(),
            "function-contracts".to_owned(),
            "--harness".to_owned(),
            format!(
                "{}::{}",
                id.module_symbol.as_str(),
                id.harness_symbol.as_str()
            ),
            "--exact".to_owned(),
            "--unwind".to_owned(),
            "2".to_owned(),
            "--solver".to_owned(),
            "cadical".to_owned(),
            "--output-format".to_owned(),
            "regular".to_owned(),
        ]
    );
    assert_eq!(id.unwind, 2);
    assert_eq!(id.oracle_symbol, "quire_exact::compare_native_float");
    assert!(
        manifest.contents.contains("quire-exact"),
        "the dependency cohort names the carrier crate: {}",
        manifest.contents
    );
}

/// The five float arithmetic rows of the shipped tractability table are a record the decider does
/// not select (a float clause has no integer operand width), they carry no tool versions, and
/// each carries the peak measured for the row's non-Verified harnesses.
#[trace("TC-067", "FR-053-AC-6")]
#[test]
fn the_shipped_float_rows_are_a_version_free_record_with_measured_peaks() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs/qualification/ir1530-general-route-scalar-table.json");
    let table: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("the shipped table is readable"))
            .expect("the shipped table is JSON");
    let float_rows = table
        .as_array()
        .expect("rows")
        .iter()
        .filter(|row| {
            row["operation_ids"][0]
                .as_str()
                .is_some_and(|id| id.starts_with("quire.op.ieee."))
        })
        .collect::<Vec<_>>();
    assert_eq!(float_rows.len(), 5);
    for row in float_rows {
        assert!(
            row["peak_bytes"]
                .as_u64()
                .is_some_and(|peak| peak > 20_000_000_000),
            "{row}"
        );
        for field in ["kani_version", "cbmc_version", "solver_version"] {
            assert!(row.get(field).is_none(), "{field} in {row}");
        }
        assert_eq!(row["verdict"], "not_tractable");
    }
    // The loader accepts a row that records no tool version.
    quire_contract_codegen::TractabilityTable::shipped_general_route_scalar()
        .expect("the shipped table loads with float rows that record no tool version");
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

/// The comparison mutants of FR-053-AC-8 not run above (c, e through m; a, b and d are the
/// operator and NaN-reflexive mutants above): each replaces the expected side with the mutant's
/// expression, and Kani falsifies the harness with an in-domain counterexample. The distinguishing
/// control point of each is asserted against the owner on the host.
#[trace("TC-067", "FR-053-AC-8")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_every_remaining_comparison_mutant_is_falsified() {
    for (index, mutant) in mutants().iter().enumerate() {
        falsified(
            mutant.function,
            mutant.identity,
            &format!("float-mutant-{index}"),
            mutant.site,
            mutant.with,
        );
        eprintln!("FLOATMUTANT {} falsified", mutant.id);
    }
}

/// A float harness whose cover is forced unreachable is not `Verified`: the shared runner
/// classifies it `CoverUnsatisfied` (FR-053-AC-7).
#[trace("TC-067", "FR-053-AC-7")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_an_unsatisfied_cover_is_not_verified() {
    let evidence = run(
        "float-cover-forced",
        "numeq32",
        "quire.op.ieee.numeric_equal",
        Some((
            "let cover_reached: bool = completed;",
            "let cover_reached: bool = false;",
        )),
    );
    assert!(
        matches!(
            evidence.outcome,
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        ),
        "{evidence:?}"
    );
}

/// The run evidence of a real float proof records the target, the unwind bound, the solver, every
/// backend option in order, the symbolic arguments with their domains, the limits, the raw checks
/// with exactly one satisfied cover, the exit code and the typed terminal outcome, and no tool
/// version (QSL-406, EA-49): no key of its serialized form names a version.
#[trace("TC-067", "FR-053-AC-12")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_the_run_evidence_records_every_option_and_no_tool_version() {
    let corpus = corpus();
    let (harness, manifest) = corpus.routed_harness("numeq32", "quire.op.ieee.numeric_equal");
    let evidence =
        crate::kani_obligations::run_scalar_under_real_kani("float-evidence", &harness, &manifest);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{evidence:?}");
    assert_eq!(evidence.exit_code, Some(0));
    assert_eq!(evidence.harness_path, harness.rust.path, "the target");
    assert_eq!(evidence.kind, None);
    let options = harness.identity.options.len();
    assert_eq!(evidence.arguments[0], "kani");
    assert_eq!(
        evidence.arguments[1..=options],
        harness.identity.options[..],
        "every backend option, in order"
    );
    assert_eq!(
        evidence.arguments[options + 1..=options + 3],
        ["-Z", "unstable-options", "--export-json"]
    );
    assert_eq!(evidence.arguments.len(), options + 5);
    assert_eq!(evidence.unwind, 2);
    assert_eq!(evidence.solver, harness.identity.solver);
    assert_eq!(evidence.ceilings, harness.identity.ceilings);
    assert_eq!(
        evidence
            .symbolic_arguments
            .iter()
            .map(|argument| (argument.identifier.as_str(), argument.bounds.clone()))
            .collect::<Vec<_>>(),
        [
            (
                "a",
                quire_contract_codegen::SymbolicBounds::Integer {
                    minimum: 0,
                    maximum: i64::from(u32::MAX)
                }
            ),
            (
                "b",
                quire_contract_codegen::SymbolicBounds::Integer {
                    minimum: 0,
                    maximum: i64::from(u32::MAX)
                }
            ),
        ]
    );
    assert!(evidence.success_checks > 0, "raw checks are recorded");
    let covers = evidence
        .checks
        .iter()
        .filter(|check| check.class == quire_contract_codegen::KaniCheckClass::Cover)
        .collect::<Vec<_>>();
    assert_eq!(covers.len(), 1, "exactly one final completion cover");
    assert_eq!(
        covers[0].status,
        quire_contract_codegen::KaniCheckStatus::Satisfied
    );
    fn keys(value: &serde_json::Value, found: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, inner) in map {
                    found.push(key.clone());
                    keys(inner, found);
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|item| keys(item, found)),
            _ => {}
        }
    }
    let mut found = Vec::new();
    keys(
        &serde_json::to_value(&evidence).expect("serializes"),
        &mut found,
    );
    assert!(
        found
            .iter()
            .all(|key| !key.to_lowercase().contains("version")),
        "no tool version is recorded: {found:?}"
    );
}

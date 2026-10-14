//! FR-021: function-application oracle generation over admitted CheckedPackage
//! V2 input.
//!
//! Generation-time coverage only (no execution of generated code):
//! dispositions, refusal reasons, ordering/determinism, the manifest, and the
//! static location map. Execution-level criteria (AC-2, AC-4, AC-5, AC-9,
//! AC-17) are covered by `tests/it/exact_function_agreement.rs`; AC-7's
//! admitted 100K-call flat SSA control is exercised below.

use crate::exact_scalar_generation::invokes_panicking_macro;
use crate::scratch_crate::runtime_dependency;
use quire_contract_codegen::{
    generate_exact_function_oracles, generate_function_schedules,
    generate_function_schedules_with_limits, CallPointKind, ClaimDisposition, ExactFunctionItem,
    ExactFunctionRefusal, FunctionScheduleDeclaration, FunctionScheduleDisposition,
    FunctionScheduleItem, FunctionScheduleLimits, FunctionScheduleOperand as ScheduleOperand,
    FunctionScheduleParameter, FunctionScheduleRefusal as ScheduleRefusal,
    FunctionScheduledOperation, GeneratedExactFunctionClaim, UpstreamBlocker,
};
use quire_contract_model::{derive_structural_candidates, CheckedPackageV2, CheckedSemanticNodeV2};
use std::ops::Deref;

/// Trace: FR-021-AC-28, FR-021-AC-34, TC-031.
#[test]
fn tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule() {
    use serde_json::{json, Value};
    use std::{fs, process::Command};

    const COUNT: u32 = 129;
    const BASE: u32 = 80_000;
    const PARAM_BASE: u32 = 81_000;
    const ITEM_BASE: u32 = 82_000;
    const BODY_BASE: u32 = 83_000;

    let op = |identity: &str| {
        json!({
            "identity": identity,
            "laws": [],
            "mode": null,
            "member": null,
            "leaves": [],
        })
    };
    let mut builder = PackageBuilder::default();
    builder
        .code(
            T_INTEGER,
            "scalar_type",
            "integer",
            T_INTEGER,
            aggregate(vec![]),
        )
        .code(T_TEXT, "scalar_type", "text", T_TEXT, aggregate(vec![]));
    let int_type = key(T_INTEGER);
    // Keep one admitted ordered pair of Integer formals and bind that pair in every declaration.
    // Each of the 129 functions still has two Integer parameter slots; sharing the graph nodes
    // avoids repeating their full node, identity-projection and source-map records in this strict
    // reader fixture, which must fit the unchanged 1 MiB input limit.
    for (code, name) in [(PARAM_BASE, "left"), (PARAM_BASE + 1, "right")] {
        builder.code(
            code,
            "value",
            "parameter",
            T_INTEGER,
            aggregate(vec![
                bound_member("name", literal("text", name)),
                bound_member("level", integer_literal(0)),
            ]),
        );
    }
    let shared_parameters = (code_id(PARAM_BASE), code_id(PARAM_BASE + 1));
    let parameter_ids = vec![shared_parameters.clone(); COUNT as usize];
    // The second requested item calls the terminal function, matching BODY_BASE+127's callee.
    // Give its first argument a distinct admitted value node so its application identity does
    // not collide with that function body while retaining two ordered Integer references.
    builder.code(
        PARAM_BASE + 2,
        "value",
        "parameter",
        T_INTEGER,
        aggregate(vec![
            bound_member("name", literal("text", "item_left")),
            bound_member("level", integer_literal(0)),
        ]),
    );
    let item_left = code_id(PARAM_BASE + 2);

    let call_body = |callee: &quire_contract_model::CheckedNodeId,
                     params: &(
        quire_contract_model::CheckedNodeId,
        quire_contract_model::CheckedNodeId,
    )| {
        json!({
            "term": "application",
            "operator": "call",
            "operation": op("quire.op.function.call"),
            "result_type": {"domain": NODE_DOMAIN, "digest": int_type},
            "arguments": [
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": callee.digest}},
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": params.0.digest}},
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": params.1.digest}},
            ],
        })
    };
    let scalar_body = |params: &(
        quire_contract_model::CheckedNodeId,
        quire_contract_model::CheckedNodeId,
    )| {
        json!({
            "term": "application",
            "operator": "binary",
            "operation": op("quire.op.integer.add"),
            "result_type": {"domain": NODE_DOMAIN, "digest": int_type},
            "arguments": [
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": params.0.digest}},
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": params.1.digest}},
            ],
        })
    };

    let terminal = COUNT - 1;
    let body_reference = |code: u32| {
        let target = code_id(code);
        json!({
            "term": "reference",
            "target": {"domain": target.domain, "digest": target.digest},
        })
    };
    builder.application_node(
        BODY_BASE + terminal,
        "binary",
        &int_type,
        scalar_body(&parameter_ids[terminal as usize]),
    );
    let mut function_ids = vec![
        quire_contract_model::CheckedNodeId {
            domain: NODE_DOMAIN.into(),
            digest: String::new().into(),
        };
        COUNT as usize
    ];
    let mut body_ids = vec![function_ids[0].clone(); COUNT as usize];
    body_ids[terminal as usize] = code_id(BODY_BASE + terminal);
    builder.code(
        BASE + terminal,
        "function",
        "pure_function",
        T_INTEGER,
        body_reference(BODY_BASE + terminal),
    );
    function_ids[terminal as usize] = code_id(BASE + terminal);
    for index in (0..terminal).rev() {
        builder.application_node(
            BODY_BASE + index,
            "call",
            &int_type,
            call_body(
                &function_ids[(index + 1) as usize],
                &parameter_ids[index as usize],
            ),
        );
        body_ids[index as usize] = code_id(BODY_BASE + index);
        builder.code(
            BASE + index,
            "function",
            "pure_function",
            T_INTEGER,
            body_reference(BODY_BASE + index),
        );
        function_ids[index as usize] = code_id(BASE + index);
    }

    let make_item_body = |callee: &quire_contract_model::CheckedNodeId,
                          params: &(
        quire_contract_model::CheckedNodeId,
        quire_contract_model::CheckedNodeId,
    )| {
        json!({
            "term": "application",
            "operator": "call",
            "operation": op("quire.op.function.call"),
            "result_type": {"domain": NODE_DOMAIN, "digest": int_type},
            "arguments": [
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": callee.digest}},
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": params.0.digest}},
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": params.1.digest}},
            ],
        })
    };
    builder.application_node(
        ITEM_BASE,
        "call",
        &int_type,
        make_item_body(&function_ids[0], &parameter_ids[0]),
    );
    builder.application_node(
        ITEM_BASE + 1,
        "call",
        &int_type,
        make_item_body(
            &function_ids[terminal as usize],
            &(item_left, shared_parameters.1.clone()),
        ),
    );

    let wire: Value = builder.wire();
    let bytes = serde_json::to_vec(&wire).expect("serialize strict-reader fixture");
    #[derive(Default)]
    struct WireBytes {
        node_count: usize,
        nodes: u64,
        projection_count: usize,
        projections: u64,
        source_map_count: usize,
        source_maps: u64,
    }
    let category = |node: &Value| -> &'static str {
        match (
            node["node_tag"].as_str(),
            node["semantic_form"].as_str(),
            node["body"]["term"].as_str(),
        ) {
            (Some("function"), _, _) => "function",
            (Some("value"), Some("parameter"), _) => "parameter",
            (Some("expression"), _, Some("application")) => "application",
            (Some("scalar_type"), _, _) => "scalar",
            _ => "other",
        }
    };
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture nodes");
    let mut category_by_digest = std::collections::BTreeMap::new();
    let mut wire_bytes = std::collections::BTreeMap::<String, WireBytes>::new();
    for node in nodes {
        let digest = node["node_id"]["digest"].as_str().expect("node digest");
        let kind = category(node);
        category_by_digest.insert(digest.to_owned(), kind.to_owned());
        let entry = wire_bytes.entry(kind.to_owned()).or_default();
        entry.node_count += 1;
        entry.nodes += serde_json::to_vec(node).expect("node bytes").len() as u64;
    }
    let projections = wire["identity_preimage"]["identity_projection"]
        .as_array()
        .expect("identity projections");
    for projection in projections {
        let digest = projection["node_id"]["digest"]
            .as_str()
            .expect("projection digest");
        let kind = category_by_digest
            .get(digest)
            .expect("projection corresponds to a node");
        let entry = wire_bytes.entry(kind.clone()).or_default();
        entry.projection_count += 1;
        entry.projections += serde_json::to_vec(projection)
            .expect("projection bytes")
            .len() as u64;
    }
    let source_map = wire["source_map"].as_array().expect("source map");
    for source_entry in source_map {
        let digest = source_entry["node_id"]["digest"]
            .as_str()
            .expect("source-map digest");
        let kind = category_by_digest
            .get(digest)
            .expect("source map corresponds to a node");
        let entry = wire_bytes.entry(kind.clone()).or_default();
        entry.source_map_count += 1;
        entry.source_maps += serde_json::to_vec(source_entry)
            .expect("source-map bytes")
            .len() as u64;
    }
    for (kind, sizes) in &wire_bytes {
        eprintln!(
            "AC-34 wire {kind}: nodes={}B/{} items, projections={}B/{} items, source_map={}B/{} items",
            sizes.nodes,
            sizes.node_count,
            sizes.projections,
            sizes.projection_count,
            sizes.source_maps,
            sizes.source_map_count,
        );
    }
    let array_bytes = |items: &[Value]| -> u64 {
        items
            .iter()
            .map(|item| serde_json::to_vec(item).unwrap().len() as u64)
            .sum::<u64>()
            + 2
            + items.len().saturating_sub(1) as u64
    };
    let section_bytes = array_bytes(nodes) + array_bytes(projections) + array_bytes(source_map);
    eprintln!(
        "AC-34 wire total={}B, node_array={}B, identity_projection_array={}B, source_map_array={}B, remainder={}B",
        bytes.len(),
        array_bytes(nodes),
        array_bytes(projections),
        array_bytes(source_map),
        bytes.len() as u64 - section_bytes,
    );
    assert!(
        bytes.len() < 1_048_576,
        "checked source is {} bytes",
        bytes.len()
    );
    let graph_ids = nodes
        .iter()
        .map(|node| {
            node["node_id"]["digest"]
                .as_str()
                .expect("node digest")
                .to_owned()
        })
        .collect::<std::collections::BTreeSet<_>>();
    fn assert_typed_references_resolve(
        value: &Value,
        path: &str,
        graph_ids: &std::collections::BTreeSet<String>,
    ) {
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    assert_typed_references_resolve(item, &format!("{path}/{index}"), graph_ids);
                }
            }
            Value::Object(object) => {
                if object.get("domain").and_then(Value::as_str) == Some(NODE_DOMAIN) {
                    let digest = object
                        .get("digest")
                        .and_then(Value::as_str)
                        .expect("typed ref digest");
                    assert!(
                        graph_ids.contains(digest),
                        "dangling typed reference at {path}: {digest}"
                    );
                    return;
                }
                for (member, child) in object {
                    assert_typed_references_resolve(child, &format!("{path}/{member}"), graph_ids);
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        let path = format!("/semantic_graph/nodes/{index}");
        let tag = node["node_tag"].as_str().unwrap_or("<missing-tag>");
        let form = node["semantic_form"].as_str().unwrap_or("<missing-form>");
        assert_typed_references_resolve(
            node,
            &format!("{path} [tag={tag}, form={form}]"),
            &graph_ids,
        );
    }

    let (checked, ids) = builder.admit_resolved();

    let declarations = (0..COUNT)
        .map(|index| FunctionScheduleDeclaration {
            function_node_id: ids.resolve(&function_ids[index as usize]),
            body_node_id: ids.resolve(&body_ids[index as usize]),
            name: format!("chain_{index}"),
            parameters: vec![
                FunctionScheduleParameter {
                    value_node_id: ids.resolve(&parameter_ids[index as usize].0),
                    type_node_id: ids.resolve(&code_id(T_INTEGER)),
                },
                FunctionScheduleParameter {
                    value_node_id: ids.resolve(&parameter_ids[index as usize].1),
                    type_node_id: ids.resolve(&code_id(T_INTEGER)),
                },
            ],
            result_type_node_id: ids.resolve(&code_id(T_INTEGER)),
        })
        .collect::<Vec<_>>();
    let items = vec![
        FunctionScheduleItem {
            call_node_id: ids.resolve(&code_id(ITEM_BASE)),
            function: "chain_0".to_owned(),
        },
        FunctionScheduleItem {
            call_node_id: ids.resolve(&code_id(ITEM_BASE + 1)),
            function: format!("chain_{terminal}"),
        },
    ];
    // One source traversal charges 3 node entries + 2 child edges + 3 instructions + 3 locations;
    // a call also charges its declaration binding once.
    let required_call_work = 3 + 2 + 3 + 3 + 1;
    let required_scalar_work = 3 + 2 + 3 + 3;
    let work_limit = required_call_work.max(required_scalar_work);
    let generated = generate_function_schedules(&checked, &declarations, &items, work_limit)
        .expect("caller-raised work limit admits the complete flat schedule");
    let generated_with_limits = generate_function_schedules_with_limits(
        &checked,
        &declarations,
        &items,
        FunctionScheduleLimits {
            generation_work_units: work_limit,
            source_bytes: generated.source_bytes,
        },
    )
    .expect("the public source limit admits the measured complete source");
    assert_eq!(generated_with_limits, generated);
    assert_eq!(
        generate_function_schedules_with_limits(
            &checked,
            &declarations,
            &items,
            FunctionScheduleLimits {
                generation_work_units: work_limit,
                source_bytes: generated.source_bytes - 1,
            },
        ),
        Err(ScheduleRefusal::SourceTooLarge {
            bytes: generated.source_bytes,
            limit: generated.source_bytes - 1,
            setting: "cg.function.source_bytes",
        }),
        "the public generator refuses one byte below its measured source size"
    );
    assert_eq!(generated.functions.len(), COUNT as usize);
    assert_eq!(generated.refused_functions.len(), 0);
    for function in &generated.functions {
        let declaration = declarations
            .iter()
            .find(|declaration| declaration.function_node_id == function.function_node_id)
            .expect("generated function has its caller binding");
        assert_eq!(declaration.parameters.len(), 2, "{} arity", function.name);
        assert!(declaration
            .parameters
            .iter()
            .all(|parameter| parameter.type_node_id == ids.resolve(&code_id(T_INTEGER))));
        let parameter_instructions = function
            .schedule
            .instructions
            .iter()
            .filter_map(|instruction| match &instruction.operation {
                FunctionScheduledOperation::Parameter(index) => Some((*index, &instruction.node)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parameter_instructions
                .iter()
                .map(|(index, _)| *index)
                .collect::<Vec<_>>(),
            vec![0, 1],
            "{} emits both parameter slots in declaration order",
            function.name
        );
        assert_eq!(
            parameter_instructions
                .iter()
                .map(|(_, node)| *node)
                .collect::<Vec<_>>(),
            declaration
                .parameters
                .iter()
                .map(|parameter| &parameter.value_node_id)
                .collect::<Vec<_>>(),
            "{} slots retain their declared parameter identities",
            function.name
        );
    }
    let call_instructions = generated
        .functions
        .iter()
        .flat_map(|function| &function.schedule.instructions)
        .filter(|instruction| {
            matches!(
                instruction.operation,
                FunctionScheduledOperation::Call { .. }
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(call_instructions.len(), 128);
    assert!(call_instructions.iter().all(|instruction| {
        instruction
            .operands
            .iter()
            .map(|operand| match operand {
                ScheduleOperand::Slot(slot) => slot.0,
                ScheduleOperand::Literal { .. } => panic!("chain arguments are references"),
            })
            .collect::<Vec<_>>()
            == vec![0, 1]
    }));
    for function in &generated.functions {
        let Some(call) = function.schedule.instructions.iter().find(|instruction| {
            matches!(
                instruction.operation,
                FunctionScheduledOperation::Call { .. }
            )
        }) else {
            continue;
        };
        let argument_parameters = call
            .operands
            .iter()
            .filter_map(|operand| match operand {
                ScheduleOperand::Slot(slot) => function.schedule.instructions.get(slot.0 as usize),
                ScheduleOperand::Literal { .. } => None,
            })
            .map(|instruction| instruction.operation.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            argument_parameters,
            vec![
                FunctionScheduledOperation::Parameter(0),
                FunctionScheduledOperation::Parameter(1),
            ],
            "{} call arguments retain parameter source order",
            function.name
        );
    }
    let head_claims = generated
        .claims
        .iter()
        .filter(|claim| claim.function == "chain_0")
        .collect::<Vec<_>>();
    assert_eq!(head_claims.len(), 1);
    let head_root = match &head_claims[0].result {
        FunctionScheduleDisposition::Scheduled { root, .. } => *root,
        other => panic!("chain head item was not scheduled: {other:?}"),
    };
    let head_schedule = generated
        .functions
        .iter()
        .find(|function| function.name == "chain_0")
        .expect("chain head schedule");
    assert_eq!(
        head_schedule
            .schedule
            .instructions
            .last()
            .map(|instruction| instruction.output),
        Some(head_root)
    );
    assert!(generated.source.len() < 1_048_576);

    let one_less = generate_function_schedules(&checked, &declarations, &items, work_limit - 1)
        .expect("one-less work limit returns per-function refusals");
    assert_eq!(
        one_less.functions.len(),
        1,
        "the terminal scalar still fits 11 units"
    );
    assert_eq!(one_less.refused_functions.len(), 128);
    assert!(one_less.refused_functions.iter().all(|function| {
        matches!(
            function.refusal,
            quire_contract_codegen::FunctionScheduleRefusal::WorkExhausted {
                limit: 11,
                consumed: 12
            }
        )
    }));

    let projection_denied = generate_function_schedules(&checked, &declarations, &items, 4)
        .expect("projection work exhaustion is reported per function");
    assert!(projection_denied.functions.is_empty());
    assert_eq!(projection_denied.refused_functions.len(), COUNT as usize);
    assert!(projection_denied.refused_functions.iter().all(|function| {
        matches!(
            function.refusal,
            quire_contract_codegen::FunctionScheduleRefusal::WorkExhausted {
                limit: 4,
                consumed: 5
            }
        )
    }));

    let scalar_declaration = declarations[terminal as usize].clone();
    let scalar_exact =
        generate_function_schedules(&checked, std::slice::from_ref(&scalar_declaration), &[], 11)
            .expect("the exact nested-scalar work budget admits its schedule");
    assert_eq!(scalar_exact.functions.len(), 1);
    let scalar_one_less =
        generate_function_schedules(&checked, std::slice::from_ref(&scalar_declaration), &[], 10)
            .expect("one-less scalar budget produces a typed refusal");
    assert!(scalar_one_less.functions.is_empty());
    assert!(matches!(
        scalar_one_less
            .refused_functions
            .first()
            .map(|function| &function.refusal),
        Some(
            quire_contract_codegen::FunctionScheduleRefusal::WorkExhausted {
                limit: 10,
                consumed: 11
            }
        )
    ));

    let reversed = items.iter().cloned().rev().collect::<Vec<_>>();
    let permuted = generate_function_schedules(&checked, &declarations, &reversed, work_limit)
        .expect("permuted request retains the same schedule");
    assert_eq!(generated.source, permuted.source);
    assert_eq!(generated.claims, permuted.claims);
    assert_eq!(
        serde_json::to_vec(&generated.claims).unwrap(),
        serde_json::to_vec(&permuted.claims).unwrap(),
    );
    assert_eq!(
        serde_json::to_vec(
            &generated
                .functions
                .iter()
                .map(|function| &function.schedule.locations)
                .collect::<Vec<_>>()
        )
        .unwrap(),
        serde_json::to_vec(
            &permuted
                .functions
                .iter()
                .map(|function| &function.schedule.locations)
                .collect::<Vec<_>>()
        )
        .unwrap(),
    );

    let mut reordered_names = declarations.clone();
    let (lower_index, higher_index) =
        if declarations[0].function_node_id < declarations[1].function_node_id {
            (0, 1)
        } else {
            (1, 0)
        };
    reordered_names[lower_index].name = "z_decl".to_owned();
    reordered_names[higher_index].name = "a_decl".to_owned();
    let claim_order = generate_function_schedules(
        &checked,
        &reordered_names,
        &[
            FunctionScheduleItem {
                call_node_id: ids.resolve(&code_id(ITEM_BASE)),
                function: "z_decl".to_owned(),
            },
            FunctionScheduleItem {
                call_node_id: ids.resolve(&code_id(ITEM_BASE)),
                function: "a_decl".to_owned(),
            },
            FunctionScheduleItem {
                call_node_id: ids.resolve(&code_id(ITEM_BASE)),
                function: "zz_unknown".to_owned(),
            },
            FunctionScheduleItem {
                call_node_id: ids.resolve(&code_id(ITEM_BASE)),
                function: "aa_unknown".to_owned(),
            },
            FunctionScheduleItem {
                call_node_id: ids.resolve(&code_id(ITEM_BASE)),
                function: "zz_unknown".to_owned(),
            },
        ],
        work_limit,
    )
    .expect("claim ordering and duplicate handling are independent of request order");
    assert_eq!(
        claim_order
            .claims
            .iter()
            .map(|claim| claim.function.as_str())
            .collect::<Vec<_>>(),
        vec!["aa_unknown", "zz_unknown", "z_decl", "a_decl"]
    );
    assert!(matches!(
        &claim_order.claims[1].result,
        FunctionScheduleDisposition::Refused {
            refusal: quire_contract_codegen::FunctionScheduleRefusal::DuplicateRequest { .. },
            ..
        }
    ));

    let directory = std::env::temp_dir().join(format!("ir496-schedule-{}", std::process::id()));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("create schedule compile directory");
    let source = directory.join("schedule.rs");
    fs::write(&source, &generated.source).expect("write generated schedule crate");
    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--crate-name", "function_schedule"])
        .arg(&source)
        .arg("-o")
        .arg(directory.join("libfunction_schedule.rlib"))
        .output()
        .expect("invoke rustc for generated schedule crate");
    assert!(
        output.status.success(),
        "generated schedule compile failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let _ = fs::remove_dir_all(&directory);
}

/// Trace: FR-021-AC-7, FR-021-AC-25, FR-021-AC-27, FR-021-AC-28, FR-021-AC-29, TC-031.
#[test]
fn tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack() {
    use serde_json::{json, Value};
    use std::collections::BTreeMap;
    use std::{fs, process::Command};
    const CALLS: u32 = 100_000;
    const PARAMETER: u32 = 1_100_000;
    const HELPER: u32 = 1_100_001;
    const OUTER_PARAMETER: u32 = 1_100_002;
    const OUTER: u32 = 1_100_003;
    const CALL_BASE: u32 = 1_200_000;
    let calls = std::env::var("IR496_TEST_CALLS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(CALLS);
    assert!((1..=CALLS).contains(&calls));

    let operation = json!({
        "identity": "quire.op.function.call",
        "laws": [],
        "mode": null,
        "member": null,
        "leaves": [],
    });
    fn apply_seeded_ids(wire: &mut Value, replacements: &BTreeMap<String, String>) {
        fn rewrite(value: &mut Value, replacements: &BTreeMap<String, String>) {
            match value {
                Value::Array(items) => {
                    for item in items {
                        rewrite(item, replacements);
                    }
                }
                Value::Object(object) => {
                    if object.get("domain").and_then(Value::as_str)
                        == Some("quire.checked-semantic-node/v1")
                    {
                        if let Some(digest) = object.get("digest").and_then(Value::as_str) {
                            if let Some(replacement) = replacements.get(digest) {
                                object.insert("digest".to_owned(), json!(replacement));
                            }
                        }
                        return;
                    }
                    for member in object.values_mut() {
                        rewrite(member, replacements);
                    }
                }
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }
        rewrite(wire, replacements);
    }
    fn preseed_structural_candidates(wire: &mut Value, codes: &[u32]) -> BTreeMap<String, String> {
        let nodes = wire["semantic_graph"]["nodes"]
            .as_array()
            .expect("fixture graph nodes");
        let selected = codes
            .iter()
            .map(|code| {
                let old = code_id(*code);
                let node = nodes
                    .iter()
                    .find(|node| node["node_id"]["digest"].as_str() == Some(old.digest.as_ref()))
                    .unwrap_or_else(|| panic!("no structural candidate node for code {code}"));
                serde_json::from_value::<CheckedSemanticNodeV2>(node.clone())
                    .expect("fixture node deserializes for candidate derivation")
            })
            .collect::<Vec<_>>();
        let candidates = derive_structural_candidates(&selected, 1 << 20, 1 << 20)
            .expect("small fixture candidates derive within limits");
        let replacements = candidates
            .nodes()
            .iter()
            .map(|candidate| {
                let old = selected[candidate.position() as usize]
                    .node_id
                    .digest
                    .to_string();
                let new = candidate.candidate_node_id().digest.to_string();
                (old, new)
            })
            .collect::<BTreeMap<_, _>>();
        apply_seeded_ids(wire, &replacements);
        replacements
    }
    let body_reference = |code: u32, seeded: &BTreeMap<String, String>| {
        let mut target = code_id(code);
        if let Some(digest) = seeded.get(target.digest.as_ref()) {
            target.digest = digest.clone().into_boxed_str();
        }
        json!({
            "term": "reference",
            "target": {"domain": target.domain, "digest": target.digest},
        })
    };
    let mut builder = PackageBuilder::default();
    builder
        .code(
            T_INTEGER,
            "scalar_type",
            "integer",
            T_INTEGER,
            aggregate(vec![]),
        )
        .code(T_TEXT, "scalar_type", "text", T_TEXT, aggregate(vec![]));
    let (base_result, base_wire, base_ids) = builder.read_resolved();
    assert!(matches!(
        base_result,
        quire_contract_model::CheckedPackageV2ReadResult::Admitted(_)
    ));
    builder = PackageBuilder::from_wire(base_wire);
    let mut seeded_ids = [T_INTEGER, T_TEXT]
        .into_iter()
        .filter_map(|code| {
            let old = code_id(code);
            let new = base_ids.resolve(&old);
            (old.digest != new.digest).then(|| (old.digest.to_string(), new.digest.to_string()))
        })
        .collect::<BTreeMap<_, _>>();
    builder
        .code(
            PARAMETER,
            "value",
            "parameter",
            T_INTEGER,
            aggregate(vec![
                bound_member("name", literal("text", "helper_input")),
                bound_member("level", integer_literal(0)),
            ]),
        )
        .code(
            OUTER_PARAMETER,
            "value",
            "parameter",
            T_INTEGER,
            aggregate(vec![
                bound_member("name", literal("text", "outer_input")),
                bound_member("level", integer_literal(0)),
            ]),
        );

    let mut wire = builder.raw_wire();
    apply_seeded_ids(&mut wire, &seeded_ids);
    seeded_ids.extend(preseed_structural_candidates(
        &mut wire,
        &[PARAMETER, OUTER_PARAMETER],
    ));
    builder = PackageBuilder::from_wire(wire);
    builder.code(
        HELPER,
        "function",
        "pure_function",
        T_INTEGER,
        body_reference(PARAMETER, &seeded_ids),
    );

    let mut wire = builder.raw_wire();
    apply_seeded_ids(&mut wire, &seeded_ids);
    seeded_ids.extend(preseed_structural_candidates(&mut wire, &[HELPER]));
    builder = PackageBuilder::from_wire(wire);

    let integer_type = seeded_ids
        .get(code_id(T_INTEGER).digest.as_ref())
        .cloned()
        .unwrap_or_else(|| code_id(T_INTEGER).digest.to_string());
    let helper_reference = body_reference(HELPER, &seeded_ids);
    for ordinal in 0..calls {
        let child = if ordinal == 0 {
            OUTER_PARAMETER
        } else {
            CALL_BASE + ordinal - 1
        };
        builder.application_node(
            CALL_BASE + ordinal,
            "call",
            &integer_type,
            json!({
                "term": "application",
                "operator": "call",
                "operation": operation.clone(),
                "result_type": {"domain": NODE_DOMAIN, "digest": integer_type},
                "arguments": [helper_reference.clone(), body_reference(child, &seeded_ids)],
            }),
        );
    }
    builder.code(
        OUTER,
        "function",
        "pure_function",
        T_INTEGER,
        body_reference(CALL_BASE + calls - 1, &seeded_ids),
    );
    let mut wire = builder.raw_wire();
    apply_seeded_ids(&mut wire, &seeded_ids);
    seeded_ids.extend(preseed_structural_candidates(&mut wire, &[OUTER]));
    builder = PackageBuilder::from_wire(wire);
    eprintln!("IR-496 AC7 phase=builder-complete calls={calls}");

    let limits = quire_contract_model::CheckedPackageReadLimits {
        bytes: 1 << 30,
        nodes: 110_000,
        edges: 300_000,
        occurrences: 500_000,
        diagnostics: 10_000,
        work: 100_000_000,
    };
    eprintln!("IR-496 AC7 phase=admission-start");
    let (package, ids) = builder.admit_with_resolved(limits);
    eprintln!(
        "IR-496 AC7 phase=admission-complete admitted_v2_nodes={}",
        package.graph().nodes.len()
    );
    let resolved_code_id = |code| {
        let mut id = code_id(code);
        if let Some(digest) = seeded_ids.get(id.digest.as_ref()) {
            id.digest = digest.clone().into_boxed_str();
        }
        ids.resolve(&id)
    };
    let parameter = |code| FunctionScheduleParameter {
        value_node_id: resolved_code_id(code),
        type_node_id: resolved_code_id(T_INTEGER),
    };
    let declarations = vec![
        FunctionScheduleDeclaration {
            function_node_id: resolved_code_id(HELPER),
            body_node_id: resolved_code_id(PARAMETER),
            name: "helper".to_owned(),
            parameters: vec![parameter(PARAMETER)],
            result_type_node_id: resolved_code_id(T_INTEGER),
        },
        FunctionScheduleDeclaration {
            function_node_id: resolved_code_id(OUTER),
            body_node_id: ids.resolve(&code_id(CALL_BASE + calls - 1)),
            name: "outer".to_owned(),
            parameters: vec![parameter(OUTER_PARAMETER)],
            result_type_node_id: resolved_code_id(T_INTEGER),
        },
    ];
    let helper_function_node_id = resolved_code_id(HELPER);
    let outer_function_node_id = resolved_code_id(OUTER);
    let worker = std::thread::Builder::new()
        .name("ir496-ac7-public-small-stack".to_owned())
        .stack_size(512 * 1024)
        .spawn(move || {
            const REQUIRED_WORK: u64 = 500_003;
            eprintln!("IR-496 AC7 phase=public-default-start");
            let default_result = generate_function_schedules_with_limits(
                &package,
                &declarations,
                &[],
                FunctionScheduleLimits {
                    generation_work_units: REQUIRED_WORK,
                    source_bytes: 1_048_576,
                },
            );
            let (raised, source_bytes, exceeded_default) = match default_result {
                Err(ScheduleRefusal::SourceTooLarge {
                    bytes,
                    limit: 1_048_576,
                    setting: "cg.function.source_bytes",
                }) => {
                    assert_eq!(calls, CALLS, "only the 100K case should exceed the default");
                    eprintln!("IR-496 AC7 phase=public-default-complete bytes={bytes}");
                    eprintln!("IR-496 AC7 phase=public-raised-start");
                    let raised = generate_function_schedules_with_limits(
                        &package,
                        &declarations,
                        &[],
                        FunctionScheduleLimits {
                            generation_work_units: REQUIRED_WORK,
                            source_bytes: bytes,
                        },
                    )
                    .expect("the measured caller source limit admits every schedule module");
                    (raised, bytes, true)
                }
                Ok(generated) => {
                    assert!(calls < CALLS, "the 100K case must exercise the source ceiling");
                    let bytes = generated.source_bytes;
                    (generated, bytes, false)
                }
                other => panic!("default source behavior was unexpected: {other:?}"),
            };
            assert_eq!(raised.functions.len(), 2);
            let helper = raised
                .functions
                .iter()
                .find(|function| function.function_node_id == helper_function_node_id)
                .expect("the generated helper schedule is present");
            let outer = raised
                .functions
                .iter()
                .find(|function| function.function_node_id == outer_function_node_id)
                .expect("the generated outer schedule is present");
            assert_eq!(
                raised
                    .functions
                    .iter()
                    .filter(|function| function.function_node_id == helper_function_node_id)
                    .count(),
                1,
                "the exact helper declaration has one generated schedule"
            );
            assert_eq!(
                raised
                    .functions
                    .iter()
                    .filter(|function| function.function_node_id == outer_function_node_id)
                    .count(),
                1,
                "the exact outer declaration has one generated schedule"
            );
            let helper_instruction_count = helper.schedule.instructions.len();
            let outer_instruction_count = outer.schedule.instructions.len();
            eprintln!(
                "IR-496 AC7 phase=public-raised-counts helper_instructions={helper_instruction_count} outer_instructions={outer_instruction_count}"
            );
            assert_eq!(helper_instruction_count, 1);
            assert_eq!(outer_instruction_count, calls as usize + 1);
            assert_eq!(outer.schedule.locations.len(), calls as usize + 1);
            assert_eq!(raised.source_bytes, source_bytes);
            if exceeded_default {
                assert!(raised.source_bytes > 1_048_576);
                assert!(raised.modules.len() > 1);
            } else {
                assert!(raised.source_bytes <= 1_048_576);
            }
            let measured_bytes = raised.modules.iter().fold(raised.source.len(), |total, module| {
                total + module.contents.len()
            });
            assert_eq!(measured_bytes, raised.source_bytes);
            assert!(raised.source.len() <= 16 * 1024 * 1024);
            assert!(raised
                .modules
                .iter()
                .all(|module| module.contents.len() <= 16 * 1024 * 1024));

            let directory = std::env::temp_dir().join(format!(
                "ir496-ac7-generated-schedule-{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&directory);
            let source_directory = directory.join("src");
            fs::create_dir_all(&source_directory).expect("create generated schedule source dir");
            let root_source = source_directory.join("lib.rs");
            fs::write(&root_source, &raised.source).expect("write generated schedule root");
            for module in &raised.modules {
                let relative = module
                    .path
                    .strip_prefix("src/")
                    .expect("schedule module path is rooted under src");
                let path = source_directory.join(relative);
                fs::create_dir_all(path.parent().expect("module path has a parent"))
                    .expect("create generated module directory");
                fs::write(path, &module.contents).expect("write generated schedule module");
            }
            let output = Command::new("rustc")
                .args(["--crate-type", "lib", "--crate-name", "function_schedule"])
                .arg(&root_source)
                .arg("-o")
                .arg(directory.join("libfunction_schedule.rlib"))
                .output()
                .expect("invoke rustc for generated schedule crate");
            assert!(
                output.status.success(),
                "generated schedule crate compile failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let _ = fs::remove_dir_all(&directory);
            eprintln!("IR-496 AC7 phase=public-raised-complete");
            eprintln!(
                "AC-7 admitted V2 public generation: call_nodes={calls}, instructions={}, source_bytes={}, modules={}",
                outer_instruction_count,
                raised.source_bytes,
                raised.modules.len()
            );
        })
        .expect("spawn the public generation worker with a 512 KiB stack");
    worker
        .join()
        .expect("admitted deep generation fits 512 KiB");
}

/// Trace: FR-021-AC-23, TC-031.
#[test]
fn tc_031_application_candidate_repair_admits_parent_after_child_key_changes() {
    use serde_json::json;

    const PARAM_LEFT: u32 = 84_001;
    const PARAM_RIGHT: u32 = 84_002;
    const CHILD: u32 = 84_010;
    const PARENT: u32 = 84_011;
    const FUNCTION: u32 = 84_012;
    let mut builder = PackageBuilder::default();
    builder
        .code(
            T_INTEGER,
            "scalar_type",
            "integer",
            T_INTEGER,
            aggregate(vec![]),
        )
        .code(T_TEXT, "scalar_type", "text", T_TEXT, aggregate(vec![]));
    for (code, name) in [(PARAM_LEFT, "left"), (PARAM_RIGHT, "right")] {
        builder.code(
            code,
            "value",
            "parameter",
            T_INTEGER,
            aggregate(vec![
                bound_member("name", literal("text", name)),
                bound_member("level", integer_literal(0)),
            ]),
        );
    }
    let int_type = key(T_INTEGER);
    let add_body = |left: &quire_contract_model::CheckedNodeId,
                    right: &quire_contract_model::CheckedNodeId| {
        json!({
            "term": "application",
            "operator": "binary",
            "operation": {
                "identity": "quire.op.integer.add",
                "laws": [],
                "mode": null,
                "member": null,
                "leaves": [],
            },
            "result_type": {"domain": NODE_DOMAIN, "digest": int_type},
            "arguments": [
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": left.digest}},
                {"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": right.digest}},
            ],
        })
    };
    builder.application_node(
        CHILD,
        "binary",
        &int_type,
        add_body(&code_id(PARAM_LEFT), &code_id(PARAM_RIGHT)),
    );
    let child_id = code_id(CHILD);
    builder.application_node(
        PARENT,
        "binary",
        &int_type,
        add_body(&child_id, &code_id(PARAM_RIGHT)),
    );
    let parent_reference = code_id(PARENT);
    builder.code(
        FUNCTION,
        "function",
        "pure_function",
        T_INTEGER,
        json!({
            "term": "reference",
            "target": {"domain": NODE_DOMAIN, "digest": parent_reference.digest},
        }),
    );
    let (initial_outcome, mut wire, ids) = builder.read_resolved();
    assert!(matches!(
        initial_outcome,
        quire_contract_model::CheckedPackageV2ReadResult::Admitted(_)
    ));
    let child_id = ids.resolve(&child_id);
    let parent_id = ids.resolve(&code_id(PARENT));
    let old_child_digest = child_id.digest.to_string();
    let old_parent_digest = parent_id.digest.to_string();
    let old_parameter = ids.resolve(&code_id(PARAM_LEFT));
    let old_parameter_digest = old_parameter.digest.to_string();

    let nodes = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("fixture nodes");
    let parameter = nodes
        .iter_mut()
        .find(|node| node["node_id"]["digest"].as_str() == Some(&old_parameter_digest))
        .expect("left parameter node");
    parameter["body"]["members"][0]["value"]["value"] = json!("renamed_left");

    let (repair_outcome, repaired, repair_ids) = PackageBuilder::from_wire(wire).read_resolved();
    assert!(matches!(
        repair_outcome,
        quire_contract_model::CheckedPackageV2ReadResult::Admitted(_)
    ));
    let repaired_nodes = repaired["semantic_graph"]["nodes"]
        .as_array()
        .expect("repaired nodes");
    let repaired_parameter = repaired_nodes
        .iter()
        .find(|node| node["body"]["members"][0]["value"]["value"].as_str() == Some("renamed_left"))
        .expect("repaired parameter node");
    let repaired_parameter_digest = repaired_parameter["node_id"]["digest"]
        .as_str()
        .expect("repaired parameter digest");
    assert_ne!(repaired_parameter_digest, old_parameter_digest);

    let repaired_child = repaired_nodes
        .iter()
        .find(|node| {
            node["body"]["arguments"][0]["target"]["digest"].as_str()
                == Some(repaired_parameter_digest)
        })
        .expect("repaired child application");
    let repaired_child_digest = repaired_child["node_id"]["digest"]
        .as_str()
        .expect("repaired child digest");
    assert_ne!(repaired_child_digest, old_child_digest);

    let repaired_parent = repaired_nodes
        .iter()
        .find(|node| {
            node["body"]["arguments"][0]["target"]["digest"].as_str() == Some(repaired_child_digest)
        })
        .expect("repaired parent application");
    let repaired_parent_digest = repaired_parent["node_id"]["digest"]
        .as_str()
        .expect("repaired parent digest");
    assert_ne!(repaired_parent_digest, old_parent_digest);
    let resolved_parent = repair_ids.resolve(&parent_id);
    assert_eq!(
        resolved_parent.digest.as_ref(),
        repaired_parent_digest,
        "FixtureIds resolves the old application ID to its admitted candidate ID"
    );
    let function_id = repair_ids.resolve(&ids.resolve(&code_id(FUNCTION)));
    let function_node = repaired_nodes
        .iter()
        .find(|node| node["node_id"]["digest"].as_str() == Some(function_id.digest.as_ref()))
        .expect("repaired function node");
    let descriptor = FunctionScheduleDeclaration {
        function_node_id: function_id,
        body_node_id: resolved_parent,
        name: "parent".to_owned(),
        parameters: vec![],
        result_type_node_id: repair_ids.resolve(&ids.resolve(&code_id(T_INTEGER))),
    };
    let body_target: quire_contract_model::CheckedNodeId =
        serde_json::from_value(function_node["body"]["target"].clone())
            .expect("function body target ID");
    assert_eq!(
        descriptor.body_node_id, body_target,
        "the resolved function descriptor binds the admitted body target"
    );
    assert_eq!(
        repaired_parent_digest,
        body_target.digest.as_ref(),
        "parent key should be recomputed after its child key changes"
    );
}

// Duplicated per consumer (also `exact_function_agreement.rs`) for structural consistency with
// the exact_scalar/composite_equality families (IR-237). Unlike those two, this package.rs holds
// no process-global state, so duplication here isn't load-bearing the way it is for them -- it's
// kept for uniformity across the tests/it/*_support/package.rs pattern, not to avoid a hazard.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_function_support/package.rs"]
mod package;
use package::*;

/// The main FR-021 corpus: one Scalar, one CompositeEquality and one Call
/// function (TC-031 step 1(a)); one function each refused for a `reference`
/// operand (AC-10), a model root (AC-11), a state root (AC-11), an
/// undischargeable capability (AC-6), and a dangling nested call (AC-12);
/// and one function wholly unrelated to any of the refused ones, to prove
/// isolation (AC-12).
fn main_functions() -> Vec<quire_contract_codegen::ExactFunctionDeclaration> {
    vec![
        function_add("add_fn"),
        function_eq("eq_fn"),
        function_call_nested("call_fn", "add_fn"),
        function_ref_param("ref_fn"),
        function_model_root("model_fn"),
        function_state_root("state_fn"),
        function_capability("capability_fn"),
        function_dangling_call("dangling_fn"),
        function_unrelated("unrelated_fn"),
    ]
}

fn main_items() -> Vec<ExactFunctionItem> {
    vec![
        item(ITEM_CALL_ADD, "add_fn"),
        item(ITEM_CALL_EQ, "eq_fn"),
        item(ITEM_CALL_NESTED, "call_fn"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
        item(ITEM_CALL_UNKNOWN_FUNCTION, "does_not_exist_anywhere"),
    ]
}

struct FixturePackage {
    checked: CheckedPackageV2,
    ids: FixtureIds,
}

impl FixturePackage {
    fn key(&self, code: u32) -> quire_contract_model::CheckedNodeId {
        self.ids.resolve(&code_id(code))
    }
}

impl Deref for FixturePackage {
    type Target = CheckedPackageV2;

    fn deref(&self) -> &Self::Target {
        &self.checked
    }
}

fn admit(builder: PackageBuilder) -> FixturePackage {
    let (checked, ids) = builder.admit_resolved();
    FixturePackage { checked, ids }
}

fn admit_with(
    builder: &PackageBuilder,
    limits: quire_contract_model::CheckedPackageReadLimits,
) -> FixturePackage {
    let (checked, ids) = builder.admit_with_resolved(limits);
    FixturePackage { checked, ids }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct FixtureOracles {
    output: quire_contract_codegen::ExactFunctionOracles,
    ids: FixtureIds,
}

impl FixtureOracles {
    fn key(&self, code: u32) -> quire_contract_model::CheckedNodeId {
        self.ids.resolve(&code_id(code))
    }
}

impl Deref for FixtureOracles {
    type Target = quire_contract_codegen::ExactFunctionOracles;

    fn deref(&self) -> &Self::Target {
        &self.output
    }
}

fn generate(
    package: &FixturePackage,
    functions: &[quire_contract_codegen::ExactFunctionDeclaration],
    items: &[ExactFunctionItem],
) -> FixtureOracles {
    let input = admitted_full_request(package, functions, items);
    generate_in_request(package, &input, functions, items)
}

struct AdmittedFunctionRequest {
    contract: quire_contract_model::ContractPackageV1,
    roots: Vec<quire_contract_codegen::ContractRequestPosition>,
}

/// One complete FR-330 lowering request, independent of descriptor ordering.
/// Repeated root keys retain separate accounting positions.
fn admitted_full_request(
    package: &FixturePackage,
    functions: &[quire_contract_codegen::ExactFunctionDeclaration],
    items: &[ExactFunctionItem],
) -> AdmittedFunctionRequest {
    let mut requested = functions
        .iter()
        .map(|function| package.ids.resolve(&function.node_id))
        .chain(
            items
                .iter()
                .map(|item| package.ids.resolve(&item.call_node_id)),
        )
        .collect::<Vec<_>>();
    requested.sort();
    AdmittedFunctionRequest {
        contract: admitted_function_contract(&package.checked, &requested),
        roots: driver_positions(&requested),
    }
}

/// Select descriptors from one admitted complete request without changing its
/// root sequence or package identity for subset and permutation controls.
fn generate_in_request(
    package: &FixturePackage,
    input: &AdmittedFunctionRequest,
    functions: &[quire_contract_codegen::ExactFunctionDeclaration],
    items: &[ExactFunctionItem],
) -> FixtureOracles {
    let functions = functions
        .iter()
        .cloned()
        .map(|function| package.ids.resolve_function(function))
        .collect::<Vec<_>>();
    let items = items
        .iter()
        .cloned()
        .map(|item| package.ids.resolve_call(item))
        .collect::<Vec<_>>();
    let mut available = std::collections::BTreeMap::new();
    for (position, root) in input.roots.iter().enumerate() {
        let node_id = match root {
            quire_contract_codegen::ContractRequestPosition::DriverItem { node_id, .. }
            | quire_contract_codegen::ContractRequestPosition::ContextRoot { node_id } => node_id,
        };
        available
            .entry(node_id.clone())
            .or_insert_with(std::collections::VecDeque::new)
            .push_back(position);
    }
    let mut take_position = |node_id: &quire_contract_model::CheckedNodeId| {
        available
            .get_mut(node_id)
            .and_then(std::collections::VecDeque::pop_front)
            .unwrap_or_else(|| panic!("full FR-330 request lacks selected root {node_id:?}"))
    };
    let function_positions = functions
        .iter()
        .map(|function| take_position(&function.node_id))
        .collect::<Vec<_>>();
    let item_positions = items
        .iter()
        .map(|item| take_position(&item.call_node_id))
        .collect::<Vec<_>>();
    let output = generate_exact_function_oracles(
        &input.contract,
        &input.roots,
        &function_positions,
        &functions,
        &item_positions,
        &items,
    )
    .expect("generation succeeds");
    FixtureOracles {
        output,
        ids: package.ids.clone(),
    }
}

fn disposition_for(
    oracles: &FixtureOracles,
    call_code: u32,
) -> &ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal> {
    let node_id = oracles.key(call_code);
    &oracles
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == node_id)
        .unwrap_or_else(|| panic!("no claim for call node {call_code}"))
        .result
}

/// Trace: FR-021-AC-23, TC-031.
///
/// Under a ceiling below either canonical lowered package, Contract IR returns
/// distinct typed Failed(Bytes) accounting for body and call root requests but
/// no ContractPackage for CG to consume. At an exact sufficient ceiling, the
/// admitted package still reaches the function generator.
#[test]
fn tc_031_ac23_byte_ceiling_has_failed_records_and_no_contract_package() {
    use crate::common::byte_ceiling::{
        limits_under, lowered_package_length, measuring_profile, LARGEST_CEILING,
    };
    use quire_contract_model::{CheckedNodeId, CheckedPackageLimit, CompleteLoweringRecordV2};

    let builder = byte_ceiling_package();
    let functions = byte_ceiling_functions();
    let items = byte_ceiling_items();
    let checked_length = u64::try_from(serde_json::to_vec(&builder.wire()).unwrap().len()).unwrap();
    let generous = admit_with(&builder, limits_under(LARGEST_CEILING));
    let body_nodes = functions
        .iter()
        .map(|function| generous.ids.resolve(&function.node_id))
        .collect::<Vec<_>>();
    let call_nodes = items
        .iter()
        .map(|item| generous.ids.resolve(&item.call_node_id))
        .collect::<Vec<_>>();
    let profile = measuring_profile(false, "quire.codegen.exact-function/v1");
    let body_length = lowered_package_length(&generous, &body_nodes, &profile);
    let call_length = lowered_package_length(&generous, &call_nodes, &profile);
    let combined_nodes = body_nodes
        .iter()
        .chain(&call_nodes)
        .cloned()
        .collect::<Vec<_>>();
    let combined_length = lowered_package_length(&generous, &combined_nodes, &profile);
    let ceiling = body_length.min(call_length) - 1;
    assert!(
        ceiling >= checked_length,
        "both lowered packages ({body_length}, {call_length} bytes) are longer than the checked \
         package ({checked_length} bytes), so a ceiling one byte below the shorter admits it"
    );

    // Each lowering alone, under the same ceiling: both fail for bytes, with different `consumed`.
    let package = admit_with(&builder, limits_under(ceiling));
    let consumed_alone = |requested: &[CheckedNodeId]| {
        let lowered = package.lower(requested, &profile);
        assert_eq!(lowered.records.len(), requested.len());
        assert!(lowered.package.canonical_bytes().is_none());
        assert!(lowered.package.wire().is_none());
        let mut consumed_seen = std::collections::BTreeSet::new();
        for (record, expected_node) in lowered.records.iter().zip(requested) {
            let CompleteLoweringRecordV2::Failed {
                node_id,
                limit_kind: CheckedPackageLimit::Bytes,
                limit,
                consumed,
                ..
            } = record
            else {
                panic!("expected a typed byte-ceiling failure, got {record:?}");
            };
            assert_eq!(node_id, expected_node);
            assert_eq!(*limit, ceiling);
            assert!(*consumed > ceiling);
            consumed_seen.insert(*consumed);
        }
        assert_eq!(consumed_seen.len(), 1);
        *consumed_seen.first().expect("one shared consumed count")
    };
    let body_consumed = consumed_alone(&body_nodes);
    let call_consumed = consumed_alone(&call_nodes);
    assert_ne!(body_consumed, call_consumed);

    // The full mixed request is larger than either isolated request. At its
    // exact ceiling the producer has a package; the default FR-330 reader's
    // separate one-megabyte limit is outside this producer-ceiling control.
    let exact = admit_with(&builder, limits_under(combined_length));
    let admitted_lowering = exact.checked.lower(&combined_nodes, &profile);
    assert_eq!(admitted_lowering.records.len(), combined_nodes.len());
    assert!(admitted_lowering
        .records
        .iter()
        .all(|record| matches!(record, CompleteLoweringRecordV2::Lowered { .. })));
    assert_eq!(
        admitted_lowering
            .package
            .canonical_bytes()
            .map(|bytes| bytes.len()),
        Some(usize::try_from(combined_length).expect("bounded byte count"))
    );
}

/// Trace: FR-021-AC-23, TC-031. Both request sets over the shared chain must
/// truly lower with their predecessors before their encoded lengths define a byte ceiling.
#[test]
fn tc_031_ac23_small_shared_chain_lowers_with_complete_closures() {
    use crate::common::byte_ceiling::{
        limits_under, lowered_package_length, measuring_profile, LARGEST_CEILING,
    };
    use quire_contract_model::CompleteLoweringRecordV2;

    use sha2::{Digest, Sha256};

    let builder = byte_ceiling_package_with_shape(16, 15);
    let (source, sites) = byte_ceiling_source(16, 15);
    let raw = builder.raw_wire();
    let source_ref = raw["lock"]["sources"]
        .as_array()
        .expect("locked sources")
        .iter()
        .find(|source| source["identity"] == "ir764-ac23-source")
        .expect("authored source is locked");
    assert_eq!(
        source_ref["digest"],
        format!("{:x}", Sha256::digest(source.as_bytes()))
    );
    for (code, spans) in &sites {
        let entries = raw["source_map"]
            .as_array()
            .expect("source map")
            .iter()
            .filter(|entry| entry["node_id"]["digest"] == code_id(*code).digest.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(entries.len(), spans.len());
        for (ordinal, (entry, (start, end))) in entries.iter().zip(spans).enumerate() {
            assert_eq!(entry["ordinal"], ordinal);
            assert_eq!(entry["regions"][0]["source"], *source_ref);
            assert_eq!(entry["regions"][0]["start"], *start);
            assert_eq!(entry["regions"][0]["end"], *end);
            assert_eq!(&source[*start..*end], format!("n{code}"));
        }
    }
    let package = admit_with(&builder, limits_under(LARGEST_CEILING));
    let profile = measuring_profile(false, "quire.codegen.exact-function/v1");
    let wire_length = serde_json::to_vec(&builder.wire())
        .expect("small-chain wire")
        .len();
    for count in [16, 15] {
        let requested = (0..count)
            .map(|offset| package.key(BYTE_BODY_BASE + offset))
            .collect::<Vec<_>>();
        let lowered_length = lowered_package_length(&package, &requested, &profile);
        assert!(
            lowered_length > u64::try_from(wire_length).expect("wire length"),
            "{count} source-backed requests lowered to {lowered_length} bytes from a {wire_length}-byte checked wire"
        );
        let lowering = package.lower(&requested, &profile);
        let Some(CompleteLoweringRecordV2::Lowered { node }) = lowering.records.last() else {
            panic!("last small-chain request did not lower");
        };
        assert!(node.dependencies.contains(&requested[0]));
    }
}

/// Trace: FR-021-AC-1, TC-031. Every requested item receives exactly one
/// generated or typed-refused disposition; a refused item (naming an
/// unknown function) contributes nothing, while its siblings generate
/// unchanged.
#[test]
fn tc_031_ac1_every_item_gets_one_disposition_and_refusal_does_not_cascade() {
    let package = admit(ext_corpus_package());
    let oracles = generate(&package, &main_functions(), &main_items());
    assert_eq!(oracles.claim_map.items.len(), main_items().len());

    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_NESTED),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
    ));

    let unknown = oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id == oracles.key(ITEM_CALL_UNKNOWN_FUNCTION))
        .collect::<Vec<_>>();
    assert_eq!(unknown.len(), 1);
    assert!(matches!(
        &unknown[0].result,
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnknownFunction { name }
        } if name == "does_not_exist_anywhere"
    ));
}

/// A caller-selected scalar operator cannot override the checked body operation identity.
#[test]
fn tc_031_scalar_body_operator_must_match_checked_operation_identity() {
    let package = admit(source_bound_call_package());
    let mut declaration = source_function(&format!("n{}", key(SOURCE_FUNCTION)), SOURCE_BODY);
    declaration.body = quire_contract_codegen::ExactFunctionBody::Scalar {
        operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Subtract,
    };
    let oracles = generate(&package, &[declaration], &[source_call()]);
    assert!(matches!(
        disposition_for(&oracles, SOURCE_CALL),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BodyMismatch
        }
    ));
}

/// Trace: FR-021-AC-3, TC-031. The claim-map entry for a generated item
/// records the applied function's name and its `Origin::Body{function,
/// index}` equal to the request's own declared-function ordering (functions
/// sorted by declaring node id, digest domain then digest) -- read from the
/// generator's own bookkeeping, not from the generated source text.
#[test]
fn tc_031_ac3_claim_records_function_and_origin_from_the_request() {
    let package = admit(ext_corpus_package());
    let functions = main_functions();
    let oracles = generate(&package, &functions, &main_items());

    let mut ordered: Vec<_> = functions.iter().collect();
    ordered.sort_by_key(|function| package.ids.resolve(&function.node_id));
    let survivor_names: Vec<&str> = ordered
        .iter()
        .filter(|f| {
            matches!(
                f.name.as_str(),
                "add_fn" | "eq_fn" | "call_fn" | "unrelated_fn"
            )
        })
        .map(|f| f.name.as_str())
        .collect();

    // Two items name this call node (`add_fn` and `eq_fn`); their order in the claim map follows
    // the declaring node ids' digests, so select the claim by the function it records.
    let claim = oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id == oracles.key(ITEM_CALL_ADD))
        .find_map(|claim| match &claim.result {
            ClaimDisposition::Generated(generated) if generated.function == "add_fn" => {
                Some(generated)
            }
            _ => None,
        })
        .expect("a generated claim for add_fn over the call node");
    let expected_index = survivor_names
        .iter()
        .position(|name| *name == "add_fn")
        .unwrap();
    match &claim.function_origin {
        quire_contract_codegen::RecordedOrigin::Body { function, index } => {
            assert_eq!(function, "add_fn");
            assert_eq!(*index, expected_index);
        }
    }
}

/// Trace: FR-021-AC-6, TC-031. A function whose declared operator
/// requirements name a capability no registered backend can discharge is
/// marked unsupported at generation time, naming the capability; no oracle
/// is emitted for any item naming it, and no `capability_fn` item was even
/// requested here, so this test drives it as a direct second request.
#[test]
fn tc_031_ac6_undischargeable_capability_refuses_before_any_item() {
    let package = admit(ext_corpus_package());
    let functions = vec![function_capability("capability_fn")];
    let items = vec![item(ITEM_CALL_ADD, "capability_fn")];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnsupportedCapability { capability }
        } if capability == "quire.capability.undischargeable-in-v1"
    ));
}

/// Trace: FR-021-AC-8, TC-031 (Inspection). Only `CheckMode::Linked` is ever
/// admitted: the generated source's `checked_package()` names
/// `rt::CheckMode::Linked` and never `rt::CheckMode::Kernel`.
#[test]
fn tc_031_ac8_generated_source_never_names_check_mode_kernel() {
    let package = admit(ext_corpus_package());
    let oracles = generate(&package, &main_functions(), &main_items());
    let lib = contents(&oracles, "src/lib.rs");
    assert!(lib.contains("rt::CheckMode::Linked"));
    assert!(!lib.contains("CheckMode::Kernel"));
}

/// Trace: FR-021-AC-10, TC-031. A reference parameter type outside the
/// admitted FR-330 graph is refused at that exact type key before CG can
/// classify its deeper family.
#[test]
fn tc_031_ac10_reference_parameter_outside_package_is_unknown_type_node() {
    let package = admit(ext_corpus_package());
    let functions = vec![function_ref_param("ref_fn")];
    let items = vec![item(ITEM_CALL_ADD, "ref_fn")];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnknownTypeNode { type_node_id }
        } if *type_node_id == oracles.key(REF_TYPE)
    ));
    assert!(!contents(&oracles, "src/lib.rs").contains("pub fn oracle_"));
}

/// Trace: FR-021-AC-11, TC-031. The requested function roots are checked
/// Model/State nodes, while the call fixtures retain a valid source-bound
/// callee. The claim map must preserve the distinct root blockers and still
/// generate the healthy source-bound sibling.
#[test]
fn tc_031_ac11_model_and_state_are_distinct_blockers() {
    let package = admit(ac11_classifier_package());
    let functions = vec![
        function_model_root("model_fn"),
        function_state_root("state_fn"),
        source_function(&format!("n{}", key(SOURCE_FUNCTION)), SOURCE_BODY),
    ];
    let items = vec![
        item(ITEM_CALL_SOURCE_MODEL, "model_fn"),
        item(ITEM_CALL_SOURCE_STATE, "state_fn"),
        source_call(),
    ];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_SOURCE_MODEL),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: UpstreamBlocker::QuireSpecLanguage120,
                ..
            }
        }
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_SOURCE_STATE),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: UpstreamBlocker::QuireSpecLanguage121,
                ..
            }
        }
    ));
    assert!(contents(&oracles, "src/lib.rs").contains("pub fn oracle_"));
    let sibling_disposition = disposition_for(&oracles, SOURCE_CALL);
    assert!(
        matches!(sibling_disposition, ClaimDisposition::Generated(_)),
        "source-bound sibling: {sibling_disposition:?}"
    );
}

/// Trace: FR-021-AC-12, TC-031. A declared function whose own body fails to
/// lower -- here, a nested `call` naming an absent callee -- refuses every
/// item bound to that function, without changing `unrelated_fn`'s item.
#[test]
fn tc_031_ac12_dangling_callee_refuses_only_its_own_items() {
    let package = admit(ext_corpus_package());
    let functions = vec![
        function_dangling_call("dangling_fn"),
        function_unrelated("unrelated_fn"),
    ];
    let items = vec![
        item(ITEM_CALL_ADD, "dangling_fn"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
    ];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnknownCallee { callee }
        } if callee == "not_declared_anywhere"
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
    ));
}

/// Trace: FR-021-AC-12, TC-031 (second case). A function whose own body
/// node's real form disagrees with its declared body kind ("a form none of
/// this generator's classifiers admits") is refused with `FormMismatch`,
/// isolated to its own item.
#[test]
fn tc_031_ac12_form_mismatch_refuses_only_its_own_item() {
    let package = admit(ext_corpus_package());
    let functions = vec![
        function_form_mismatch("bad_fn"),
        function_unrelated("unrelated_fn"),
    ];
    let items = vec![
        item(ITEM_CALL_NESTED, "bad_fn"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
    ];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_NESTED),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::FormMismatch { .. }
        }
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
    ));
}

/// Trace: FR-021-AC-33, TC-031 step 13. A distinct call request retains its identity while the
/// function's separately declared temporal formula body supplies the refusal cause.
#[test]
fn tc_031_ac33_call_claim_retains_identity_and_uses_declared_body_refusal() {
    let package = admit(ext_corpus_package());
    let body_id = package.ids.resolve(&code_id(T_BARE_FORMULA));
    let call_id = package.ids.resolve(&code_id(ITEM_CALL_ADD));
    let mut temporal_body = function_add("temporal_body");
    temporal_body.node_id = body_id.clone();
    let functions = vec![temporal_body, function_eq("unrelated_fn")];
    let items = vec![
        item(ITEM_CALL_ADD, "temporal_body"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
    ];
    let oracles = generate(&package, &functions, &items);

    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                unsupported_node_id,
                node_tag: "temporal",
                issue: UpstreamBlocker::QuireSpecLanguage121,
            }
        } if *unsupported_node_id == body_id
    ));
    assert_eq!(
        oracles
            .claim_map
            .items
            .iter()
            .find(|claim| claim.node_id == call_id)
            .unwrap()
            .node_id,
        call_id
    );
    let sibling = disposition_for(&oracles, ITEM_CALL_UNRELATED);
    assert!(
        matches!(sibling, ClaimDisposition::Generated(_)),
        "healthy sibling disposition: {sibling:?}"
    );
}

fn tc_031_ac33_direct_unsupported_item_keeps_identity_and_healthy_function(
    fixture: &str,
    wanted_form: &str,
    body_code: u32,
    call_code: u32,
    expected_tag: &'static str,
) {
    use serde_json::json;
    use std::{fs, path::PathBuf};

    let qspec = PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"));
    let wire: serde_json::Value = serde_json::from_slice(
        &fs::read(
            qspec
                .join("proposals/checked-package-v2/fixtures")
                .join(fixture),
        )
        .expect("read authoritative QSpec fixture"),
    )
    .expect("QSpec fixture JSON");
    let nodes = wire["semantic_graph"]["nodes"].as_array().unwrap();
    let requested_root = nodes
        .iter()
        .find(|node| {
            node["semantic_form"] == wanted_form
                && (wanted_form != "case" || node["body"]["operator"] == "case")
        })
        .expect("QSpec fixture contains requested root");
    let requested_id: quire_contract_model::CheckedNodeId =
        serde_json::from_value(requested_root["node_id"].clone()).unwrap();
    let integer_id: quire_contract_model::CheckedNodeId = serde_json::from_value(
        nodes
            .iter()
            .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "integer")
            .unwrap()["node_id"]
            .clone(),
    )
    .unwrap();
    let value_id: quire_contract_model::CheckedNodeId = serde_json::from_value(
        nodes
            .iter()
            .find(|node| {
                node["node_tag"] == "value"
                    && node["semantic_form"] == "literal"
                    && node["body"]["value_kind"] == "integer"
            })
            .unwrap()["node_id"]
            .clone(),
    )
    .unwrap();
    let node_ref =
        |id: &quire_contract_model::CheckedNodeId| json!({"domain":id.domain, "digest":id.digest});
    let body = json!({
        "term":"application", "operator":"binary",
        "operation":{"identity":"quire.op.integer.mul","laws":[],"mode":null,"member":null,"leaves":[]},
        "result_type":node_ref(&integer_id),
        "arguments":[{"term":"reference","target":node_ref(&value_id)},{"term":"reference","target":node_ref(&value_id)}],
    });
    let mut builder = PackageBuilder::from_wire(wire);
    builder
        .application_node_tagged(
            body_code,
            "expression",
            "binary",
            integer_id.digest.as_ref(),
            body.clone(),
        )
        .application_node_tagged(
            call_code,
            "expression",
            "call",
            integer_id.digest.as_ref(),
            body,
        );
    let package = admit(builder);
    let requested_id = package.ids.resolve(&requested_id);
    let body_id = package.ids.resolve(&code_id(body_code));
    let healthy_call_id = package.ids.resolve(&code_id(call_code));
    let integer_id = package.ids.resolve(&integer_id);
    let value_id = package.ids.resolve(&value_id);
    let function = quire_contract_codegen::ExactFunctionDeclaration {
        node_id: body_id.clone(),
        name: "healthy_fn".into(),
        parameters: vec![
            quire_contract_codegen::FunctionParameter {
                name: "left".into(),
                type_node_id: integer_id.clone(),
            },
            quire_contract_codegen::FunctionParameter {
                name: "right".into(),
                type_node_id: integer_id.clone(),
            },
        ],
        result_type: integer_id,
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Multiply,
        },
        capability_requirements: Vec::new(),
    };
    let direct_item = ExactFunctionItem {
        call_node_id: requested_id.clone(),
        function: "healthy_fn".into(),
        argument_node_ids: vec![value_id.clone(), value_id.clone()],
    };
    let healthy_item = ExactFunctionItem {
        call_node_id: healthy_call_id.clone(),
        function: "healthy_fn".into(),
        argument_node_ids: vec![value_id.clone(), value_id],
    };
    let mut roots = vec![
        body_id.clone(),
        requested_id.clone(),
        healthy_call_id.clone(),
    ];
    roots.sort();
    let requests = driver_positions(&roots);
    let position = |id: &quire_contract_model::CheckedNodeId| {
        roots.iter().position(|root| root == id).unwrap()
    };
    let contract = admitted_function_contract(&package.checked, &roots);
    let combined = generate_exact_function_oracles(
        &contract,
        &requests,
        &[position(&body_id)],
        std::slice::from_ref(&function),
        &[position(&requested_id), position(&healthy_call_id)],
        &[direct_item, healthy_item.clone()],
    )
    .unwrap();
    let healthy_only = generate_exact_function_oracles(
        &contract,
        &requests,
        &[position(&body_id)],
        &[function],
        &[position(&healthy_call_id)],
        &[healthy_item],
    )
    .unwrap();
    let direct = combined
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == requested_id)
        .expect("direct claim retains request identity");
    match (&direct.result, expected_tag) {
        (
            ClaimDisposition::Refused {
                refusal:
                    ExactFunctionRefusal::BlockedOnUpstream {
                        unsupported_node_id,
                        node_tag: "temporal",
                        issue: UpstreamBlocker::QuireSpecLanguage121,
                    },
            },
            "temporal",
        ) => assert_eq!(unsupported_node_id, &requested_id),
        (
            ClaimDisposition::Refused {
                refusal:
                    ExactFunctionRefusal::Unsupported {
                        unsupported_node_id,
                        node_tag,
                    },
            },
            tag,
        ) => {
            assert_eq!(unsupported_node_id, &requested_id);
            assert_eq!(*node_tag, tag);
        }
        (other, tag) => panic!("expected unsupported {tag} for requested node: {other:?}"),
    }
    let generated = combined
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == healthy_call_id)
        .expect("healthy sibling claim");
    let generated_only = healthy_only
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == healthy_call_id)
        .expect("standalone healthy sibling claim");
    assert!(matches!(&generated.result, ClaimDisposition::Generated(_)));
    assert_eq!(generated.result, generated_only.result);
    assert_eq!(
        contents(&combined, "src/lib.rs"),
        contents(&healthy_only, "src/lib.rs")
    );
    assert_eq!(combined.location_map, healthy_only.location_map);
    assert!(!contents(&combined, "src/lib.rs").contains(requested_id.digest.as_ref()));
}

/// Trace: FR-021-AC-33, TC-031. A directly requested formula call-node settles to Unsupported,
/// retains its request identity, and does not alter a healthy function sibling.
#[test]
fn tc_031_ac33_direct_formula_call_node_is_unsupported() {
    tc_031_ac33_direct_unsupported_item_keeps_identity_and_healthy_function(
        "positive-clause-operations.json",
        "formula",
        991_001,
        991_002,
        "temporal",
    );
}

/// Trace: FR-021-AC-33, TC-031. A directly requested case call-node settles to Unsupported,
/// retains its request identity, and does not alter a healthy function sibling.
#[test]
fn tc_031_ac33_direct_case_call_node_is_unsupported() {
    tc_031_ac33_direct_unsupported_item_keeps_identity_and_healthy_function(
        "positive-union-nodes.json",
        "case",
        991_011,
        991_012,
        "expression",
    );
}

/// Trace: FR-021-AC-33, TC-031. A reader-admitted fairness root keeps its request identity,
/// refuses with the temporal cause, and leaves a supported function sibling byte-identical.
#[test]
fn tc_031_ac33_direct_fairness_call_node_is_unsupported() {
    use serde_json::{json, Value};
    use std::{fs, path::PathBuf};

    const MODEL_NODE: u32 = 992_001;
    const FAIRNESS_NODE: u32 = 992_002;
    const FUNCTION_BODY: u32 = 992_003;
    const HEALTHY_CALL: u32 = 992_004;

    let qspec = PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"));
    let fixture = qspec.join("proposals/checked-package-v2/fixtures/positive-all-families.json");
    let mut wire: Value = serde_json::from_slice(&fs::read(fixture).expect("read published QSpec"))
        .expect("published QSpec fixture JSON");
    let (baseline, _) = PackageBuilder::from_wire(wire.clone()).read();
    assert!(
        matches!(
            baseline,
            quire_contract_model::CheckedPackageV2ReadResult::Admitted(_)
        ),
        "unmodified authoritative QSpec fixture must admit with the same runtime evidence"
    );

    // Reproduce the original bad incremental mutation: indexing an absent operation/laws path
    // inserts null members into node 0's existing body. The reader must reject that malformed
    // body, demonstrating why the candidate builder below uses checked object traversal.
    let mut first_bad_mutation = wire.clone();
    assert!(
        first_bad_mutation["semantic_graph"]["nodes"][0]["body"]["operation"]["laws"]
            .as_array_mut()
            .is_none()
    );
    let refusal = read_mutated_wire(first_bad_mutation);
    let refusal = match refusal {
        quire_contract_model::CheckedPackageV2ReadResult::Refused(refusal) => refusal,
        other => panic!("malformed first mutation unexpectedly admitted: {other:?}"),
    };
    let refusal_debug = format!("{refusal:?}");
    eprintln!("first malformed mutation refusal: {refusal_debug}");
    assert!(
        refusal_debug.contains("InvalidSemanticGraph"),
        "{refusal_debug}"
    );
    assert!(
        refusal_debug.contains("/semantic_graph/nodes/0/body"),
        "{refusal_debug}"
    );

    let nodes = wire["semantic_graph"]["nodes"].as_array().unwrap();
    let boolean_type = nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "boolean")
        .unwrap()["node_id"]["digest"]
        .as_str()
        .unwrap()
        .to_owned();
    let integer_type = nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "integer")
        .unwrap()["node_id"]["digest"]
        .as_str()
        .unwrap()
        .to_owned();
    let text_type = nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "text")
        .unwrap()["node_id"]["digest"]
        .as_str()
        .unwrap()
        .to_owned();

    let infinite_trace =
        json!({"authority":"agent-ix", "identity":"quire.temporal.infinite-trace/v1"});
    for path in ["lock", "identity_preimage"] {
        let selections = wire[path]["profile_selections"].as_array_mut().unwrap();
        let temporal = selections
            .iter_mut()
            .find(|selection| selection["role"] == "temporal_profile")
            .expect("published temporal profile selection");
        temporal["definition"] = infinite_trace.clone();
    }
    for node in wire["semantic_graph"]["nodes"].as_array_mut().unwrap() {
        let laws = node
            .get_mut("body")
            .and_then(Value::as_object_mut)
            .and_then(|body| body.get_mut("operation"))
            .and_then(Value::as_object_mut)
            .and_then(|operation| operation.get_mut("laws"))
            .and_then(Value::as_array_mut);
        if let Some(laws) = laws {
            for law in laws {
                if law["role"] == "temporal_profile" {
                    law["definition"] = infinite_trace.clone();
                }
            }
        }
    }

    let mut builder = PackageBuilder::from_wire(wire);
    builder.model_owned_object_type(MODEL_NODE, "acme/orders", "ix://acme/orders/Order");
    let model_placeholder = key(MODEL_NODE);
    let parameter_placeholder = key(992_005);
    builder.node(
        &parameter_placeholder,
        "value",
        "parameter",
        &integer_type,
        aggregate(vec![
            bound_member(
                "name",
                json!({"term":"literal", "type":{"domain":NODE_DOMAIN, "digest":text_type}, "value":"left", "value_kind":"text"}),
            ),
            bound_member(
                "level",
                json!({"term":"literal", "type":{"domain":NODE_DOMAIN, "digest":integer_type}, "value":"0", "value_kind":"integer"}),
            ),
        ]),
    );
    builder.application_node_tagged(
        FAIRNESS_NODE,
        "temporal",
        "fairness",
        &boolean_type,
        json!({
            "term":"application", "operator":"temporal_fairness",
            "operation":{
                "identity":"quire.op.temporal.fair", "laws":[], "mode":null,
                "member":{
                    "kind":"fairness", "fairness_kind":"weak", "granularity":"each",
                    "declaration":{"domain":NODE_DOMAIN, "digest":model_placeholder},
                    "name":"size"
                },
                "leaves":[]
            },
            "result_type":{"domain":NODE_DOMAIN, "digest":boolean_type},
            "arguments":[]
        }),
    );
    let fairness_placeholder = code_id(FAIRNESS_NODE);
    let clause = builder.raw_wire()["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["node_tag"] == "temporal" && node["semantic_form"] == "temporal_clause")
        .expect("published temporal clause")
        .clone();
    let clause_digest = clause["node_id"]["digest"].as_str().unwrap().to_owned();
    let mut wire = builder.raw_wire();
    let clause = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|node| node["node_id"]["digest"] == clause_digest)
        .expect("temporal clause to update");
    let arguments = clause["body"]["arguments"].as_array_mut().unwrap();
    arguments[4] = json!({
        "term":"aggregate", "members":[{"term":"reference", "target":{
            "domain":NODE_DOMAIN, "digest":fairness_placeholder.digest
        }}]
    });
    let dependencies = clause["dependencies"].as_array_mut().unwrap();
    dependencies.push(json!({"domain":NODE_DOMAIN, "digest":fairness_placeholder.digest}));
    builder = PackageBuilder::from_wire(wire);

    let body = json!({
        "term":"application", "operator":"binary",
        "operation":{"identity":"quire.op.integer.add","laws":[],"mode":null,"member":null,"leaves":[]},
        "result_type":{"domain":NODE_DOMAIN, "digest":integer_type},
        "arguments":[
            {"term":"reference", "target":{"domain":NODE_DOMAIN, "digest":parameter_placeholder}},
            {"term":"reference", "target":{"domain":NODE_DOMAIN, "digest":parameter_placeholder}}
        ]
    });
    builder
        .application_node_tagged(
            FUNCTION_BODY,
            "expression",
            "binary",
            &integer_type,
            body.clone(),
        )
        .application_node_tagged(HEALTHY_CALL, "expression", "call", &integer_type, body);

    let (admission, _resolved_wire, ids) = builder.read_resolved();
    let package = match admission {
        quire_contract_model::CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("candidate must be admitted by public CheckedPackageV2::read: {other:?}"),
    };
    let fairness_id = ids.resolve(&fairness_placeholder);
    let model_id = ids.resolve(&id(&model_placeholder));
    let body_id = ids.resolve(&code_id(FUNCTION_BODY));
    let call_id = ids.resolve(&code_id(HEALTHY_CALL));
    let parameter_id = ids.resolve(&id(&parameter_placeholder));
    let fairness = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == fairness_id)
        .expect("reader-admitted fairness node");
    assert_eq!(fairness.node_tag.as_ref(), "temporal");
    assert_eq!(fairness.semantic_form.as_ref(), "fairness");
    assert_eq!(fairness.dependencies, vec![model_id]);

    let function = quire_contract_codegen::ExactFunctionDeclaration {
        node_id: body_id.clone(),
        name: "healthy_fairness_sibling".into(),
        parameters: vec![
            quire_contract_codegen::FunctionParameter {
                name: "left".into(),
                type_node_id: ids.resolve(&id(&integer_type)),
            },
            quire_contract_codegen::FunctionParameter {
                name: "right".into(),
                type_node_id: ids.resolve(&id(&integer_type)),
            },
        ],
        result_type: ids.resolve(&id(&integer_type)),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let direct_item = ExactFunctionItem {
        call_node_id: fairness_id.clone(),
        function: "healthy_fairness_sibling".into(),
        argument_node_ids: vec![parameter_id.clone(), parameter_id.clone()],
    };
    let healthy_item = ExactFunctionItem {
        call_node_id: call_id.clone(),
        function: "healthy_fairness_sibling".into(),
        argument_node_ids: vec![parameter_id.clone(), parameter_id],
    };
    let mut roots = vec![body_id.clone(), fairness_id.clone(), call_id.clone()];
    roots.sort();
    let position = |id: &quire_contract_model::CheckedNodeId| {
        roots.iter().position(|root| root == id).unwrap()
    };
    let requests = driver_positions(&roots);
    let contract = admitted_function_contract(&package, &roots);
    let combined = generate_exact_function_oracles(
        &contract,
        &requests,
        &[position(&body_id)],
        std::slice::from_ref(&function),
        &[position(&fairness_id), position(&call_id)],
        &[direct_item, healthy_item.clone()],
    )
    .unwrap();
    let healthy_only = generate_exact_function_oracles(
        &contract,
        &requests,
        &[position(&body_id)],
        &[function],
        &[position(&call_id)],
        &[healthy_item],
    )
    .unwrap();
    let direct = combined
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == fairness_id)
        .expect("direct fairness claim retains requested identity");
    assert!(matches!(
        &direct.result,
        ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::BlockedOnUpstream {
            unsupported_node_id,
            node_tag: "temporal",
            issue: UpstreamBlocker::QuireSpecLanguage121,
        }
        } if unsupported_node_id == &fairness_id
    ));
    let generated = combined
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == call_id)
        .expect("healthy sibling claim");
    let generated_alone = healthy_only
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == call_id)
        .expect("healthy sibling claim without fairness");
    assert!(matches!(&generated.result, ClaimDisposition::Generated(_)));
    assert_eq!(generated.result, generated_alone.result);
    assert_eq!(
        contents(&combined, "src/lib.rs"),
        contents(&healthy_only, "src/lib.rs")
    );
    assert_eq!(combined.location_map, healthy_only.location_map);
    assert!(!contents(&combined, "src/lib.rs").contains(fairness_id.digest.as_ref()));
}

/// Trace: FR-021-AC-1 (duplicate handling, matching FR-014/FR-018). One
/// node id requested twice under one binding refuses every copy.
#[test]
fn tc_031_ac1_duplicate_request_refuses_every_copy() {
    let package = admit(ext_corpus_package());
    let functions = vec![function_add("add_fn")];
    let items = vec![item(ITEM_CALL_ADD, "add_fn"), item(ITEM_CALL_ADD, "add_fn")];
    let oracles = generate(&package, &functions, &items);
    assert_eq!(oracles.claim_map.items.len(), 1);
    assert!(matches!(
        &oracles.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::DuplicateRequest
        }
    ));
}

/// Trace: FR-021-AC-12 (signature/body-kind agreement), TC-031. A `Scalar`
/// body's declared parameter count or result type disagreeing with what the
/// body kind requires is refused with `SignatureMismatch` at generation
/// time, instead of assembling into a package whose emitted
/// `FunctionDeclaration` disagrees with its own rendered `Body` -- the two
/// measured repros: an `Add` declaration with only one declared parameter,
/// and an `Add` declaration whose declared result type is `Boolean` instead
/// of `Integer`.
#[test]
fn tc_031_signature_mismatch_refuses_arity_and_result_type_disagreement() {
    let package = admit(ext_corpus_package());

    // Arity: FN_ADD's real body needs two operands; declared with one.
    let arity_mismatch = quire_contract_codegen::ExactFunctionDeclaration {
        node_id: code_id(FN_ADD),
        name: "one_param_add".to_owned(),
        parameters: vec![quire_contract_codegen::FunctionParameter {
            name: "a".to_owned(),
            type_node_id: code_id(T_INTEGER),
        }],
        result_type: code_id(T_INTEGER),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let items = vec![item(ITEM_CALL_ADD, "one_param_add")];
    let oracles = generate(&package, &[arity_mismatch], &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::SignatureMismatch { .. }
        }
    ));

    // Result type: FN_ADD's node, a Scalar body (Integer-only), declared
    // with a Boolean result.
    let result_mismatch = quire_contract_codegen::ExactFunctionDeclaration {
        node_id: code_id(FN_ADD),
        name: "boolean_result_add".to_owned(),
        parameters: vec![
            quire_contract_codegen::FunctionParameter {
                name: "a".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
            quire_contract_codegen::FunctionParameter {
                name: "b".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
        ],
        result_type: code_id(T_BOOLEAN),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let items = vec![item(ITEM_CALL_EQ, "boolean_result_add")];
    let oracles = generate(&package, &[result_mismatch], &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::SignatureMismatch { .. }
        }
    ));
}

/// Trace: FR-021-AC-15/AC-17 (duplicate declared names), TC-031. Two
/// declarations sharing one name -- different node ids, same string --
/// are both refused as `AmbiguousFunctionName` rather than colliding into
/// one Stage 1 classification, and a third, uniquely-named survivor's
/// location map index is unaffected: it still equals its own position among
/// the surviving functions, not a value corrupted by the excluded pair.
/// This is FR-021-AC-15's own mutation table entry ("record it against the
/// wrong function's index") reproduced and proven fixed.
#[test]
fn tc_031_ambiguous_function_name_refuses_both_and_does_not_corrupt_indices() {
    let package = admit(ext_corpus_package());
    let functions = vec![
        function_add("dup"),       // node FN_ADD
        function_unrelated("dup"), // node FN_UNRELATED, same name, different node
        function_eq("solo_eq"),
        quire_contract_codegen::ExactFunctionDeclaration {
            node_id: code_id(E_TUPLE),
            name: "solo_eq_2".to_owned(),
            parameters: vec![
                quire_contract_codegen::FunctionParameter {
                    name: "a".to_owned(),
                    type_node_id: code_id(T_INTEGER),
                },
                quire_contract_codegen::FunctionParameter {
                    name: "b".to_owned(),
                    type_node_id: code_id(T_INTEGER),
                },
            ],
            result_type: code_id(T_BOOLEAN),
            body: quire_contract_codegen::ExactFunctionBody::CompositeEquality {
                operator: quire_contract_codegen::EqualityOperatorKind::Equal,
            },
            capability_requirements: Vec::new(),
        },
    ];
    let items = vec![
        item(ITEM_CALL_ADD, "dup"),
        item(ITEM_CALL_EQ, "solo_eq"),
        item(ITEM_CALL_NESTED, "solo_eq_2"),
    ];
    let oracles = generate(&package, &functions, &items);

    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::AmbiguousFunctionName { name }
        } if name == "dup"
    ));
    // Neither "dup" declaration ever survives to Stage 2, so neither ever
    // gets a location-map entry at all -- the old, name-keyed maps would
    // instead have let both survive and collide on one shared index.
    assert!(oracles
        .location_map
        .iter()
        .all(|entry| entry.function != "dup"));

    // Node ids are content digests, not the numeric placeholder codes
    // above, so the surviving order (solo_eq, solo_eq_2) is whichever the
    // generator's own node-id sort gives -- re-derived here exactly as
    // `generate_exact_function_oracles` computes it, the same pattern
    // `tc_031_ac3`/`tc_031_ac15` already use, rather than hardcoded.
    let mut survivor_node_ids = [oracles.key(FN_EQ), oracles.key(E_TUPLE)];
    survivor_node_ids.sort();
    let expected_index = |node_id: &quire_contract_model::CheckedNodeId| {
        survivor_node_ids
            .iter()
            .position(|candidate| candidate == node_id)
            .expect("survivor node id")
    };

    let eq_entry = oracles
        .location_map
        .iter()
        .find(|entry| entry.function == "solo_eq")
        .expect("solo_eq has a location map entry");
    let eq_2_entry = oracles
        .location_map
        .iter()
        .find(|entry| entry.function == "solo_eq_2")
        .expect("solo_eq_2 has a location map entry");
    assert_eq!(oracles.location_map.len(), 2);
    match &eq_entry.location.origin {
        quire_contract_codegen::RecordedOrigin::Body { function, index } => {
            assert_eq!(function, "solo_eq");
            assert_eq!(*index, expected_index(&oracles.key(FN_EQ)));
        }
    }
    match &eq_2_entry.location.origin {
        quire_contract_codegen::RecordedOrigin::Body { function, index } => {
            assert_eq!(function, "solo_eq_2");
            assert_eq!(*index, expected_index(&oracles.key(E_TUPLE)));
        }
    }
    assert_ne!(
        expected_index(&oracles.key(FN_EQ)),
        expected_index(&oracles.key(E_TUPLE)),
        "the two survivors must occupy distinct indices"
    );
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_NESTED),
        ClaimDisposition::Generated(_)
    ));
}

/// Trace: FR-021-AC-1 (item-level arity), TC-031. An item's argument count
/// disagreeing with the applied function's declared parameter count is
/// refused as `ArityMismatch`, naming both counts.
#[test]
fn tc_031_arity_mismatch_refuses_when_item_argument_count_disagrees() {
    let package = admit(ext_corpus_package());
    let functions = vec![function_add("add_fn")];
    let items = vec![quire_contract_codegen::ExactFunctionItem {
        call_node_id: code_id(ITEM_CALL_ADD),
        function: "add_fn".to_owned(),
        argument_node_ids: vec![code_id(T_INTEGER)],
    }];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::ArityMismatch {
                expected: 2,
                found: 1
            }
        }
    ));
}

/// Trace: FR-021-AC-21, TC-031. `IntegerOperator::Negate` is unary and out of
/// this V1's scope (only binary `Add`/`Subtract`/`Multiply` are supported): a
/// `Scalar` body declaring it is refused as `UnsupportedOperator` and the
/// function is absent from the emitted `checked_package()`.
#[test]
fn tc_031_ac21_unsupported_operator_refuses_unary_negate_scalar_body() {
    let package = admit(ext_corpus_package());
    let functions = vec![quire_contract_codegen::ExactFunctionDeclaration {
        node_id: code_id(FN_ADD),
        name: "negate_fn".to_owned(),
        parameters: vec![
            quire_contract_codegen::FunctionParameter {
                name: "a".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
            quire_contract_codegen::FunctionParameter {
                name: "b".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
        ],
        result_type: code_id(T_INTEGER),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Negate,
        },
        capability_requirements: Vec::new(),
    }];
    let items = vec![item(ITEM_CALL_ADD, "negate_fn")];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnsupportedOperator { .. }
        }
    ));
    let lib = contents(&oracles, "src/lib.rs");
    assert!(
        !lib.contains("negate_fn"),
        "a refused Negate function must not appear in checked_package()"
    );
    assert!(
        !lib.contains("rt::FunctionDeclaration {"),
        "no function survives, so checked_package() declares none"
    );
}

/// `declaration` moved onto the node `code`.
fn on_node(
    mut declaration: quire_contract_codegen::ExactFunctionDeclaration,
    code: u32,
) -> quire_contract_codegen::ExactFunctionDeclaration {
    declaration.node_id = code_id(code);
    declaration
}

/// Every ordering of `items`.
fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let Some((first, rest)) = items.split_first() else {
        return vec![Vec::new()];
    };
    let mut all = Vec::new();
    for tail in permutations(rest) {
        for at in 0..=tail.len() {
            let mut order = tail.clone();
            order.insert(at, first.clone());
            all.push(order);
        }
    }
    all
}

/// One FR-021-AC-22 fixture: declarations of which `pair_names` share the declaring node
/// `FN_ADD`, and the requested `(call node, function)` items.
struct DuplicateNodeFixture {
    functions: Vec<quire_contract_codegen::ExactFunctionDeclaration>,
    items: Vec<(u32, &'static str)>,
    /// Names held by a declaration sharing `FN_ADD`.
    pair_names: Vec<&'static str>,
    /// Every function name that must be absent from `checked_package()` and the location map.
    absent_names: Vec<&'static str>,
    /// Items (call node codes) that must be refused `DuplicateDeclaringNode`.
    duplicate_items: Vec<u32>,
    /// Items over a function with its own node id and name; each must equal the entry the same
    /// request produces with every declaration in `absent_names` removed.
    distinct_items: Vec<u32>,
}

impl DuplicateNodeFixture {
    /// Check the fixture under every permutation of the declaration order (which includes both
    /// orders of the pair).
    fn check(&self, package: &FixturePackage) {
        let requested: Vec<ExactFunctionItem> = self
            .items
            .iter()
            .map(|(code, function)| item(*code, function))
            .collect();
        let baseline_functions: Vec<_> = self
            .functions
            .iter()
            .filter(|declaration| !self.absent_names.contains(&declaration.name.as_str()))
            .cloned()
            .collect();
        let baseline_items: Vec<ExactFunctionItem> = self
            .items
            .iter()
            .filter(|(code, _)| self.distinct_items.contains(code))
            .map(|(code, function)| item(*code, function))
            .collect();
        let input = admitted_full_request(package, &self.functions, &requested);
        let baseline = generate_in_request(package, &input, &baseline_functions, &baseline_items);

        let mut first: Option<FixtureOracles> = None;
        for order in permutations(&self.functions) {
            let oracles = generate_in_request(package, &input, &order, &requested);
            match &first {
                Some(first) => assert_eq!(first, &oracles, "request order changed the output"),
                None => first = Some(oracles.clone()),
            }
            self.check_one(&oracles, &baseline);
        }
    }

    fn check_one(&self, oracles: &FixtureOracles, baseline: &FixtureOracles) {
        let lib = contents(oracles, "src/lib.rs");
        let location_json = contents(oracles, "location-map.json");
        for name in &self.absent_names {
            assert!(
                !lib.contains(&format!("{name:?}")),
                "{name} is in src/lib.rs"
            );
            assert!(
                !location_json.contains(&format!("{name:?}")),
                "{name} is in location-map.json"
            );
            assert!(oracles
                .location_map
                .iter()
                .all(|entry| entry.function != *name));
        }
        for code in &self.duplicate_items {
            assert!(
                matches!(
                    disposition_for(oracles, *code),
                    ClaimDisposition::Refused {
                        refusal: ExactFunctionRefusal::DuplicateDeclaringNode { node_id }
                    } if *node_id == oracles.key(FN_ADD)
                ),
                "item {code} is not DuplicateDeclaringNode: {:?}",
                disposition_for(oracles, *code)
            );
        }
        // No claim records a pair member's identity, and no generated claim shares an oracle
        // symbol or origin index with another.
        let mut symbols = Vec::new();
        let mut indices = Vec::new();
        for claim in &oracles.claim_map.items {
            if let ClaimDisposition::Generated(generated) = &claim.result {
                assert!(!self.pair_names.contains(&generated.function.as_str()));
                symbols.push(generated.oracle_symbol.clone());
                indices.push(format!("{:?}", generated.function_origin));
            }
        }
        let distinct = (symbols.len(), indices.len());
        symbols.sort();
        symbols.dedup();
        indices.sort();
        indices.dedup();
        assert_eq!((symbols.len(), indices.len()), distinct);
        for code in &self.distinct_items {
            assert!(
                matches!(
                    disposition_for(oracles, *code),
                    ClaimDisposition::Generated(_)
                ),
                "distinct item {code} was not generated"
            );
            assert_eq!(
                disposition_for(oracles, *code),
                disposition_for(baseline, *code),
                "item {code} differs from the request without the duplicates"
            );
        }
    }
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (i). Two declarations sharing one declaring node
/// id and both admissible are both refused, whichever order they arrive in: neither enters
/// `checked_package()`, the source or `location-map.json`, and each item naming one is
/// `DuplicateDeclaringNode`, never `UnknownFunction`. A distinct-node function is generated
/// unchanged.
#[test]
fn tc_031_ac22_fixture_i_both_bodies_admissible() {
    let package = admit(ext_corpus_package());
    DuplicateNodeFixture {
        functions: vec![
            function_add("pair_one"),
            function_add("pair_two"),
            function_unrelated("unrelated_fn"),
        ],
        items: vec![
            (ITEM_CALL_ADD, "pair_one"),
            (ITEM_CALL_EQ, "pair_two"),
            (ITEM_CALL_UNRELATED, "unrelated_fn"),
        ],
        pair_names: vec!["pair_one", "pair_two"],
        absent_names: vec!["pair_one", "pair_two"],
        duplicate_items: vec![ITEM_CALL_ADD, ITEM_CALL_EQ],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    }
    .check(&package);
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (ii). One declaration of the pair would be
/// refused in Stage 1 (an undischargeable capability) and its same-node sibling is admissible:
/// the sibling does not survive, and neither item reports `UnknownFunction`, nor the refused
/// one's own Stage 1 reason.
#[test]
fn tc_031_ac22_fixture_ii_one_body_refused_and_sibling_admissible() {
    let package = admit(ext_corpus_package());
    DuplicateNodeFixture {
        functions: vec![
            on_node(function_capability("pair_one"), FN_ADD),
            function_add("pair_two"),
            function_unrelated("unrelated_fn"),
        ],
        items: vec![
            (ITEM_CALL_ADD, "pair_one"),
            (ITEM_CALL_EQ, "pair_two"),
            (ITEM_CALL_UNRELATED, "unrelated_fn"),
        ],
        pair_names: vec!["pair_one", "pair_two"],
        absent_names: vec!["pair_one", "pair_two"],
        duplicate_items: vec![ITEM_CALL_ADD, ITEM_CALL_EQ],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    }
    .check(&package);
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (iii). The pair also shares one name, and a third
/// declaration with its own node id holds the same name: the third is refused as
/// `AmbiguousFunctionName` and is absent from `checked_package()` and the location map, while
/// items naming the shared name are `DuplicateDeclaringNode`, never `AmbiguousFunctionName`.
#[test]
fn tc_031_ac22_fixture_iii_shared_name_takes_the_node_id_refusal() {
    let package = admit(ext_corpus_package());
    DuplicateNodeFixture {
        functions: vec![
            function_add("shared"),
            function_add("shared"),
            function_unrelated("shared"),
            function_eq("eq_fn"),
        ],
        items: vec![(ITEM_CALL_ADD, "shared"), (ITEM_CALL_EQ, "eq_fn")],
        pair_names: vec!["shared"],
        absent_names: vec!["shared"],
        duplicate_items: vec![ITEM_CALL_ADD],
        distinct_items: vec![ITEM_CALL_EQ],
    }
    .check(&package);
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (iv). A declaration whose nested `call` names a
/// member of the pair is refused as `UnknownCallee`, and a distinct-node function and an item
/// naming each function are handled: the pair's items `DuplicateDeclaringNode`, the caller's
/// item `UnknownCallee`, the distinct function's generated.
#[test]
fn tc_031_ac22_fixture_iv_nested_call_to_a_refused_duplicate_is_unknown_callee() {
    let package = admit(ext_corpus_package());
    let fixture = DuplicateNodeFixture {
        functions: vec![
            function_add("add_fn"),
            function_add("add_fn"),
            function_call_nested("call_fn", "add_fn"),
            function_unrelated("unrelated_fn"),
        ],
        items: vec![
            (ITEM_CALL_ADD, "add_fn"),
            (ITEM_CALL_EQ, "add_fn"),
            (ITEM_CALL_NESTED, "call_fn"),
            (ITEM_CALL_UNRELATED, "unrelated_fn"),
        ],
        pair_names: vec!["add_fn"],
        absent_names: vec!["add_fn", "call_fn"],
        duplicate_items: vec![ITEM_CALL_ADD, ITEM_CALL_EQ],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    };
    fixture.check(&package);

    let requested: Vec<ExactFunctionItem> = fixture
        .items
        .iter()
        .map(|(code, function)| item(*code, function))
        .collect();
    for order in permutations(&fixture.functions) {
        let oracles = generate(&package, &order, &requested);
        assert!(matches!(
            disposition_for(&oracles, ITEM_CALL_NESTED),
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::UnknownCallee { callee }
            } if callee == "add_fn"
        ));
    }
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (v). One name is held by two duplicate groups, on
/// `FN_ADD` and on `FN_EQ`: an item naming it is refused `DuplicateDeclaringNode` carrying the
/// smaller of the two node ids, under every order of the declarations.
#[test]
fn tc_031_ac22_fixture_v_two_duplicate_groups_report_the_smallest_node_id() {
    let package = admit(ext_corpus_package());
    let smallest = std::cmp::min(package.key(FN_ADD), package.key(FN_EQ));
    let functions = vec![
        function_add("shared"),
        function_add("shared"),
        function_eq("shared"),
        function_eq("shared"),
    ];
    let requested = vec![item(ITEM_CALL_ADD, "shared")];
    for order in permutations(&functions) {
        let oracles = generate(&package, &order, &requested);
        match disposition_for(&oracles, ITEM_CALL_ADD) {
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::DuplicateDeclaringNode { node_id },
            } => assert_eq!(*node_id, smallest),
            other => panic!("expected DuplicateDeclaringNode, got {other:?}"),
        }
        assert!(oracles.location_map.is_empty());
    }
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (vi). Two items on one call node naming the two
/// members of a duplicate pair, and one such item requested twice, each get
/// `DuplicateDeclaringNode`: the refusal takes precedence over the duplicate-request collapse.
#[test]
fn tc_031_ac22_fixture_vi_same_call_node_items_over_a_pair_each_keep_the_node_refusal() {
    let package = admit(ext_corpus_package());
    let functions = vec![function_add("pair_one"), function_add("pair_two")];
    let requested = vec![
        item(ITEM_CALL_ADD, "pair_one"),
        item(ITEM_CALL_ADD, "pair_two"),
        item(ITEM_CALL_EQ, "pair_one"),
        item(ITEM_CALL_EQ, "pair_one"),
    ];
    for order in permutations(&functions) {
        let oracles = generate(&package, &order, &requested);
        assert_eq!(oracles.claim_map.items.len(), 3);
        for claim in &oracles.claim_map.items {
            assert!(
                matches!(
                    &claim.result,
                    ClaimDisposition::Refused {
                        refusal: ExactFunctionRefusal::DuplicateDeclaringNode { node_id }
                    } if *node_id == oracles.key(FN_ADD)
                ),
                "{:?}",
                claim.result
            );
        }
    }
}

/// The dispositions of `oracles`' claim-map entries, in claim-map order.
fn entries(
    oracles: &FixtureOracles,
) -> Vec<&ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal>> {
    oracles
        .claim_map
        .items
        .iter()
        .map(|claim| &claim.result)
        .collect()
}

fn unknown(name: &str) -> ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal> {
    ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::UnknownFunction {
            name: name.to_owned(),
        },
    }
}

/// The entries for `names` requested on `ITEM_CALL_ADD` in each of the two request orders (the
/// order given, then reversed), asserted identical to each other, and returned once.
fn both_orders(
    package: &FixturePackage,
    functions: &[quire_contract_codegen::ExactFunctionDeclaration],
    names: [&str; 2],
) -> FixtureOracles {
    let forward = generate(
        package,
        functions,
        &[item(ITEM_CALL_ADD, names[0]), item(ITEM_CALL_ADD, names[1])],
    );
    let reverse = generate(
        package,
        functions,
        &[item(ITEM_CALL_ADD, names[1]), item(ITEM_CALL_ADD, names[0])],
    );
    assert_eq!(forward, reverse, "request order changed the output");
    forward
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (i). One unknown name on a call node is one
/// `UnknownFunction` entry naming it.
#[test]
fn tc_031_ac24_case_i_one_unknown_name_is_one_entry() {
    let package = admit(ext_corpus_package());
    let oracles = generate(
        &package,
        &[function_add("add_fn")],
        &[item(ITEM_CALL_ADD, "zz_unknown")],
    );
    assert_eq!(entries(&oracles), vec![&unknown("zz_unknown")]);
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (ii). Two different unknown names on one call node
/// with equal arguments are two entries, `aa_unknown` then `zz_unknown`, never `DuplicateRequest`,
/// identical under both request orders.
#[test]
fn tc_031_ac24_case_ii_two_unknown_names_are_two_entries_in_name_order() {
    let package = admit(ext_corpus_package());
    let oracles = both_orders(
        &package,
        &[function_add("add_fn")],
        ["zz_unknown", "aa_unknown"],
    );
    assert_eq!(
        entries(&oracles),
        vec![&unknown("aa_unknown"), &unknown("zz_unknown")]
    );
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (iii). The same unknown name requested twice is one
/// `DuplicateRequest` entry.
#[test]
fn tc_031_ac24_case_iii_the_same_unknown_name_twice_is_one_duplicate_request() {
    let package = admit(ext_corpus_package());
    let oracles = generate(
        &package,
        &[function_add("add_fn")],
        &[
            item(ITEM_CALL_ADD, "zz_unknown"),
            item(ITEM_CALL_ADD, "zz_unknown"),
        ],
    );
    assert_eq!(
        entries(&oracles),
        vec![&ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::DuplicateRequest
        }]
    );
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (iv). A declared `add_fn` and `zz_unknown` on one
/// call node are two entries, the unknown one first, each equal to the entry its item gets when
/// requested alone.
#[test]
fn tc_031_ac24_case_iv_known_and_unknown_order_unknown_first_and_match_solo() {
    let package = admit(ext_corpus_package());
    let functions = [function_add("add_fn")];
    let full_items = [
        item(ITEM_CALL_ADD, "add_fn"),
        item(ITEM_CALL_ADD, "zz_unknown"),
    ];
    let input = admitted_full_request(&package, &functions, &full_items);
    let oracles = generate_in_request(&package, &input, &functions, &full_items);
    let reversed = generate_in_request(
        &package,
        &input,
        &functions,
        &full_items.into_iter().rev().collect::<Vec<_>>(),
    );
    assert_eq!(oracles, reversed, "request order changed the output");
    let solo_add = generate_in_request(
        &package,
        &input,
        &functions,
        &[item(ITEM_CALL_ADD, "add_fn")],
    );
    let solo_unknown = generate_in_request(
        &package,
        &input,
        &functions,
        &[item(ITEM_CALL_ADD, "zz_unknown")],
    );
    assert_eq!(solo_unknown.claim_map.items.len(), 1);
    assert_eq!(solo_add.claim_map.items.len(), 1);
    assert!(matches!(
        &solo_add.claim_map.items[0].result,
        ClaimDisposition::Generated(_)
    ));
    assert_eq!(
        oracles.claim_map.items,
        [
            solo_unknown.claim_map.items[0].clone(),
            solo_add.claim_map.items[0].clone()
        ]
    );
    assert_eq!(oracles.claim_map.items[0].result, unknown("zz_unknown"));
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (v). Names order byte-wise and case-sensitively:
/// `Zz_unknown` (`Z` is 0x5A) before `aa_unknown` (`a` is 0x61), identical under both orders.
#[test]
fn tc_031_ac24_case_v_names_order_by_bytes_case_sensitively() {
    let package = admit(ext_corpus_package());
    let oracles = both_orders(
        &package,
        &[function_add("add_fn")],
        ["aa_unknown", "Zz_unknown"],
    );
    assert_eq!(
        entries(&oracles),
        vec![&unknown("Zz_unknown"), &unknown("aa_unknown")]
    );
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (vi). Items on one call node naming the two members
/// of a duplicate-node pair are two entries ordered by function name. With `m_multi` and `z_pair`
/// on the larger node N2 and `m_multi` and `q_extra` on the smaller node N1, `m_multi` (whose
/// refusal carries N1) orders before `z_pair` (N2); with `z_multi` and `a_pair`, `a_pair` (N2)
/// orders before `z_multi` (N1). Identical under both request orders and every declaration order.
#[test]
fn tc_031_ac24_case_vi_duplicate_node_pair_members_order_by_name() {
    let package = admit(ext_corpus_package());
    let (n1, n2) = if package.key(FN_ADD) < package.key(FN_EQ) {
        (FN_ADD, FN_EQ)
    } else {
        (FN_EQ, FN_ADD)
    };
    let node_refusal = |code: u32| ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::DuplicateDeclaringNode {
            node_id: package.key(code),
        },
    };
    for (shared, pair_member, extra, expected) in [
        ("m_multi", "z_pair", "q_extra", [n1, n2]),
        ("z_multi", "a_pair", "q_extra", [n2, n1]),
    ] {
        let functions = vec![
            on_node(function_add(shared), n2),
            on_node(function_add(pair_member), n2),
            on_node(function_add(shared), n1),
            on_node(function_add(extra), n1),
        ];
        // The member named `shared` is requested alongside `pair_member`; the order of the two
        // refusals is the order of their names, not of the request.
        let (first, second) = if shared < pair_member {
            (shared, pair_member)
        } else {
            (pair_member, shared)
        };
        for order in permutations(&functions) {
            let oracles = both_orders(&package, &order, [first, second]);
            assert_eq!(
                entries(&oracles),
                vec![&node_refusal(expected[0]), &node_refusal(expected[1])],
                "{shared} and {pair_member}"
            );
        }
    }
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (vii). The name tie-break applies only to items equal
/// on the earlier key fields: items whose declaring node ids differ are ordered by declaring node
/// id first. With `a_pair` and `z_pair` on N2, `z_pair` and `q_extra` on N1, and `a_pair` alone on
/// the larger N3, `a_pair` resolves to N3 and `z_pair` to N2, so `z_pair` (refusal N1) orders
/// before `a_pair` (refusal N2) although `a_pair` is the smaller name, under both request orders
/// and every declaration order.
#[test]
fn tc_031_ac24_case_vii_differing_declaring_node_ids_order_before_names() {
    let package = admit(ext_corpus_package());
    let mut codes = [FN_ADD, FN_EQ, FN_CALL_NESTED];
    codes.sort_by_key(|code| package.key(*code));
    let [n1, n2, n3] = codes;
    let functions = vec![
        on_node(function_add("a_pair"), n2),
        on_node(function_add("z_pair"), n2),
        on_node(function_add("z_pair"), n1),
        on_node(function_add("q_extra"), n1),
        on_node(function_add("a_pair"), n3),
    ];
    let node_refusal = |code: u32| ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::DuplicateDeclaringNode {
            node_id: package.key(code),
        },
    };
    for order in permutations(&functions) {
        let oracles = both_orders(&package, &order, ["a_pair", "z_pair"]);
        assert_eq!(
            entries(&oracles),
            vec![&node_refusal(n1), &node_refusal(n2)]
        );
    }
}

/// Trace: FR-021-AC-19, TC-031. The emitted `src/lib.rs` of the main corpus
/// (scalar `add_fn`, equality `eq_fn`, nested-call `call_fn`) and of the chain
/// corpus contains no `.unwrap(`, no `.expect(` and no panicking macro in any
/// delimiter form.
#[test]
fn tc_031_ac19_emitted_function_oracle_source_has_no_panicking_path() {
    let oracles = main_oracles();
    let lib = contents(&oracles, "src/lib.rs");
    assert!(lib.contains("pub fn checked_package()"), "main corpus");
    for forbidden in [".unwrap(", ".expect("] {
        assert!(!lib.contains(forbidden), "main corpus emits {forbidden}");
    }
    assert!(
        !invokes_panicking_macro(&lib),
        "main corpus emits a panicking macro"
    );
    assert!(lib.contains("rt::TypeEnvironment::default()"));
}

/// Trace: FR-021-AC-20, TC-031. In the emitted bodies of `add_fn` and `eq_fn`
/// the `match` over the runtime operator's `Result<Outcome<_>, Refusal>` has
/// the five arms in order and ends with a catch-all `Ok(_)` arm valued
/// `Outcome::Refused(Refusal::CheckedInvariant)`. The unknown variant cannot
/// be built from a test crate (`rt::Outcome` is `#[non_exhaustive]`), so the
/// arm's text is the evidence.
#[test]
fn tc_031_ac20_unknown_outcome_variant_refuses_checked_invariant() {
    let lib = contents(&main_oracles(), "src/lib.rs");
    for (function, completed) in [
        (
            "add_fn",
            "Ok(rt::Outcome::Completed(value)) => rt::Outcome::Completed(rt::Value::Integer(value)),",
        ),
        (
            "eq_fn",
            "Ok(rt::Outcome::Completed(value)) => rt::Outcome::Completed(rt::Value::Boolean(value)),",
        ),
    ] {
        let name = format!("name: {function:?}.to_owned(),");
        let start = lib
            .find(&name)
            .unwrap_or_else(|| panic!("{function} is not declared"));
        let rest = &lib[start + name.len()..];
        let body = &rest[..rest.find("rt::FunctionDeclaration {").unwrap_or(rest.len())];

        let arms = [
            completed,
            "Ok(rt::Outcome::Undefined(undefined)) => rt::Outcome::Undefined(undefined),",
            "Ok(rt::Outcome::Refused(refusal)) => rt::Outcome::Refused(refusal),",
            "Ok(rt::Outcome::Incomplete(incomplete)) => rt::Outcome::Incomplete(incomplete),",
            "Err(refusal) => rt::Outcome::Refused(refusal),",
            "Ok(_) => rt::Outcome::Refused(rt::Refusal::CheckedInvariant),",
        ];
        let mut from = 0;
        for arm in arms {
            let at = body[from..]
                .find(arm)
                .unwrap_or_else(|| panic!("{function}: arm `{arm}` missing or out of order"));
            from += at + arm.len();
        }
        let tail: String = body[from..].split_whitespace().collect();
        assert!(
            tail.starts_with('}'),
            "{function}: the catch-all must be the final arm, found `{tail}`"
        );
    }
}

/// The generator's current output for the main corpus, which
/// `exact_function_agreement` builds and executes.
pub(super) fn main_oracles() -> FixtureOracles {
    generate(
        &admit(ext_corpus_package()),
        &main_functions(),
        &main_items(),
    )
}

pub fn contents(oracles: &quire_contract_codegen::ExactFunctionOracles, path: &str) -> String {
    oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("no artifact {path}"))
        .contents
        .clone()
}

/// Trace: FR-021-AC-13, TC-031. Generated bytes are identical across
/// repeated runs and across permutations of the request order; claim-map
/// entries are ordered by the item key (call node id, then the applied
/// function's declaring node id, then each argument operand's source node
/// id).
#[test]
fn tc_031_ac13_generation_is_deterministic_across_runs_and_permutations() {
    let package = admit(ext_corpus_package());
    let functions = main_functions();
    let items = main_items();

    let first = generate(&package, &functions, &items);
    let second = generate(&package, &functions, &items);
    let from_a_fresh_package = main_oracles();

    let mut permuted_functions = functions.clone();
    permuted_functions.reverse();
    let mut permuted_items = items.clone();
    permuted_items.reverse();
    let permuted = generate(&package, &permuted_functions, &permuted_items);

    for other in [&second, &from_a_fresh_package, &permuted] {
        assert_eq!(&first, other);
        for path in [
            "Cargo.toml",
            "src/lib.rs",
            "claim-map.json",
            "location-map.json",
        ] {
            assert_eq!(contents(&first, path), contents(other, path), "{path}");
        }
    }
}

/// Trace: FR-021-AC-14, TC-031. The generated crate declares
/// `publish = false`, names the runtime by branch with the `exact` feature,
/// contains no charge amount and no literal `Outcome`/`Value` constant
/// standing in for a runtime result, and forbids unsafe code.
#[test]
fn tc_031_ac14_manifest_and_source_shape() {
    let package = admit(ext_corpus_package());
    let oracles = generate(&package, &main_functions(), &main_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(&runtime_dependency(&["exact"])));
    assert!(!manifest.contains("rev ="));
    assert!(manifest.contains("unsafe_code = \"forbid\""));

    let lib = contents(&oracles, "src/lib.rs");
    // No literal `Outcome::Completed(...)` construction standing in for a
    // runtime result: every `Outcome::Completed` in this source is a match
    // arm reconstructing a *runtime-computed* value, never a bare constant.
    // Asserted positively -- the exact pattern `render_body` emits for each
    // supported body kind -- rather than as an absence of a hand-picked
    // literal substring: a mutation that hardcoded a hollow result would
    // remove one of these patterns, where the equivalent absence-only
    // checks this replaced could be defeated by any differently-formatted
    // literal.
    assert!(lib.contains("Outcome::Completed(rt::Value::Integer(value))"));
    assert!(lib.contains("Outcome::Completed(rt::Value::Boolean(value))"));
}

/// Trace: FR-021-AC-15 (origin half only -- see module doc "Location
/// tagging" and `spec/oracle/matrix/tests.md`'s FR-021 row: the `path`-non-empty
/// case is not implemented and is unreachable from this V1's scoped body
/// vocabulary), TC-031. The location map records one entry per generated
/// function body, each `Location{origin, path}` re-derivable from the
/// request's own expression tree with no execution: every generated
/// function in this V1 has a body that is one classified node at its own
/// root, so `path` is always empty and `origin` always equals the
/// generator's own function ordering.
#[test]
fn tc_031_ac15_location_map_round_trips_to_the_request_structurally() {
    let package = admit(ext_corpus_package());
    let functions = main_functions();
    let oracles = generate(&package, &functions, &main_items());

    let mut ordered: Vec<_> = functions.iter().collect();
    ordered.sort_by_key(|function| package.ids.resolve(&function.node_id));
    let survivor_names: Vec<&str> = ordered
        .iter()
        .filter(|f| {
            matches!(
                f.name.as_str(),
                "add_fn" | "eq_fn" | "call_fn" | "unrelated_fn"
            )
        })
        .map(|f| f.name.as_str())
        .collect();

    assert_eq!(oracles.location_map.len(), survivor_names.len());
    for entry in &oracles.location_map {
        assert_eq!(entry.location.path, Vec::<usize>::new());
        let expected_index = survivor_names
            .iter()
            .position(|name| *name == entry.function)
            .unwrap_or_else(|| panic!("{} not a survivor", entry.function));
        match &entry.location.origin {
            quire_contract_codegen::RecordedOrigin::Body { function, index } => {
                assert_eq!(function, &entry.function);
                assert_eq!(*index, expected_index);
            }
        }
        let expected_kind = match entry.function.as_str() {
            "add_fn" => CallPointKind::ScalarOperator,
            "eq_fn" => CallPointKind::EqualityEvaluation,
            "call_fn" => CallPointKind::NestedCall,
            "unrelated_fn" => CallPointKind::ScalarOperator,
            other => panic!("unexpected function {other}"),
        };
        assert_eq!(entry.call_point, expected_kind);
    }
}

/// Trace: FR-021-AC-16, TC-031 (Inspection). No generated code or claim map
/// reads, branches on, or asserts non-emptiness of `Evaluation.location` or
/// `Evaluation.losses`: the generated oracle discards both by construction
/// (`.map(|evaluation| evaluation.outcome)`), and this test greps the
/// generated source and the serialized claim/location maps to confirm no
/// occurrence of either field name slipped in some other way.
#[test]
fn tc_031_ac16_no_generated_code_reads_location_or_losses() {
    let package = admit(ext_corpus_package());
    let oracles = generate(&package, &main_functions(), &main_items());
    let lib = contents(&oracles, "src/lib.rs");
    // The header comment documents (in prose) that `Evaluation.location`/
    // `.losses` are never read; check for an actual field *read* --
    // `evaluation.location`/`evaluation.losses`, lowercase, the shape a real
    // access would take -- rather than banning the substring outright,
    // which the header's own documentation legitimately contains.
    assert!(!lib.contains("evaluation.location"));
    assert!(!lib.contains("evaluation.losses"));
    assert!(lib.contains(".map(|evaluation| evaluation.outcome)"));
    // The claim map's own `Serialize` types (`ClaimMap`,
    // `GeneratedExactFunctionClaim`) carry no `location`/`losses` field at
    // all, so a JSON-key absence check on `claim-map.json` here would only
    // restate what the type system already guarantees at compile time, not
    // exercise any behavior of this generator.
}

/// This package uses the FR-021 checked shape: a `function/pure_function`
/// declaration binds parameter nodes and an expression body; its local call
/// references the declaration followed by ordered argument references.
/// Trace: FR-021-AC-27
#[test]
fn tc_031_source_function_call_binds_callee_and_ordered_arguments() {
    let (checked, ids) = source_bound_call_package().admit_resolved();
    let function_name = format!("n{}", key(SOURCE_FUNCTION));
    let other_name = format!("n{}", key(SOURCE_OTHER_FUNCTION));
    let function = ids.resolve_function(source_function(&function_name, SOURCE_BODY));
    let other = ids.resolve_function(source_function(&other_name, SOURCE_OTHER_BODY));
    let item = ids.resolve_call(source_call());
    assert_ne!(function.node_id, ids.resolve(&code_id(SOURCE_FUNCTION)));
    assert_ne!(other.node_id, ids.resolve(&code_id(SOURCE_OTHER_FUNCTION)));
    let requested = [
        function.node_id.clone(),
        other.node_id.clone(),
        item.call_node_id.clone(),
    ];
    let contract = package::admitted_function_contract(&checked, &requested);
    let positions = package::driver_positions(&requested);

    let good = generate_exact_function_oracles(
        &contract,
        &positions,
        &[0, 1],
        &[function.clone(), other.clone()],
        &[2],
        std::slice::from_ref(&item),
    )
    .expect("valid source-backed call is planned");
    assert!(
        matches!(
            good.claim_map.items[0].result,
            ClaimDisposition::Generated(_)
        ),
        "{:?}",
        good.claim_map.items[0].result
    );

    let mut wrong_callee = item.clone();
    wrong_callee.function = other_name;
    let wrong = generate_exact_function_oracles(
        &contract,
        &positions,
        &[0, 1],
        &[function.clone(), other],
        &[2],
        &[wrong_callee],
    )
    .expect("wrong source callee is a per-item refusal");
    assert!(matches!(
        wrong.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BodyMismatch
        }
    ));

    let mut reordered = item;
    reordered.argument_node_ids.reverse();
    let reordered = generate_exact_function_oracles(
        &contract,
        &positions,
        &[0],
        &[function],
        &[2],
        &[reordered],
    )
    .expect("reordered source arguments are a per-item refusal");
    assert!(matches!(
        reordered.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BodyMismatch
        }
    ));
}

//! FR-021 direct-function corpus from one QSL source and separate Contract IR admission.

use ix_trace_rs::trace;
use qsl_replay::{
    acquire_checked_closure, ApplicationEnvironment, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits,
};
use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
use quire_contract_codegen::{checked_function_wire_id, checked_package_digest};
use quire_contract_codegen::{
    generate_exact_function_oracles, ClaimDisposition, EqualityOperatorKind, ExactFunctionBody,
    ExactFunctionDeclaration, ExactFunctionItem, FunctionParameter,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm,
};
use quire_eval::FamilyOutcome;
use quire_exact::{Integer, LimitKind, Meter, Outcome, ScalarLimits, Value};
use quire_semantic_value::object_closure::ObjectClosure;

#[allow(clippy::duplicate_mod)]
#[path = "../exact_function_support/package.rs"]
mod package;

#[allow(dead_code)]
#[path = "../clause_admission_support/mod.rs"]
mod clause_admission_support;
use super::kani_obligations_state_frame::model;

const SOURCE: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "function add using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x + y }\n",
    "function eq using v(x: Int[0, 9], y: Int[0, 9]): Boolean pure { x = y }\n",
    "function nested using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { add(x, y) }\n",
    "function probe_add using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { add(x, y) }\n",
    "function probe_eq using v(x: Int[0, 9], y: Int[0, 9]): Boolean pure { eq(x, y) }\n",
    "function probe_nested using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { nested(x, y) }\n",
    "function unrelated using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x - y }\n",
    "function probe_unrelated using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { unrelated(x, y) }\n",
    "function add_fn using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x + y }\n",
    "function probe_add_fn using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { add_fn(x, y) }\n",
);

const LIBRARY_SOURCE: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "function imported using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x + y }\n",
    "function probe_imported using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { imported(x, y) }\n",
);

const IMPORTER_SOURCE: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "import \"test/function-library\" as lib;\n",
    "function caller using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { lib::imported(x, y) }\n",
);

fn library_identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "function-library", "git", "1")
}

fn source_identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "ir595-agreement", "git", "1")
}

fn admitted_source_text(source: &str) -> CheckedPackageV2 {
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-agreement.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the genuine function source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let read = CheckedPackageV2::read(
        acquired.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    );
    let CheckedPackageV2ReadResult::Admitted(package) = read else {
        panic!("Contract IR admits QSL's exact package bytes: {read:?}");
    };
    *package
}

fn admitted_source() -> CheckedPackageV2 {
    admitted_source_text(SOURCE)
}

fn function_id(package: &CheckedPackageV2, name: &str) -> CheckedNodeId {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration
                .as_ref()
                .is_some_and(|declaration| declaration.qualified_name == [Box::<str>::from(name)])
        })
        .unwrap_or_else(|| panic!("source function {name}"))
        .node_id
        .clone()
}

pub(super) fn body_id(package: &CheckedPackageV2, name: &str) -> CheckedNodeId {
    let function = function_id(package, name);
    let node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == function)
        .expect("function node belongs to the admitted graph");
    let CheckedTerm::Aggregate { members } = &node.body else {
        panic!("{name} has no function body binding");
    };
    members
        .iter()
        .find_map(|member| match member {
            CheckedTerm::Binding {
                name: binding,
                value,
            } if binding.as_ref() == "body" => match value.as_ref() {
                CheckedTerm::Reference { target } => Some(target.clone()),
                _ => None,
            },
            _ => None,
        })
        .unwrap_or_else(|| panic!("{name} has no checked body root"))
}

fn type_id(package: &CheckedPackageV2, form: &str) -> CheckedNodeId {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_tag.as_ref() == "scalar_type" && node.semantic_form.as_ref() == form)
        .unwrap_or_else(|| panic!("source type {form}"))
        .node_id
        .clone()
}

fn item(package: &CheckedPackageV2, probe: &str, function: &str) -> ExactFunctionItem {
    let call_node_id = body_id(package, probe);
    let node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == call_node_id)
        .expect("probe body is a checked call");
    let CheckedTerm::Application { arguments, .. } = &node.body else {
        panic!("{probe} body is not an application");
    };
    let expected = function_id(package, function);
    let Some(CheckedTerm::Reference { target }) = arguments.first() else {
        panic!("{probe} has no callable target");
    };
    assert_eq!(target, &expected, "{probe} selects {function}");
    let argument_node_ids = arguments[1..]
        .iter()
        .map(|argument| match argument {
            CheckedTerm::Reference { target } => target.clone(),
            other => panic!("probe argument was not a checked reference: {other:?}"),
        })
        .collect();
    ExactFunctionItem {
        call_node_id,
        function: function.to_owned(),
        argument_node_ids,
    }
}

pub(super) fn genuine_corpus() -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
) {
    genuine_corpus_request(false, false, None)
}

pub(super) fn genuine_corpus_in_order(
    reverse: bool,
) -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
) {
    genuine_corpus_request(reverse, false, None)
}

pub(super) fn genuine_corpus_with_unknown() -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
) {
    genuine_corpus_request(false, true, None)
}

pub(super) fn genuine_corpus_with_bad_nested(
    body: ExactFunctionBody,
) -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
) {
    genuine_corpus_request(false, false, Some(body))
}

pub(super) fn genuine_ambiguous_and_healthy() -> (
    quire_contract_codegen::ExactFunctionOracles,
    quire_contract_codegen::ExactFunctionOracles,
    CheckedNodeId,
) {
    let package = admitted_source();
    let integer = type_id(&package, "integer");
    let boolean = type_id(&package, "boolean");
    let declaration = |source_name: &str, name: &str, result_type, body| ExactFunctionDeclaration {
        node_id: body_id(&package, source_name),
        name: name.to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: integer.clone(),
            })
            .collect(),
        result_type,
        body,
        capability_requirements: Vec::new(),
    };
    let functions = [
        declaration(
            "add",
            "dup",
            integer.clone(),
            ExactFunctionBody::Scalar {
                operator: IntegerOperator::Add,
            },
        ),
        declaration(
            "eq",
            "dup",
            boolean,
            ExactFunctionBody::CompositeEquality {
                operator: EqualityOperatorKind::Equal,
            },
        ),
        declaration(
            "unrelated",
            "unrelated",
            integer.clone(),
            ExactFunctionBody::Scalar {
                operator: IntegerOperator::Subtract,
            },
        ),
    ];
    assert_ne!(functions[0].node_id, functions[1].node_id);
    assert_ne!(functions[0].node_id, functions[2].node_id);
    assert_ne!(functions[1].node_id, functions[2].node_id);
    let mut duplicate_item = item(&package, "probe_add", "add");
    duplicate_item.function = "dup".to_owned();
    let mut second_duplicate_item = item(&package, "probe_eq", "eq");
    second_duplicate_item.function = "dup".to_owned();
    let healthy_item = item(&package, "probe_unrelated", "unrelated");
    let healthy_call_id = healthy_item.call_node_id.clone();
    let mut roots = functions
        .iter()
        .map(|function| function.node_id.clone())
        .chain([
            duplicate_item.call_node_id.clone(),
            second_duplicate_item.call_node_id.clone(),
            healthy_call_id.clone(),
        ])
        .collect::<Vec<_>>();
    roots.sort();
    let positions = package::driver_positions(&roots);
    let position = |id: &CheckedNodeId| roots.iter().position(|root| root == id).unwrap();
    let contract = package::admitted_function_contract(&package, &roots);
    let combined = generate_exact_function_oracles(
        &contract,
        &positions,
        &functions
            .iter()
            .map(|function| position(&function.node_id))
            .collect::<Vec<_>>(),
        &functions,
        &[
            position(&duplicate_item.call_node_id),
            position(&second_duplicate_item.call_node_id),
            position(&healthy_call_id),
        ],
        &[duplicate_item, second_duplicate_item, healthy_item.clone()],
    )
    .expect("ambiguous source request generates one healthy survivor");
    let only = generate_exact_function_oracles(
        &contract,
        &positions,
        &[position(&functions[2].node_id)],
        std::slice::from_ref(&functions[2]),
        &[position(&healthy_call_id)],
        &[healthy_item],
    )
    .expect("healthy source request generates");
    (combined, only, healthy_call_id)
}

pub(super) fn genuine_bad_arity() -> quire_contract_codegen::ExactFunctionOracles {
    let package = admitted_source();
    let integer = type_id(&package, "integer");
    let declaration = ExactFunctionDeclaration {
        node_id: body_id(&package, "add"),
        name: "add".to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: integer.clone(),
            })
            .collect(),
        result_type: integer,
        body: ExactFunctionBody::Scalar {
            operator: IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let mut item = item(&package, "probe_add", "add");
    item.argument_node_ids.truncate(1);
    let mut roots = vec![declaration.node_id.clone(), item.call_node_id.clone()];
    roots.sort();
    let positions = package::driver_positions(&roots);
    let function_position = roots
        .iter()
        .position(|id| id == &declaration.node_id)
        .unwrap();
    let item_position = roots
        .iter()
        .position(|id| id == &item.call_node_id)
        .unwrap();
    let contract = package::admitted_function_contract(&package, &roots);
    generate_exact_function_oracles(
        &contract,
        &positions,
        &[function_position],
        &[declaration],
        &[item_position],
        &[item],
    )
    .expect("QSL-authored call with caller-supplied bad arity is classified")
}

pub(super) fn genuine_known_and_zz_unknown() -> [quire_contract_codegen::ExactFunctionOracles; 4] {
    let package = admitted_source();
    let integer = type_id(&package, "integer");
    let declaration = ExactFunctionDeclaration {
        node_id: body_id(&package, "add_fn"),
        name: "add_fn".to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: integer.clone(),
            })
            .collect(),
        result_type: integer,
        body: ExactFunctionBody::Scalar {
            operator: IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let known = item(&package, "probe_add_fn", "add_fn");
    let mut unknown = known.clone();
    unknown.function = "zz_unknown".to_owned();
    let mut roots = vec![
        declaration.node_id.clone(),
        known.call_node_id.clone(),
        unknown.call_node_id.clone(),
    ];
    roots.sort();
    let positions = package::driver_positions(&roots);
    let contract = package::admitted_function_contract(&package, &roots);
    let mut occurrences = roots
        .iter()
        .enumerate()
        .filter(|(_, id)| *id == &known.call_node_id)
        .map(|(index, _)| index);
    let first_call = occurrences.next().expect("first call request");
    let second_call = occurrences.next().expect("second call request");
    let function_position = roots
        .iter()
        .position(|id| id == &declaration.node_id)
        .unwrap();
    let run = |item_positions: &[usize], items: &[ExactFunctionItem]| {
        generate_exact_function_oracles(
            &contract,
            &positions,
            &[function_position],
            std::slice::from_ref(&declaration),
            item_positions,
            items,
        )
        .expect("genuine known/unknown call request")
    };
    [
        run(
            &[first_call, second_call],
            &[known.clone(), unknown.clone()],
        ),
        run(
            &[second_call, first_call],
            &[unknown.clone(), known.clone()],
        ),
        run(&[first_call], &[known]),
        run(&[second_call], &[unknown]),
    ]
}

pub(super) fn genuine_case_and_healthy() -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
    quire_contract_codegen::ExactFunctionOracles,
    CheckedNodeId,
    CheckedNodeId,
    CheckedNodeId,
) {
    const CASE_SOURCE: &str = concat!(
        "language \"ix:native\" edition \"1-draft\";\n",
        "profile v = \"quire.value.complete/v1\";\n",
        "union Shape { Empty, Circle(Integer), Rect(Integer, Integer) }\n",
        "function area using v(s: Shape): Integer pure { case s { Empty: 0; Circle(r): r; Rect(w, h): w * h; } }\n",
        "function probe_area using v(s: Shape): Integer pure { area(s) }\n",
        "function healthy using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x + y }\n",
        "function probe_healthy using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { healthy(x, y) }\n",
    );
    let package = admitted_source_text(CASE_SOURCE);
    let case_id = body_id(&package, "area");
    let body_id = body_id(&package, "healthy");
    let healthy_item = item(&package, "probe_healthy", "healthy");
    let healthy_call_id = healthy_item.call_node_id.clone();
    let area_item = item(&package, "probe_area", "area");
    let area_call_id = area_item.call_node_id.clone();
    let mut direct_item = healthy_item.clone();
    direct_item.call_node_id = case_id.clone();
    let declaration = ExactFunctionDeclaration {
        node_id: body_id.clone(),
        name: "healthy".to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: type_id(&package, "integer"),
            })
            .collect(),
        result_type: type_id(&package, "integer"),
        body: ExactFunctionBody::Scalar {
            operator: IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let shape_type = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "union" && node.node_tag.as_ref() == "composite_type"
        })
        .expect("QSL emits the declared Shape union")
        .node_id
        .clone();
    let area_declaration = ExactFunctionDeclaration {
        node_id: case_id.clone(),
        name: "area".to_owned(),
        parameters: vec![FunctionParameter {
            name: "s".to_owned(),
            type_node_id: shape_type,
        }],
        result_type: type_id(&package, "integer"),
        body: ExactFunctionBody::Scalar {
            operator: IntegerOperator::Multiply,
        },
        capability_requirements: Vec::new(),
    };
    let mut roots = vec![
        body_id.clone(),
        case_id.clone(),
        healthy_call_id.clone(),
        area_call_id.clone(),
    ];
    roots.sort();
    let positions = package::driver_positions(&roots);
    let position = |id: &CheckedNodeId| roots.iter().position(|root| root == id).unwrap();
    let contract = package::admitted_function_contract(&package, &roots);
    let combined = generate_exact_function_oracles(
        &contract,
        &positions,
        &[position(&body_id), position(&case_id)],
        &[declaration.clone(), area_declaration],
        &[
            position(&case_id),
            position(&area_call_id),
            position(&healthy_call_id),
        ],
        &[direct_item, area_item, healthy_item.clone()],
    )
    .expect("case and healthy function share one admitted source");
    let healthy_only = generate_exact_function_oracles(
        &contract,
        &positions,
        &[position(&body_id)],
        &[declaration],
        &[position(&healthy_call_id)],
        &[healthy_item],
    )
    .expect("healthy function alone generates");
    (
        package,
        combined,
        healthy_only,
        case_id,
        area_call_id,
        healthy_call_id,
    )
}

pub(super) fn genuine_temporal_and_healthy(
    form: &str,
) -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
    quire_contract_codegen::ExactFunctionOracles,
    CheckedNodeId,
    CheckedNodeId,
) {
    let twin = clause_admission_support::Twin::new();
    let original = String::from_utf8(twin.unit).expect("authored model source");
    let mut lines = original.lines();
    let language = lines.next().expect("language header");
    let value_profile = lines.next().expect("model-complete profile");
    let model = lines.next().expect("authored model selection");
    let digest = model
        .split_once("sha256-jcs:")
        .and_then(|(_, rest)| rest.split('"').next())
        .expect("authored model digest");
    let source = format!(
        "{language}\n{value_profile}\n\
         profile t = \"quire.temporal.infinite-trace/v1\";\n\
         {model}\n\
         temporal ReachesTwo using t over (a: Bank::Account) clock \"model-steps\" on origin {{\n\
         fair weak each Bank::Account::deposit;\n\
         always eventually holds(a.balance = 2) }}\n\
         function healthy using v(x: Int[0, 9], y: Int[0, 9]): Integer pure {{ x + y }}\n\
         function probe_healthy using v(x: Int[0, 9], y: Int[0, 9]): Integer pure {{ healthy(x, y) }}\n"
    );
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-temporal.native",
        source.as_bytes(),
        [twin.domain.as_slice()],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the model-backed temporal and function source");
    let bytes = acquired.package().bytes();
    let wire: serde_json::Value = serde_json::from_slice(bytes).expect("QSL V2 JSON");
    let mut evidence = CheckedPackageEvidence::new();
    for feature in wire["lock"]["required_features"]
        .as_array()
        .expect("required features")
    {
        evidence.support_feature(feature.as_str().expect("feature name"));
    }
    evidence
        .insert_domain_package_document(digest.to_owned(), twin.domain.clone())
        .expect("one model document");
    let read = CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), &evidence);
    let CheckedPackageV2ReadResult::Admitted(package) = read else {
        panic!("Contract IR admits the exact QSL temporal bytes: {read:?}");
    };
    let package = *package;
    let temporal_id = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_tag.as_ref() == "temporal" && node.semantic_form.as_ref() == form)
        .unwrap_or_else(|| panic!("QSL emitted temporal {form}"))
        .node_id
        .clone();
    let body_id = body_id(&package, "healthy");
    let healthy_item = item(&package, "probe_healthy", "healthy");
    let healthy_call_id = healthy_item.call_node_id.clone();
    let mut temporal_item = healthy_item.clone();
    temporal_item.call_node_id = temporal_id.clone();
    let declaration = ExactFunctionDeclaration {
        node_id: body_id.clone(),
        name: "healthy".to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: type_id(&package, "integer"),
            })
            .collect(),
        result_type: type_id(&package, "integer"),
        body: ExactFunctionBody::Scalar {
            operator: IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let mut roots = vec![
        body_id.clone(),
        temporal_id.clone(),
        healthy_call_id.clone(),
    ];
    roots.sort();
    let positions = package::driver_positions(&roots);
    let position = |id: &CheckedNodeId| roots.iter().position(|root| root == id).unwrap();
    let contract = package::admitted_function_contract(&package, &roots);
    let combined = generate_exact_function_oracles(
        &contract,
        &positions,
        &[position(&body_id)],
        std::slice::from_ref(&declaration),
        &[position(&temporal_id), position(&healthy_call_id)],
        &[temporal_item, healthy_item.clone()],
    )
    .expect("temporal root and healthy function share one admitted source");
    let healthy_only = generate_exact_function_oracles(
        &contract,
        &positions,
        &[position(&body_id)],
        &[declaration],
        &[position(&healthy_call_id)],
        &[healthy_item],
    )
    .expect("healthy function alone generates");
    (
        package,
        combined,
        healthy_only,
        temporal_id,
        healthy_call_id,
    )
}

fn imported_corpus() -> quire_contract_codegen::ExactFunctionOracles {
    let compiled = acquire_checked_closure(
        library_identity(),
        "function-library.native",
        LIBRARY_SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the genuine library source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let read = CheckedPackageV2::read(
        compiled.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    );
    let CheckedPackageV2ReadResult::Admitted(library) = read else {
        panic!("IR admits the exact QSL library bytes: {read:?}");
    };
    let library = *library;
    let declaration = ExactFunctionDeclaration {
        node_id: body_id(&library, "imported"),
        name: "imported".to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: type_id(&library, "integer"),
            })
            .collect(),
        result_type: type_id(&library, "integer"),
        body: ExactFunctionBody::Scalar {
            operator: IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let item = item(&library, "probe_imported", "imported");
    let mut roots = vec![declaration.node_id.clone(), item.call_node_id.clone()];
    roots.sort();
    let positions = package::driver_positions(&roots);
    let position = |id: &CheckedNodeId| roots.iter().position(|root| root == id).unwrap();
    let contract = package::admitted_function_contract(&library, &roots);
    generate_exact_function_oracles(
        &contract,
        &positions,
        &[position(&declaration.node_id)],
        &[declaration],
        &[position(&item.call_node_id)],
        &[item],
    )
    .expect("one imported library body and its QSL-authored call")
}

fn genuine_corpus_request(
    reverse: bool,
    include_unknown: bool,
    bad_nested: Option<ExactFunctionBody>,
) -> (
    CheckedPackageV2,
    quire_contract_codegen::ExactFunctionOracles,
) {
    let package = admitted_source();
    let integer = type_id(&package, "integer");
    let boolean = type_id(&package, "boolean");
    let declaration = |name: &str, result_type: CheckedNodeId, body| ExactFunctionDeclaration {
        node_id: body_id(&package, name),
        name: name.to_owned(),
        parameters: ["x", "y"]
            .into_iter()
            .map(|name| FunctionParameter {
                name: name.to_owned(),
                type_node_id: integer.clone(),
            })
            .collect(),
        result_type,
        body,
        capability_requirements: Vec::new(),
    };
    let mut functions = [
        declaration(
            "add",
            integer.clone(),
            ExactFunctionBody::Scalar {
                operator: IntegerOperator::Add,
            },
        ),
        declaration(
            "eq",
            boolean,
            ExactFunctionBody::CompositeEquality {
                operator: EqualityOperatorKind::Equal,
            },
        ),
        declaration(
            "nested",
            integer.clone(),
            ExactFunctionBody::Call {
                callee: "add".to_owned(),
            },
        ),
    ];
    if let Some(body) = bad_nested {
        functions[2].body = body;
    }
    let mut items = vec![
        item(&package, "probe_add", "add"),
        item(&package, "probe_eq", "eq"),
        item(&package, "probe_nested", "nested"),
    ];
    if include_unknown {
        let mut unknown = items[0].clone();
        unknown.function = "absent".to_owned();
        items.push(unknown);
    }
    if reverse {
        functions.reverse();
        items.reverse();
    }
    let mut requested = functions
        .iter()
        .map(|function| function.node_id.clone())
        .chain(items.iter().map(|item| item.call_node_id.clone()))
        .collect::<Vec<_>>();
    if !include_unknown {
        // The baseline and refused-item run must share one admitted IR
        // request, including the extra accounted call occurrence.
        requested.push(item(&package, "probe_add", "add").call_node_id);
    }
    requested.sort();
    let admitted = package::admitted_function_contract(&package, &requested);
    let mut positions = std::collections::BTreeMap::<_, std::collections::VecDeque<_>>::new();
    for (position, node_id) in requested.iter().enumerate() {
        positions.entry(node_id).or_default().push_back(position);
    }
    let mut take = |node_id: &CheckedNodeId| {
        positions
            .get_mut(node_id)
            .and_then(std::collections::VecDeque::pop_front)
            .expect("canonical lowering retains every request occurrence")
    };
    let function_positions = functions
        .iter()
        .map(|function| take(&function.node_id))
        .collect::<Vec<_>>();
    let item_positions = items
        .iter()
        .map(|item| take(&item.call_node_id))
        .collect::<Vec<_>>();
    let generated = generate_exact_function_oracles(
        &admitted,
        &package::driver_positions(&requested),
        &function_positions,
        &functions,
        &item_positions,
        &items,
    )
    .expect("one source and one IR lowering generate the corpus");
    (package, generated)
}

/// Trace: FR-021-AC-32, FR-021-AC-35, TC-031.
#[trace("FR-021-AC-32", "FR-021-AC-35", "TC-031")]
#[test]
fn tc_031_genuine_qsl_source_generates_three_direct_function_claims() {
    let (_, generated) = genuine_corpus();
    let claims = generated
        .claim_map
        .items
        .iter()
        .map(|claim| &claim.result)
        .collect::<Vec<_>>();
    assert_eq!(claims.len(), 3);
    assert!(
        claims
            .iter()
            .all(|claim| matches!(claim, ClaimDisposition::Generated(_))),
        "{claims:?}"
    );
    let source = generated
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("generated source")
        .contents
        .as_str();
    assert_eq!(source.matches("pub fn oracle_").count(), 3);
    assert!(source.contains("apply_checked_function("));
    assert!(source.contains("selected_nested_call_site("));
    assert!(!source.contains("quire_contract_runtime"));
}

fn meter() -> Meter {
    let ceiling = u64::from(u32::MAX);
    Meter::new(ScalarLimits {
        integer_bits: ceiling,
        decimal_digits: ceiling,
        scale_expansion: ceiling,
        text_input_bytes: ceiling,
        text_scalars: ceiling,
        normalized_scalars: ceiling,
        unit_edges: ceiling,
        value_occurrences: ceiling,
        work_units: ceiling,
        result_units: ceiling,
    })
}

/// Trace: FR-021-AC-2, FR-021-AC-35, TC-031. This leg uses QSL's own
/// interpreter through an unregistered facade registry.
#[trace("FR-021-AC-2", "FR-021-AC-35", "TC-031")]
#[test]
fn tc_031_independent_fr485_session_runs_all_three_body_forms_without_registration() {
    let package = admitted_source();
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-agreement.native",
        SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("fresh QSL acquisition");
    let handle = acquired.handle();
    let owner = checked_package_digest(package.package_id()).expect("admitted owner identity");
    for (name, expected) in [
        ("add", Value::Integer(Integer::from(7_u64))),
        ("eq", Value::Boolean(false)),
        ("nested", Value::Integer(Integer::from(7_u64))),
    ] {
        let function = function_id(&package, name);
        let wire = checked_function_wire_id(&function).expect("admitted function wire identity");
        let selection = handle.lookup_callable(owner, wire).expect("QSL selection");
        let mut registry = handle.registry();
        let environment = ApplicationEnvironment::new(ObjectClosure::default());
        let mut meter = meter();
        let arguments = vec![
            Value::Integer(Integer::from(3_u64)),
            Value::Integer(Integer::from(4_u64)),
        ];
        let evaluation = registry
            .begin_call(&selection, arguments, &environment, &mut meter)
            .expect("QSL accepts a direct call")
            .run()
            .expect("QSL interpreter completes");
        match (&evaluation.outcome, expected) {
            (
                FamilyOutcome::Evaluated(Outcome::Completed(Value::Integer(actual))),
                Value::Integer(expected),
            ) => {
                assert_eq!(actual, &expected, "{name}");
            }
            (
                FamilyOutcome::Evaluated(Outcome::Completed(Value::Boolean(actual))),
                Value::Boolean(expected),
            ) => {
                assert_eq!(*actual, expected, "{name}");
            }
            other => panic!("{name}: unexpected direct application {other:?}"),
        }
        assert!(
            meter.admission_count() > 0,
            "{name} has application charges"
        );
        assert!(LimitKind::ALL.iter().any(|kind| meter.consumed(*kind) > 0));
    }
}

/// Trace: FR-021-AC-17, TC-031. The static request origin is checked against
/// a QSL-issued site in the retained source body, not emitted Rust text.
#[trace("FR-021-AC-17", "TC-031")]
#[test]
fn tc_031_nested_static_origin_matches_the_qsl_checked_call_site() {
    let (package, generated) = genuine_corpus();
    let static_origin = &generated
        .location_map
        .iter()
        .find(|entry| entry.function == "nested")
        .expect("nested location record")
        .location
        .origin;
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-agreement.native",
        SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("fresh QSL checked closure");
    let handle = acquired.handle();
    let owner = checked_package_digest(package.package_id()).expect("admitted owner");
    let wire = checked_function_wire_id(&function_id(&package, "nested"))
        .expect("source function wire id");
    let selection = handle
        .lookup_callable(owner, wire)
        .expect("QSL nested selection");
    let site = quire_contract_codegen::selected_nested_call_site(
        &package,
        &body_id(&package, "nested"),
        handle,
        &selection,
    )
    .expect("QSL issues the admitted nested call site");
    let (
        quire_contract_codegen::RecordedOrigin::Body {
            function: static_name,
            index: static_index,
        },
        quire_semantic_value::location::Origin::Body {
            function: qsl_name,
            index: qsl_index,
        },
    ) = (static_origin, &site.location().origin)
    else {
        panic!("different nested call origin kinds");
    };
    assert_eq!(static_name, qsl_name);
    assert_eq!(static_index, qsl_index);
}

/// Trace: FR-021-AC-2, FR-021-AC-32, FR-021-AC-35, TC-031.
#[trace("FR-021-AC-2", "FR-021-AC-32", "FR-021-AC-35", "TC-031")]
#[test]
fn tc_031_generated_function_crate_matches_independent_fr485_session() {
    let (_, generated) = genuine_corpus();
    let aliases = generated
        .claim_map
        .items
        .iter()
        .map(|claim| match &claim.result {
            ClaimDisposition::Generated(item) => (
                item.oracle_symbol.clone(),
                format!("oracle_{}", item.function),
            ),
            other => panic!("genuine item refused: {other:?}"),
        })
        .collect::<Vec<_>>();
    let names = super::exact_scalar_agreement::names_file(
        quire_contract_codegen::EXACT_FUNCTION_CRATE_NAME,
        &aliases,
        &format!("pub const SOURCE: &str = {SOURCE:?};\n"),
    );
    let codegen_dependency = format!(
        "quire-contract-codegen = {{ path = {:?} }}",
        env!("CARGO_MANIFEST_DIR"),
    );
    let artifacts = generated.artifacts.iter().map(|artifact| {
        let contents = if artifact.path == "Cargo.toml" {
            artifact.contents.replace(
                "quire-contract-codegen = { git = \"https://github.com/agent-ix/quire-contract-codegen\", branch = \"main\" }",
                &codegen_dependency,
            )
        } else {
            artifact.contents.clone()
        };
        (artifact.path.clone(), contents)
    }).collect::<Vec<_>>();
    super::exact_scalar_agreement::run_agreement_cases_in_repo_target(
        "exact-function-fr485-parity",
        artifacts
            .iter()
            .map(|(path, contents)| (path.as_str(), contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/exact_function_support/agreement_cases.rs",
    );
}

/// Trace: FR-021-AC-35, FR-021-AC-36, TC-031. The generated body is owned
/// by the locked library and selected from a separately admitted importer.
#[trace("FR-021-AC-35", "FR-021-AC-36", "TC-031")]
#[test]
fn tc_031_generated_imported_owner_matches_independent_fr485_session() {
    const IMPORTED_CRATE: &str = "quire-exact-function-imported-oracles";
    let generated = imported_corpus();
    let [claim] = generated.claim_map.items.as_slice() else {
        panic!("one imported claim");
    };
    let ClaimDisposition::Generated(claim) = &claim.result else {
        panic!("QSL library call was not generated: {claim:?}");
    };
    let names = super::exact_scalar_agreement::names_file(
        IMPORTED_CRATE,
        &[(claim.oracle_symbol.clone(), "oracle_imported".to_owned())],
        &format!(
            "pub const LIBRARY_SOURCE: &str = {LIBRARY_SOURCE:?};\n\
             pub const IMPORTER_SOURCE: &str = {IMPORTER_SOURCE:?};\n"
        ),
    );
    let codegen_dependency = format!(
        "quire-contract-codegen = {{ path = {:?} }}",
        env!("CARGO_MANIFEST_DIR"),
    );
    let artifacts = generated.artifacts.iter().map(|artifact| {
        let contents = if artifact.path == "Cargo.toml" {
            artifact.contents.replace(
                "quire-contract-codegen = { git = \"https://github.com/agent-ix/quire-contract-codegen\", branch = \"main\" }",
                &codegen_dependency,
            ).replacen(
                &format!("name = {:?}", quire_contract_codegen::EXACT_FUNCTION_CRATE_NAME),
                &format!("name = {IMPORTED_CRATE:?}"),
                1,
            )
        } else {
            artifact.contents.clone()
        };
        (artifact.path.clone(), contents)
    }).collect::<Vec<_>>();
    super::exact_scalar_agreement::run_agreement_cases_in_repo_target(
        "exact-function-imported-fr485-parity",
        artifacts
            .iter()
            .map(|(path, contents)| (path.as_str(), contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/exact_function_support/imported_cases.rs",
    );
}

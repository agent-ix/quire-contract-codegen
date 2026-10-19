//! Admitted CheckedPackage V2 fixtures for function-application oracle
//! generation (FR-021).
//!
//! Built directly on `composite_equality_support::package`'s own corpus
//! (`#[path]`-included below, a fresh, independent module instance in this
//! test binary): its `T_BOOLEAN`/`T_INTEGER` scalar types, its `REF_TYPE`
//! reference composite, and its `M_BARE`/`S_BARE` model/state nodes are
//! exactly what FR-021's own AC-10/AC-11 need, and its
//! `application_code`/`binary_body` machinery computes real,
//! `validate_application_keys`-conformant node ids -- reused here rather
//! than reimplemented, so this module never hand-rolls a node id that
//! bypasses the real V2 reader.
//!
//! FR-021 adds its own function-declaration and call-item nodes on top of
//! that base. Scalar function roots carry a checked `quire.op.integer.add`
//! operation and Integer result; the equality root retains its equality
//! operation, and call-item roots retain their call form. The function
//! classifier checks scalar operation identity and body arity against the
//! caller-declared [`quire_contract_codegen::ExactFunctionBody`].

#![allow(dead_code)]

#[path = "../composite_equality_support/package.rs"]
mod ce;
pub use ce::*;

use quire_contract_codegen::{
    ContractRequestPosition, ExactFunctionBody, ExactFunctionDeclaration, ExactFunctionItem,
    FunctionParameter,
};
use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageReadLimits, CheckedPackageV2,
    CompleteLoweringProfileV2, ContractDependencies, ContractPackageReadResult, ContractPackageV1,
    ContractPackageWireV1, LoweringProfileRef,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// Lower all function bodies and call items once, then admit the one FR-330 value.
pub fn admitted_function_contract(
    checked: &CheckedPackageV2,
    requested: &[CheckedNodeId],
) -> ContractPackageV1 {
    let profile = CompleteLoweringProfileV2 {
        reference: LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.exact-function/v1".into(),
        },
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Function,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds: false,
        work_limit: 65_536,
    };
    let lowered = checked.lower(requested, &profile);
    match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => *package,
        other => panic!("Contract IR must admit its FR-330 output: {other:?}"),
    }
}

/// Mark every lowering root in this function fixture as one driver request.
pub fn driver_positions(requested: &[CheckedNodeId]) -> Vec<ContractRequestPosition> {
    requested
        .iter()
        .cloned()
        .map(|node_id| ContractRequestPosition::DriverItem {
            node_id,
            result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect()
}

impl FixtureIds {
    /// Resolves every checked node key in one declaration against its admitted fixture.
    pub fn resolve_function(
        &self,
        mut function: ExactFunctionDeclaration,
    ) -> ExactFunctionDeclaration {
        function.node_id = self.resolve(&function.node_id);
        for parameter in &mut function.parameters {
            parameter.type_node_id = self.resolve(&parameter.type_node_id);
        }
        function.result_type = self.resolve(&function.result_type);
        function
    }

    /// Resolves a requested call and its arguments against the same admitted fixture.
    pub fn resolve_call(&self, mut item: ExactFunctionItem) -> ExactFunctionItem {
        item.call_node_id = self.resolve(&item.call_node_id);
        item.argument_node_ids = item
            .argument_node_ids
            .iter()
            .map(|argument| self.resolve(argument))
            .collect();
        item
    }
}

// ---------------------------------------------------------------------------
// FR-021's own node codes
// ---------------------------------------------------------------------------

/// `f_add(a, b)`: `Scalar { IntegerOperator::Add }`, two `T_INTEGER`
/// parameters, `T_INTEGER` result.
pub const FN_ADD: u32 = 500;
/// `f_eq(a, b)`: `CompositeEquality { Equal }`, two `T_INTEGER` parameters,
/// `T_BOOLEAN` result.
pub const FN_EQ: u32 = 501;
/// `f_call(a, b)`: `Call { callee: "f_add" }`, two `T_INTEGER` parameters,
/// `T_INTEGER` result -- nests into [`FN_ADD`].
pub const FN_CALL_NESTED: u32 = 502;
/// Same node [`FN_CALL_NESTED`] reuses, declared instead with `Scalar` body
/// kind: its real `semantic_form` is `"call"`, so a `Scalar`-declared body
/// against it is a `FormMismatch` -- AC-12's "a form none of this
/// generator's classifiers admits" without a dedicated malformed node.
pub const FN_FORM_MISMATCH_NODE: u32 = FN_CALL_NESTED;
/// `f_dangling(a, b)`: `Call { callee: "not_declared" }` -- AC-12's dangling
/// nested-call case.
pub const FN_DANGLING_CALL: u32 = 503;
/// `f_ref(a)`: one parameter typed `REF_TYPE` (a `reference` composite) --
/// AC-10.
pub const FN_REF_PARAM: u32 = 504;
/// Valid scalar-signature call target used by the model-root classifier case.
pub const FN_MODEL_PARAM: u32 = 505;
/// Valid scalar-signature call target used by the state-root classifier case.
pub const FN_STATE_PARAM: u32 = 506;
/// `f_capability(a, b)`: `Scalar { Add }` with a non-empty
/// `capability_requirements` -- AC-6.
pub const FN_CAPABILITY: u32 = 507;
/// `f_unrelated(a, b)`: `Scalar { Add }`, no relation to any refused
/// function -- AC-12's "without changing an unrelated ... function's
/// items" isolation control.
pub const FN_UNRELATED: u32 = 508;

/// One `call` expression node per requested item.
pub const ITEM_CALL_ADD: u32 = 520;
pub const ITEM_CALL_EQ: u32 = 521;
pub const ITEM_CALL_NESTED: u32 = 522;
pub const ITEM_CALL_UNRELATED: u32 = 523;
/// Names a function absent from every request's own declarations.
pub const ITEM_CALL_UNKNOWN_FUNCTION: u32 = 524;
/// Classifier-only source call for the model root. Kept distinct from the
/// corpus call ID because the shared fixture registry is process-wide.
pub const ITEM_CALL_SOURCE_MODEL: u32 = 534;
/// Classifier-only source call for the state root.
pub const ITEM_CALL_SOURCE_STATE: u32 = 535;

const BODY_PARAMETER_BASE: u32 = 2000;

/// A genuinely checked FR-021 package used for source-binding adversarial
/// cases. Unlike `ext_corpus_package`, this declaration is a `function`
/// node whose parameter and body bindings point to separate checked nodes.
pub const SOURCE_PARAM_A: u32 = 2600;
pub const SOURCE_PARAM_B: u32 = 2601;
pub const SOURCE_BODY: u32 = 2602;
pub const SOURCE_FUNCTION: u32 = 2603;
pub const SOURCE_OTHER_FUNCTION: u32 = 2604;
pub const SOURCE_CALL: u32 = 2605;
pub const SOURCE_OTHER_BODY: u32 = 2606;

fn source_reference(code: u32) -> serde_json::Value {
    let id = code_id(code);
    serde_json::json!({
        "term": "reference",
        "target": {"domain": id.domain, "digest": id.digest}
    })
}

/// Parameter and body fixture IDs for the actual source-bound declarations below.
const FUNCTION_PARAMETER_BASE: u32 = 50_000;
const FUNCTION_BODY_BASE: u32 = 70_000;
const ITEM_ARGUMENT_BASE: u32 = 100_000;

pub fn function_parameter_code(function_code: u32, index: usize) -> u32 {
    FUNCTION_PARAMETER_BASE + function_code * 2 + u32::try_from(index).expect("parameter index")
}

fn function_body_code(function_code: u32) -> u32 {
    FUNCTION_BODY_BASE + function_code
}

pub fn item_argument_code(item_code: u32, index: usize) -> u32 {
    ITEM_ARGUMENT_BASE + item_code * 2 + u32::try_from(index).expect("argument index")
}

fn reference_code(code: u32) -> serde_json::Value {
    serde_json::json!({"term": "reference", "target": {"domain": NODE_DOMAIN, "digest": key(code)}})
}

fn application(
    operator: &str,
    operation: serde_json::Value,
    result_type: u32,
    arguments: Vec<serde_json::Value>,
) -> serde_json::Value {
    serde_json::json!({
        "term": "application",
        "operator": operator,
        "operation": operation,
        "result_type": {"domain": NODE_DOMAIN, "digest": key(result_type)},
        "arguments": arguments,
    })
}

fn integer_add_body(arguments: Vec<serde_json::Value>) -> serde_json::Value {
    application(
        "binary",
        serde_json::json!({
            "identity": "quire.op.integer.add", "laws": [], "mode": null,
            "member": null, "leaves": []
        }),
        T_INTEGER,
        arguments,
    )
}

fn call_body(function_code: u32, argument_codes: &[u32], result_type: u32) -> serde_json::Value {
    application(
        "call",
        serde_json::json!({
            "identity": "quire.op.function.call", "laws": [], "mode": null,
            "member": null, "leaves": []
        }),
        result_type,
        std::iter::once(reference_code(function_code))
            .chain(argument_codes.iter().copied().map(reference_code))
            .collect(),
    )
}

fn source_parameter(builder: &mut PackageBuilder, code: u32, name: &str, type_code: u32) {
    builder.code(
        code,
        "value",
        "parameter",
        type_code,
        aggregate(vec![
            bound_member("name", literal("text", name)),
            bound_member("level", integer_literal(0)),
        ]),
    );
}

fn source_function_node(
    builder: &mut PackageBuilder,
    code: u32,
    name: &str,
    parameter_types: &[u32],
    result_type: u32,
    body_form: &str,
    body: serde_json::Value,
) {
    let parameter_codes = parameter_types
        .iter()
        .enumerate()
        .map(|(index, type_code)| {
            let code = function_parameter_code(code, index);
            source_parameter(
                builder,
                code,
                if index == 0 { "a" } else { "b" },
                *type_code,
            );
            code
        })
        .collect::<Vec<_>>();
    let body_code = function_body_code(code);
    builder.application_node(body_code, body_form, &key(result_type), body);
    let declaration_body = aggregate(vec![
        bound_member(
            "parameters",
            aggregate(
                parameter_codes
                    .iter()
                    .copied()
                    .map(reference_code)
                    .collect(),
            ),
        ),
        bound_member("body", reference_code(body_code)),
    ]);
    builder.code_named(
        code,
        name,
        "function",
        "pure_function",
        result_type,
        declaration_body,
    );
}

fn source_item_call(
    builder: &mut PackageBuilder,
    item_code: u32,
    function_code: u32,
    argument_types: &[u32],
    result_type: u32,
) {
    let argument_codes = argument_types
        .iter()
        .enumerate()
        .map(|(index, type_code)| {
            let code = item_argument_code(item_code, index);
            source_parameter(
                builder,
                code,
                &format!("call_{item_code}_arg_{index}"),
                *type_code,
            );
            code
        })
        .collect::<Vec<_>>();
    builder.application_node(
        item_code,
        "call",
        &key(result_type),
        call_body(function_code, &argument_codes, result_type),
    );
}

/// Ordinary FR-021 corpus for generation, determinism, and agreement tests.
/// Source-bound V2 function declarations and source calls live in
/// `source_bound_call_package` so they do not bloat every ordinary case.
pub fn ext_corpus_package() -> PackageBuilder {
    let mut builder = corpus_package();

    let roots = [
        (FN_ADD, "binary", true),
        (FN_EQ, "binary", false),
        (FN_CALL_NESTED, "call", false),
        (FN_DANGLING_CALL, "call", false),
        (FN_REF_PARAM, "binary", true),
        (FN_MODEL_PARAM, "binary", true),
        (FN_STATE_PARAM, "binary", true),
        (FN_CAPABILITY, "binary", true),
        (FN_UNRELATED, "binary", true),
        (ITEM_CALL_ADD, "call", false),
        (ITEM_CALL_EQ, "call", false),
        (ITEM_CALL_NESTED, "call", false),
        (ITEM_CALL_UNRELATED, "call", false),
        (ITEM_CALL_UNKNOWN_FUNCTION, "call", false),
    ];
    for (index, (code, form, integer_result)) in roots.into_iter().enumerate() {
        let parameter = BODY_PARAMETER_BASE + u32::try_from(index).expect("body parameter index");
        // These classifier roots have no source function declaration or name binding.
        // Their descriptors own runtime parameters; this typed source operand names
        // the root rather than colliding with the shared corpus's `p2` parameter.
        source_parameter(
            &mut builder,
            parameter,
            &format!("body_{code}_left"),
            T_INTEGER,
        );
        let mut body = binary_body(T_INTEGER, T_INTEGER);
        body["arguments"][0] = reference_code(parameter);
        let result_type = if integer_result {
            body["operation"]["identity"] = serde_json::json!("quire.op.integer.add");
            body["result_type"] =
                serde_json::json!({"domain": NODE_DOMAIN, "digest": key(T_INTEGER)});
            T_INTEGER
        } else {
            T_BOOLEAN
        };
        builder.application_node(code, form, &key(result_type), body);
    }
    builder
}

/// Minimal admitted corpus for the AC-11 defensive classifier boundary. The
/// requested Model/State roots intentionally do not match the source-bound
/// function on their call items; source-bound Model/State application remains
/// planned until the reader admits a suitable representation.
pub fn ac11_classifier_package() -> PackageBuilder {
    let mut builder = source_bound_call_package();
    source_item_call(
        &mut builder,
        ITEM_CALL_SOURCE_MODEL,
        SOURCE_FUNCTION,
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
    );
    source_item_call(
        &mut builder,
        ITEM_CALL_SOURCE_STATE,
        SOURCE_FUNCTION,
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
    );
    builder
}

/// One admitted FR-021 function declaration and local call. The call's
/// first argument is the declaration reference; its remaining references are
/// ordered exactly as the two declared parameters.
pub fn source_bound_call_package() -> PackageBuilder {
    let mut builder = corpus_package();
    for (code, name) in [(SOURCE_PARAM_A, "a"), (SOURCE_PARAM_B, "b")] {
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
    let add_op = serde_json::json!({
        "identity": "quire.op.integer.add", "laws": [], "mode": null,
        "member": null, "leaves": []
    });
    let body = serde_json::json!({
        "term": "application", "operator": "binary", "operation": add_op,
        "result_type": {"domain": NODE_DOMAIN, "digest": key(T_INTEGER)},
        "arguments": [source_reference(SOURCE_PARAM_A), source_reference(SOURCE_PARAM_B)]
    });
    let other_body = serde_json::json!({
        "term": "application", "operator": "binary",
        "operation": {"identity": "quire.op.integer.sub", "laws": [], "mode": null,
        "member": null, "leaves": []},
        "result_type": {"domain": NODE_DOMAIN, "digest": key(T_INTEGER)},
        "arguments": [source_reference(SOURCE_PARAM_A), source_reference(SOURCE_PARAM_B)]
    });
    builder.application_node(SOURCE_BODY, "binary", &key(T_INTEGER), body);
    builder.application_node(SOURCE_OTHER_BODY, "binary", &key(T_INTEGER), other_body);
    let declaration_body = |body_code| {
        aggregate(vec![
            serde_json::json!({"term": "binding", "name": "parameters", "value": aggregate(vec![source_reference(SOURCE_PARAM_A), source_reference(SOURCE_PARAM_B)])}),
            serde_json::json!({"term": "binding", "name": "body", "value": source_reference(body_code)}),
        ])
    };
    builder
        .code(
            SOURCE_FUNCTION,
            "function",
            "pure_function",
            T_INTEGER,
            declaration_body(SOURCE_BODY),
        )
        .code(
            SOURCE_OTHER_FUNCTION,
            "function",
            "pure_function",
            T_INTEGER,
            declaration_body(SOURCE_OTHER_BODY),
        );
    let call_op = serde_json::json!({
        "identity": "quire.op.function.call", "laws": [], "mode": null,
        "member": null, "leaves": []
    });
    builder.application_node(
        SOURCE_CALL,
        "call",
        &key(T_INTEGER),
        serde_json::json!({
            "term": "application", "operator": "call", "operation": call_op,
            "result_type": {"domain": NODE_DOMAIN, "digest": key(T_INTEGER)},
            "arguments": [source_reference(SOURCE_FUNCTION), source_reference(SOURCE_PARAM_A), source_reference(SOURCE_PARAM_B)]
        }),
    );
    builder
}

/// A V2-admitted source function whose lowered FR-330 package exceeds the
/// function preflight reader's bounded byte ceiling.
pub fn oversized_source_bound_call_package() -> (CheckedPackageV2, FixtureIds) {
    source_bound_call_package_with_name_len(1_100_000)
}

/// Admit a source-backed function after changing only its declared name length.
pub fn source_bound_call_package_with_name_len(name_len: usize) -> (CheckedPackageV2, FixtureIds) {
    source_bound_call_package_with_name_segments(vec!["f".repeat(name_len)])
}

/// Change only the source function's qualified name, retaining its admitted call.
pub fn source_bound_call_package_with_name_segments(
    segments: Vec<String>,
) -> (CheckedPackageV2, FixtureIds) {
    let builder = source_bound_call_package();
    let (ordinary, ids) = builder.admit_resolved();
    let function_id = ids.resolve(&code_id(SOURCE_FUNCTION));
    let function_name = ordinary
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == function_id)
        .expect("resolved source function")
        .declaration
        .as_ref()
        .expect("function declaration")
        .qualified_name
        .clone();
    let mut wire = builder.raw_wire();
    let function = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("graph nodes")
        .iter_mut()
        .find(|node| node["declaration"]["qualified_name"][0] == function_name[0].as_ref())
        .expect("source function node");
    function["declaration"]["qualified_name"] = serde_json::json!(segments);
    let mut limits = CheckedPackageReadLimits::bounded();
    limits.bytes = 4_000_000;
    PackageBuilder::from_wire(wire).admit_with_resolved(limits)
}

pub fn source_function(name: &str, body_code: u32) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(body_code),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn source_call() -> ExactFunctionItem {
    ExactFunctionItem {
        call_node_id: code_id(SOURCE_CALL),
        function: format!("n{}", key(SOURCE_FUNCTION)),
        argument_node_ids: vec![code_id(SOURCE_PARAM_A), code_id(SOURCE_PARAM_B)],
    }
}

// ---------------------------------------------------------------------------
// Declaration builders
// ---------------------------------------------------------------------------

fn parameter(name: &str, type_code: u32) -> FunctionParameter {
    FunctionParameter {
        name: name.to_owned(),
        type_node_id: code_id(type_code),
    }
}

pub fn function_add(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_ADD),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_eq(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_EQ),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_BOOLEAN),
        body: ExactFunctionBody::CompositeEquality {
            operator: quire_contract_codegen::EqualityOperatorKind::Equal,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_call_nested(name: &str, callee: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_CALL_NESTED),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Call {
            callee: callee.to_owned(),
        },
        capability_requirements: Vec::new(),
    }
}

/// [`FN_CALL_NESTED`]'s own node, declared as a `Scalar` body: a real
/// `FormMismatch` (its `semantic_form` is `"call"`, not `"binary"`).
pub fn function_form_mismatch(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_FORM_MISMATCH_NODE),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_dangling_call(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_DANGLING_CALL),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Call {
            callee: "not_declared_anywhere".to_owned(),
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_ref_param(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_REF_PARAM),
        name: name.to_owned(),
        parameters: vec![parameter("a", REF_TYPE)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_model_root(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(M_BARE),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_state_root(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(S_BARE),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_capability(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_CAPABILITY),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: vec!["quire.capability.undischargeable-in-v1".to_owned()],
    }
}

pub fn function_unrelated(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_UNRELATED),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Item builders
// ---------------------------------------------------------------------------

pub fn item(call_code: u32, function: &str) -> ExactFunctionItem {
    ExactFunctionItem {
        call_node_id: code_id(call_code),
        function: function.to_owned(),
        argument_node_ids: vec![code_id(T_INTEGER), code_id(T_INTEGER)],
    }
}

// ---------------------------------------------------------------------------
// The byte-ceiling fixture (FR-021-AC-23)
// ---------------------------------------------------------------------------

/// The first code of the chain of nodes the byte-ceiling fixture declares as function bodies,
/// and how many it has.
pub const BYTE_BODY_BASE: u32 = 30_000;
pub const BYTE_BODY_COUNT: u32 = 220;
/// Call items request a prefix of the same chain. The two request sets differ in length, so
/// their lowerings produce distinct byte charges while sharing admitted graph nodes.
pub const BYTE_CALL_BASE: u32 = BYTE_BODY_BASE;
pub const BYTE_CALL_COUNT: u32 = BYTE_BODY_COUNT - 1;

/// Author the chain's source text and record every actual node-use token in it: one declaration,
/// one requested call where applicable, and two predecessor references in the next equality.
pub fn byte_ceiling_source(
    body_count: u32,
    call_count: u32,
) -> (String, BTreeMap<u32, Vec<(usize, usize)>>) {
    assert!(body_count > 0 && call_count > 0 && call_count <= body_count);
    let mut source = String::new();
    let mut sites: BTreeMap<u32, Vec<(usize, usize)>> = BTreeMap::new();
    let mut token = |source: &mut String, code: u32| {
        let start = source.len();
        source.push_str(&format!("n{code}"));
        sites.entry(code).or_default().push((start, source.len()));
    };
    for offset in 0..body_count {
        let code = BYTE_BODY_BASE + offset;
        source.push_str("let ");
        token(&mut source, code);
        source.push_str(" = eq(");
        if offset == 0 {
            source.push_str("true, true");
        } else {
            token(&mut source, code - 1);
            source.push_str(", ");
            token(&mut source, code - 1);
        }
        source.push_str(")\n");
        if offset < call_count {
            source.push_str("call ");
            token(&mut source, code);
            source.push('\n');
        }
    }
    (source, sites)
}

/// A small reader-admitted base plus one shared chain. Source-map correspondence and request
/// accounting are represented separately in the two lowerings; the test measures their actual
/// canonical byte lengths before choosing a ceiling.
pub fn byte_ceiling_package() -> PackageBuilder {
    byte_ceiling_package_with_shape(BYTE_BODY_COUNT, BYTE_CALL_COUNT)
}

/// The same topology at a smaller size for a bounded reader/lowering control.
pub fn byte_ceiling_package_with_shape(body_count: u32, call_count: u32) -> PackageBuilder {
    let mut builder = PackageBuilder::default();
    builder
        .code(
            T_BOOLEAN,
            "scalar_type",
            "boolean",
            T_BOOLEAN,
            aggregate(vec![]),
        )
        .code(
            T_INTEGER,
            "scalar_type",
            "integer",
            T_INTEGER,
            aggregate(vec![]),
        )
        .boolean_equality_chain_from(BYTE_BODY_BASE, body_count, "true");

    let (source, sites) = byte_ceiling_source(body_count, call_count);
    let source_ref = serde_json::json!({
        "authority": "agent-ix",
        "identity": "ir764-ac23-source",
        "digest_domain": "quire.source.bytes/v1",
        "digest": format!("{:x}", Sha256::digest(source.as_bytes())),
    });
    let mut wire = builder.raw_wire();
    wire["lock"]["sources"]
        .as_array_mut()
        .expect("locked sources")
        .push(source_ref.clone());
    let mut entries = Vec::new();
    let mut replaced = std::collections::BTreeSet::new();
    for (code, spans) in sites {
        let node_id = code_id(code);
        replaced.insert(node_id.digest.to_string());
        let node = wire["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("graph nodes")
            .iter_mut()
            .find(|node| node["node_id"]["digest"] == node_id.digest.as_ref())
            .expect("source-backed chain node");
        node["occurrences"] = serde_json::Value::Array(
            spans
                .iter()
                .enumerate()
                .map(|(ordinal, _)| {
                    serde_json::json!({
                        "role": if ordinal == 0 { "declaration" } else { "expression" },
                        "ordinal": ordinal,
                    })
                })
                .collect(),
        );
        for (ordinal, (start, end)) in spans.into_iter().enumerate() {
            entries.push(serde_json::json!({
                "node_id": node_id,
                "role": if ordinal == 0 { "declaration" } else { "expression" },
                "ordinal": ordinal,
                "regions": [{"source": source_ref, "start": start, "end": end}],
            }));
        }
    }
    let map = wire["source_map"].as_array_mut().expect("source map");
    map.retain(|entry| {
        !replaced.contains(
            entry["node_id"]["digest"]
                .as_str()
                .expect("source node digest"),
        )
    });
    map.extend(entries);
    PackageBuilder::from_wire(wire)
}

/// One function per body-chain node of [`byte_ceiling_package`].
pub fn byte_ceiling_functions() -> Vec<ExactFunctionDeclaration> {
    (0..BYTE_BODY_COUNT)
        .map(|offset| ExactFunctionDeclaration {
            node_id: code_id(BYTE_BODY_BASE + offset),
            name: format!("byte_fn_{offset}"),
            parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
            result_type: code_id(T_INTEGER),
            body: ExactFunctionBody::Scalar {
                operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
            },
            capability_requirements: Vec::new(),
        })
        .collect()
}

/// One item per call-chain node of [`byte_ceiling_package`], each applying its own function.
pub fn byte_ceiling_items() -> Vec<ExactFunctionItem> {
    (0..BYTE_CALL_COUNT)
        .map(|offset| item(BYTE_CALL_BASE + offset, &format!("byte_fn_{offset}")))
        .collect()
}

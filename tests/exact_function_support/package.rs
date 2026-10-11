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
//! that base: every one of them is built the same way `E_CALL` already is
//! in the base module (`application_code(code, "call"|"binary",
//! binary_body(T_INTEGER, T_INTEGER))`) -- the body's own content is inert
//! for this generator's classifiers (they read only `semantic_form` and the
//! caller-declared [`quire_contract_codegen::ExactFunctionBody`], never the
//! node's `operation`/`arguments`), so every new node below reuses the same
//! `binary_body(T_INTEGER, T_INTEGER)` shape and differs only in its own
//! `semantic_form` and code.

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
pub const FN_FORM_MISMATCH_NODE: u32 = 513;
/// `f_dangling(a, b)`: `Call { callee: "not_declared" }` -- AC-12's dangling
/// nested-call case.
pub const FN_DANGLING_CALL: u32 = 503;
/// `f_ref(a)`: one parameter typed `REF_TYPE` (a `reference` composite) --
/// AC-10.
pub const FN_REF_PARAM: u32 = 504;
/// `f_model(a)`: one parameter typed `M_BARE` (`model` family) -- AC-11's
/// `QuireSpecLanguage120` half.
pub const FN_MODEL_PARAM: u32 = 505;
/// `f_state(a)`: one parameter typed `S_BARE` (`state` family) -- AC-11's
/// `QuireSpecLanguage121` half.
pub const FN_STATE_PARAM: u32 = 506;
/// `f_capability(a, b)`: `Scalar { Add }` with a non-empty
/// `capability_requirements` -- AC-6.
pub const FN_CAPABILITY: u32 = 507;
/// `f_unrelated(a, b)`: `Scalar { Add }`, no relation to any refused
/// function -- AC-12's "without changing an unrelated ... function's
/// items" isolation control.
pub const FN_UNRELATED: u32 = 508;
pub const FN_ONE_PARAM_ADD: u32 = 509;
pub const FN_BOOLEAN_RESULT_ADD: u32 = 510;
pub const FN_NEGATE: u32 = 511;
pub const FN_DANGLING_TARGET: u32 = 512;

/// One `call` expression node per requested item.
pub const ITEM_CALL_ADD: u32 = 520;
pub const ITEM_CALL_EQ: u32 = 521;
pub const ITEM_CALL_NESTED: u32 = 522;
pub const ITEM_CALL_UNRELATED: u32 = 523;
/// Names a function absent from every request's own declarations.
pub const ITEM_CALL_UNKNOWN_FUNCTION: u32 = 524;
pub const ITEM_CALL_REF: u32 = 525;
pub const ITEM_CALL_MODEL: u32 = 526;
pub const ITEM_CALL_STATE: u32 = 527;
pub const ITEM_CALL_CAPABILITY: u32 = 528;
pub const ITEM_CALL_DANGLING: u32 = 529;
pub const ITEM_CALL_BAD_FORM: u32 = 530;
pub const ITEM_CALL_ONE_PARAM: u32 = 531;
pub const ITEM_CALL_BOOLEAN_RESULT: u32 = 532;
pub const ITEM_CALL_NEGATE: u32 = 533;

/// Base code for the >128-deep nested-call chain (AC-7). `CHAIN_LENGTH`
/// functions `chain_0.. calling chain_(i+1)..`, the last calling [`FN_ADD`].
pub const CHAIN_BASE: u32 = 1000;
pub const CHAIN_LENGTH: u32 = 140; // > rt::MAX_CALL_DEPTH (128)
pub const ITEM_CALL_CHAIN: u32 = 1500;

/// A genuinely checked FR-021 package used for source-binding adversarial
/// cases. Unlike `ext_corpus_package`, this declaration is a `function`
/// node whose parameter and body bindings point to separate checked nodes.
pub const SOURCE_PARAM_A: u32 = 2600;
pub const SOURCE_PARAM_B: u32 = 2601;
pub const SOURCE_BODY: u32 = 2602;
pub const SOURCE_FUNCTION: u32 = 2603;
pub const SOURCE_OTHER_FUNCTION: u32 = 2604;
pub const SOURCE_CALL: u32 = 2605;

fn source_reference(code: u32) -> serde_json::Value {
    let id = code_id(code);
    serde_json::json!({
        "term": "reference",
        "target": {"domain": id.domain, "digest": id.digest}
    })
}

/// `application_code`'s preimage is `{version, node_tag, semantic_form,
/// semantic_type: T_BOOLEAN (fixed), declaration: None, recursion: null,
/// body}` -- two calls sharing the same `(semantic_form, body)` pair
/// collide on one digest (`InvalidSemanticGraph` at admission: a duplicate
/// `node_id`). Every node this module adds therefore gets its own
/// `result_type` disambiguator from a dedicated, freshly registered
/// `scalar_type` placeholder pool, so no two of this module's own
/// `application_code` calls can ever share a preimage.
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

/// The full FR-021 corpus: the composite-equality base package plus every
/// node above.
pub fn ext_corpus_package() -> PackageBuilder {
    let mut builder = corpus_package();

    let disambiguator_count = 9 + 5 + CHAIN_LENGTH + 1;
    for i in 0..disambiguator_count {
        builder.code(
            DISAMBIGUATOR_BASE + i,
            "scalar_type",
            "boolean",
            DISAMBIGUATOR_BASE + i,
            aggregate(vec![]),
        );
    }

    let add_body = |code: u32| {
        integer_add_body(
            (0..2)
                .map(|index| reference_code(function_parameter_code(code, index)))
                .collect(),
        )
    };
    source_function_node(
        &mut builder,
        FN_ADD,
        "add_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "binary",
        add_body(FN_ADD),
    );
    source_function_node(
        &mut builder,
        FN_EQ,
        "eq_fn",
        &[T_INTEGER, T_INTEGER],
        T_BOOLEAN,
        "binary",
        binary_body(T_INTEGER, T_INTEGER),
    );
    source_function_node(
        &mut builder,
        FN_CALL_NESTED,
        "call_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "call",
        call_body(
            FN_ADD,
            &[
                function_parameter_code(FN_CALL_NESTED, 0),
                function_parameter_code(FN_CALL_NESTED, 1),
            ],
            T_INTEGER,
        ),
    );
    for (code, name, parameter_type) in [
        (FN_REF_PARAM, "ref_fn", REF_TYPE),
        (FN_MODEL_PARAM, "model_fn", M_BARE),
        (FN_STATE_PARAM, "state_fn", S_BARE),
    ] {
        source_function_node(
            &mut builder,
            code,
            name,
            &[parameter_type, T_INTEGER],
            T_INTEGER,
            "binary",
            integer_add_body(vec![
                literal("integer", "1"),
                literal("integer", &code.to_string()),
            ]),
        );
    }
    source_function_node(
        &mut builder,
        FN_CAPABILITY,
        "capability_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "binary",
        add_body(FN_CAPABILITY),
    );
    source_function_node(
        &mut builder,
        FN_UNRELATED,
        "unrelated_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "binary",
        add_body(FN_UNRELATED),
    );
    source_function_node(
        &mut builder,
        FN_DANGLING_TARGET,
        "not_declared_anywhere",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "binary",
        integer_add_body(vec![literal("integer", "2"), literal("integer", "3")]),
    );
    source_function_node(
        &mut builder,
        FN_DANGLING_CALL,
        "dangling_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "call",
        call_body(
            FN_DANGLING_TARGET,
            &[
                function_parameter_code(FN_DANGLING_CALL, 0),
                function_parameter_code(FN_DANGLING_CALL, 1),
            ],
            T_INTEGER,
        ),
    );
    source_function_node(
        &mut builder,
        FN_FORM_MISMATCH_NODE,
        "bad_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "call",
        call_body(
            FN_ADD,
            &[
                function_parameter_code(FN_FORM_MISMATCH_NODE, 0),
                function_parameter_code(FN_FORM_MISMATCH_NODE, 1),
            ],
            T_INTEGER,
        ),
    );
    source_function_node(
        &mut builder,
        FN_ONE_PARAM_ADD,
        "one_param_add",
        &[T_INTEGER],
        T_INTEGER,
        "binary",
        integer_add_body(vec![literal("integer", "4"), literal("integer", "5")]),
    );
    source_function_node(
        &mut builder,
        FN_BOOLEAN_RESULT_ADD,
        "boolean_result_add",
        &[T_INTEGER, T_INTEGER],
        T_BOOLEAN,
        "binary",
        integer_add_body(vec![literal("integer", "6"), literal("integer", "7")]),
    );
    source_function_node(
        &mut builder,
        FN_NEGATE,
        "negate_fn",
        &[T_INTEGER, T_INTEGER],
        T_INTEGER,
        "unary",
        application(
            "unary",
            serde_json::json!({"identity": "quire.op.integer.neg", "laws": [], "mode": null, "member": null, "leaves": []}),
            T_INTEGER,
            vec![reference_code(function_parameter_code(FN_NEGATE, 0))],
        ),
    );

    let fixed_calls = [
        (ITEM_CALL_ADD, FN_ADD, vec![T_INTEGER, T_INTEGER], T_INTEGER),
        (ITEM_CALL_EQ, FN_EQ, vec![T_INTEGER, T_INTEGER], T_BOOLEAN),
        (
            ITEM_CALL_NESTED,
            FN_CALL_NESTED,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_UNRELATED,
            FN_UNRELATED,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_UNKNOWN_FUNCTION,
            FN_ADD,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_REF,
            FN_REF_PARAM,
            vec![REF_TYPE, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_MODEL,
            FN_MODEL_PARAM,
            vec![M_BARE, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_STATE,
            FN_STATE_PARAM,
            vec![S_BARE, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_CAPABILITY,
            FN_CAPABILITY,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_DANGLING,
            FN_DANGLING_CALL,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_BAD_FORM,
            FN_FORM_MISMATCH_NODE,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_ONE_PARAM,
            FN_ONE_PARAM_ADD,
            vec![T_INTEGER],
            T_INTEGER,
        ),
        (
            ITEM_CALL_BOOLEAN_RESULT,
            FN_BOOLEAN_RESULT_ADD,
            vec![T_INTEGER, T_INTEGER],
            T_BOOLEAN,
        ),
        (
            ITEM_CALL_NEGATE,
            FN_NEGATE,
            vec![T_INTEGER, T_INTEGER],
            T_INTEGER,
        ),
    ];
    for (call_code, function_code, argument_types, result_type) in fixed_calls {
        source_item_call(
            &mut builder,
            call_code,
            function_code,
            &argument_types,
            result_type,
        );
    }
    for index in 0..CHAIN_LENGTH {
        let code = CHAIN_BASE + index;
        let callee = if index + 1 == CHAIN_LENGTH {
            FN_ADD
        } else {
            code + 1
        };
        let args = [
            function_parameter_code(code, 0),
            function_parameter_code(code, 1),
        ];
        source_function_node(
            &mut builder,
            code,
            &format!("chain_{index}"),
            &[T_INTEGER, T_INTEGER],
            T_INTEGER,
            "call",
            call_body(callee, &args, T_INTEGER),
        );
    }
    source_item_call(
        &mut builder,
        ITEM_CALL_CHAIN,
        CHAIN_BASE,
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
    builder.application_node(SOURCE_BODY, "binary", &key(T_INTEGER), body);
    let declaration_body = || {
        aggregate(vec![
            serde_json::json!({"term": "binding", "name": "parameters", "value": aggregate(vec![source_reference(SOURCE_PARAM_A), source_reference(SOURCE_PARAM_B)])}),
            serde_json::json!({"term": "binding", "name": "body", "value": source_reference(SOURCE_BODY)}),
        ])
    };
    builder
        .code(
            SOURCE_FUNCTION,
            "function",
            "pure_function",
            T_INTEGER,
            declaration_body(),
        )
        .code(
            SOURCE_OTHER_FUNCTION,
            "function",
            "pure_function",
            T_INTEGER,
            declaration_body(),
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

pub fn source_function(name: &str, code: u32) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(code),
        name: name.to_owned(),
        parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::IntegerOperator::Add,
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
        parameters: vec![parameter("a", REF_TYPE), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_model_param(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_MODEL_PARAM),
        name: name.to_owned(),
        parameters: vec![parameter("a", M_BARE), parameter("b", T_INTEGER)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_state_param(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_STATE_PARAM),
        name: name.to_owned(),
        parameters: vec![parameter("a", S_BARE), parameter("b", T_INTEGER)],
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

/// One function per link of a >128-deep nested-call chain:
/// `chain_0..chain_(CHAIN_LENGTH - 1)`, each calling the next, the last
/// calling `add_fn` (see [`function_add`]).
pub fn chain_functions() -> Vec<ExactFunctionDeclaration> {
    let mut functions = Vec::with_capacity(CHAIN_LENGTH as usize);
    for i in 0..CHAIN_LENGTH {
        let callee = if i + 1 < CHAIN_LENGTH {
            format!("chain_{}", i + 1)
        } else {
            "add_fn".to_owned()
        };
        functions.push(ExactFunctionDeclaration {
            node_id: code_id(CHAIN_BASE + i),
            name: format!("chain_{i}"),
            parameters: vec![parameter("a", T_INTEGER), parameter("b", T_INTEGER)],
            result_type: code_id(T_INTEGER),
            body: ExactFunctionBody::Call { callee },
            capability_requirements: Vec::new(),
        });
    }
    functions
}

// ---------------------------------------------------------------------------
// Item builders
// ---------------------------------------------------------------------------

pub fn item(call_code: u32, function: &str) -> ExactFunctionItem {
    let argument_codes = match call_code {
        ITEM_CALL_ONE_PARAM => vec![item_argument_code(call_code, 0)],
        ITEM_CALL_ADD
        | ITEM_CALL_EQ
        | ITEM_CALL_NESTED
        | ITEM_CALL_UNRELATED
        | ITEM_CALL_UNKNOWN_FUNCTION
        | ITEM_CALL_CAPABILITY
        | ITEM_CALL_DANGLING
        | ITEM_CALL_BAD_FORM
        | ITEM_CALL_BOOLEAN_RESULT
        | ITEM_CALL_NEGATE
        | ITEM_CALL_CHAIN => vec![
            item_argument_code(call_code, 0),
            item_argument_code(call_code, 1),
        ],
        _ => vec![T_INTEGER, T_INTEGER],
    };
    ExactFunctionItem {
        call_node_id: code_id(call_code),
        function: function.to_owned(),
        argument_node_ids: argument_codes.into_iter().map(code_id).collect(),
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

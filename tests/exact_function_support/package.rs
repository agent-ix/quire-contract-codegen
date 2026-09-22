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
    ExactFunctionBody, ExactFunctionDeclaration, ExactFunctionItem, FunctionParameter,
};

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

/// One `call` expression node per requested item.
pub const ITEM_CALL_ADD: u32 = 520;
pub const ITEM_CALL_EQ: u32 = 521;
pub const ITEM_CALL_NESTED: u32 = 522;
pub const ITEM_CALL_UNRELATED: u32 = 523;
/// Names a function absent from every request's own declarations.
pub const ITEM_CALL_UNKNOWN_FUNCTION: u32 = 524;

/// Base code for the >128-deep nested-call chain (AC-7). `CHAIN_LENGTH`
/// functions `chain_0.. calling chain_(i+1)..`, the last calling [`FN_ADD`].
pub const CHAIN_BASE: u32 = 1000;
pub const CHAIN_LENGTH: u32 = 140; // > rt::MAX_CALL_DEPTH (128)
pub const ITEM_CALL_CHAIN: u32 = 1500;

/// `application_code`'s preimage is `{version, node_tag, semantic_form,
/// semantic_type: T_BOOLEAN (fixed), declaration: None, recursion: null,
/// body}` -- two calls sharing the same `(semantic_form, body)` pair
/// collide on one digest (`InvalidSemanticGraph` at admission: a duplicate
/// `node_id`). Every node this module adds therefore gets its own
/// `result_type` disambiguator from a dedicated, freshly registered
/// `scalar_type` placeholder pool, so no two of this module's own
/// `application_code` calls can ever share a preimage.
const DISAMBIGUATOR_BASE: u32 = 2000;

fn disambiguated_body(index: u32) -> serde_json::Value {
    binary_body_with_result(T_INTEGER, T_INTEGER, DISAMBIGUATOR_BASE + index)
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
            T_BOOLEAN,
            aggregate(vec![]),
        );
    }

    let mut next = 0u32;
    let mut body = || {
        let value = disambiguated_body(next);
        next += 1;
        value
    };

    builder
        .application_code(FN_ADD, "binary", body())
        .application_code(FN_EQ, "binary", body())
        .application_code(FN_CALL_NESTED, "call", body())
        .application_code(FN_DANGLING_CALL, "call", body())
        .application_code(FN_REF_PARAM, "binary", body())
        .application_code(FN_MODEL_PARAM, "binary", body())
        .application_code(FN_STATE_PARAM, "binary", body())
        .application_code(FN_CAPABILITY, "binary", body())
        .application_code(FN_UNRELATED, "binary", body())
        .application_code(ITEM_CALL_ADD, "call", body())
        .application_code(ITEM_CALL_EQ, "call", body())
        .application_code(ITEM_CALL_NESTED, "call", body())
        .application_code(ITEM_CALL_UNRELATED, "call", body())
        .application_code(ITEM_CALL_UNKNOWN_FUNCTION, "call", body());
    for i in 0..CHAIN_LENGTH {
        builder.application_code(CHAIN_BASE + i, "call", body());
    }
    builder.application_code(ITEM_CALL_CHAIN, "call", body());
    builder
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
            operator: quire_contract_codegen::IntegerOperator::Add,
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
            operator: quire_contract_codegen::IntegerOperator::Add,
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
            operator: quire_contract_codegen::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_model_param(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_MODEL_PARAM),
        name: name.to_owned(),
        parameters: vec![parameter("a", M_BARE)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    }
}

pub fn function_state_param(name: &str) -> ExactFunctionDeclaration {
    ExactFunctionDeclaration {
        node_id: code_id(FN_STATE_PARAM),
        name: name.to_owned(),
        parameters: vec![parameter("a", S_BARE)],
        result_type: code_id(T_INTEGER),
        body: ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::IntegerOperator::Add,
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
            operator: quire_contract_codegen::IntegerOperator::Add,
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
            operator: quire_contract_codegen::IntegerOperator::Add,
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
    ExactFunctionItem {
        call_node_id: code_id(call_code),
        function: function.to_owned(),
        argument_node_ids: vec![code_id(T_INTEGER), code_id(T_INTEGER)],
    }
}

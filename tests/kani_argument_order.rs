//! IR-213: couples `symbolic_arguments`'s `kani::any()` emission order
//! (`src/kani_obligations.rs:1450`) to the order persisted in `identity.arguments`. That coupling
//! is what `decode_falsification` (`src/kani_witness_join.rs`) depends on: it binds
//! `identity.arguments` *positionally* onto Kani's concrete playback bytes, which is only sound
//! if position *i* of the persisted arguments is position *i* of the emitted `kani::any()` calls.
//!
//! `tc_025_assumptions_constrain_only_arguments_to_their_ir_bounds`
//! (`tests/kani_obligations.rs:855`) already asserts this for every harness kind, by reading the
//! trailing `kani::assume` line each bounded argument gets and comparing its order to
//! `identity.arguments`. Since `symbolic_arguments` emits each `kani::any()` immediately followed
//! by its own `kani::assume`, the assume line's order *is* the emission order, so tc_025 already
//! catches a reorder or a drop of any *bounded* argument.
//!
//! What tc_025 cannot see is an argument with no integer bounds: it has no `kani::assume` line to
//! read, and `tests/kani_obligations.rs:869` does `integer_bounds.as_ref().unwrap()` on every
//! binding it does find, so a `Boolean` argument in that fixture would make tc_025 panic rather
//! than check anything. This test reads the `let <id>: .. = kani::any();` binding itself — the
//! same identifier tc_025 reads a line later, but before the bounds check that only a bounded
//! argument has — parsing it out of the generated harness's own `harness.rust.contents` with
//! `syn` (the same parser `render()` itself uses to validate generated source before returning
//! it), and asserts the identifiers it finds, in the order they appear in the generated source,
//! equal `identity.arguments`'s identifiers in order. The fixture (`tests/common/withdraw_fixture.rs`)
//! declares one such unbounded `Boolean` input (`priority`) precisely so this axis is exercised,
//! not just asserted; it is the only strictly-stronger thing this test adds over tc_025.
//!
//! It asserts this over every harness `withdraw_harnesses` negotiates — precondition,
//! postcondition and invariant — because `symbolic_arguments` is called from two independent
//! sites (`render_precondition` at `src/kani_obligations.rs:1487` and `render_contract`, serving
//! both Postcondition and Invariant, at `src/kani_obligations.rs:1586`), and a divergence at
//! either is the same silent misbind in `decode_falsification` this test exists to exclude.
//!
//! Trace: IR-213.

mod common;

use common::withdraw_fixture::withdraw_harnesses;
use quire_contract_codegen::{KaniObligationHarness, KaniToolPins};
use syn::{
    visit::{self, Visit},
    Expr, Local, Pat,
};

// ---- generic, honest extraction of kani::any() bindings from generated source -----------------

/// Collects every `kani::any()`-bound identifier a parsed file contains, in the order `syn`
/// visits them — declaration order through the whole file, including the embedded oracle sources
/// and the harness module nested inside it, which is what a generated harness file actually is
/// (not a single straight-line block).
#[derive(Default)]
struct KaniAnyBindings {
    identifiers: Vec<String>,
}

impl<'ast> Visit<'ast> for KaniAnyBindings {
    fn visit_local(&mut self, node: &'ast Local) {
        if let Some(identifier) = kani_any_binding_identifier(node) {
            self.identifiers.push(identifier);
        }
        visit::visit_local(self, node);
    }
}

/// Returns the bound identifier when `local` is exactly `let <id>: <ty> = kani::any();`: a
/// zero-argument call to the two-segment path `kani::any`, bound to a single typed identifier
/// pattern. Every other `let` — bound to some other call, or to an untyped or destructured
/// pattern — returns `None`. This does not know or assume any of the identifiers
/// `symbolic_arguments` happens to emit today; it recognizes only the `kani::any()` call shape
/// itself, so it reports whatever bindings the generated source actually contains.
fn kani_any_binding_identifier(local: &Local) -> Option<String> {
    let Pat::Type(pat_type) = &local.pat else {
        return None;
    };
    let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
        return None;
    };
    let init = local.init.as_ref()?;
    let Expr::Call(call) = init.expr.as_ref() else {
        return None;
    };
    if !call.args.is_empty() {
        return None;
    }
    let Expr::Path(expr_path) = call.func.as_ref() else {
        return None;
    };
    let segments = expr_path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    (segments == ["kani", "any"]).then(|| pat_ident.ident.to_string())
}

/// Parses `source` as a Rust file — the same parser `render()` (`src/kani_obligations.rs`) itself
/// uses to validate generated source before returning it — and returns every `kani::any()`-bound
/// identifier found, in the order it appears in the source.
fn kani_any_identifiers_in_source_order(source: &str) -> Vec<String> {
    let file = syn::parse_file(source).expect("generated harness source must parse as Rust");
    let mut bindings = KaniAnyBindings::default();
    bindings.visit_file(&file);
    bindings.identifiers
}

/// Asserts one harness's `kani::any()` emission order, in its own generated source, equals its
/// own `identity.arguments` order — not merely as a set, since a positional misbinding between
/// two same-typed arguments (e.g. `amount_current`/`balance_pre`, both `i64`) is exactly the
/// failure mode `decode_falsification` exists to exclude, and a set comparison cannot see it.
fn assert_emission_order_matches_identity(harness: &KaniObligationHarness) {
    let declared_order = harness
        .identity
        .arguments
        .iter()
        .map(|binding| binding.identifier.clone())
        .collect::<Vec<_>>();
    let emitted_order = kani_any_identifiers_in_source_order(&harness.rust.contents);
    assert_eq!(
        emitted_order, declared_order,
        "kani::any() emission order in the generated harness (left) must match \
         identity.arguments order (right) for the {:?} harness; decode_falsification in \
         src/kani_witness_join.rs binds them positionally, so any divergence here is a silent \
         misbind there",
        harness.identity.kind,
    );
}

/// IR-213: asserts the invariant `decode_falsification` depends on, directly against the
/// generator's own output, for every harness kind `symbolic_arguments` is called for.
#[test]
fn ir_213_kani_any_emission_order_matches_persisted_identity_arguments() {
    let pins = KaniToolPins::pinned();
    let harnesses = withdraw_harnesses(&pins);

    // The join this test protects is only interesting for two or more positions: a
    // single-argument harness could never observe a reorder or a dropped binding. Every harness
    // negotiated here carries three (amount_current, balance_pre, priority_current), but the
    // guard is stated as "at least one" so it reports the fixture, not this count, if that
    // changes.
    assert!(
        harnesses
            .iter()
            .any(|harness| harness.identity.arguments.len() >= 2),
        "fixture must negotiate at least one harness with two or more arguments to make a \
         positional check meaningful: {:?}",
        harnesses
            .iter()
            .map(|harness| harness.identity.arguments.len())
            .collect::<Vec<_>>()
    );

    for harness in &harnesses {
        assert_emission_order_matches_identity(harness);
    }
}

---
id: TC-042
title: "Verify no panic on a generation or analysis path"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-005
    type: verifies
---
# TC-042: Verify no panic on a generation or analysis path

## Description

Verify the production code under `src/` holds no panic token and that each measured site that a
test can reach returns its typed value instead of aborting (NFR-005).

## Test Procedure

1. Walk `src/` and scan every `.rs` file except a file its parent declares under `#[cfg(test)]`
   with `non_test_code_outside_literals` of `tests/common/panic_scan.rs`; assert that
   `panic_tokens_in` finds nothing, and that the walk read the files FR-014-AC-39, FR-018-AC-19 and
   FR-021-AC-21 name (NFR-005-AC-1). Assert on a hand-built source that the helper keeps an
   `unwrap` in code, drops one in a string literal, a comment and a `#[cfg(test)]` item, and keeps
   `abort` as a bare identifier.
2. Hand `lower_scalar_claim` a claim with empty `checked_bounds`, and one whose first checked bound
   has no `IntegerRange` among the derived domains; assert `Err(ScalarLoweringRefusal::NoRenderer)`
   for each and `UnsupportedObligation::OperationNotRendered` through `classify_claim`
   (NFR-005-AC-2).
3. Build an `ItemSettlement` whose disposition is `Unsupported` for each of the six
   `invalid_capability` causes and assert `warning()` is `None`; build one per `unsupported_projection`
   cause and assert it still returns its warning (NFR-005-AC-3).
4. Hand `observe_clause` a map whose evaluation region has no probe, and one whose consequent region
   has none; assert the `MapMismatch` diagnostic, no classification and, for the first,
   `evaluation_count` of `None` (NFR-005-AC-4).
5. Hand the record-pairing step of `generate_kani` one record fewer and one record more than the
   group has items, and an equal count; assert `KaniRecordCountMismatch` with both counts for the
   first two and a paired item per record for the third (NFR-005-AC-5).

## Expected Results

The scan finds zero panic tokens; each seam test returns its typed refusal or value with no panic.
The four `src/oracle/function/mod.rs` sites, the two corpus serializations and the `boolean_v1`
assertion have no fixture and are held by step 1 alone.

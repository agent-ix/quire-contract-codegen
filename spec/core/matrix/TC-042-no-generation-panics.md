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
test can reach returns its typed value instead of aborting (NFR-005), and that the four index and
arithmetic sites of IR-577 hold no unchecked index, slice or subtraction.

## Test Procedure

1. Walk `src/` and scan every `.rs` file except a file its parent declares under `#[cfg(test)]`
   with `non_test_code_outside_literals` of `tests/common/panic_scan.rs`; assert that
   `panic_tokens_in` finds nothing, and that the walk read the files FR-014-AC-39, FR-018-AC-19 and
   FR-021-AC-21 name, every file that holds a measured site, at least 60 files, and that it found exactly
   one `expect` inside the body of `fn digest` of `impl CaseIdentity`, the dated IR-344 exception,
   and none elsewhere in that file (NFR-005-AC-1). Assert on a hand-built source that the helper keeps an
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
   has none, with a coverage export supplied; assert the `MapMismatch` diagnostic, no
   classification and, for the first, `evaluation_count` of `None`; with no export assert
   `UnavailableObservation` (NFR-005-AC-4).
5. Hand the record-pairing step of `generate_kani` one record fewer and one record more than the
   group has items, and an equal count; assert `KaniRecordCountMismatch` with both counts for the
   first two and a paired item per record for the third (NFR-005-AC-5).
6. PLANNED (IR-577). Hand `rewrite_duplicate_position` of `src/routed/generate.rs` a `DuplicateItem` record
   whose `first_index` equals the group's length, one beyond it, and one at the last valid position;
   assert `KaniDuplicatePositionOutOfRange` with the position and the length for the first two and
   the earlier item's request index for the third (NFR-005-AC-6).
7. PLANNED (IR-577). Hand `observe_clause` a map of zero regions and one of one region, with and
   without a coverage export; assert the `MapMismatch` diagnostic `typed implication census differs
   from map`, no classification, `evaluation_count` of `None` and no consequents. Hand it two
   probed regions with an export and assert no consequents and no `MapMismatch` (NFR-005-AC-7).
8. PLANNED (IR-577). Locate the bodies of `generate_kani`, `rewrite_duplicate_position`,
   `observe_clause` and `generate_boolean_oracle_inner` in the literal-free non-test code of their
   files, as the scan locates `fn digest`; assert each is found and that none holds an index token or
   a subtraction token as NFR-005-AC-8 defines them (NFR-005-AC-8). Assert on a hand-built body that
   the check flags `a[1]`, `a[2..]`, `f(x)[0]`, `x?[0]`, `t.0[1]`, `{ v }[0]`, `n - 1`, `x? - 1` and `n -= 1`, and passes `for l in [a, b]`,
   `vec![a]`, `let [x, ..] = y`, `#[must_use]`, an array type, `fn f() -> u8` and `-n`.

## Expected Results

The scan finds zero panic tokens; each seam test returns its typed refusal or value with no panic.
The four `src/oracle/function/mod.rs` sites, the `ScalarOperation::reachable` arm, the proof-graph
serialization and the `boolean_v1` assertion have no fixture and are held by step 1 alone.
`CaseIdentity::digest` is left to IR-344. Once IR-577's code lands, the two IR-577 seam tests return
their typed refusal or row with no panic, and the four bodies hold no index or subtraction token; the `generate_boolean_oracle_inner`
lookup and `regions[0]` write have no fixture and are held by step 8 alone. The other index, slice
and arithmetic sites NFR-005 Scope measured have no step: some are guarded inside their own
function, and the rest rest on an invariant that code or a type outside the reading function
establishes. NFR-005 leaves them out by a scoping choice for IR-577: they were found unreachable
from untrusted input today, and are neither shown safe nor covered.

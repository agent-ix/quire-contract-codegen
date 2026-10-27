---
id: TC-036
title: "Verify the generated harness subject ABI"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: verifies
---
# TC-036: Verify the generated harness subject ABI

## Description

Verify that a harness's physical bindings follow row-specific expansion of its identity's
ascending logical `arguments` order, that
each binding names its parameter node id and declared domain, that each family's argument has its
stated Rust type and reaches the oracle without loss, and that unbounded and frame obligations
yield no harness.

## Test Procedure

1. For a precondition, a postcondition, an invariant and a scalar-claim harness, read the emitted
   `kani::any()` bindings in source order with `syn` and compare them with the persisted
   physical schema expanded from the identity's logical `arguments`. Boolean/integer expansion
   remains one binding per argument.
2. Read every binding of each identity.
3. Generate a harness over one Boolean and one bounded-integer argument. Run its oracle with the
   integer argument at `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` through the generated widening.
4. Request an obligation whose argument domain has no declared finite bound.
5. Request a frame obligation.
6. Request an obligation whose function has a parameter no binding covers, and one whose binding
   names no parameter.
7. Request an obligation over an argument of each family that has no row in FR-025's Rust-type
   table, retaining the planned Rational row's explicit scope and prerequisite failures.
8. Generate a harness over a state-reading obligation and read the subject call and the pre-state
   copy with `syn`.

9. For planned composite leaf bindings, use
   [TC-048](../../replay/matrix/TC-048-composite-parity-replay-binding.md)
   to check original-node/path/domain/draw ordering and missing/duplicate/conflicting leaf refusals
   with no harness (FR-025-AC-9).

10. For the planned [FR-046](../functional/FR-046-rational-function-predicate-harnesses.md)
    Rational row, select two real function parameters whose declared order differs from ascending
    parameter order. Inspect ascending logical argument order and exactly two physical I64 draws
    per parameter, numerator then denominator, preserving original identities and component domains.
    Reconstruct with owner `Rational::new` and exact accessor roundtrip, including zero `0/1`,
    negative and fractional values. Retain exact source domains, not a denominator-one subset.
    Decode actual retained playback using the planned
    [TC-060](./TC-060-rational-function-predicate-harnesses.md) component/schema, byte/value-count,
    canonicality and public typed replay controls. Request missing, duplicate, foreign,
    role-swapped and domain-conflicting component schemas; no invented parameter may replace
    either component. Keep all legacy Boolean/integer controls unchanged.

## Expected Results

1. Physical binding order equals row-specific expansion of the identity's ascending logical
   argument order. Boolean/integer binding order stays unchanged (FR-025-AC-1).
2. Every binding carries a parameter node id and a declared domain (FR-025-AC-2).
3. The Boolean argument is a `bool`, and the integer argument is an `i64`. Each integer value
   reaches the oracle as an `rt::Integer` equal to its mathematical value (FR-025-AC-3).
4. No harness is emitted, and no machine-range assumption appears (FR-025-AC-4).
5. The frame obligation is accounted `unsupported` with a typed reason, and no harness is emitted
   (FR-025-AC-5).
6. Both requests are refused with a typed reason and no harness (FR-025-AC-6).
7. Every family outside an applicable row, including Rational outside the scoped planned row or
   lacking its lossless decoding/replay prerequisites, is accounted `unsupported` with a typed
   family/capability reason and no harness (FR-025-AC-7).
8. Each state value is passed as `&mut` to a harness-owned value, a copy is taken before the call,
   the pre-state assertion reads the copy and the post-state assertion reads the value after the
   call (FR-025-AC-8).

9. Original composite leaf bindings and literal singleton identity/value satisfy FR-025-AC-9;
   TC-048 defines the reconstruction scenario, with no fabricated machine composite argument.

10. One original Rational parameter retains both uniquely identified physical bindings, original
    bounds and explicit component roles. Lossless decoding respects the physical byte/value-count
    limits before reconstructing one canonical owner Rational and submitting one typed witness
    value per parameter. Malformed schema/entries and noncanonical pairs refuse with typed causes
    and no replayed failure; legacy values/order remain unchanged (FR-025-AC-10).

## Status

Planned. The former step 1 order equality was asserted only for the precondition, postcondition and invariant
harnesses of the retired V1 `BoundClause` arm (`tests/it/kani_argument_order.rs`, removed under IR-348);
the ascending check and the scalar-claim harness remain unbacked. Steps 2 and 6 are planned,
because `ObligationBinding` carries no parameter node id at this revision. Steps 3 to 5, 7 and 8 are
planned. Step 9 is planned (IR-806) and has no executable coverage.

Step 10 and the Rational extensions of steps 1, 2 and 7 are planned (IR-258), with
no executable binder, decoder/grouping implementation or actual backend/replay credit yet.
The planned row does not retrospectively qualify the existing legacy ABI controls.

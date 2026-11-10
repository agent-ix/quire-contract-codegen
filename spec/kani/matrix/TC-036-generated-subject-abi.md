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
11. For the planned scalar V2 postcondition result path, select an admitted QSL operation anchor
    and compare its complete `model_operations(context, 65_536)` signature with the clause's
    `self`, optional `result` and ordered parameter binders. Exercise a work-limit refusal and
    an unavailable or mismatched signature, retaining the IR read refusal's fields.
12. Select admitted scalar postconditions for a Boolean result, a bounded Integer result, no
    result, an unbounded result and an Integer range beyond signed-i64 capacity. Inspect the
    generated subject return type, parameter draws and call arguments; keep the existing
    supported collection result path as a separate control.
13. For a bounded Integer result, compare the value returned by the subject with the value
    passed to the clause oracle. Check an in-domain value and a produced out-of-domain value,
    and inspect the order of the result-domain assertion, clause assertion and final cover. For
    a Boolean result, check `true` and `false` subject returns against the exact oracle-bound
    Boolean; change only that oracle argument to its opposite as a negative control.
14. For the planned [FR-049](../functional/FR-049-enum-scalar-harnesses.md) enum row, generate a
    comparison over an enum operand and read its persisted identity: assert exactly one `i64`
    physical binding grouped to the original parameter with declaration, ordered member list and
    index bounds. Decode retained playback with the existing I64 width, byte and comment checks and
    caps; assert an in-range index replays as the declaration's member at that position, and an
    out-of-range, missing, duplicate or foreign binding refuses with a typed reason and no replay.
    Re-run the Boolean and integer controls unchanged.

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
11. The complete admitted signature is read under the fixed per-read provision and matched to
    the binder order. Failed or partial reads and mismatches produce typed no-harness refusals;
    an IR read refusal retains code, context, work and limit (FR-025-AC-11).
12. Boolean, bounded Integer and absent results produce `bool`, `i64` and `()` subject returns,
    respectively. Unsupported result domains refuse; no result is drawn, recorded as a parameter
    or passed as a subject argument. Collection result support is unchanged (FR-025-AC-12).
13. The returned Integer reaches the oracle unchanged after lossless widening. An out-of-domain
    return fails an assertion after the subject call, without an assumption hiding it; the clause
    assertion and final cover follow. For both Boolean returns, the oracle-bound value equals
    the subject-returned `bool`; the opposite-value control fails this identity check
    (FR-025-AC-13).
14. One enum operand has exactly one grouped `i64` binding with declaration, members and bounds;
    an in-range index replays as the member at that position, and each out-of-range, missing,
    duplicate or foreign binding refuses with no replay; Boolean and integer controls are
    unchanged (FR-025-AC-14).

## Status

Planned. The former step 1 order equality was asserted only for the precondition, postcondition and invariant
harnesses of the retired V1 `BoundClause` arm (`tests/it/kani_argument_order.rs`, removed under IR-348);
the ascending check and the scalar-claim harness remain unbacked. Steps 2 and 6 are planned,
because `ObligationBinding` carries no parameter node id at this revision. Steps 3 to 5, 7 and 8 are
planned. Step 9 is planned (IR-806) and has no executable coverage.

Step 10 and the Rational extensions of steps 1, 2 and 7 are planned (IR-258), with
no executable binder, decoder/grouping implementation or actual backend/replay credit yet.
The planned row does not retrospectively qualify the existing legacy ABI controls.

Steps 11 to 13 are planned until IR-948 lands. IR-1289 currently refuses a scalar V2 clause
postcondition with a declared result, including when the clause body does not read that result;
the supported collection result path is separate. No generated scalar return, produced-result
domain assertion or `model_operations` binder-mapping evidence is credited yet.

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

Verify that a harness's symbolic arguments follow its identity's ascending `arguments` order, that
each binding names its parameter node id and declared domain, that each family's argument has its
stated Rust type and reaches the oracle without loss, and that unbounded and frame obligations
yield no harness.

## Test Procedure

1. For a precondition, a postcondition, an invariant and a scalar-claim harness, read the emitted
   `kani::any()` bindings in source order with `syn` and compare them with the identity's
   `arguments`.
2. Read every binding of each identity.
3. Generate a harness over one Boolean and one bounded-integer argument. Run its oracle with the
   integer argument at `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` through the generated widening.
4. Request an obligation whose argument domain has no declared finite bound.
5. Request a frame obligation.
6. Request an obligation whose function has a parameter no binding covers, and one whose binding
   names no parameter.
7. Request an obligation over an argument of each family that has no row in FR-025's Rust-type
   table.
8. Generate a harness over a state-reading obligation and read the subject call and the pre-state
   copy with `syn`.

## Expected Results

1. The binding order equals the `arguments` order in every harness, and that order ascends by
   identifier (FR-025-AC-1).
2. Every binding carries a parameter node id and a declared domain (FR-025-AC-2).
3. The Boolean argument is a `bool`, and the integer argument is an `i64`. Each integer value
   reaches the oracle as an `rt::Integer` equal to its mathematical value (FR-025-AC-3).
4. No harness is emitted, and no machine-range assumption appears (FR-025-AC-4).
5. The frame obligation is accounted `unsupported` with a typed reason, and no harness is emitted
   (FR-025-AC-5).
6. Both requests are refused with a typed reason and no harness (FR-025-AC-6).
7. Each is accounted `unsupported` with a typed reason naming its family, and no harness is emitted
   (FR-025-AC-7).
8. Each state value is passed as `&mut` to a harness-owned value, a copy is taken before the call,
   the pre-state assertion reads the copy and the post-state assertion reads the value after the
   call (FR-025-AC-8).

## Status

Planned. Step 1's order equality is asserted only for the precondition, postcondition and invariant
harnesses of the retired V1 `BoundClause` arm (`tests/it/kani_argument_order.rs`, traced to
FR-016-AC-8); the ascending check and the scalar-claim harness are not. Steps 2 and 6 are planned,
because `ObligationBinding` carries no parameter node id at this revision. Steps 3 to 5, 7 and 8 are
planned.

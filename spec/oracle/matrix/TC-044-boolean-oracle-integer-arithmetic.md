---
id: TC-044
title: "Verify Boolean oracle integer arithmetic and comparison take the runtime's meaning"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
---
# TC-044: Verify Boolean oracle integer arithmetic and comparison take the runtime's meaning

## Description

Verify that the Boolean oracle renders integer add, subtract, multiply, divide and remainder as
Contract Runtime exact operations and the six integer comparisons as the meaning the runtime
gives them, that overflow and a zero divisor are the runtime's typed outcomes and never a panic or
a wrapped value, that arithmetic over a `saturate` integer type is refused, and that every
consumer that cannot carry an outcome refuses an arithmetic clause. It also verifies that the
defect cases fail on the tree before the change and pass after it.

This case is 🚧 Planned. No test exists for it yet.

## Test Procedure

1. Generate the oracle for one expression per operator over a `reject` integer type, and inspect
   the source: Add, Subtract and Multiply call `evaluate_integer_arithmetic` with their variant
   and the type's interval as the bound; Divide and Remainder call `divide` with the truncating
   profile and a bounded domain, and read the quotient and the remainder respectively. Assert no
   Rust `+`, `-`, `*`, `/` or `%` between operand expressions, none of `checked_`, `wrapping_`,
   `saturating_` or `overflowing_`, no `exact::modulo`, and no `unwrap`, `expect` or panic macro
   (FR-031-AC-1, AC-2, AC-3).
2. Generate an arithmetic-free oracle and an arithmetic-bearing one. Assert the first returns
   `bool` with no meter and is byte-identical to the output of the tree before the change, and
   the second returns `Outcome<bool>` with a trailing `&mut Meter`. Regenerate both and compare
   bytes (FR-031-AC-3).
3. Generate one oracle per comparison over two arithmetic-free operands and one per comparison
   with an arithmetic operand. Assert the operator or runtime call in the FR-031 comparison table.
   Compile and run the arithmetic-free set in a generated crate against `quire-contract-runtime`
   over the seven boundary values, two interior values, taken pairwise in both orders, and
   compare each result with `exact::order_numbers` for the four orderings and with the runtime's
   equality evaluation for equal and not-equal. This half passes on the tree before the change and
   is a held agreement (FR-031-AC-4).
4. Overflow cases under `reject`. For each row, build the expression through
   `DeclarationEnvironment::check_expression` so IR admits it, generate the oracle, compile and
   run it in a generated crate, and assert the expected outcome. Run each case first on the tree
   before the change and record the failure.

   | Case | Operation | Operands | Expected outcome |
   |---|---|---|---|
   | O-1 | Add | `i64::MAX`, `1` | `Refused(IntegerOutOfDomain)` |
   | O-2 | Subtract | `i64::MIN`, `1` | `Refused(IntegerOutOfDomain)` |
   | O-3 | Divide | `i64::MIN`, `-1` | `Refused(DivisionPairOutOfDomain)` |
   | O-4 | Multiply | `i64::MIN`, `2` | `Refused(IntegerOutOfDomain)` |
   | O-5 | Remainder | `i64::MIN`, `-1` | `Refused(DivisionPairOutOfDomain)` |

   Each operand vector is outside the declared domain of the operands. IR admits an arithmetic
   node under `reject` only when the declared bounds and the dominating guards prove its result in
   range, so an in-domain operand cannot overflow, and a guard that fails short-circuits the
   operation. The test chooses a declaration and a guard for each case so that the proof rests on a
   bound the vector violates and the guard still passes. These shapes are suggested starting
   points; they were derived from IR's discharge rules and not run at specification time, so the
   author confirms that `check_expression` returns `Ok` and adjusts the declaration, keeping the
   operation and the operand vector: O-4 over `0..=10` with the guard `x < 5` and the clause
   `x * 2 <= 10`, called with `x = i64::MIN`; O-2 as `i64::MIN + -1` over `0..=10` with the guard
   `x <= 0` and the clause `x + y <= 10`, called with `x = i64::MIN` and `y = -1`; and O-3 over
   `-5..=5` with guards that bound the divisor to `-1` on both sides, called with
   `x = i64::MIN`. A positive overflow such as O-1 needs an operand that passes a lower-bound
   guard while its upper bound rests on a declaration it violates, and may have no admitted
   `reject` construction; where IR admits no expression that reaches a row's vector, the test uses
   the nearest reachable vector of the same operation and the code change names the substitution,
   and the `saturate` rows of step 6 carry the literal vector. The expected outcome is asserted as
   the exact variant, and in no case does the generated function panic (FR-031-AC-5).
5. Zero-divisor cases under `reject`, built as in step 4 and run first on the tree before the
   change:

   | Case | Operation | Operands | Expected outcome |
   |---|---|---|---|
   | Z-1 | Divide | `x`, `0` for a non-zero `x` | `Undefined(DivisionByZero)` |
   | Z-2 | Divide | `0`, `0` | `Undefined(DivisionByZero)` |
   | Z-3 | Remainder | `x`, `0` for a non-zero `x` | `Undefined(DivisionByZero)` |
   | Z-4 | Remainder | `0`, `0` | `Undefined(DivisionByZero)` |

   A suggested shape for Z-1 and Z-2, derived and not run at specification time, is a divisor
   declared over `1..=10`, so that the declaration proves it non-zero, with a guard `x >= 10` on
   the dividend that keeps the quotient in range, called with the divisor `0`. Remainder needs a
   declaration and guards of the test's choosing on the same rule, and uses the nearest reachable
   vector as in step 4 when `reject` admits none (FR-031-AC-6). A zero divisor is also
   reached through a `saturate` type declared non-zero, which step 6 asserts is refused.
6. `saturate` cases. For each of Add, Subtract, Multiply, Divide and Remainder, build an expression
   over a `saturate` integer type that IR admits, and assert generation is refused with
   `UnsupportedExpression` at the arithmetic node's span and no artifact. Cover the types
   `i64::MIN..=i64::MAX` and a type whose declaration excludes zero, and the rows
   `i64::MAX + 1` (Add), `i64::MIN - 1` (Subtract), `i64::MIN / -1` (Divide, with a divisor guard
   that admits `-1`), `x / 0` and `x % 0` (a divisor declared non-zero). Add a clause holding a
   `saturate` addition and then a later negation, and assert the refusal span is the addition's.
   Run each on the tree before the change and record that it generated, and that the generated
   function panics on the named operands in a debug build (FR-031-AC-7). The existing
   unsupported-diagnostics test, whose first unsupported node is the negation after a `saturate`
   addition, is rewritten for the new first locus.
7. Differential grid. Generate an arithmetic-bearing oracle per operator under `reject`, compile
   and run it in a generated crate over a grid of operand values inside the declared domain, at its
   edges and outside it, and compare the outcome, kind included, with the direct runtime
   evaluation of the same expression (FR-031-AC-8).
8. Propagation. Build clauses where the arithmetic stop is in the left operand of a short-circuit
   connective, in a right operand the left decides, and in each operand of a total connective;
   assert the oracle returns the first stop, never reaches the skipped one, and never maps a stop
   to `Completed(true)` or `Completed(false)` (FR-031-AC-9).
9. Consumers. Request the tri-state harness, the bound strategies and the Kani obligation clause
   lowering for a clause holding a `reject` arithmetic node and for one holding a `saturate` node.
   Assert the refusal codes FR-031 names, no emitted raw arithmetic operator, and no artifact
   (FR-031-AC-10).
10. V1 Kani bundle. Generate the bundle for `amount < 1000` implies `amount + 1 <= 1000` over
    `0..=1000` under `reject` and assert it generates and its harness asserts `Completed(true)`
    with no `unwrap`, `expect` or raw operator. With the installed Kani backend, run the unmutated
    clause and assert it verifies, and run a subject that produces an out-of-domain post-state and
    assert it fails on the runtime's refusal. Skip the backend half only where Kani is not
    installed, reporting the skip (FR-031-AC-11).

## Expected Results

Every arithmetic operation in the oracle is a runtime `exact` call; overflow under `reject` is the
runtime's `Refused` and a zero divisor is `Undefined`, with no panic and no wrapped value; a
`saturate` arithmetic clause is refused at its node; the six comparisons agree with the runtime;
every consumer that needs a `bool` refuses an arithmetic clause; and the V1 Kani bundle carries the
outcome and fails closed on a non-completed one. The overflow, zero-divisor and `saturate` cases
each failed on the tree before the change and pass after it, and the code change records both
runs.

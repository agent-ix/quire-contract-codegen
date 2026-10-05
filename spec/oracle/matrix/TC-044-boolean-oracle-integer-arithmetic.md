---
id: TC-044
title: "Verify Boolean oracle integer arithmetic and comparison take the exact kernel's meaning"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
---
# TC-044: Verify Boolean oracle integer arithmetic and comparison take the exact kernel's meaning

## Description

Verify that the native Boolean oracle renders integer add, subtract, multiply, divide and
remainder as exact-kernel operations (`quire_exact`) and the six integer comparisons as the meaning
the kernel gives them, that overflow, a zero divisor and the minimum divided by negative one there
are the kernel's typed outcomes and never a panic or a wrapped value, that arithmetic over a
`saturate` integer type is refused with its typed code in both consumers and the interim division
code no longer exists, and that every consumer other than the V1 Kani bundle that cannot carry an
outcome refuses an arithmetic clause. It also verifies the V1 Kani bundle oracle: its fixed-width
arithmetic, a second implementation of the rule (infix for add, subtract, multiply and divide,
`wrapping_rem` for remainder), keeps the exemplar's text shape, proves the exemplar, makes overflow
and a zero divisor a falsifiable Kani property, and agrees with the native oracle inside the
domain. Finally it verifies that the defect cases fail on the tree before the change and pass
after it.

This case is ⚠️ Partially covered. IR-596's closing code change implemented the steps that do not
concern division, in `tests/it/oracle_arithmetic.rs`, with the real-Kani steps (the exemplar
controls of step 10 and the overflow property of step 12) in `tests/it/kani_generation.rs`, run
through `make kani`. The QSL exemplar run of step 10 (`tests/qsl_kani_exemplar.rs` in the
integration repository) is pending: it needs a checkout of the integration repository, and the CG
run of the same bundle shape is the closest equivalent, not a substitute. The divide and remainder
change (IR-602) is planned: the O-3 and O-5 rows of step 4, step 5, the divide and remainder half
of step 7 and the rewritten step 11 (FR-031-AC-2, AC-6, AC-16, AC-17 and AC-18), and the new steps
14 to 17 (FR-031-AC-22 to AC-27). Until that code lands the divide and remainder constructions are
refused at generation with the interim code, which is the failing run those steps record first.
The steps that call `quire_exact::divide` or migrate the native oracle (1's divide half, 4's O-3
and O-5, 5, 7's divide and remainder half, 11's removed-code assertion (FR-031-AC-27), 14 and 16)
are gated on a prerequisite: QSL moves to the
single-member `divide`, and CG then bumps QSL and `quire-exact` together, because CG's one
`quire-exact` lock entry is shared with QSL's crates. Step 14 is also contingent on the owner's
decision OD-2. Steps 10's divide and remainder bundle shape, 15, and 11's `saturate` refusal rows
(FR-031-AC-18) call no kernel and are buildable before the prerequisite.

## Test Procedure

1. Generate the native oracle for one add, one subtract and one multiply over a `reject` integer type,
   and inspect the source: each calls `evaluate_integer_arithmetic` with its variant and the
   type's interval as the bound, and the body holds no Rust `+`, `-` or `*` between operand
   expressions and none of `checked_`, `wrapping_`, `saturating_` or `overflowing_`
   (FR-031-AC-1). Generate the native oracle for one divide and one remainder over a `reject`
   integer type (the constructions R-1 and R-2 of step 7) and inspect the source: each calls
   `divide` with `DivisionProfile::Truncating`, `DivisionMember::Quotient` for the divide and
   `DivisionMember::Remainder` for the remainder, and `IntegerDomain::Bounded` over the node's
   interval, and the body holds no `/` or `%`, none of `checked_div`, `checked_rem`,
   `wrapping_rem`, `div_euclid`, `rem_euclid` or `modulo`, and no division of a pair
   (FR-031-AC-2).
2. Generate an arithmetic-free oracle and an arithmetic-bearing one. Assert the first returns
   `bool` with no meter, and the second returns `Outcome<bool>` with a trailing `&mut Meter`
   (FR-031-AC-3). Assert the arithmetic-free oracle is byte-identical to the output of the tree
   before the change (FR-031-AC-12). Regenerate the arithmetic-bearing oracle twice and from a
   permuted request and compare bytes (FR-031-AC-13). Scan the arithmetic-bearing source for
   `unwrap`, `expect` and panic macros (FR-031-AC-14). These four assertions are separate tests, and the
   AC-12, AC-13 and AC-14 assertions pass on the tree before the change: they are held
   agreements, not defect proofs.
3. Generate one oracle per comparison over two arithmetic-free operands, assert the native operator
   the FR-031 comparison table names, compile and run them in a generated crate against
   `quire-contract-runtime` over the seven boundary values and two interior values, taken pairwise
   in both orders, and compare each result with `exact::order_numbers` for the four orderings and
   with `check_equality` then `CheckedEquality::evaluate` for equal and not-equal. This step passes
   on the tree before the change and is a held agreement (FR-031-AC-4). Generate one oracle per
   comparison with an arithmetic operand and assert the runtime call the table names and no Rust
   comparison operator between the operands (FR-031-AC-15).
4. Overflow cases under `reject`. Build each expression through
   `DeclarationEnvironment::check_expression`, generate the oracle, compile and run it in a
   generated crate with the named operands, and assert the outcome. Run each first on the tree
   before the change and record the failure. All of these constructions were admitted by
   `check_expression` and generated by the tree before the change when this case was written, and
   each of them has every operand in a declared domain except the vector's own.

   | Case | Operation | Declared type | Clause | Vector | Expected outcome | Belongs to |
   |---|---|---|---|---|---|---|
   | O-1 | Add | `-10..=0` | `x >= -5 && (y >= -5 && x + y <= 0)` | `x = i64::MAX`, `y = 1` | `Refused(IntegerOutOfDomain)` | AC-5 |
   | O-2 | Subtract | `-10..=0` | `x <= -5 && (y >= -5 && x - y <= 0)` | `x = i64::MIN`, `y = 1` | `Refused(IntegerOutOfDomain)` | AC-5 |
   | O-4 | Multiply | `0..=10` | `x < 5 && x * 2 <= 10` | `x = i64::MIN` | `Refused(IntegerOutOfDomain)` | AC-5 |
   | O-3 | Divide | `-5..=5` | `y >= -1 && (y <= -1 && x / y <= 5)` | `x = i64::MIN`, `y = -1` | `Refused(DivisionOutOfDomain)`, member `Quotient`, domain `-5..=5`, code `division_out_of_domain`, cause `quotient-outside-domain` | AC-16 |
   | O-5 | Remainder | `-5..=5` | `y >= -1 && (y <= -1 && x % y <= 5)` | `x = i64::MIN`, `y = -1` | `Completed(true)` (the remainder is the exact 0, in range) | AC-16 |

   The proof of safety in each admitted clause rests on a declared bound that the vector violates,
   while the guards still pass. For O-1 the upper bound 0 of `x` comes from the declaration and the
   guard only bounds it below; for O-2 the lower bound of `x` is declared and the guard bounds it
   above; for O-4 the guard bounds `x` above and the declared lower bound is violated; for O-3 and
   O-5 the guards fix `y` at -1 and the declaration bounds `x` at -5, which the vector violates.
   The expected outcome of every row is the exact variant, and in no case does the generated
   function panic. O-3 and O-5 are the pair that separates out of domain from overflow: the
   quotient of the minimum by negative one is the exact integer 2^63, which `-5..=5` does not
   hold, so it is refused, and the remainder is 0, which it does, so it completes. A test that
   asserted `Refused` for O-5, or `Completed` for O-3, fails.
5. Zero-divisor cases (FR-031-AC-6). Build each through `check_expression` as in step 4 and run
   first on the tree before the change, where each is refused at generation with the interim
   code; after the change the expected outcome of every row is `Undefined(DivisionByZero)`, never
   `Refused` and never a panic:

   | Case | Operation | Declared type | Clause | Vector |
   |---|---|---|---|---|
   | Z-1 | Divide | `1..=10` | `y <= 1 && x / y <= 10` | `x = 5`, `y = 0` |
   | Z-2 | Divide | `1..=10` | `y <= 1 && x / y <= 10` | `x = 0`, `y = 0` |
   | Z-3 | Remainder | `-10..=11` | `y <= 0 && x % (y + 11) <= 10` | `x = 5`, `y = -11` |
   | Z-4 | Remainder | `-10..=11` | `y <= 0 && x % (y + 11) <= 10` | `x = 0`, `y = -11` |

   Z-1 and Z-2 share one construction and differ in the dividend, because a guard on the dividend
   would short-circuit before `0 / 0`; the declaration proves the divisor at least 1 and the guard
   `y <= 1` leaves the vector `y = 0` outside it. Remainder over `1..=10` with the same guard is
   refused by IR, because its result can be 0 and leave the interval, so the remainder rows use a
   compound divisor `y + 11` that is 0 at `y = -11`.
6. `saturate` cases. For an add, a subtract and a multiply over a `saturate` integer type that IR
   admits, assert generation is refused with `UnsupportedSaturatingArithmetic` at the node's span
   and no artifact, and that the message names the missing runtime saturating operation. The rows
   are `x + 1` and `x - 1` over `i64::MIN..=i64::MAX`, evaluated before the change at
   `x = i64::MAX` and `x = i64::MIN`, and `x * 2` over `0..=10`. Add a clause holding a
   `saturate` addition and then a later negation, and assert the refusal span is the addition's.
   Run each on the tree before the change and record that it generated and that the generated
   function panics in a debug build on the named operands (FR-031-AC-7). The existing
   unsupported-diagnostics test, whose first unsupported node is the negation after a `saturate`
   addition, is rewritten for the new first locus. A `saturate` divide or remainder is covered by
   step 11.
7. Differential grid (FR-031-AC-8). Generate an add, a subtract and a multiply oracle under
   `reject`, compile and run them in a generated crate over a grid of operand values inside the
   declared domain, at its edges and outside it, and compare the outcome with a plain-integer
   model in the test: the exact result in `i128`, `Completed` with it when it lies in the type's
   interval and `Refused(IntegerOutOfDomain)` when it does not. The model calls no `exact::`
   function and reads nothing from the emitter. The divide and remainder differential
   (FR-031-AC-17) runs the same way over divide and remainder oracles, with the model in `i128`
   under the truncating law: a zero divisor is `Undefined(DivisionByZero)`; otherwise the selected
   member, the quotient for a divide and the remainder for a remainder, is `Completed` when it
   lies in the type's interval and `Refused(DivisionOutOfDomain)` with that member when it does
   not. Its grid covers dividends and divisors inside the declared domain, at its edges, outside
   it, and with negative operands (where the laws differ, so the truncating one is pinned). It
   includes the two reproductions as named rows, each built through `check_expression`
   (both are admitted by IR):

   | Case | Operation | Declared type | Clause | Vector | Expected outcome |
   |---|---|---|---|---|---|
   | R-1 | Divide | `1..=10` | `10 / y <= 10` | `y = 5` | `Completed(true)`: the quotient 2 is in range, the remainder 0 is not |
   | R-2 | Remainder | `-10..=5` | `x % -1 <= 5` | `x = -10` | `Completed(true)`: the remainder 0 is in range, the quotient 10 is not |

   The oracle returns `Outcome<bool>`, so a member's value shows only through the comparison, and
   R-1 and R-2 complete `true` under every law. The grid therefore also carries the law vector
   `y != 0 && x / y <= -4` over a `reject` type of `-10..=10` (built through `check_expression`)
   at `x = -7`, `y = 2`: `Completed(false)` under truncating (quotient -3), where floor and
   Euclidean give `Completed(true)` (quotient -4). Step 1 pins the law in the source.

   A division that checked the unselected member as well, as the earlier pair operation did,
   returns `Refused` on both rows, which is why they are the named vectors.
8. Propagation (FR-031-AC-9). Build clauses with an add, subtract or multiply stop in the left
   operand of a short-circuit connective, in a right operand the left decides, and in each operand
   of a total connective; assert the oracle returns the first stop, never reaches the skipped one,
   and never maps a stop to `Completed(true)` or `Completed(false)`.
9. Consumers (FR-031-AC-10). Request the tri-state harness, the bound strategies and the Kani
   obligation clause lowering for a clause holding a `reject` add and for one holding a `saturate`
   add. Assert the refusal codes the FR-031 Consumers section names, no emitted raw arithmetic
   operator, and no artifact.
10. V1 Kani bundle (FR-031-AC-11, AC-19). Request `generate_kani_bundle` for `amount < 1000`
    implies `amount + 1 <= 1000` over `0..=1000` under `reject`, with the canonical absent
    precondition and a postcondition-only subject as the exemplar has. Parse the bundle with
    `syn`, find the postcondition oracle function, and assert it returns `bool`, takes no meter,
    holds exactly one `ExprBinary` with `BinOp::Add`, holds no `exact::` call, and that its file
    states that the arithmetic is checked by Kani. Then run real `cargo kani` over it with the
    bundle's own option vector and assert it verifies; append ` + 1_i64` after the addition and
    assert the run is falsified with the counterexample `amount_current = 999`; drop the `+ 1`
    and assert the run verifies. These three runs are the exemplar's own runs: the closing change
    also runs `tests/qsl_kani_exemplar.rs` of the integration repository (IT-011 and IT-010-SC-05)
    against its branch before and after, and records both. Repeat the generation over a `saturate`
    add, subtract, multiply, divide and remainder and assert `ClauseGenerationFailed` retaining
    `UnsupportedSaturatingArithmetic` and no harness. Request the bundle over R-1 (a divide) and
    R-2 (a remainder) and assert each generates, and that the postcondition oracle returns
    `bool`, takes no meter, holds one `ExprBinary` with `BinOp::Div` for R-1 and one
    `wrapping_rem` method call and no `BinOp::Rem` for R-2, holds no `exact::` call and no other
    arithmetic method, and that its file states that the arithmetic is checked by Kani
    (FR-031-AC-23). Assert an arithmetic-free clause's bundle is unchanged.
11. Saturate-division refusal (FR-031-AC-18, buildable before the lock prerequisite) and the
    removed interim code (FR-031-AC-27, gated on it). For each of the
    `saturate` rows below, in the native oracle and in the bundle, assert generation is refused
    with `UnsupportedSaturatingArithmetic` at the node's span, that its message names the missing
    saturating operation, that its terminal state is `unsupported`, and that no output holds a raw
    `/` or `%`. Run each first on the tree before the change and record that it is refused with
    `UnsupportedIntegerDivision` instead, which fails the assertion. Then, only once the prerequisite
    has landed and the native division exists (FR-031-AC-27), assert the interim code
    is gone: `GenerationErrorCode` has no variant `UnsupportedIntegerDivision`, nothing under
    `src/`, `tests/` or `schemas/` spells it, and no diagnostic message that generation can emit
    names IR-601. The rows below are admitted by `check_expression`.

    | Case | Operation | Declared type, policy | Clause | Vector |
    |---|---|---|---|---|
    | S-1 | Divide | `1..=10`, `saturate` | `x / y <= 10` | `x = 5`, `y = 0` |
    | S-2 | Remainder | `1..=10`, `saturate` | `x % y <= 10` | `x = 5`, `y = 0` |
    | S-3 | Divide | `i64::MIN..=i64::MAX`, `saturate` | `y != 0 && x / y <= 0` | `x = i64::MIN`, `y = -1` |
    | S-4 | Remainder | `i64::MIN..=i64::MAX`, `saturate` | `y != 0 && x % y <= 0` | `x = i64::MIN`, `y = -1` |

    Two existing tests change again with the divide and remainder change. `oracle_generation`
    `tc_023_native_proven_division_is_refused_until_the_ir_601_ruling` (IR-596's rewrite of
    `tc_023_native_proven_numeric_obligations_render_without_assumptions`) asserts the refusal of
    a guarded division, and is rewritten to assert that the guarded division generates, calls
    `divide` and holds no `/`, under a name that does not mention IR-601. The guarded-division
    case of TC-017 in `bound_strategy_generation`, which IR-596 changed to expect the oracle's
    refusal and an `UnsupportedClause` carrying `UnsupportedIntegerDivision`, expects again what it
    expected before IR-596: `generate_bound_oracles` generates and the strategy refuses with
    `UnsupportedRelation`. The `tc_003` test for the first unsupported locus (step 6) is
    unchanged.
12. Kani overflow is a falsifiable property (FR-031-AC-20). Generate the bundle oracle of
    `x < 5 && x * 2 <= 10` over `0..=10` under `reject` (construction O-4), and in a Kani crate
    call it from a harness with `x = kani::any()` and no assumption. Assert real Kani fails
    the "attempt to multiply with overflow" check, and that the counterexample's playback value
    satisfies `x < 5` and overflows `x * 2`; do not assert a particular value, because any
    `x` below `-2^62` is a valid answer. Assert a harness that assumes the declared domain on `x` verifies the same oracle.
13. Differential between the two implementations (FR-031-AC-21). For add, subtract and multiply,
    generate the native oracle and the Kani bundle oracle of the same typed expression (the
    constructions O-1, O-2 and O-4 and one with no guard), compile both in a generated crate
    against `quire-contract-runtime` in plain `cargo test` with no Kani (against `quire-exact`
    once the migration of step 14 lands), and over a grid of
    vectors inside the declared domain, its edges included, at which the guards pass, assert the
    bundle oracle's `bool` equals the `bool` inside `Completed` of the native oracle.
14. Kernel family and connectives (FR-031-AC-22, AC-26). Generate a native oracle with an arithmetic node and the
    construction Z-3 (an add and a remainder in one oracle), and assert the source aliases
    `quire_exact` as `rt`, takes `&mut rt::Meter`, returns `rt::Outcome<bool>`, and holds no
    `quire_contract_runtime::exact` path. Compile and run them in a generated crate whose manifest
    names `quire-exact`, and assert the crate's resolved graph holds one `quire-exact`. Assert
    CG's `Cargo.toml` names `quire-exact` directly from `agent-ix/quire-exact`, and that `make
    deny`'s one-copy check passes over CG's lock, with `qsl-replay` and `qsl-semantics` resolving
    that one entry. This step is contingent on the owner's decision OD-2 and gated on the lock
    prerequisite of FR-031. Assert, for the migrated oracle (FR-031-AC-26), that no
    `operators::*_short_circuit` path appears, that the connectives are inline matches, that the
    outcomes of step 8's cases and of Z-3 equal their outcomes under Contract Runtime's
    rendering, and that no connective charges a point.
15. Kani divide and remainder are falsifiable properties (FR-031-AC-24). Generate the bundle
    oracle of Z-1 and of Z-3, and in a Kani crate call each from a harness with unconstrained
    operands. Assert real Kani fails "attempt to divide by zero" and "attempt to divide with
    overflow" for Z-1, each with a counterexample, and "attempt to calculate the remainder with
    a divisor of zero" and no other arithmetic check for Z-3. Assert a harness that assumes the
    declared domain and the guards verifies the oracles of R-1 and R-2, and that a probe of the
    remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX` at `x = i64::MIN`, `y = -1`
    verifies, the oracle's remainder being `wrapping_rem`.
16. Differential for divide and remainder (FR-031-AC-25). Generate the native oracle and the
    Kani bundle oracle of R-1, R-2 and `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX`, and of
    a guarded divide and a guarded remainder with negative operands, compile both in a generated
    crate against `quire-exact` in plain `cargo test` with no Kani, and over a grid inside the
    declared domain with a non-zero divisor, its edges included (the last oracle at `x = i64::MIN`,
    `y = -1` among them), assert the bundle oracle's `bool` equals the `bool` inside `Completed`
    of the native oracle.
17. Order of the runs. Run steps 4, 5, 7 (the divide and remainder rows), 11, 14 and 16 first on
    the tree before the change and record the failing run of each, before the passing run, as the
    FR-031 Test obligations state. Steps 15 and 10's divide and remainder generation need `make
    kani` and the installed backend; a run that cannot be made is reported as not run.

## Expected Results

Every add, subtract, multiply, divide and remainder in the native oracle is a `quire_exact` call.
Overflow under `reject` is `Refused(IntegerOutOfDomain)`, a zero divisor is
`Undefined(DivisionByZero)`, a divide or remainder member outside the interval is
`Refused(DivisionOutOfDomain)`, the minimum divided by negative one is refused with member
`Quotient` and the minimum remainder negative one completes with 0, R-1 and R-2 complete, with no
panic and no wrapped value, all agreeing with an independent plain-integer model; the Kani bundle
oracle keeps the infix operators (`wrapping_rem` for a remainder), in the shape the exemplar's
mutation targets, proves the exemplar and its equivalent mutant, falsifies the `left + right + 1`
mutant at `amount_current = 999`, fails Kani's overflow and zero-divisor checks on an operand
outside the discharge, and agrees with the native oracle inside the domain; a `saturate`
arithmetic clause of any of the five operators is refused at its node with
`UnsupportedSaturatingArithmetic` in both consumers, and `UnsupportedIntegerDivision` exists
nowhere; the six comparisons agree with the kernel; every other consumer that needs a `bool`
refuses an arithmetic clause; and the overflow, zero-divisor, minimum-by-negative-one, `saturate`
and differential cases each failed on the tree before the change and pass after it, with the code
change recording both runs and the exemplar's runs. The divide and remainder rows (FR-031-AC-2,
AC-6, AC-16, AC-17, AC-18 and AC-22 to AC-27) are verified by the IR-602 code change, which is
gated on the lock prerequisite as the status above states.

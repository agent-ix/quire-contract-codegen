---
id: SR-17940486709564847047747
title: "IR-1447 PR #808 spec-review failure-domain"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen PR #808 at 56f1887d; IR-1447; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1447-kani-control-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1447. Unstated failure modes of the control lowering: evaluation strategy, stop
propagation, and the topology of scrutinee and arms.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Evaluation strategy and stop propagation are stated only for `if` ("both branches evaluated by the kernel only when selected", AC-153). FR-015-AC-154 does not require that only the selected `case` arm is evaluated, so an eager rendering would turn a refusal (out-of-bound arithmetic) in an unselected arm into a false failure. No AC covers a selected branch, arm or `let` value whose kernel evaluation refuses (FR-015-AC-37: `Refused` outside the result bound): whether it is a `ClauseStop` and what the independent assertion expects. Add a lazy-arm statement to AC-154, and one AC (or one TC-025 fixture) for a refusing selected branch and a refusing unselected branch. | spec/kani/functional/FR-015-bounded-kani-obligations.md:980-982; FR-015-AC-37 |
| FND-002 | low | No fixture covers `let` binder shadowing or capture: a `let` whose name equals an operation parameter, and a nested `let` rebinding the same name. Native Rust `let` shadows silently, so a lowering that resolves the name to the wrong binder would still compile. A one-line fixture in AC-152 (binder named like a parameter) would make the binder resolution falsifiable. | spec/kani/functional/FR-015-bounded-kani-obligations.md:978 |

## Verdict

Two gaps in the failure domain. The nesting and precedence gaps are recorded once, in the
integrity review (FND-001 and FND-002), not repeated here.

## New findings (disposition pass 1)

Reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10.

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-003 | medium | The new laziness bullet ("evaluate only the selected branch, the selected arm and the bound value that the body reads") and FR-015-AC-169 ("a ... bound value that the body reads") make `let` lazy. QSpec FR-006 says "Value if evaluates only its selected branch. Let evaluates its initializer once", which reads as a strict initializer. A `let` whose initializer refuses but whose body never reads it would refuse in the authoritative semantics and complete in CG's oracle, so a harness could verify a clause the kernel refuses. Either drop "that the body reads" (strict, evaluated once), or cite the authority that makes an unread initializer unevaluated or unreachable. | spec/kani/functional/FR-015-bounded-kani-obligations.md:446-457 (laziness bullet), AC-169; quire-specification spec/functional/foundation/FR-006-preserve-control-evaluation.md |
| FND-004 | medium | FR-015-AC-168 and FR-015-AC-169 name no concrete fixture: no clause, input domains or refusing draw. FR-015-AC-150 itself records that "No in-domain input of the `Int[0, 1000]` fixture leaves its result domain", so a reachable refusing branch has to be built on purpose. AC-169 also states no expected harness verdict. A selected refusing value stops the oracle, so the healthy harness is `Falsified` at `the V2 postcondition holds for the produced result`. The `false` mutant fails the same assertion, so it is distinguished only by the AC-150 stop classifier. Name each fixture (clause, domain, the draw that refuses) and the expected verdict and step for both the healthy case and the mutant. | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-150, AC-168, AC-169) |
| FND-005 | low | FR-015-AC-170's fixture `let amount = amount + 2 in amount <= 1000` cannot come from QSL source. QSL FR-016 says "Nested shadowing and duplicate active local names refuse", and QSpec binders "cannot shadow ... another visible binding". The nested-rebinding half of the AC also has no fixture. State that the fixture is a constructed admitted package (and confirm Contract IR admission accepts a binder name equal to a parameter's), or reframe it as hygiene of the Rust identifiers CG renders. | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-170); quire-spec-language spec/functional/FR-016-check-native-clauses.md:88 |

## Dispositions

Round 1, reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | AC-168: only the selected branch or arm is evaluated, and the eager-oracle mutant stops with `ClauseStop::IntegerOutOfDomain` and fails `the V2 postcondition holds for the produced result`, the right assertion: v2.rs:1110-1128 gates the agreement assert on completion. AC-169: a selected refusing value stops and is not assumed away. The missing concrete fixtures and the `let` strictness are new findings FND-003 and FND-004. |
| FND-002 | fixed | AC-170 adds a binder-named-like-a-parameter fixture with a distinguishing subject mutant (`amount <= 1000` at 999: healthy false, mutant true). Its QSL reachability is new finding FND-005. |

## New findings (disposition pass 2)

Reviewed head ebd98b5841e41b0e6c138862ea3d63ba2455fdcb. This settles the author's open item on result types. The kernel has no `if`, `case` or `let`: those are rendered as native Rust. An integer refusal arises only at an arithmetic node, against that node's own bound. quire-exact `native_integer.rs` `finish_arithmetic` and `numeric.rs` return `Refusal::IntegerOutOfDomain` when the result is outside the operation's `result_domain`/`bound`, and CG takes that domain from the arithmetic node's own `IntegerRange` bound (src/oracle/scalar/derive.rs `integer_domain`; src/oracle/scalar/lower.rs `Bounds`). The catalog types `control.if`/`control.let` results as `operand:1`, so an `if`'s type is its then-branch's type and is never checked by the kernel.

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | medium | FR-015-AC-168 and FR-015-AC-169 attach the refusing bound to the wrong node. They say "whose `if` node has result type `Int[0, 999]`" or "`Int[0, 1000]`", but the refusal depends on the bound of the else-branch arithmetic node (`amount + 1000`, `amount + 1`). If that node is typed `Integer` or a derived wider interval, nothing refuses: AC-168's eager mutant verifies and AC-169's harness verifies. The fixtures also pull in opposite directions. AC-168, AC-169 and AC-172 need narrow arithmetic bounds, while the healthy fixtures need bounds wide enough for the values they reach: AC-152/156 `amount + 1` and AC-173 `a + 1` reach 1000 to 1001, AC-170 `amount + 2` reaches 1001, AC-158 `amount * 2` reaches 2000. A peer-typed `Int[0, 999]` or `Int[0, 1000]` on those nodes would stop the healthy proofs. QSL's arithmetic result typing is not stated anywhere in this spec. Fix: give each fixture's arithmetic-node bound explicitly (e.g. "`amount + 1000` with result bound `Int[0, 999]`", "`amount * 2` with result bound `Int[0, 2000]`"), as constructed packages where QSL would type them otherwise. AC-172 is already correct, because its let value is the arithmetic node. | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-152, AC-158, AC-168, AC-169, AC-170, AC-172, AC-173); quire-exact src/native_integer.rs:406-421, src/numeric.rs:285; src/oracle/scalar/derive.rs:470-484 |

## Dispositions (round 2)

Round 2, reviewed head ebd98b5841e41b0e6c138862ea3d63ba2455fdcb.

| FND | outcome | reason |
|---|---|---|
| FND-003 | fixed | The laziness bullet now says "shall evaluate each `let` initializer exactly once whether or not the body reads it". FR-015-AC-172 cites QSpec FR-006 and has a fixture (unread `y = amount + 1` refusing at 1000, `Falsified` with `ClauseStop::IntegerOutOfDomain`) and a skip-unread mutant that verifies. AC-169 dropped "that the body reads". |
| FND-004 | fixed | AC-168 has a fixture (`amount: Int[0, 999]`, never-selected else `amount + 1000`), a healthy `Verified`, and an eager mutant `Falsified` at `the V2 postcondition holds for the produced result` at amount 0. AC-169 has a fixture (`amount: Int[0, 1000]`, selected else `amount + 1` at 1000), expects `Falsified` at that step with stop `ClauseStop::IntegerOutOfDomain`, and says the `false` mutant is told apart only by the recorded stop. The node that carries the bound is new finding FND-006. |
| FND-005 | fixed | AC-170 says "QSL refuses shadowing (QSL FR-016), so the fixture is a constructed package admitted through Contract IR". FR-015-AC-173 adds the nested-rebinding fixture `let a = amount + 1 in (let a = a + 1 in a <= 1000)` with the outer-binder mutant at 999 (healthy false, mutant true). |

## Dispositions (round 3)

Round 3, reviewed head e5f05e4eabb226474d2292fb7dfa74fc25634c14.

| FND | outcome | reason |
|---|---|---|
| FND-006 | fixed | Every fixture now gives the bound of each arithmetic node and says that `if`, `let` and `case` nodes are never checked. Checked against the values each node reaches when it is evaluated. Healthy fixtures, not blocked: AC-152 `amount + 1` reaches 1..1000 within `Int[0, 1000]`. AC-153/165 `amount + 1` reaches 1..500 within `Int[0, 1001]` (then-branch only), and `amount - 1` reaches 499..999 within `Int[-1, 999]`. AC-154 `amount + 1` reaches 1..1001 within `Int[0, 1001]`, and `amount * 2` reaches 0..2000 within `Int[0, 2000]`. AC-170 `amount + 2` reaches 2..1001 within `Int[0, 1001]`. AC-173 `amount + 1` reaches 1..1000 within `Int[0, 1000]`, and `a + 1` reaches 2..1001 within `Int[0, 1001]`. Refusing fixtures, each deciding node bounded narrowly: AC-168 `amount + 1000` reaches 1000..1999 against the constructed `Int[0, 999]`, so it refuses on every draw if evaluated, and the healthy run only selects the bare `amount` branch. AC-169 `amount + 1` gives 1001 at amount 1000, outside `Int[0, 1000]`. AC-172's unread initializer `amount + 1` gives 1001 at 1000, outside `Int[0, 1000]`. The other fixtures (AC-155/159 and the subject-side mutants of AC-156 to AC-158) have no oracle arithmetic that a bound could block. |

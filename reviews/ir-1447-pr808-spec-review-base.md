---
id: SR-17940486709564847047743
title: "IR-1447 PR #808 spec-review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #808 at 56f1887d; IR-1447; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1447-kani-control-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1447. Reviewed head 56f1887dc3af80169357d9e6d492e9f295a82f43. The PR adds FR-015
prose (three EARS bullets), FR-015-AC-152 to FR-015-AC-163 (all PLANNED, owner IR-1447), an
AC-42 exception, TC-025 step 57, a TC-025 matrix row and the probe record. Checked against CG
main 340ada62 (src/kani/generate/clause/v2.rs classifier, src/kani/generate/outcome.rs
`UnsupportedObligation`), the checked operation catalog
(quire-verification-contracts contracts/checked-operation-catalog-v1.json: `control.let`
operands `[binder, any_value]`; `control.if` `[boolean, any_value, any_value]` with
`same_family`; `control.case` operands `[union]`, rest `binder`, result `arm_body`,
constraint `union_arms`), QSpec FR-440/FR-322 and QSL FR-093 lowering, and the probe
directory. `quire validate` over spec/plan/reviews: the 6 pre-existing warnings only.
`quire matrix`: AC-152..163 resolve, untagged (expected for PLANNED). No AC id collision with
open PRs.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-015-AC-157's mutant control cannot fail as written. The clause is `result = ((if amount < 500 then amount + 1 else amount - 1) >= 1)`: the subject returns the Boolean, and the mutant test `amount <= 500` changes the value only at 500 (501 instead of 499), where both are `>= 1`, so the mutant is equivalent at the Boolean result. The named step `the selected branch agrees with the independent expectation` is oracle-internal; a subject mutant never changes the oracle, so that step cannot fail either. The probe compared the integer selected value (`s_if` returns `i64`), which is why it failed there. Fix: either a distinguishing comparison (e.g. `>= 500`: healthy false at 500, mutant true) with the produced-result step, or place the mutant in the generated oracle's `if` (AC-41 style construct mutant) and keep the oracle-internal step. | spec/kani/functional/FR-015-bounded-kani-obligations.md:981,985; scratchpad/probe-1447/src/lib.rs (s_if, prove_if) |
| FND-002 | high | FR-015-AC-154 gives no clause: no comparison, result type or clause kind around the `case`, and AC-158 names a subject mutant (`B -> amount * 3`, input `kind = B`, `amount = 1`) failing at the oracle-internal step `the selected arm agrees with the independent expectation`, which a subject mutant cannot reach. Whether the mutant is distinguishing depends on the missing comparison (with e.g. `>= 0` it is equivalent). The control is unverifiable as written. | spec/kani/functional/FR-015-bounded-kani-obligations.md:982,986 |
| FND-003 | high | FR-015-AC-159's mutant site is incoherent for the collection claim. AC-155 does not say which clause kind (invariant/function-body direct harness, or pre/postcondition) the `forall` is; a direct or precondition collection harness has no produced subject result, and a postcondition subject produces the collection, not the element body. "The mutant subject whose else branch yields 501" therefore has no site, and the named step `the per-element selected value agrees with the independent expectation` is oracle-internal. State the claim kind and put the mutant in the generated oracle's element `if` (or name a step a subject mutant reaches). | spec/kani/functional/FR-015-bounded-kani-obligations.md:983,987 |
| FND-004 | medium | FR-015-AC-154 depends on a union-typed input CG cannot draw or pass today and the spec does not specify: no FR-015-AC-39 domain rule for a union parameter, no subject ABI for it, and no statement of whether the oracle matches a native index or a kernel `Value::Union`. CG `src/kani` and `src/oracle` have no union support. The `FR-049-AC-2` ref is the `enum`-family index draw (`0..member_count`), a different catalog family from `union`. The probe used a native `Copy` Rust enum, so it does not bound the cost of a kernel `Value::Union` arm (the IR-1022 fat-enum cost driver). State the draw (one index per member, every member reachable), the ABI and the oracle carrier. | spec/kani/functional/FR-015-bounded-kani-obligations.md:982; spec/kani/functional/FR-049-enum-scalar-harnesses.md:138 |
| FND-005 | medium | The PR conflicts with main (340ada62, PR #806 IR-1448) and needs a rebase. `git merge-tree` reports one conflict: spec/kani/functional/FR-015-bounded-kani-obligations.md, the FR-015-AC-42 row (main line 863, PR head line 875): main appended "Amended by IR-1448: `quire.op.option.present` and `quire.op.option.value` are lowered or refused by FR-054, and every other `quire.op.option.*` operator is refused as `OptionNotLowered`, not here." at the end of the row, and the PR rewrote the parenthetical in the same row. spec/kani/matrix/tests.md auto-merges (main's FR-054 row is in the top table, the PR's row is at the end of the TC table). Resolution: keep the IR-1448 sentence verbatim (FR-054-AC-23 inspects it) and add the control exception the same way, as an "Amended by IR-1447: ..." sentence after it, rather than inside the operator-list parenthetical. | spec/kani/functional/FR-015-bounded-kani-obligations.md:875 (head), 863 (main) |
| FND-006 | low | spec/kani/matrix/tests.md gets a TC-table row for AC-152..163 but no row in the top requirement-status table (`FR-015 / FR-015-AC-152 through FR-015-AC-163 / TC-025 / PLANNED (IR-1447)`), unlike FR-015-AC-151 and main's FR-054 row. | spec/kani/matrix/tests.md:19-58, 149 |

## Verdict

Not mergeable. The lowering scope and the refusal family are the right shape, the AC-42
exception is consistent with the classifier, all rows are PLANNED with owner IR-1447 and no
new validation warning is added. But three of the four installed-Kani controls (if, case,
collection) name subject mutants that cannot fail at the named oracle-internal steps (the if
mutant is also Boolean-equivalent), the case fixture lacks its clause and its union input
contract, and the branch conflicts with main on the AC-42 row. The let control (AC-156) is
sound: `amount + 2 <= 1000` differs only at 999 and fails the real produced-result step.

## New findings (disposition pass 1)

Reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10 (rebased onto main 7fb4f53c, the merge base).

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-007 | medium | FR-015-AC-154 now passes the arity-0 union parameter to the subject as one `i64` member index, but FR-025 owns the subject ABI and its family table has no union row: it has rows for Boolean, bounded integer, Rational, Decimal, IEEE float, Text, the `enum` operand and carried quantity. Its rule "If an argument's family has no row in that table, then the generator shall account the obligation `unsupported` with a typed reason naming the family" therefore refuses the very parameter AC-154 lowers. Add a planned row to FR-025, e.g. "Arity-0 union operand (planned FR-015-AC-154): one `i64` call in `0..member_count`, the member at that declared position". | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-154); spec/kani/functional/FR-025-generated-subject-abi.md:68-81 |
| FND-008 | low | The new TC-025 row in spec/kani/matrix/tests.md lists its criteria as the range `FR-015-AC-152..FR-015-AC-170`. Every other TC row in that table enumerates its ids. Enumerate them, or use the top table's "through" wording. | spec/kani/matrix/tests.md:151 |

## Dispositions

Round 1, reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | AC-153 is now `(if amount < 500 then amount + 1 else amount - 1) >= 500`, and AC-157's mutant test `amount <= 500` at `amount = 500` gives healthy false (499) and mutant true (501), failing at the real produced-result step (src/kani/generate/clause/v2.rs:1114). |
| FND-002 | fixed | AC-154 gives the clause `(case kind of A -> amount + 1, B -> amount * 2, C -> 0) >= 1000`. AC-158's arm `B` as `amount * 3` at member 1 and amount 400 gives healthy 800 (false) and mutant 1200 (true), at the produced-result step. |
| FND-003 | fixed | AC-155 is an invariant on the FR-015-AC-99 direct harness. AC-159 puts the mutant in the generated oracle's element `if` (else branch 501) and fails the body assertion at element 500. The probe's v2 collection mutant failed likewise. |
| FND-004 | fixed | AC-154 now states the draw (one `i64` index in `0..member_count`, every member reachable), the subject ABI (the index as `i64`), the oracle carrier (a `match` on the index in declaration order, no wildcard, no `Value::Union`). That matches Contract IR `union_arms`, which requires one arm per member in declaration order. The FR-025 table gap is new finding FND-007. |
| FND-005 | fixed | Rebased: the merge base is main 7fb4f53c and no conflict remains. The AC-42 row keeps the IR-1448 sentence verbatim and appends "Amended by IR-1447: ... never reported as `V2ClauseOperatorNotSupported`." |
| FND-006 | fixed | The top table has the row `FR-015 / FR-015-AC-152 through FR-015-AC-170 / TC-025 / PLANNED (IR-1447)`. |

## New findings (disposition pass 2)

Reviewed head ebd98b5841e41b0e6c138862ea3d63ba2455fdcb (merge base main 7fb4f53c).

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-009 | low | FR-025-AC-19's verification column is `Test (TC-036)`, but TC-036 has no step for it and the TC-036 row of spec/kani/matrix/tests.md does not list it. FR-025-AC-15 to AC-18 each have a TC-036 step and appear in that row. FR-025-AC-19 is listed only in the TC-025 row and TC-025 step 57. Either add a TC-036 step and row entry, or change its verification to TC-025. | spec/kani/functional/FR-025-generated-subject-abi.md:194; spec/kani/matrix/tests.md:142, 151; spec/kani/matrix/TC-036-generated-subject-abi.md |

## Dispositions (round 2)

Round 2, reviewed head ebd98b5841e41b0e6c138862ea3d63ba2455fdcb.

| FND | outcome | reason |
|---|---|---|
| FND-007 | still-open | FR-025-AC-19 was added, but the FR-025 family table that the bullet "The generator shall bind each argument by the Rust type its family's row states" governs still has no union row (Boolean, bounded integer, Rational, Decimal, IEEE float, Text, enum operand, carried quantity). The next bullet, "If an argument's family has no row in that table, then the generator shall account the obligation `unsupported`", therefore still refuses the arity-0 union parameter that AC-154 and FR-025-AC-19 bind. FR-025-AC-15 to AC-18 each have a matching table row. Add "Arity-0 union operand (planned FR-015-AC-154): one `i64`, assumed in `0..member_count` / the member at that declared position". |
| FND-008 | fixed | The TC-025 row now enumerates FR-015-AC-152 to FR-015-AC-173 and FR-025-AC-19. |

## Dispositions (round 3)

Round 3, reviewed head e5f05e4eabb226474d2292fb7dfa74fc25634c14 (merge base main 7fb4f53c; `quire validate` 6 pre-existing warnings; `quire matrix` resolves FR-015-AC-152 to AC-173 and FR-025-AC-19).

| FND | outcome | reason |
|---|---|---|
| FND-007 | fixed | The FR-025 family table now has the row "Arity-0 union operand (planned FR-015-AC-154 `case` scrutinee row only) / one `i64` call, assumed in `0..member_count` of the union declaration / the member at that declared position". FR-025-AC-7's no-row list adds "union outside the planned FR-015-AC-154 row". The no-row refusal no longer contradicts AC-154 or FR-025-AC-19. |
| FND-009 | fixed | TC-036 step 19 covers FR-025-AC-19 (grouped `i64` binding, member at the declared position, typed refusal for an out-of-range, missing, duplicate or foreign binding). tests.md has a top-table row `FR-025 / FR-025-AC-19 / TC-036` and adds AC-19 to the TC-036 row. TC-025 step 57 now says "FR-025-AC-19 is TC-036 step 19". The TC-025 row still lists AC-19, which is defensible because step 57 exercises the union binding. |

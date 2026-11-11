---
id: SR-17940486709564847047678
title: "IR-1502 PR #786 FR-014 IEEE sqrt and fma spec criterion-strength review"
type: SpecReview
analysis: criterion-strength
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #786 @ 06dcdaf1; IR-1502; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
---

## Summary

Ticket: IR-1502. No Jev client is available in this environment, so criterion strength was assessed by hand: each mutation control was traced to a distinguishing input, a failing step and a mutant site. Controls (a) through (d) were recomputed with exact rationals in Python:
- (a) Removing the sqrt/fma arms from `family_of_identity` brings back `OperationNotDerivable`, so step 1 fails.
- (b) For multiply-then-add, the rounded product 0x3F800002 plus 0xBF800002 is 0, against the expected 0x28800000.
- (c) Swapping the second and third fma arguments gives fma(2, 1, 3) = 5, not 7.
- (d) Ignoring the rounding gives 0x3FB504F3 where 0x3FB504F4 is expected under toward-positive at binary32.

Swapping the first and second arguments is not detected, but fma(3, 2, 1) = 7 is harmless because multiplication is commutative.

## Verdict

FAIL ON MEDIUMS. Controls (a) through (d) can fail at binary32, and each names a mutant site and an input that tells the right result from the wrong one. Control (e) has neither. Two smaller problems: one step asserts something no test can observe, and one AC clause asserts code structure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Control (e), 'accept an operand of the other width: the step 4 width control fails', names no mutant site (derivation, generation shape, or the rendered `expect_width` prelude) and has no admitted input that tells right from wrong: the strict reader refuses a cross-width sqrt/fma before CG. The generated `expect_width` stop (FR-014-AC-9) is the one reachable width boundary. Name the site, for example dropping `expect_width` in the rendered oracle, and the distinguishing call: a binary32 `IeeeValue` passed to the binary64 sqrt oracle must stop before any charge. | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:560-565 |
| FND-002 | low | Step 1 says to 'assert ... that before this change the same fixtures were `OperationNotDerivable` (record that as the failing step, then require it to pass)'. A test on the changed code cannot observe the old behavior. This is a red-then-green procedure note, not an assertion, and control (a) already covers it. Reword it as a recorded red run, or drop it. | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:542-546 |
| FND-003 | low | AC-72's last sentence, 'The exhaustive identity matches of derivation and of the descriptor check shall cover all four identities and the round-trip test shall walk them', asserts code structure and the content of a test. Its verification method is Test, but no test result can show it. Rewrite it as observable behavior: every one of the four identities round-trips identity to descriptor to identity. Or mark it Inspection. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| TC-024 IR-1502 step 3 | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:560-565 | Mutation controls, each of which must fail the named step: (a) remove the `sqrt`/`fma` arms from derivation: step 1 fails; (b) render `fma` as a multiply then an add: the `0x28800000` vector in step 2 fails; (c) swap the second and third `fma` arguments: the `7.0` vector fails; (d) ignore the descriptor rounding: the `0x3FB504F4` vector fails; (e) accept an operand of the other width: the step 4 width control fails. |
| TC-024 IR-1502 step 1 | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:542-546 | Assert each item is derived, `ir_confirmed`, and that before this change the same fixtures were `OperationNotDerivable` (record that as the failing step, then require it to pass). |
| FR-014-AC-72 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725 | ... The exhaustive identity matches of derivation and of the descriptor check shall cover all four identities and the round-trip test shall walk them. |
| FR-014-AC-73 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:726 | The generated oracle for each of the four identities of FR-014-AC-72 shall call `evaluate_ieee` with `IeeeOperation::SquareRoot(a)` or `IeeeOperation::FusedMultiplyAdd(a, b, c)` in argument order ... |
| FR-014-AC-74 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 | If a `sqrt` or `fma` node of FR-014-AC-72 has the wrong operand count ... |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | AC-77 says the other-width call 'stops as the owner's width refusal (FR-014-AC-9)'. The stop actually comes from CG's rendered `expect_width` prelude, `OracleStop::OperandWidth` (render.rs:70-76). The owner has no width refusal for unary `sqrt`: without the prelude it computes at the operand's own width. For a mixed-width `fma` the owner returns `IllTyped(DistinctIeeeWidths)`. A test that asserts the owner's refusal would fail a correct implementation. Name the generated oracle's `OperandWidth` stop instead. The 'before any charge, counters unchanged' clause and control (e) are correct, and (e) fails through the `sqrt` call. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:734 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified at be0daf1b |
| FND-002 | fixed | verified at be0daf1b |
| FND-003 | fixed | verified at be0daf1b |
| FND-004 | still-open | New this round: AC-77 attributes the width stop to the owner; it is CG's rendered OperandWidth stop. |

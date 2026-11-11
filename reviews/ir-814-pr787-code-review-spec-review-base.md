---
id: SR-787003
title: "IR-814 PR #787 spec-review/base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #787 at 7ba3df5b; IR-814; spec/replay/functional/FR-033-composite-parity-replay-binding.md; spec/replay/matrix/tests.md; spec/replay/matrix/TC-048-composite-parity-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
---

## Summary

Ticket: IR-814. The spec edits flip FR-033-AC-14 and AC-15 from PLANNED to DELIVERED and split the
tests.md row. AC-14/15 become Covered and AC-16/17 stay Planned. `make spec` passes, and `quire matrix`
shows AC-14/15 tagged. The flips are justified. The PR leaves stale "still planned" text in TC-048 and
the TC-048 summary row. Its new AC-16/17 row says no authentic fixture reaches the inline branch,
which contradicts TC-048 step 1 ("Only the non-integer case is testable") and IR's own admitted
`none`-literal fixture (SR-787002 FND-001).

Required spec edit, if and only if AC-17 is shown unreachable after trying the `none`-literal route:
(1) FR-033-AC-17 row: status `DELIVERED (IR-814; verified by Inspection ...)`, Verification column
`Test` -> `Inspection`, replace "No test of this path exists yet." with the inspected locus (the
single `.map_err(|cause| CompositeBuildError::Operands(Box::new(cause)))` in `selected_claim`) and
the owning IR projection test. (2) FR-033-AC-16 row: drop "the single conversion that AC-17 tests"
and "which AC-17 exercises"; AC-16's Inspection method is otherwise already correct. (3) TC-048
step 1: replace "Only the non-integer case is testable; ..." with both cases being verified by
Inspection. (4) tests.md AC-16/AC-17 row: Inspection status, not Planned. If AC-17 gets a tagged
test instead, only AC-17's status and the tests.md row change, and AC-16 stays as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-048 and its summary row still say AC-14/AC-15 (Ne literal/literal, Ne self-comparison) remain planned under IR-814, contradicting the DELIVERED flips | spec/replay/matrix/TC-048-composite-parity-replay-binding.md:54, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:239, spec/replay/matrix/tests.md:52 |
| FND-002 | medium | New AC-16/AC-17 matrix row asserts no authentic fixture reaches the inline branch, contradicting TC-048:60 ("Only the non-integer case is testable") and IR's admitted none-literal composite fixture; AC-17 is left a Test AC with neither a test nor the Inspection treatment | spec/replay/matrix/tests.md:35, spec/replay/functional/FR-033-composite-parity-replay-binding.md:374-375 |

## Verdict

**Changes requested.** The AC-14/15 flips are sound. Fix the stale planned text, and resolve AC-17
one way or the other: preferably with a tagged test, otherwise with the four-part Inspection edit
above. Do not leave a Test AC that the PR claims cannot be tested.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-033-AC-14 | examined | DELIVERED (IR-814; ...). When the original application node is a structural.ne whose two operands are both composite literals ... empty Bounds and no parameter position. |
| FR-033-AC-15 | examined | DELIVERED (IR-814). When the original application node is a structural.ne comparing one parameter with itself ... neither deduplicated. |
| FR-033-AC-16 | examined | PLANNED (IR-814; verified by Inspection of the single conversion that AC-17 tests). ... Both inline refusals pass through the same CompositeBuildError::Operands conversion, which AC-17 exercises. |
| FR-033-AC-17 | examined | PLANNED (IR-814). If the owning operand projection refuses an inline non-integer literal operand (InlineNonInteger), then the builder returns that refusal through CompositeBuildError::Operands ... No test of this path exists yet. |
| tests.md FR-033 AC-14/AC-15 row | examined | Covered (IR-814); original_ne_literal_literal_keeps_two_empty_bounds_children and original_ne_self_comparison_keeps_two_positional_arguments |
| tests.md FR-033 AC-16/AC-17 row | examined | Planned (IR-814); ... IR's reader refuses an inline operand of structural.eq/structural.ne at admission, so no authentic fixture reaches the operand projection's inline branch |
| TC-048 | context_only | Only the non-integer case is testable; the integer case (AC-16) is verified by Inspection of the shared Operands conversion. |

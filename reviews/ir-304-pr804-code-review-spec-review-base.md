---
id: SR-17940486709564847047731
title: "IR-304 PR #804 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #804 at b168f9b3; IR-304 slice (b); spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/tests.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/replay/matrix/tests.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, docs/qualification/ir304-kani-guard-cover-attempt.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. The spec edits are status and retirement edits: FR-022-AC-35 marked PLANNED with
the Kani attempt, AC-36/37 drop their slice (a) wording, AC-38 retired (struck through,
Inspection), `GuardLoweringNotImplemented` removed from the reason list, precedence and
rationale, FR-015 drops its AC-38 cross-reference, FR-032-AC-19 and TC-047 step 9 become
covered. No new requirement statement or AC was added, so the EARS and integrity sub-analyses
were not run as separate methods; their structural checks are folded in here. `quire validate`
on the four edited spec files: no new warning.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | Stale slice (a) text remains in TC-033: the AC-36 procedure still says "reaches FR-022-AC-38 (slice a) or FR-022-AC-27 (slice b)", a whole "For FR-022-AC-38 (slice a only)" procedure paragraph remains, and the routed `tests.md` TC-033 row still lists FR-022-AC-38 and says "IR-304 slice (b) cases (AC-27, AC-31 to AC-35) planned" | spec/routed/matrix/TC-033-routed-generation.md:96, 108-110, spec/routed/matrix/tests.md:66 |
| FND-002 | low | FR-022 is silent on two behaviours the code has: a guard parameter whose declared domain exceeds `i64` settles `domain_not_representable_in_i64` (precedence step 4 names only "an operand bound or literal"), and a direct FR-015 caller's guard whose node is not admitted at all settles `GuardRootNotLowered` (the reason is defined only as a non-`lowered` accounting record) | spec/routed/functional/FR-022-routed-generation.md:293-295, 327-331 |
| FND-003 | low | The FR-032-AC-19 row lost its status prefix; every sibling row starts `IMPLEMENTED (IR-631)` / `PARTIAL` / `PLANNED`, so it should read `IMPLEMENTED (IR-304 slice (b))` | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:413 |

## Verdict

The removal of `GuardLoweringNotImplemented` and retirement of FR-022-AC-38 are complete in
FR-022 and FR-015 (no remaining reference in `src`, `tests` or FR text); the boxed
`PathConditionUnsatisfiable` fields keep the documented JSON shape. The FR-022 per-item
precedence list matches the code except for the step 2/3 inversion recorded in the code-review
SR (FND-002 there). The narrowed-harness replay outcome (code-review SR FND-001) is a spec gap as
well as a code one. Three low findings; nothing here blocks alone.

## Dispositions

Round 1, reviewed head f00e67a6.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | TC-033 AC-36 procedure now reads "reaches the guard handling of FR-022-AC-27", the slice (a) AC-38 paragraph is removed, and the `tests.md` TC-033 row no longer lists AC-38 and states AC-35 planned (IR-304) |
| FND-002 | fixed | FR-022 precedence step 4 now names a guard parameter's declared domain beyond `i64`, and `GuardRootNotLowered` covers a direct caller's unadmitted node |
| FND-003 | fixed | the FR-032-AC-19 row starts `IMPLEMENTED (IR-304 slice (b)).` |

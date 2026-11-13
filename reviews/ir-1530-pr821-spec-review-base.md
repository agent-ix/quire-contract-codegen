---
id: SR-17940486709564847047957
title: "IR-1530 PR #821 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #821 at c3a58c73, git diff origin/main...HEAD; IR-1530; spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md, spec/kani/functional/FR-017-kani-execution-evidence.md, docs/qualification/ir1530-general-route-scalar-not-tractable.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

## Summary

Ticket: IR-1530. Base spec-review of the edited requirement text (no new FR, no relationship
edges changed, so no dependency, object or integrity sub-analysis applies). `quire validate` on
the three spec files reports no errors; its grammar warnings are on pre-existing lines
(FR-056 line 112 is AC-11; FR-017 lines 396 and 915), not on the edits.

Examined: FR-056 Terms (`route` sentence), FR-056-AC-14, FR-056 Status, TC-070 step 14, TC-070
Expected Results and Status, FR-017-AC-27, and the #816 qualification doc's refusal section. Trace
tags on both new tests are `#[trace("TC-070", "FR-056-AC-14")]` and agree with TC-070 step 14. The
qualification doc's new section matches the code and correctly keeps the generation-level
blast-radius evidence as the rejected alternative.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-056-AC-14 exceeds its parent statement. FR-056's title and Description scope the requirement to V2 clauses ("Refuse a V2 clause ...", "The decider runs AFTER the operator check of FR-015-AC-42: it sees only operations the V2 clause path accepts"), the Terms place the table in "the generation request's options", and the Behavior currency rule still reads "A row is current when its generator and kernel version equal CG's". AC-14 adds a run-level decision on routed exact-scalar harnesses, a table passed to run wrappers rather than generation options, and route equality as a currency condition, none of which the statement or Behavior says. Amend the Description and Behavior (route as part of currency; the run-level general-scalar entry point; the deliberate reuse of the `V2ClauseNotTractable` arm for a non-V2 harness) so the AC traces to the statement. | spec/kani/functional/FR-056-kani-tractability-refusal.md:1-40, spec/kani/functional/FR-056-kani-tractability-refusal.md:58-59, spec/kani/functional/FR-056-kani-tractability-refusal.md:96, spec/kani/functional/FR-056-kani-tractability-refusal.md:118 |
| FND-002 | low | TC-070 Status attributes step 14 to IR-1526: "Steps 1 to 9, 11, 12 and 14 are implemented and tagged (IR-1526)". Step 14 is IR-1530's. | spec/kani/matrix/TC-070-kani-tractability-refusal.md:71 |
| FND-003 | low | FR-017-AC-27 now embeds a second, independently verified claim (a general-route harness through `execute_kani_obligation_decided` is refused, FR-056-AC-14) inside a criterion verified by `Test (TC-027)` about the native-route paired runner, making it compound. A cross-reference ("see FR-056-AC-14") is enough; the refusal assertion belongs only to FR-056-AC-14. | spec/kani/functional/FR-017-kani-execution-evidence.md:879 |

## Verdict

The spec edits are accurate to the code. FND-001 is a statement/AC scope mismatch worth fixing in
this PR (text only); FND-002 and FND-003 are wording nits. None changes behaviour.

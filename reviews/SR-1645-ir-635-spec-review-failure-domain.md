---
id: SR-1645
title: "IR-635 spec review (failure domain): unmapped bounded_shadow outcome combinations"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen#298; spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
---

# SR-1645: IR-635 spec review, failure domain

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. I enumerated the outcome × strength × route combinations a `bounded_shadow` composite item can produce and checked that each has exactly one defined result. Two medium findings.

## Method

I took FR-028-AC-17's strengths and the conditions under which each is assigned, including `refinement_failed` "whatever the shadow harness settled". I crossed them with FR-029's backend outcomes: verified with n ≥ 1, inconclusive (vacuous, timed out, memory, unwind, no verdict, missing summary) and falsified. I then traced each pair through FR-029's priority table, the falsified parity mapping and the interim refusals. I also checked the FR-033 setup refusal precedence for lost or duplicated paths.

## Verdict

**PASS with two medium findings.** The following were clean:
- Disagreement takes priority over zero checks and ceiling.
- The ceiling is never `Tested`.
- `Proved0` keeps its inconclusive record category.
- Another claim's result yields no settlement.
- A missing upstream capability is a distinct refusal that comes after earlier malformed-input refusals.
- Iterative lifecycle and byte, count and work bounds have no depth cap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Removed: "an inconclusive outcome of a bounded_shadow harness maps as every other inconclusive outcome does" (old Behavior and old AC-17). New AC-17 does not restate it, and priority row 3 reads "other verified evidence, SUCCESS count zero". FR-029's first table makes `verified` imply n ≥ 1 and classifies zero SUCCESS as `inconclusive` (vacuous-proof). Two readings follow: row 3 is unreachable, or a vacuous shadow run now enters the parity route as "verified". Also, nothing now states how a shadow run's no-verdict, missing-summary or unwind-bound inconclusive outcome maps. Line 174 covers only timeout and memory. Restore the explicit rule that a bounded_shadow inconclusive outcome takes the ordinary inconclusive rows, and say how a vacuous shadow reaches row 3 or drop row 3. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:169, spec/kani/functional/FR-029-run-outcome-terminal-record.md:174, spec/kani/functional/FR-029-run-outcome-terminal-record.md:265 |
| FND-002 | medium | FR-028-AC-17 assigns `refinement_failed` "whatever the shadow harness settled", so a falsified shadow harness can carry a refinement disagreement. FR-029 and FR-033 apply the strength projection, including `Disagreed` → `Failed`/`CgDefect`, only to verified evidence. The falsified mapping reads only the QSL replay comparison. Take a falsified shadow whose replayed case agrees but whose refinement run disagreed elsewhere: it settles parity-agreement `Inconclusive` and drops a known CG defect. Nothing says which result wins. State that a `refinement_failed` strength yields `Failed`/`CgDefect` whatever the backend outcome, or define the precedence explicitly. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:181, spec/replay/functional/FR-033-composite-parity-replay-binding.md:124 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The fix gates FR-029-AC-24, which makes a retained `refinement_failed` yield `Failed`/`CgDefect` whatever the backend outcome, on QSL-640. FR-029-AC-25 applies the ordinary inconclusive rows only "with no retained refinement disagreement". AC-21 gives the interim refusals only for a verified nonproduction shadow and a falsified shadow. So before QSL-640 delivery, a `bounded_shadow` inconclusive or cover-unsatisfied outcome that carries `refinement_failed` matches no stated row. State its interim reading, for example the ordinary inconclusive row or a typed interim refusal, so the gated period has a total map. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:296, spec/kani/functional/FR-029-run-outcome-terminal-record.md:299, spec/kani/functional/FR-029-run-outcome-terminal-record.md:300 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #298, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | FR-029 restores the rule: "In the absence of a retained refinement disagreement, every `bounded_shadow` inconclusive backend outcome takes the ordinary inconclusive rows". It lists vacuous and cover-unsatisfied, timed-out, memory and unwind, and no-verdict and missing-output, as AC-25 asserts. Priority row 3 now reads "vacuous-proof `inconclusive` or `cover-unsatisfied`, zero SUCCESS checks" and states it does not coerce a run into verified evidence. AC-26 says zero-count cases use AC-25, "not an unreachable verified-count-zero row". |
| FND-002 | fixed | FR-029 now applies refinement disagreement "before any backend outcome or replay agreement ... including a falsified replay that agrees on its replayed case". Priority row 1 is "any outcome with `refinement_failed` / `Disagreed`". FR-029-AC-24 asserts `Failed`/`CgDefect` for verified, falsified, inconclusive and cover-unsatisfied outcomes, including zero checks and agreeing replay. FR-033's falsified bullet and the FR-029 Inputs section carry the same rule. |

---
id: SR-11731
title: IR-807 integrity review of the FR-029 terminal-pair wording cleanup
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@f9ef0f5c246793ee5b0f48f8c52b9e083f577448 (PR 398, diff vs origin/main 2897d5e); spec/kani/functional/FR-029-run-outcome-terminal-record.md,
  spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/tests.md
review_set: subset
---

## Summary

Ticket: IR-807. Consistency check: does the retired claim (CG owns interim `NonProductionProof` /
`ShadowCounterexample` refusals) survive anywhere in `spec/` or `plan/` outside historical review
files, and does the rewritten text read consistently? Searched with
`grep -rnE 'NonProductionProof|ShadowCounterexample|ProofStrength'` and an `interim` + shadow /
strength / FR-029 search over spec/ and plan/, excluding reviews.

## Verdict

**CONDITIONAL**: one residual of the retired claim in FR-028, and one dangling sentence in FR-029.

The variant names no longer appear anywhere in spec/ or plan/. The other `interim` hits
(FR-015:608, FR-028:364-371, FR-031, AD-004) are unrelated interim refusals or strengths, not
this claim. The interface-001, tests.md, TC-040 and TC-048 edits agree with the new FR-029 Outputs
text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-028's Open Questions still say "until actual API delivery the interim shadow refusals still yield no terminal value". The PR's own FR-029 text says those refusals do not exist, and QSL-640 delivered the API (FR-029:352). This is the retired claim, left in a file the PR edits. Reword it to match FR-029, or delete it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:312-314 |
| FND-002 | low | In the FR-029 Behavior paragraph, "Outcomes without retained disagreement still take the ordinary inconclusive rows" lost its antecedent. It used to follow the bounded-shadow inconclusive/cover-unsatisfied case. It now follows "The admitted route takes the rows below", so it can be read as overriding the parity rows for every outcome. State which outcomes it means (bounded-shadow inconclusive or cover-unsatisfied without retained refinement_failed). | spec/kani/functional/FR-029-run-outcome-terminal-record.md:159-164 |

## Examined units

```yaml
scope:
- id: FR-028
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: "Delivery of QSL composite parity settlement. FR-029 AC-17 specifies the planned QSL-640-gated parity settlement; until actual API delivery the interim shadow refusals still yield no terminal value."
- id: FR-029
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "Until the production CG path invokes the delivered QSL parity facade, `run_terminal_value` receives no bounded-shadow evidence: it has no proof-subject or proof-strength input and `TerminalPairError` has no variant for it. The admitted route takes the rows below. Outcomes without retained disagreement still take the ordinary inconclusive rows."
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "The function takes no proof-subject or proof-strength input and TerminalPairError has no bounded-shadow variant; IR-635 owns those inputs and the parity route."
- id: tests.md
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: "Planned (IR-635); no traced terminal-map strength/outcome integration yet. ... IR-635 owns the proof-subject/strength inputs, original-artifact invocation, ordinary shadow-inconclusive rows and actual record category"
bindings: []
```

## New findings (disposition pass 1)

Reviewed ec2e2f4c116bd393d2fd2e63351a2a712ee4109e (diff f9ef0f5..ec2e2f4).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Four places scope the shadow exclusion with "until the production path invokes the delivered parity API": FR-028:312-314 ("until ..., no bounded-shadow outcome is passed to `run_terminal_value` (FR-029-AC-30)"), FR-028:376-378, the FR-029 Behavior sentence at :159-160 and TC-048 step 8. AC-30 says a bounded-shadow outcome "never" reaches `run_terminal_value`, and AC-21 has no time limit. The "until" wording implies that after IR-806 the exclusion lapses. Drop "until ..." or say "on every path, including after IR-806". | spec/kani/functional/FR-028-bounded-proof-ceilings.md:312-314,376-378, spec/kani/functional/FR-029-run-outcome-terminal-record.md:159-160, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:116-118 |
| FND-004 | low | TC-048:230, a line this PR edits, still says "Ne awaits IR-690". IR-690 is Done, CG #394 (76dca65) added structural.ne to the original builder, and FR-033-AC-1 and AC-11 credit IR-635 with the Ne parameter/parameter and parameter/literal controls. The replay tests.md:39 status ("Ne ... remain planned") has the same stale claim. This text predates the PR, but it sits on an edited line. | spec/replay/matrix/TC-048-composite-parity-replay-binding.md:230, spec/replay/matrix/tests.md:39 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |

## Disposition round 1 verdict

**PASS with two new low findings.** FND-001: FR-028:312-314 no longer mentions interim refusals. FND-002: the dangling "Outcomes without retained disagreement" sentence is removed from the FR-029 Behavior paragraph. `grep -rnE 'NonProductionProof|ShadowCounterexample|ProofStrength' spec plan` (excluding reviews) is still empty.

## Disposition round 2

Reviewed e6ef3b24fe965efa5da2d471742f5b43fb9c8dd0.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | - |
| FND-004 | still-open | TC-048:229 and replay tests.md:22 are fixed. replay tests.md:39 is the TC-048 summary row, which this PR edited in ec2e2f4 to add AC-30/31. It still says "The Eq constructor covers ...; Ne, native/generated-family and proof-strength integration remain planned". That contradicts FR-033-AC-1 and AC-11, which credit IR-635 with Ne parameter/parameter and parameter/literal controls (CG #394). Change "Ne" to "Ne literal/literal". |

## Disposition round 2 verdict

**CONDITIONAL**. FND-003: no shadow-exclusion statement is scoped with "until" any more. FR-028:312-314 and 376-378, FR-029:159-163 and TC-048 step 8 now state the exclusion as permanent. FND-004 is still open, as a one-word fix at replay tests.md:39.

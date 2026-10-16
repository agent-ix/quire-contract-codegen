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

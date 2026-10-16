---
id: SR-11732
title: IR-807 EARS conformance review of FR-029-AC-21 and the rewritten FR-029 statements
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen@f9ef0f5c246793ee5b0f48f8c52b9e083f577448 (PR 398, diff vs origin/main 2897d5e); spec/kani/functional/FR-029-run-outcome-terminal-record.md
review_set: subset
---

## Summary

Ticket: IR-807. EARS check of the requirement text this PR rewrites in FR-029: the Inputs and
Outputs bullets, the Behavior parity paragraph and AC-21. The FR's own requirement statement is
unchanged.

## Verdict

**PASS with one low finding.** The rewritten bullets are descriptive Inputs/Outputs prose, not
requirement statements. The missing "shall" in the Behavior paragraph is reported as SR-11730
FND-002, not again here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-21 is compound. It packs four claims into one criterion: the signature has no input, the enum has no variant, no CG function maps a shadow outcome, and none fabricates a cause. It then adds a fifth sentence that points to AC-17 and AC-19 to AC-27. A test can pass the first two and say nothing about the rest. Keep one checkable condition (the ordinary map has no bounded-shadow input or variant) and move the pointer sentence to prose. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:334 |

## Examined units

```yaml
scope:
- id: FR-029-AC-21
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "PLANNED (IR-635): `run_terminal_value` has no proof-subject or proof-strength input and `TerminalPairError` has no variant for a bounded-shadow outcome, so until the production CG path invokes the delivered QSL parity facade no CG function maps a verified, falsified, inconclusive or cover-unsatisfied bounded-shadow outcome ... to a terminal value or a typed refusal, and none fabricates a predicate replay or terminal cause. The terminal value for such an outcome is the parity route's under AC-17 and AC-19 to AC-27."
- id: FR-029
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "Planned (IR-635): for the composite parity route, the proof subject (`production` or `bounded_shadow`, FR-028), the verified outcome's proof strength (FR-028-AC-17) and retained `refinement_failed` disagreement whatever the backend outcome. `run_terminal_value` takes none of these today"
bindings: []
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |

## Disposition round 1 verdict

**PASS**. Reviewed ec2e2f4. AC-21 is split into three criteria:
- AC-21: the map has no input or variant for a bounded-shadow outcome.
- AC-30: the production path routes a bounded_shadow outcome only to the parity route, with the reason given.
- AC-31: the parity route fabricates no replay or cause.

Each makes one checkable claim, apart from AC-21's trailing "deliberate, tested change" clause, which is harmless. No new EARS findings. The new Behavior sentence "The production path shall not pass a `bounded_shadow` outcome to `run_terminal_value`" is a well-formed ubiquitous shall-statement.

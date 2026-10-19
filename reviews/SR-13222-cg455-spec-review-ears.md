---
id: "SR-13222"
title: "CG PR 455 EARS conformance: FR-019-AC-48 to AC-50"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@cfa6dcf6bf52f886e780e8234bffa249b8bd44b9; spec/routed/functional/FR-019-capability-settlement.md lines 114-119 and 464-466 (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---

# SR-13222: CG PR 455 EARS conformance

## Summary

Ticket: IR-892.

AC-48 and AC-49 follow the unwanted-behaviour pattern: "If <reader returns Refused or Incomplete>,
then the generator shall return ...". AC-50 uses "If <Admitted> and <claim generated>, then the
generator shall retain ...". Each criterion has an explicit subject (the generator), its own
"shall", one response, and a single trigger.

The added Outputs bullet is a list entry under `## Outputs`, not a requirement statement.

## Verdict

PASS. `quire validate` emits `ac:non-canonical-shape` warnings on lines 464-466 because the
criteria are obligation-shaped rather than direct assertions. The brief requires the EARS
"If ..., then the generator shall ..." form, so this is a style tension, not a defect. The
warnings do not fail `make spec`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

---
id: SR-15228
title: "IR-604 PR493 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-016, spec/replay/matrix/TC-026, spec/replay/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed refusal, mismatch and verdict partition in PR #493 at head 415b634ab5c0b6960a1cf2c914b4415256a21349. Measured KaniExecutionEvidence and ProofCeilings against QSL WitnessArmResult charges and CG ReplayInputs.

## Verdict

**PASS — malformed, out-of-domain, QSL refusal, inconclusive and non-violation paths remain typed and distinct.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

---
id: SR-15229
title: "IR-604 PR493 scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-016, spec/replay/matrix/TC-026, spec/replay/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed Kani proof versus QSL replay accounting boundary in PR #493 at head 415b634ab5c0b6960a1cf2c914b4415256a21349. Measured KaniExecutionEvidence and ProofCeilings against QSL WitnessArmResult charges and CG ReplayInputs.

## Verdict

**PASS — Kani proof ceilings and QSL scalar replay charges describe different work; no cross-tool accounting comparison is asserted.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

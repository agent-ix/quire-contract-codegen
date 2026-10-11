---
id: SR-8562
title: "IR-768 FR-021 structural integrity review"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@6bd393f2092ba50dcdab88f918f706580c6684e6; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

# SR-8562: IR-768 FR-021 structural integrity review

## Summary

Ticket: IR-768. Examined the edited FR-021 Behavior and AC-12, AC-23, AC-25 against TC-031 steps 11 and 13 for consistency, atomicity, testability, and coverage. The matching requirement and test-case text retains the old lowering branches and introduces the new Call-root branch without a structural conflict. The missing matrix-index entry is already recorded in base review SR-8560; the overlapping-refusal case is assigned to failure-domain review SR-8563.

## Examined units

- `FR-021` Behavior, examined: source-bound Call classification, Scalar and CompositeEquality lowering, callee propagation, call-item lowering, and package assembly.
- `FR-021-AC-12`, examined: malformed Call roots and absent/refused callees cause typed, isolated refusals.
- `FR-021-AC-23`, examined: actual failed lowering records map through the shared classifier, preserving byte values and call-node-first outcome.
- `FR-021-AC-25`, examined: admitted 129-function chain within unchanged caps, emitted 128 calls, and runtime depth refusal.
- `TC-031` steps 11 and 13, examined: corresponding positive, failure, and boundary procedures.

## Verdict

No distinct structural-integrity finding beyond those recorded by SR-8560 and SR-8563.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

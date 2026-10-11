---
id: SR-8564
title: "IR-768 CG and Contract IR boundary review"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-codegen@6bd393f2092ba50dcdab88f918f706580c6684e6; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

# SR-8564: IR-768 CG and Contract IR boundary review

## Summary

Ticket: IR-768. Examined responsibility for strict-reader admission, checked Call-root request binding, Scalar/CompositeEquality and call-item lowering, runtime admission and depth checking. The edited boundary is explicit: Contract IR admits and lowers where applicable; this generator checks request-to-source Call binding and maps actual lowering failures; Contract Runtime checks and executes the assembled package.

## Examined units

- `FR-021` Behavior, examined: the strict reader supplies an admitted graph; CG validates Call-root identity and binding and uses Contract IR lowering for Scalar, CompositeEquality and requested call items; Runtime admits linked declarations and enforces call depth.
- `FR-021-AC-12`, examined: CG owns typed refusal and isolation for its body classification.
- `FR-021-AC-23`, examined: Contract IR owns actual Failed records, CG owns their refusal mapping and item precedence.
- `FR-021-AC-25`, examined: the source and lowering byte caps are unchanged; CG avoids sending checked Call closures through Contract IR and retains runtime depth behavior.
- `TC-031` steps 11 and 13, examined: independently measured lowering outcomes and an authentic strict-reader-admitted chain.

## Verdict

No ownership or system-boundary defect found. The combined-defect refusal ordering is recorded in SR-8563.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

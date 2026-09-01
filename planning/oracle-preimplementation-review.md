---
id: REV-006
title: "Deterministic oracle preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "issue #4 deterministic Boolean oracle and manifest first slice"
review_set: subset
---

# Deterministic oracle preimplementation review

## Summary

FR-001, interface-001, TC-001 through TC-003, and Task-004 define the first semantic slice. Runtime
revision `e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3` supplies allocation-free identities and Boolean
operators. Accepted IR PR #19 merge revision `5c49ebfd1c87415f74420ad047392bd03b1bd202` supplies validated typed
expressions, deterministic dependency identities, and the conformance corpus.

## Verdict

**FAIL.** FND-007 remains a high finding. The accepted IR revision is reconciled, but the explicit
typed-clause boundary must remain documented and this review must be repeated against the completed
implementation before the task can leave draft.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | high | Closed: IR PR #19 is accepted and the exact merge revision is reconciled. | IR PR #19, Task-003 |
| FND-007 | high | The accepted IR exposes `ContractPackage<ReferenceBody>` and typed expressions as separate surfaces; it does not bind a `TypedExpression` directly into each package clause, so the lowering core accepts explicit typed-clause inputs and invents no package adapter. | interface-001, IR PR #19 |
| FND-008 | medium | The full expression algebra is larger than one reviewable slice; the first slice supports Boolean literals/references/operators and fails closed for every other construct. | FR-001, TC-001, TC-003 |

## Coverage

The first slice has implemented evidence symbols for TC-001 through TC-003, but the TestMatrix rows
remain planned until their complete ticket scope is reviewed. FR-001-AC-2 is exercised by compiling
and executing a 265-expression differential corpus over Boolean literals, inputs, state
current/pre/post observations, nested operands, and every supported operator against the exact
runtime revision and an independent evaluator. TC-002 atomic publication remains outside this slice.

## Decision

Proceed with a fail-closed Boolean lowering core, deterministic source/source-map/manifest bytes,
exact dependency signatures, SPDX identity, and requirement/revision/clause propagation. Keep all
unsupported expressions diagnostic-only with no partial artifact bundle.

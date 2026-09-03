---
id: REV-007
title: "Tri-state harness and shaped-strategy preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "issue #3 harness and proptest strategy generation"
review_set: subset
---

# Tri-state harness and shaped-strategy preimplementation review

## Summary

FR-002 and TC-004 require generated code to distinguish rejected preconditions, accepted failures,
and passes. The accepted runtime supplies the tri-state verdict, proptest adapter, and indivisible
accepted/rejected/failed/discarded accounting types. The shared-assurance `main` revision supplies
deterministic Boolean lowering and packaged ProofAttestationV1 generation. This issue #3 remediation
is based directly on that revision and adds no local evidence framework.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-401 | high | A caller-assembled wrapper could evaluate postconditions against pre-state or invoke the subject before checking preconditions. Generated harness source therefore owns snapshot, precondition, invocation, and postcondition order. | FR-002, TC-004 |
| FND-402 | high | A generic `prop_filter` fallback could make supported correlated domains statistically improbable and hide discards. Supported ranges, memberships, and relations are shaped directly; residual rejection is distinct. | FR-002-AC-3, TC-004 |
| FND-403 | medium | Boundary campaigns can exercise only valid values and therefore miss immediately adjacent rejected cases. Boundary plans contain tagged inside and outside cases with overflow-safe endpoint handling. | FR-002-AC-2, TC-004 |
| FND-404 | medium | Shrinking can leave a shaped domain or silently convert a rejected case to success. Generated strategies preserve shaped constraints; residual constraints retain rejection accounting with an explicit invocation unit. | FR-002-AC-4, TC-004 |
| FND-405 | medium | The accepted IR does not yet bind full executable entry points to typed pre/post clauses. This slice accepts explicit typed-clause and subject-binding inputs and invents no package-level adapter. | interface-001, IR PR #19 |

## Decision

Proceed with a fail-closed first slice that generates deterministic harness and strategy artifacts,
uses the runtime's tri-state/accounting APIs, and tests seeded generation and shrinking behavior.
Keep TC-004 and FR-002 matrix rows planned until the complete issue scope and independent review are
closed. Postimplementation dispositions belong in `reviews/`, not in this planning record.

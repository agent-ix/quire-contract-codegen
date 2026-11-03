---
id: SR-202611020196
title: "IR-1289 AC-39 partial-status specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #719 at 85c153912ae67b6a3ac7464fd9d08efeffd73138; IR-1289; spec/kani/functional/FR-015-bounded-kani-obligations.md FR-015-AC-39"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Reviewed the edited FR-015-AC-39 status against its unchanged requirement, TC-025 method, scalar V2 negotiation, and the existing collection-operation result path. No applicable AssuranceProfile exists under `spec/`. The edit adds only implementation-status text; no new requirement grammar, dependency edge, hash, digest, or pin is introduced.

## Verdict

**CONDITIONAL pending FND-001.** The new status sentence overstates the scope of the typed refusal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new status says result-bearing postconditions receive a typed refusal without limiting that claim to the scalar V2 path. `classify_v2_clause` falls back to collection lowering after the scalar refusal, and `operation_context_checks_result_domain_and_the_authored_postcondition` asserts a supported result-bearing collection postcondition whose produced result domain is checked. Narrow the status sentence to scalar V2 clauses outside the supported collection path, preserving that existing collection behavior. | spec/kani/functional/FR-015-bounded-kani-obligations.md:850; src/kani/generate/negotiate.rs:931; tests/it/kani_collection_operation.rs:485 |

## Coverage

The criterion remains PARTIAL and retains Test (TC-025) as its verification method. The new QSL result-only and unused-result tests support the scalar fail-closed path. The existing collection test demonstrates that result-bearing collection operation clauses already have positive produced-result handling. Targeted Quire validation remains with the code lead's final gate. No additional spec sub-analysis applies to this status-only edit.

## Dispositions

Round 1 at `355aa81bbbc8f07c59b99afd9689b2dbeeb6d707`: **PASS after fix.** The AC39 status limits the refusal to the scalar V2 clause path, while the collection operation result path remains supported.

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | `Result-bearing postconditions on the scalar V2 clause path receive a typed item refusal naming the admitted result binder until that path can represent the produced value.` |

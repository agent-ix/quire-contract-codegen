---
id: SR-601005
title: "IR-595 PR #595 rebased delta code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #595 exact head cad6f99cb1e07b5a32f636e6a307f05234f1a65f; delta versus reviewed f21865c417691cb9f8c4edc42a18adfda2d69489; Cargo.lock, src/oracle/function/owner_path.rs, src/replay/function.rs, tests/it/exact_function_generation.rs, tests/it/skeleton_spine.rs"
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: references
---

## Summary

Ticket: IR-595. Reviewed exact PR head cad6f99cb1e07b5a32f636e6a307f05234f1a65f against the previously reviewed f21865c417691cb9f8c4edc42a18adfda2d69489, separating #601's inherited changes. Earlier SR-590006 and SR-590007 dispositions remain applicable.

## Verdict

PASS — The new dependency pins, replay enum arms, AC-12 index repair, and V1 spine fixture retirement contain no new code defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. Scope is the delta and prior-review carry-forward, not a fresh repository-wide assurance claim. Quoin matrix 0.29.1 has the examined FR-021 test criteria tagged; AC-18 and AC-32 are inspection rows. No new spec or plan file is in this PR's own diff.

### Examined criteria

- FR-021-AC-2: Compare only with the former runtime port or build the direct application from emitted source, so two matching errors pass.
- FR-021-AC-3: Read function identity from emitted source, so a mutated schedule and claim map agree falsely.
- FR-021-AC-12: Insert a stub instruction for a body whose lowering or classifier failed.
- FR-021-AC-17: Take function origin from request ordinal or emitted source instead of the binding, allowing claim and source to drift together.
- FR-021-AC-18: Claim or present as evidence agreement with the QSL-owned application authority in a test name, trace tag, output line, doc comment, TC-031 step or matrix row.
- FR-021-AC-32: Keep the deleted Contract Runtime `exact` port reachable through a re-export, wrapper or compatibility layer.
- FR-021-AC-33: Classify a directly requested formula or fairness root as the clause's QSL-121 blocker, classify a lowered `case` body or call item as `FormMismatch`, substitute the unsupported body root for the distinct valid call item and thereby mask the body's refusal, or omit the affected call item's claim while its healthy sibling is scheduled.
- FR-021-AC-35: For a concrete FR-485 corpus request, the caller supplies all seven acquisition inputs and the separately admitted Contract IR package. CG calls `qsl_replay::acquire_checked_closure` once for the generated run and receives `CheckedClosureCompilation` with emitted package, admitted context and retained `CheckedClosureHandle`; it does not reconstruct a QSL checked closure from the V2 graph or emitted source. Acquisition failure remains `AdmittedRefusal` and yields no handle. PLANNED pending IR-595; the QSL FR-485 facade is delivered.
- FR-021-AC-36: CG maps the declaring admitted V2 `package_id` domain and digest exactly to FR-485's `DigestRecord` owner id, and the function's `CheckedNodeId` digest exactly to `WireNodeId` only when its domain is `quire.checked-semantic-node/v1`. The own or imported declaring package is selected explicitly; no cross-domain coercion, rehash, fabricated `NodeKey` or name-only owner substitution occurs. CG calls `CheckedClosureHandle::lookup_callable(owner_package_id, wire_node_id)` and receives a checker-issued opaque `CallableSelection` before registration. It registers the generated `BodyFactory` with that
- FR-021-AC-37: A mismatch between `CheckedClosureHandle::package_id()` and the CG request's admitted unit package id refuses the item as `ExactFunctionRefusal::OwnerIdentityMismatch` before registration, Begin, body invocation or application charge. The same refusal follows a mismatch between `CallableSelection::owner_package_id()` and the selected function's declaring admitted V2 package id, or between the selection's read-only check-minted key bytes and that function's node digest. The selection comparisons run in a private post-lookup guard; TC-031 tests the key comparison with a genuine second selection

### Examined files

- `Cargo.lock`: examined.
- `src/oracle/function/owner_path.rs`: examined.
- `src/replay/function.rs`: examined.
- `tests/it/exact_function_generation.rs`: examined.
- `tests/it/skeleton_spine.rs`: examined.

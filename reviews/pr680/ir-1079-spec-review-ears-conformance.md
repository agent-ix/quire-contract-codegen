---
id: SR-2026103038
title: "IR-1079 PR 680 spec-review/ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; IR-1079; PR #680; spec/core/functional/FR-039-contract-package-consumption.md; spec/core/matrix/TC-053-contract-package-consumption.md; spec/core/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-053
    type: references
---

## Summary

Ticket: IR-1079. Frozen PR #680 head 16d83edf7a14fca2cceadaf3ea05a7624914b389. The changed normative behavior names CG, uses concrete source/key operations, and states measurable refusal and identity outcomes. Main advanced disjointly to 567bedef; the review remains scoped to FR-039, TC-053 and the core matrix.

## Verdict

No findings in this method on the scoped diff. No Rust code or full gate was run. No applicable AssuranceProfile was found under spec/.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Reviewed scope

```yaml
- id: FR-039
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: "For a V2 request CG SHALL obtain that key from the owning admitted\n \
    \ `CheckedPackageV2` through `ContractSourceKeyV1::from_v2_request(source,\n \
    \ request)`; for the corresponding V1 accounting entry it SHALL use the\n  admitted\
    \ package's `source_key_at` resolution of the referenced `source_keys`\n  entry."
- id: FR-039-AC-9
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'In an admitted package with a bound driver item, an excluded accounting-only
    position and another bound driver item, CG matches all three positions to `feature_accounting`
    by position and equal source-bound structural keys resolved from the owning V2
    source and admitted V1 source frame; unequal raw graph-local indexes alone do
    not refuse. The excluded position keeps its driver index but has no claim or generated
    artifact, and the following bound item keeps its original driver index. An interleaved
    context-only root has accounting but advances neither driver nor envelope item
    indexes. Removing '
- id: FR-039-AC-10
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Given a bound driver item followed by an excluded accounting-only position,
    a selected conversion context root outside the preceding item's contiguous span
    yields typed `IncompleteContext` for that item when the complete root sequence
    is otherwise valid. A context-only root directly after the excluded position and
    before the next bound driver item is orphaned and causes a typed whole-request
    correspondence refusal before any artifact, even when its accounting position
    and source-bound structural key match. A conversion root immediately before the
    excluded position remains in the preceding item
- id: FR-039-AC-11
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Generated scalar, composite equality and exact function claims each serialize
    `contract_package_identity` equal to the admitted FR-330 `package.source` structural
    package name plus complete lock, replacing the removed `ContractPackageV1.identity()`
    accessor, and `node_id` equal to the original requested V2 `CheckedNodeId`, including
    graph-local indexes; their containing claim-map entries use that same `node_id`,
    with no generated-claim `ir_id` key or second public V1 node identity. CG matches
    the request to V1 accounting through equal source-bound structural keys resolved
    by `ContractSourceKey
- id: TC-053
  path: spec/core/matrix/TC-053-contract-package-consumption.md
  role: examined
  excerpt: "Include a genuine indexed request whose V2 graph index differs from the\n\
    \   corresponding V1 `source_keys` index."
- id: TestMatrix
  path: spec/core/matrix/tests.md
  role: examined
  excerpt: '| FR-039 | FR-039-AC-9 | TC-053 | 🚧 Partial; tagged tests assert accounting
    positions, typed correspondence refusals, preserved driver indexes, and no-claim
    refusals. Source-bound V2-to-V1 key equivalence when raw indexes differ and unresolvable
    source-key refusal remain planned |'
```

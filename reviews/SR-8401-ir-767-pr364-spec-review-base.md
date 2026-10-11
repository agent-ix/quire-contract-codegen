---
id: SR-8401
title: IR-767 spec-review/base review of PR 364
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen@e36c1942650842b08c72c96e0772d39a754b3aa9; spec/core/functional/FR-039-contract-package-consumption.md;
  spec/core/matrix/TC-053-contract-package-consumption.md; spec/core/matrix/tests.md
review_set: subset
---
# SR-8401: IR-767 spec-review/base review

## Summary

Ticket: IR-767. Reviewed the frozen PR #364 diff at e36c1942650842b08c72c96e0772d39a754b3aa9. One infeasible acceptance control found.

## Examined criteria

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-039-AC-7 | examined | Generated scalar, composite equality and exact function claims each serialize `contract_package_identity` equal to the admitted FR-330 `identity` and `node_id` equal to the requested `CheckedNodeId`, with no generated-claim `ir_id` key. Each retains exact `semantic_type`, admitted `source_map`, family-specific provenance, and `package_id` denoting the originating checked package; each serializes typed `claims`, `bounds` and `dependencies`. For the same requested node in two genuinely admitted packages with different identities, the claim identity differs; for two different requested nodes in o |
| FR-039-AC-8 | examined | Generated claims' reached claim, bound and dependency keys and dependent declaration closure equal Contract IR's authoritative traversal over the admitted package. A missing required root, bound or selected dependency yields a typed item refusal retaining the exact requested ID and admitted source occurrences, with no generated claim or artifact; a refused or incomplete reader result yields no item claim or artifact. No per-node ID from a second lowering or wire-fragment hash appears. |
| TC-053-step-7 | examined | For one genuinely admitted package, generate scalar, composite equality and    exact function claims at two different requested node IDs. For each family,    inspect both the typed claim and serialized claim map: the generated result    has the admitted `identity` as `contract_package_identity`, the requested    `CheckedNodeId` as `node_id`, the originating checked-package `package_id`,    `semantic_type`, admitted `source_map`, family-specific provenance, typed    `claims`, `bounds` and `dependencies`, and no `ir_id` member. Compare    reached claim, bound and dependency keys, and any declara |
| TC-053-step-8 | examined | Exercise an admitted `lowered` item for which the authoritative traversal    cannot resolve a required root, bound or selected dependency. Assert a typed    item refusal retaining requested identity and available admitted source    occurrences, with no generated claim or artifact. Repeat with a reader    refusal and incomplete reader outcome; neither reaches an item claim.    Inspect the call path for a second lowering, reconstructed checked graph or    wire-fragment-derived per-node identifier.  |
| FR-039-matrix-row | examined | \| FR-039 \| FR-039-AC-1 through FR-039-AC-8 \| TC-053 \| 🚧 Planned; production FR-330 consumer and criterion-tagged integration tests are not implemented \| |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-053 step 8 requires an admitted lowered package with an unresolved required root, bound or selected dependency. Contract IR FR-035 requires every reference in the admitted package to resolve, and FR-039 already sends missing or stale selected dependencies to the reader refusal path. This test control cannot be constructed through the required lowerer and strict reader, so the claimed item-refusal branch has no valid fixture or oracle. Specify a reachable traversal-level failure distinct from an invalid package, or test reader refusal instead. | spec/core/functional/FR-039-contract-package-consumption.md:162; spec/core/matrix/TC-053-contract-package-consumption.md:79 |

## Verdict

The infeasible admitted-package test control must be corrected before this specification is handed to implementation.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 25e25eebc80340c15e1fe78b0b29e42fdc3d8bda — FR-039-AC-8 and TC-053 step 8 now put unresolved references at Contract IR lowering/reader boundary; admitted lowered traversal is total. |

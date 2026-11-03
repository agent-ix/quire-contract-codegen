---
id: SR-202611020186
title: "IR-948 AC-43 sibling-refusal code review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #717 at 8a0ac2afba18790ef9e631fb1b184fffcaa2dd22; src/kani/generate/{negotiate,collection,collection/operation,outcome}.rs; tests/clause_admission_support/mod.rs; tests/it/kani_collection_operation.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Reviewed the frozen Rust diff using code-review and rust-review. No applicable AssuranceProfile exists under `spec/`. The new request pass refuses scalar postconditions with same-anchor siblings until `requires` can be rendered and refuses collection postconditions when a sibling is absent or has an unsupported disposition. The tests use admitted QSL clauses. No CI workflow, dependency, copied upstream artifact, or compatibility path changed.

## Verdict

**PASS after FND-001 was fixed.** Both paths now use one admitted sibling selector.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `same_anchor_preconditions` duplicates the collection graph's precondition index with different node filters. `CollectionGraph::new` indexes every term with `state_clause_kind() == Precondition`, while the new selector additionally requires `state` / `state_clause` tags. The collection lowerer and request validator can therefore disagree about the sibling set as accepted node forms evolve, allowing a postcondition to assume a sibling the request validation never checked. Use one shared selector for both paths. | src/kani/generate/negotiate.rs:498; src/kani/generate/collection.rs:155 |

## Coverage

FR-015-AC-43's refusal half and its PARTIAL status were examined. The new unit tests exercise missing, supported-but-unrendered, and unsupported scalar siblings with two orderings; collection integration tests exercise missing, supported, and present-but-unsupported siblings. The positive scalar `requires` and embedded-oracle path, typed prestate, and FR-025 subject ABI remain planned by the criterion's status. The generated-agreement and real-Kani gates are outside this review; the code lead owns the final reviewed-head gate.

## Dispositions

Round 1 at `f548fad4fe8e51ff6b6d50ef870ee87d3bfe8e07`.

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | `CollectionGraph::new` stores `preconditions_by_anchor(package)`, and `same_anchor_preconditions` obtains the selected anchor's node ids from that same helper; the divergent tag filter was removed. |

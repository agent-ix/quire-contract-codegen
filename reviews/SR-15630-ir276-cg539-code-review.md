---
id: SR-15630
title: IR-276 CG PR539 clause graph-view code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-276; PR #539; src/core/ir/clause.rs; src/oracle/clause.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
---

## Summary

Independent review of CG PR #539 at 8ca7d984a14d08f306b86f60f2ab3ffd981dc949 against main 7ea6a9988a4e6ecb5369c77a1941350bde52c4b2. The view retains the first graph entry per id and shares one node map between planning and reading. Public single-clause functions retain their signatures. This slice does not yet amortize the view across a K-clause request.

## Verdict

CONDITIONAL — one package/view identity risk and two incorrect test-to-criterion bindings need correction before handoff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `with_view` API accepts a package and a graph view from different admitted packages, allowing a clause absent from the supplied package to be planned from the other package's graph. | src/oracle/clause.rs:111, src/core/ir/clause.rs:64 |
| FND-002 | medium | The first-entry lookup control claims FR-014-AC-38 although it constructs literal expression nodes and never derives a clause descriptor or oracle. | src/core/ir/clause.rs:347 |
| FND-003 | medium | The source-inspection work control claims FR-014-AC-38 although it counts `.graph()` spellings and never exercises the clause behavior that criterion specifies. | src/core/ir/clause.rs:363 |

## Coverage

Rust review checked ownership, lifetimes, panic surface, deterministic lookup, duplicate-id first-entry behavior, public API preservation, and changed tests. No new unsafe, copy, compatibility layer, gate weakening, or generated output format was found. `git diff --check` and `cargo fmt --check` passed at the reviewed head. The author reported focused tests; this review did not run a full gate. The separate K-clause caller and LLVM coverage caller remain assigned follow-on slices. No applicable AssuranceProfile was found under `spec/`.

## Examined scope and bindings

```yaml
scope:
  - id: IR-276-AC-5
    path: src/core/ir/clause.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
  - id: FR-014-AC-38
    path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
    role: examined
    excerpt: "The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle."
  - id: src/oracle/clause.rs
    path: src/oracle/clause.rs
    role: examined
    excerpt: "pub(crate) fn generate_v2_clause_oracle_with_view(package: &CheckedPackageV2, node_id: &CheckedNodeId, nodes: &ClauseGraphView<'_>)"
findings:
  - fnd: FND-001
    severity: medium
    confidence: high
    method: code-review
    check: soundness
    artifact_id: IR-276-AC-5
    related_ids: [FR-014-AC-38]
    path: src/oracle/clause.rs
    lines: "111-115,222-226"
    excerpt: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
    finding: "The package and view arguments have independent lifetimes and no identity check. A future request caller can pass package A with a view built from package B, then plan a B clause that A did not admit. Bind the package into the view, or otherwise make the pair inseparable at the API boundary."
  - fnd: FND-002
    severity: medium
    confidence: high
    method: code-review
    check: trace
    artifact_id: FR-014-AC-38
    related_ids: [IR-276-AC-5]
    path: src/core/ir/clause.rs
    lines: "347-360"
    excerpt: "The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle."
    finding: "`clause_graph_view_preserves_first_node_and_reuses_one_walk` only constructs literal expression nodes and reads a lookup map; it does not verify the clause oracle contract of FR-014-AC-38. Its actual check is IR-276 AC-5 work sharing."
    test_id: clause_graph_view_preserves_first_node_and_reuses_one_walk
    claimed_ac: FR-014-AC-38
    correct_ac: IR-276-AC-5
  - fnd: FND-003
    severity: medium
    confidence: high
    method: code-review
    check: trace
    artifact_id: FR-014-AC-38
    related_ids: [IR-276-AC-5]
    path: src/core/ir/clause.rs
    lines: "363-379"
    excerpt: "The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle."
    finding: "`clause_planning_and_reading_share_one_graph_walk` counts source spellings of `.graph()` and cannot fail for a wrong descriptor, oracle, identity, or refusal; it is an IR-276 AC-5 work control, not FR-014-AC-38 evidence."
    test_id: clause_planning_and_reading_share_one_graph_walk
    claimed_ac: FR-014-AC-38
    correct_ac: IR-276-AC-5
bindings:
  - { test_id: clause_graph_view_preserves_first_node_and_reuses_one_walk, ac_id: FR-014-AC-38, trace: wrong }
  - { test_id: clause_planning_and_reading_share_one_graph_walk, ac_id: FR-014-AC-38, trace: wrong }
```

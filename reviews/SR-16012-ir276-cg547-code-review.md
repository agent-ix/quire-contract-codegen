---
id: SR-16012
title: IR-276 CG collection graph API code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; CG #547; src/kani/generate/collection.rs; src/kani/generate/negotiate.rs; src/kani/generate/role_parity_tests.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
---

## Summary

Independent diff-scoped code and Rust review of CG #547 at exact head `4f82ade9e141d7694600ed8e79e2b6d0a2205a43` against main. Ticket: IR-276. The new `CollectionGraph` retains the admitted package and a first-match index; lower calls take that graph. The current negotiator constructs a graph at its former per-item boundary. This is the API prerequisite for the later caller reuse PR, not the caller reduction itself.

## Verdict

PASS. No code or Rust finding remains. The initial review identified that the new constructor-count test was misbound to FR-015-AC-96 while selecting a missing quantifier. Before this artifact was posted, the author changed the tag to `TC-025` only at the final head; it no longer claims AC-96 coverage. The test reads the actual `CollectionGraph::new` counter, calls lower three times on one admitted graph, and asserts a typed refusal each time and one construction. The existing synthetic test still checks original-order first-match lookup over duplicate entries.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The changed production path preserves `UnsupportedShape` on a missing quantifier and leaves `resolve_collection_field` and its typed refusal mapping unchanged. `CollectionNodeIndex::from_nodes` still uses `or_insert` for the first node id and first digest. No new compatibility wrapper, unsafe, panic path, public production API, dependency, or CI gate was introduced. The per-item construction in `negotiate.rs` remains intentional for this prerequisite; caller reuse is owned by the follow-on PR. `git diff --check origin/main...HEAD` passed. The author reports the focused constructor test passed; this reviewer did not run Cargo or aggregate gates.

## Examined scope and bindings

```yaml
scope:
  - id: IR-276-AC-5
    path: src/kani/generate/collection.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
  - id: FR-015-AC-96
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: examined
    excerpt: "An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall` or `quire.op.collection.exists` operation over a selected model collection field is a collection claim."
  - id: FR-015-AC-100
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: context_only
    excerpt: "For the collection family, changing the selected field's authored cardinality or element bound, admitted body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically."
  - id: AD-004-L-2
    path: spec/assurance/AD-004-cg-crate-layout.md
    role: examined
    excerpt: "The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]` modules included, and no file imports an item through the crate root."
findings: []
bindings:
  - { test_id: collection_lower_calls_reuse_the_admitted_graph_constructor, ac_id: TC-025, trace: correct }
  - { test_id: every_identity_component_changes_the_rendered_harness_path, ac_id: FR-015-AC-100, trace: correct }
```

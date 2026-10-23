---
id: SR-16013
title: IR-276 CG collection graph API gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; CG #547; src/kani/generate/collection.rs; src/kani/generate/negotiate.rs; src/kani/generate/role_parity_tests.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Planless, diff-scoped gap analysis of CG #547 at exact head `4f82ade9e141d7694600ed8e79e2b6d0a2205a43`. The API change allows one package-retaining collection graph to serve multiple lower calls. The actual request-boundary reuse remains in the follow-on caller PR. IR-276-AC-5 is a ticket criterion, so Quire's computed CG matrix does not mint it.

## Verdict

PASS for the changed API and its tests. This is not a claim that the complete IR-276-AC-5 performance objective is delivered by #547 alone or that the existing PARTIAL FR-015-AC-96 and AC-100 obligations are complete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional gaps found (placeholder) | - |

## Coverage

`quire matrix --scope . --format json` computed five existing binders for FR-015-AC-100 and multiple existing binders for FR-015-AC-96 on the candidate base; both criteria retain their documented PARTIAL status. The new constructor-count test is tagged only to TC-025 and claims neither AC. It checks an admitted package's actual constructor count for K=3 and exact `UnsupportedShape` refusals. The existing synthetic index test checks one ordered census, first-match node and digest resolution, and bounded lookup comparisons. The changed lower function retains its prior field-selection path and output fields, giving the new retained API an owning requirement and a behavioral control. No stub or unsupported success path was added.

Plan completion: not assessed. Reverse scope: changed files only. Optional semantic audit was not requested. No aggregate gate was run by the reviewer.

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
    role: examined
    excerpt: "For the collection family, changing the selected field's authored cardinality or element bound, admitted body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically."
findings: []
bindings:
  - { test_id: collection_lower_calls_reuse_the_admitted_graph_constructor, ac_id: TC-025, trace: correct }
  - { test_id: every_identity_component_changes_the_rendered_harness_path, ac_id: FR-015-AC-100, trace: correct }
```

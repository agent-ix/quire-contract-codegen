---
id: SR-15622
title: IR-276 CG PR538 code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-276; CG #538 at 80cfc2c1d5434e4bbcec704e9b6e89537f00ea98; src/kani/generate/collection.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

## Summary

Independent diff review of CG #538 against 7ea6a998, including the Rust lane. The BTreeMap views preserve the first matching node ID and first digest in graph order, while absent IDs still refuse. One test counter does not observe lookup work.

## Verdict

CONDITIONAL. Fix FND-001 in this PR and recheck its exact head. The request-level caller still constructs a view per collection item; this is the stated preparatory boundary, with a separate caller slice assigned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The counted lookup control increments only after a successful lookup, so its `lookup_visits <= 128` assertion stays true even if each of three lookups linearly scans all 128 nodes. Observe the actual traversed entries or use another control that fails when lookup work returns to K×R. | src/kani/generate/collection.rs:115 |

## Coverage

Diff: collection graph construction, node and digest lookup, all replaced graph scans, collection refusal/order behavior, and the new unit control. No changed CI workflow, config, vendored content, unsafe, network, concurrency, or wire schema. `git diff --check` passed. Author's focused logs report collection unit tests 8/8, collection negotiation 1/1, clippy, and fmt passing; this review did not run full gates. The local additional focused Cargo run was stopped during dependency compilation because the author already had passing focused evidence.

The preparatory PR reduces multiple internal scans within one claim to one index construction, but `lower_collection_claim` still calls `CollectionGraph::new(package)` per item. Request-level K×R remains until the separately owned `negotiate.rs` caller threads one view across all collection claims. No request-level AC-5 completion is claimed here.

No applicable AssuranceProfile was found under `spec/`. The code's `#[trace]` form matches repository conventions. Optional whole-tree semantic gap review was not run.

## Examined scope

```yaml
scope:
  - id: FR-015-AC-96
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: context_only
    excerpt: "An admitted QSL V2 clause/function body that contains a quire.op.collection.forall or quire.op.collection.exists operation over a selected model collection field is a collection claim."
  - id: FR-015-AC-97
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: context_only
    excerpt: "An admitted collection claim with no finite upper bound or finite element domain settles requires_bound without a harness; unsupported forms settle unsupported."
  - id: IR-276-AC-5
    path: src/kani/generate/collection.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear find or filter over all graph nodes, all declarations, or the whole source map."
```

## Dispositions

Exact reviewed fix head: `889cc62b464a42ec1f0fc25bf779cdac2738c590`.

Final disposition verdict: **PASS** for this preparatory API PR; no finding remains open.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Removed post-hit `node_observed`; the new `CountedKey::cmp` counts comparisons made by the production `FirstMatchIndex::get`. A deliberately linear index failed at 384 comparisons for K=3, R=128; the BTreeMap implementation passed the below-128 bound. Focused 9/9 unit and 1/1 integration, clippy, fmt, and diff checks passed at the new head. |

The fixed PR remains preparatory. The future `negotiate.rs` caller must construct one `CollectionGraph` for the same package passed to collection lowering, and a request-level counted test must fail if that caller rebuilds the view per item. The current internal lowerer constructs the view from its own package and cannot pair a view with another package.

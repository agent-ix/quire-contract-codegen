---
id: SR-15771
title: IR-276 CG caller pass code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-276 AC-5 caller pass; src/evidence/v2_llvm.rs; src/kani/generate/clause/v2.rs; src/kani/generate/negotiate.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Independent diff-scoped code and Rust review of the IR-276 AC-5 caller-pass candidate at `7f0df7c996ca870834e73c8c6dabe9a9e7544852` against CG main `2cdce220567878a20170088606e049ed67cf9cc2`. The production delta shares one package-bound clause graph view and one ordered source-occurrence index per borrowed checked package in the Kani request, and one clause graph view per LLVM analysis. The prior per-item standalone clause oracle calls and full source-map filters are removed from those production loops. The scalar cache and classifier arms from CG #534 remain intact.

## Verdict

CONDITIONAL — the new source-tree test modules include an integration fixture that imports the crate's replay facade through the crate root, contrary to AD-004 L-2. Production lookup and output preservation checks found no other defect in this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The two new `#[path]` test-module inclusions bring the replay facade and crate-root imports into the evidence and Kani module trees, violating AD-004 L-2's test-inclusive dependency direction. | src/evidence/v2_llvm.rs:419; src/kani/generate/negotiate.rs:59; tests/state_frame_support/native_twin.rs:21; spec/assurance/AD-004-cg-crate-layout.md:897 |

## Coverage

The branch's test-first commits instrument the prior V2 clause graph and source-map iterations and the two standalone LLVM clause-index builds. The final source structurally builds `ClauseGraphView` once per checked package when the first V2 clause is classified, and once per complete LLVM analysis, then passes that same view to descriptor, oracle, and Kani lowering. The source-occurrence map is populated once in original order per borrowed package, with output clones restricted to the selected node's entries. The `K=2,4,8` and `R`-varying Kani control asserts one source census and one clause view; the LLVM control asserts one original-order population census and one view. This review checked counter placement and source structure; author runtime measurements are not treated as reviewer-run gates.

The request tests compare each batched record with a single-item baseline, including a supported V2 clause and typed non-clause refusal. The LLVM tests compare full report bytes under input permutations and check population, artifact, and map refusals. The diff adds no unsafe, public compatibility wrapper, source-file copy, Cargo dependency, or gate change. `git diff --check` and `cargo fmt --check` passed at the candidate head. No aggregate gate was run by the reviewer.

## Examined scope and bindings

```yaml
scope:
  - id: IR-276-AC-5
    path: src/kani/generate/negotiate.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
  - id: AD-004-L-2
    path: spec/assurance/AD-004-cg-crate-layout.md
    role: examined
    excerpt: "The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]` modules included, and no file imports an item through the crate root."
  - id: FR-004-AC-16
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies."
  - id: FR-004-AC-18
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification, and is never treated as implication-free."
  - id: FR-004-AC-23
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "Over a V2 clause population, removing one clause, one consequent probe, one evaluation probe, or every clause from the generated artifacts, each alone, prevents successful analysis."
  - id: FR-004-AC-24
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "Over a V2 clause population, each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification."
  - id: FR-014-AC-38
    path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
    role: context_only
    excerpt: "The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle."
findings:
  - fnd: FND-001
    severity: medium
    confidence: high
    method: code-review
    check: soundness
    artifact_id: AD-004-L-2
    related_ids: [IR-276-AC-5]
    path: src/evidence/v2_llvm.rs
    lines: "419-420"
    excerpt: "The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]` modules included, and no file imports an item through the crate root."
    finding: "Both newly included `native_twin.rs` test modules import `quire_contract_codegen::{FrameReplay, ...}` through this crate's root. `FrameReplay` belongs to `replay`, which evidence and Kani cannot import under AD-004. The existing layout text scanner reads only `src/` files and misses imports reached through `#[path]`, so its green result would not certify this boundary. Move these controls to an integration-test layer that already owns the fixture, or use a fixture confined to the permitted dependency direction."
bindings:
  - { test_id: request_view_keeps_complete_population_and_report_bytes_under_input_permutation, ac_id: FR-004-AC-16, trace: correct }
  - { test_id: request_view_keeps_complete_population_and_report_bytes_under_input_permutation, ac_id: FR-004-AC-23, trace: correct }
  - { test_id: request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows, ac_id: FR-004-AC-18, trace: correct }
  - { test_id: request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows, ac_id: FR-004-AC-23, trace: correct }
  - { test_id: request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows, ac_id: FR-004-AC-24, trace: correct }
```

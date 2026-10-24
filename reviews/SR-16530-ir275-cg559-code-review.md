---
id: SR-16530
title: "IR-275 CG559 independent code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR 559 exact head f5ccbe7b1d366adefb8780b4f1824c09292f2745; typed checked-body reader adaptation, 32 changed files"
review_set: subset
---

## Summary

Reviewed the CG typed checked-body follow-up against IR-275 and the merged IR and QSL APIs. `CheckedGraph` retains separate owners for raw lowered-package metadata and typed checked-source nodes. Keyed term reads remain delegated to the package owner, while caller decisions use typed term variants. The lock selects published IR `7da208c3` and QSL `dc267af5`.

## Verdict

PASS — no diff-scoped code or Rust finding. The author owns the final reviewed-head `make ci` and Kani gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Inspected the production diff in checked_graph, scalar/equality/function oracles, Kani generation, replay, and Cargo.lock, plus changed test and fixture projections. Checked keyed node/term association across both graph owners, typed body matching, refusal classification for added TimedModel and call-site variants, source identity and operation catalog reads, and absence of a typed-body serialize-back in production. Checked the changed test assertions and `#[trace]` context; no new test function or weakened guard is introduced. `git diff --check` passed. Author-reported focused checks, tests, Clippy and rustfmt passed; I did not rerun build or aggregate gates. No unsafe, vendored file or compatibility path appears in the diff.

Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
  - id: IR-275-AC-2
    path: "src/oracle/checked_graph.rs; src/oracle/scalar/mod.rs; src/oracle/equality/mod.rs; src/oracle/function/mod.rs; src/kani/generate/negotiate.rs"
    role: examined
    excerpt: "No non-test code in IR or CG reads a term through Value::get on a string key; the public wire body remains Value under the recorded scope correction."
  - id: IR-275-AC-4
    path: "src/oracle/checked_graph.rs"
    role: examined
    excerpt: "CG has one node index shared by the three modules."
  - id: FR-039-AC-5
    path: "src/oracle/checked_graph.rs; src/oracle/equality/mod.rs; src/oracle/function/mod.rs; src/oracle/scalar/mod.rs"
    role: context_only
    excerpt: "A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source map, stale semantic identity or incomplete reader result reaches no CG consumer and emits no artifact."
bindings: []
```

---
id: SR-16531
title: "IR-275 CG559 incremental gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR 559 exact head f5ccbe7b1d366adefb8780b4f1824c09292f2745; incremental IR-275 typed-body adaptation"
review_set: subset
---

## Summary

Checked the changed production paths and tests against the typed-body and shared-index criteria. Existing traced association controls and selected integration tests exercise the adapted code. The raw lowered-package body remains wire metadata; typed source bodies remain in IR's admitted checked package.

## Verdict

PASS — no incremental trace, implementation or test-intent gap found in PR #559. Final qualification and the cross-repository IR-275 AC-2 closing audit remain pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Compared the PR's production and test changes with IR-275 AC-2 and AC-4. The changed tests retain their assertions while replacing raw body indexing with typed application, aggregate, binding, reference and literal access. Searched non-test CG source for direct `body.get`, `body.pointer` and `body["..."]` reads: none remain. The remaining changed JSON lookup is inside admitted operation catalog metadata, not term-arm dispatch. No new test function was added. No Test Matrix was authored. Optional repository-wide semantic analysis was not requested.

Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
  - id: IR-275-AC-2
    path: "src/oracle/checked_graph.rs; src/oracle/scalar/mod.rs; src/oracle/equality/mod.rs; src/oracle/function/mod.rs; tests/it"
    role: examined
    excerpt: "No non-test code in IR or CG reads a term through Value::get on a string key; the public wire body remains Value under the recorded scope correction."
  - id: IR-275-AC-4
    path: "src/oracle/checked_graph.rs"
    role: examined
    excerpt: "CG has one node index shared by the three modules."
  - id: FR-039-AC-5
    path: "src/oracle/checked_graph.rs"
    role: context_only
    excerpt: "A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source map, stale semantic identity or incomplete reader result reaches no CG consumer and emits no artifact."
bindings: []
```

## Same-reviewer carry at 67891d3b

PASS — the repair adds typed decoder fields to existing traced fixtures only. The changed tests retain their prior assertions and source-to-test bindings; the scalar refusal control still uses an operation with no identity, and the rekey controls still compare produced candidate keys. No test is removed, ignored or made tautological. The review files in the parent custody commit are byte-exact copies of the original held artifacts. The prior full gate at that parent was red on fixture shape; the author ran the repaired library suite and selected integration controls, while final-head aggregate and Kani remain pending. No new incremental gap or finding.

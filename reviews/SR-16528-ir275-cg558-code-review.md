---
id: SR-16528
title: "IR-275 CG558 independent code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR 558 head f79bfe5a5c3d572e44d3212ee669c1431e2c9ec6; ten changed Rust files in core clause, Kani collection/frame/negotiate, oracle, replay and routed capability"
review_set: subset
---

## Summary

Reviewed the IR-275 CG half against the exact PR diff and current admitted `CheckedTerm` API. Changed production readers select terms from the checked package, match typed term variants, and retain JSON lookup only inside operation catalog records.

## Verdict

PASS — no diff-scoped code or Rust finding. The author owns the final reviewed-head gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Inspected all ten changed Rust files. Checked typed access to application, aggregate, binding, reference, literal and frame variants; collection shape refusals; frame effect refusal order; state clause shape and condition reads; routed function call argument order and exact reference targets; oracle unsupported-form classification; and composite equality operator identity. `git diff --check` passed. No source or tests were edited by this review. The PR reports focused tests, rustfmt and scoped Clippy passing; those results were read as author evidence and were not rerun. No full aggregate or Kani gate is claimed. No new dependency, vendored code, compatibility layer, or unsafe block appears in the diff.

Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
  - id: IR-275-AC-2
    path: "src/core/ir/clause.rs; src/kani/generate/collection.rs; src/kani/generate/frame.rs; src/kani/generate/negotiate.rs; src/oracle/mod.rs; src/oracle/equality/mod.rs; src/oracle/function/mod.rs; src/oracle/scalar/mod.rs; src/replay/composite_builder.rs; src/routed/capability.rs"
    role: examined
    excerpt: "No non-test code in IR or CG reads a term through Value::get on a string key; the public wire body remains Value under the recorded scope correction."
  - id: FR-040-AC-18
    path: "src/kani/generate/frame.rs"
    role: examined
    excerpt: "Admitted frame field modifications retain receiver and every-object scopes; unsupported create, delete and relationship effects retain typed refusals."
  - id: FR-014-AC-2
    path: "src/kani/generate/collection.rs; src/routed/capability.rs"
    role: examined
    excerpt: "A selected claim uses admitted checked-package structure and refuses unsupported shapes instead of deriving a proof from an unchecked term."
bindings:
  - test_id: a_quantity_conversion_predicate_is_an_unsupported_shape
    ac_id: FR-014-AC-2
    trace: correct
```

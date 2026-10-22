---
id: SR-15617
title: "Code and Rust review — IR-276 CG scalar lookup"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/kani/generate/negotiate.rs, src/oracle/scalar/mod.rs, tests/it/kani_obligations.rs; IR-276, FR-015, TC-025"
review_set: subset
---

## Summary

Reviewed PR #534 at 05b54f84 against f93f2715, including the per-request package cache, graph lookup, operand-range call, source ordering, and test. The negotiation path replaces its repeated graph and source-map scans. One scalar classification path still rebuilds the new index for each item.

## Verdict

**CONDITIONAL** — fix the remaining per-item full-package scan before claiming IR-276's scalar lookup criterion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `classify_scalar_form` constructs the full graph and source index on every scalar capability item and still linearly searches the checked graph; this leaves IR-276 AC-5's per-item scan in the classification path. | src/kani/generate/negotiate.rs:859-871; src/routed/capability.rs:792 |

## Rust review

The new pointer key has request-local lifetime, so no stale package reference escapes the call. `BTreeMap::entry(...).or_insert` retains the first graph node, and `extend` retains source wire order. No new panic, unsafe, unchecked conversion, unbounded retry, or public wire surface was found in the changed lines. Existing project idioms for `#[trace]` and integration-test placement are followed. The finding above is the Rust lane's resource-bound concern as well as a code-review concern.

## Scope and evidence

- Examined IR-276 AC-5: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
- Examined FR-015 description: "the code generator shall emit one bounded Kani obligation per precondition, postcondition, invariant and frame condition, each bounded by its model-domain bounds."
- Examined FR-015-AC-3: "An unbounded or non-finite obligation is refused with a typed reason and no harness."
- Examined TC-025 and the changed `tc_025_unbounded_non_finite_and_blocked_items_are_refused_without_harnesses` test.
- At `src/routed/capability.rs:792`, each `V2ScalarClaim` preflight invokes `classify_scalar_form`. That function constructs `ContractLookup::new(contract)` at line 864, scanning the entire graph and source map per call, then searches `package.graph().nodes.iter().find(...)` at lines 867-871. The new request-local cache in `negotiate_kani_obligations` does not cover this entry.
- No full gate ran during this review; the author owns the final-head gate after fixes.

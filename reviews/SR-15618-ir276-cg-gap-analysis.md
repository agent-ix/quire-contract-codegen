---
id: SR-15618
title: "Gap analysis — IR-276 CG scalar lookup"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-276 scalar lookup, FR-015, TC-025; src/kani/generate/negotiate.rs, src/oracle/scalar/mod.rs, tests/it/kani_obligations.rs"
review_set: subset
---

## Summary

Reviewed the changed scalar lookup against IR-276's result-preserving criterion, FR-015, the computed Test Matrix, and the added TC-025 assertions. The new test covers grouped refusal parity, but its source-order check has only one source occurrence per selected node and cannot detect occurrence reordering.

## Verdict

**CONDITIONAL** — add an admitted multi-occurrence scalar fixture so the order assertion can fail on a reversed or truncated source list.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The added source-order assertion selects four nodes from `scalar_package()`, whose builder gives each node one source-map occurrence; reversing occurrences per node would still pass. Exercise at least one admitted node with distinct ordered occurrences. | tests/it/kani_obligations.rs:1374-1397; tests/exact_scalar_support/package.rs:622-631 |

## Coverage

Plan completion: not assessed

- Examined IR-276 AC-5 and AC-6 (per-item scan removal and unchanged output). The PR deliberately covers the CG scalar negotiation slice; the separate collection and capability paths remain an explicit completion limit.
- Examined FR-015 description and FR-015-AC-3, FR-015-AC-14, FR-015-AC-16, and TC-025 as the relevant behavior and test context. The computed `quoin matrix --repo . --json` finds existing binders for the cited FR-015 criteria; the added assertions are inside a `#[trace("FR-015-AC-3", ...)]` test.
- The batch/singleton comparison is a useful refusal parity control. Every newly selected item is refused or requires bound, so this addition does not add a grouped supported-harness comparison. Existing tests exercise supported scalar generation on other paths.
- The fixture builder appends one `source_map` entry with one region whenever it appends a node (`tests/exact_scalar_support/package.rs:622-631`). `scalar_package()` uses `scalar_package_with_occurrences(false)`; its selected four nodes therefore do not create two distinguishable occurrences for the same node. The test's expected list is derived by filtering that same admitted package, so it does compare against wire order once a nontrivial fixture exists.
- This review did not opt into a repository-wide semantic sweep. The changed triple of requirement, test, and code was examined directly because the assignment specifically requests an acceptance-gap review.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 6ee41c76, `tc_025_scalar_source_lookup_preserves_multiple_occurrences_per_node` admits two source-backed nodes. For each, it asserts at least two distinct occurrences and exact vector equality with the admitted wire's ordered list. A reverse or truncation now fails. |

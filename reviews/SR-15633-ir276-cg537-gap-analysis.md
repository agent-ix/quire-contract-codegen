---
id: SR-15633
title: "Gap analysis — IR-276 function preflight PR 537"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; src/oracle/function/mod.rs, src/routed/capability.rs, src/routed/contract_package_tests.rs; IR-276 AC-5, FR-019-AC-25, FR-019-AC-29, FR-039-AC-9"
review_set: subset
---

## Summary

A planless, diff-scoped audit checked the new function preflight index against the changed tests and relevant criteria. Plan completion was not assessed. The performance control counts index construction but cannot detect the prohibited repeated lookup scan.

## Verdict

**CONDITIONAL** — add an observable control for per-item node lookup work, or otherwise make the AC-5 scan bound fail on a linear lookup regression.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Both new performance assertions count only `FunctionPreflightGraph::from_nodes` visits; replacing per-item `graph.get` calls in function matching with a linear graph scan leaves the asserted count unchanged, so the tests do not guard IR-276 AC-5's per-item scan prohibition. | src/oracle/function/mod.rs:1584-1592; src/oracle/function/mod.rs:3129-3197; src/routed/contract_package_tests.rs:1080-1186 |

## Coverage

Plan completion: not assessed

- Reconciliation: attempted `quoin matrix --repo . --json` and `quire matrix --scope . --format json`; both timed out or produced no matrix before timeout. Manual diff-scoped criterion-to-test inspection was used instead.
- Examined IR-276 AC-5: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
- Examined FR-019-AC-25 as context for supported function settlement, FR-019-AC-29 for typed-form precedence, and FR-039-AC-9 for admitted-package accounting correspondence.
- Examined the two new test functions and their three changed production paths. Existing FR-039-AC-9 and FR-019-AC-29 binders outside this diff were not re-audited.
- Reverse gap: the new indexed function preflight has an IR-276 AC-5 owner. No new production stub or copied upstream artifact was found.
- Semantic review: no repository-wide optional semantic audit; the changed criterion, tests, and source paths were compared directly for the requested PR review.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | The added counter increments on each `FunctionPreflightGraph::get` call before `self.nodes.get(node_id)`; a linear scan replacing that map access still increments once per call. The asserted `5` and `49` lookup counts remain constant for any graph size R, so the test still cannot fail on per-item O(R) lookup work. |
| FND-001 | fixed | At b4dce8ef, `IndexedNodeId::cmp` counts actual ordered-key comparisons, and `FunctionPreflightGraph::get` records the delta for each map lookup. The API and routed tests require every lookup to make at least one comparison and stay below 10/16 comparisons for their graph sizes; a linear lookup through the counted key failed the focused API bound, while the restored indexed path passed both focused tests. This supersedes round 1's still-open outcome. |

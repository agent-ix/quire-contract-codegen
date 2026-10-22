---
id: SR-15632
title: "Code and Rust review — IR-276 function preflight PR 537"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/oracle/function/mod.rs, src/routed/capability.rs, src/routed/contract_package_tests.rs; IR-276 AC-5, FR-039-AC-9, FR-019-AC-25, FR-019-AC-29"
review_set: subset
---

## Summary

Reviewed PR #537 at 3ab8578d against origin/main 7ea6a998, including graph identity, indexed lookups, routed sharing, and the two new tests. The indexing implementation has no apparent new panic, unsafe, wire, or lifetime fault. Two trace tags claim behavior their tests do not exercise.

## Verdict

**CONDITIONAL** — correct the two false criterion bindings before treating these tests as evidence for those criteria.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The synthetic graph-index test claims FR-039-AC-9, but it constructs no admitted package, excluded accounting position, context root, or correspondence refusal; its assertions cannot fail when that criterion is broken. | src/oracle/function/mod.rs:3129-3197 |
| FND-002 | medium | The mixed-package test claims FR-019-AC-29, but all requests use valid finite extent and an advertised mode, so it never tests form-refusal precedence when both form and extent or mode are unsupported. | src/routed/contract_package_tests.rs:1080-1186 |

## Rust review

The borrowed graph view is request-local and its cache key is the admitted package identity. The caller retains the borrowed package values for the negotiation lifetime. The diff adds no unsafe, panic path, unchecked numeric conversion, async or lock interaction, or CI gate change. Existing repository Rust idioms and test placement are followed. `FR-039-AC-9` and `FR-019-AC-29` are false Rust trace bindings as described above.

## Scope and evidence

- Examined IR-276 AC-5: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
- Examined FR-039-AC-9: "In an admitted package with a bound driver item, an excluded accounting-only position and another bound driver item, CG matches all three positions and exact `node_id` values to `feature_accounting`; the excluded position keeps its driver index but has no claim or generated artifact, and the following bound item keeps its original driver index. An interleaved context-only root has accounting but advances neither driver nor envelope item indexes. Removing the excluded position, adding an extra one or changing its `node_id` causes a typed whole-request correspondence refusal before any artifact."
- Examined FR-019-AC-25 as context: the normative Kani form set and legal-form settlement criterion.
- Examined FR-019-AC-29: "After request and candidate/manifest validation select one Kani candidate, Kani checks the typed IR form before its extent and advertised-mode rules; if both the form and mode/extent are unsupported, the form refusal of FR-019-AC-27 settles the item."
- Examined all changed production and test lines in `src/oracle/function/mod.rs`, `src/routed/capability.rs`, and `src/routed/contract_package_tests.rs`.
- The computed matrix command timed out in this large worktree; the changed test tags and cited criteria were checked directly. Full gates belong to the author after review.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 9895f0cf, the synthetic index test now carries only `#[trace("TC-030")]`; it no longer claims FR-039-AC-9 accounting correspondence. |
| FND-002 | fixed | At 9895f0cf, the mixed-package test now carries `#[trace("FR-019-AC-25", "TC-030")]`; it no longer claims FR-019-AC-29 precedence. |

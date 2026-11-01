---
id: SR-2026103043
title: "Code review of IR-1079 cohort relock PR 684"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1079; Cargo.lock, src/core, src/kani, src/oracle, src/replay, src/routed, tests; FR-039-AC-1/2/9/10/11"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: references
---

## Summary

Reviewed PR 684 at 0eac8b6c60e5dbd33c2b74ebfefcdaffda8da620 against freshly fetched origin/main 63b8bfb2f399895764a23287b32f25cb55cee5be. The source-bound accounting path, Kani inverse lookup, generated helper changes, coherent Contract IR/model lock, and focused QSpec reader test were examined. Three assertions in the changed contract-package test module still compare V2 indexed IDs directly with admitted V1 source-frame IDs.

## Verdict

**FAIL** — the focused 29-test contract-package module has three failures (26 pass), so the candidate cannot pass its final test gate.

## Examined scope

- FR-039-AC-1, AC-2, AC-9, AC-10, and AC-11: examined; AC-3 and AC-5: context only.
- Cargo.lock and all 34 files in the PR diff: examined for the cohort and changed call paths; generated source helpers, public claim identity, and no raw-ID fallback inspected.
- Focused `cargo fmt --check`, `git diff --check`, source-name/lock unit test, contract-package unit module, and published QSpec recursive reader test executed. The aggregate gate was not run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Three changed test assertions use V2 IDs as V1 source-frame IDs, causing focused module failures | src/routed/contract_package_tests.rs:1818, src/routed/contract_package_tests.rs:2124, src/routed/contract_package_tests.rs:2793 |

## Evidence

`cargo test --locked --lib routed::contract_package_tests::tests` reported 26 passed and 3 failed: `admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling`, `duplicate_expression_uses_each_selected_conversion_context`, and `scalar_selected_occurrences_remain_package_local`. The first compares an admitted accounting ID with `requested[0]`; the second compares an admitted checked-bound ID with a V2 bound (actual `Index(6)` versus expected `Index(13)`); the third passes `first_expression` from the V2 source to `package.source_occurrences`, gets `None`, and panics. These assertions need source-key or admitted-frame resolution before comparison. Production's `PackageItems` already uses `ContractSourceKeyV1::from_v2_request` and `source_key_at`, and the genuine function claim test passes across the two frames.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At fecd5d6482ac19fba212dc701eb9751c4757ffb0, all three assertions resolve source keys or admitted nodes before V1 queries; the focused 29-test module passes. |

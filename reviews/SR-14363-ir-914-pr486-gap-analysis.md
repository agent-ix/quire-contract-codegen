---
id: SR-14363
title: "IR-914 PR #486 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; schemas/v2-clause-coverage-observations-v1.schema.json, src/evidence/{bound_coverage,v2_llvm,mod}.rs, src/lib.rs, tests/it/{main,v2_llvm_coverage}.rs, tests/state_frame_support/native_twin.rs; FR-004-AC-16..24"
review_set: subset
---

## Summary

Ticket: IR-914. Reviewed PR #486 exact head 8bc62fe54b27e9f02a6efddebc8ee88508466c6d against main 12c280f5c1a0a5cd7c0c77e17791d5b21573cf4f. The review covers the eight changed files and FR-004-AC-16..24.

## Examined

- FR-004-AC-16 (examined): PLANNED (IR-815). The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies.
- FR-004-AC-17 (examined): PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names no `BoundPackage` or `ClauseRef`, both of which today's `analyze_bound_coverage` names.
- FR-004-AC-18 (examined): PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification, and is never treated as implication-free.
- FR-004-AC-19 (examined): PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation was observed, that has at least one implication and for which no expected consequent probe was observed is `vacuous`.
- FR-004-AC-20 (examined): PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation region has no positive execution count is `unexecuted`.
- FR-004-AC-21 (examined): PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause for which some but not all expected consequent probes were observed is `partially_exercised`.
- FR-004-AC-22 (examined): PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause with every expected consequent probe observed, and an implication-free clause with its evaluation region observed, is `exercised`.
- FR-004-AC-23 (examined): PLANNED (IR-815). Over a V2 clause population, removing one clause, one consequent probe, one evaluation probe, or every clause from the generated artifacts, each alone, prevents successful analysis.
- FR-004-AC-24 (examined): PLANNED (IR-815). Over a V2 clause population, each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification.

## Verdict

**CONDITIONAL** — eight Test-method criteria have matrix binders, but the sole AC-18 binder claims a binding outcome while exercising only the standalone analyzer. AC-17 is Inspection-method and was checked directly. No plan was supplied.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-18 test tags a binding refusal but only checks analyzer invalid input; no binding path is exercised | tests/it/v2_llvm_coverage.rs:233 |

## Coverage

- Reconciliation: `quire matrix --scope . --format json` over the exact PR head. FR-004-AC-16 and AC-18..24 have 24 total test binders; AC-17 is an Inspection method with no test binder by design.
- Plan completion: not assessed
- Semantic review: skipped (not requested).
- Reverse code-to-spec check: changed V2 analyzer, schema, and tests name FR-004-AC-16..24; no new unowned behavior or stub was identified.
- AC-18 gap: `changed_map_census_and_missing_probe_refuse_all_classifications` mutates a generated source map, calls `report()`, and asserts `invalid_input` from `analyze_v2_llvm_coverage`; it never invokes a native-run binding API or asserts the criterion's binding non-success. The PR itself says native-run binding remains separate under AC-10..15. The trace must wait for the binding implementation, or the criterion must be narrowed through spec review.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At PR #486 head d5c0b6520b2a3df4bb6dfe3a0c25cb6f14385e1f, the sole AC-18 trace was removed from the analyzer-only mismatch test; it retains AC-23, whose analysis-refusal behavior the test exercises. AC-18 remains planned and unclaimed by this PR. |

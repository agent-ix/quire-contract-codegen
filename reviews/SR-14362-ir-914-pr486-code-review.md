---
id: SR-14362
title: "IR-914 PR #486 code and Rust review"
type: SpecReview
analysis: code-review
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

**CONDITIONAL** — the Rust implementation bounds supplied bytes and JSON output and retains structured incomplete or invalid states, but one test claims full AC-18 binding evidence while exercising only analyzer refusal. The strict schema is validated for every integration-test report shape. No changed workflow, new unsafe block, panic path, or copied external artifact was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-18 trace claims binding non-success without invoking binding; the test asserts only analyzer invalid input | tests/it/v2_llvm_coverage.rs:233 |

## Code-test alignment

`changed_map_census_and_missing_probe_refuse_all_classifications` calls `report()`, which only calls `analyze_v2_llvm_coverage`; it asserts `invalid_input` and empty `clauses`. AC-18 also requires a structured *binding* non-success. The PR body explicitly defers native-run binding to AC-10..15, so this test supplies useful partial evidence but its AC-18 trace claims more than it proves. The remaining changed tests exercise real QSL-emitted checked packages, generated source/map bytes, malformed LLVM classes, and one real LLVM run.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At PR #486 head d5c0b6520b2a3df4bb6dfe3a0c25cb6f14385e1f, the sole AC-18 trace was removed from the analyzer-only mismatch test; it retains AC-23, whose analysis-refusal behavior the test exercises. AC-18 remains planned and unclaimed by this PR. |

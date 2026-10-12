---
id: SR-8641
title: "IR-496 PR 367 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@fc150dc6aa02e022ab63d4097b3eeb78c1adc1eb; spec/, src/, tests/"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Planless repository matrix and reverse-gap audit, with focused source/test inspection for PR #367 at `0ce345ff1fb66f6bd9c5a445aeb6411cb9720a4b`. The computed matrix contains 323 untagged criteria; the changed FR-021 schedule slice includes untagged charge-boundary control flow and raised source-limit/deep-body criteria.

## Verdict

**FAIL** — the computed matrix has untagged criteria, including FR-021-AC-26 and AC-28, which fail the gap-analysis gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The computed matrix has 323 untagged criteria; the changed FR-021 slice leaves AC-26 and AC-28 without a bound test | FR-021-AC-26, FR-021-AC-28 |
| FND-002 | high | The repository-wide computed matrix has 318 untagged criteria; AC-18 remains an untagged QSL-owned planned criterion outside this diff, and no changed-scope FR-021 criterion is untagged | computed matrix; FR-021-AC-18 |

## Coverage

- Reconciliation: `quire matrix --scope . --format json`, Quire 0.36.2; 738 criteria: 363 tagged, 323 untagged, 11 tagged-by-ignored-test, and 41 method-without-symbol. No Quoin run evidence was read. The untagged total includes criteria outside this PR; the review records the repository gate result and identifies AC-26/AC-28 in the changed feature area.
- Focused matrix bindings: FR-021-AC-25 and AC-27 have tagged tests in `src/oracle/function/schedule.rs`; FR-021-AC-34 has a public strict-reader test in `tests/it/exact_function_generation.rs`. FR-021-AC-26 and AC-28 are untagged. The new public schedule path also refuses literal leaves, normalizes equality identities, duplicates node/edge work charges across projection and schedule traversal, and sorts claims by name before declaration identity; these are code-review findings in SR-8640.
- Reverse-gap inventory: 122 public function declarations inventoried across `src/`; the new public `generate_function_schedules` API is owned by FR-021. Untraced behaviors: 0. Changed source stubs: 0. Changed test stubs: 0.
- Plan completion: not assessed
- Semantic review: skipped; not requested.

## Dispositions

- Changed-scope AC-26: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`; explicit charge-before-action branches and denied-charge-to-Stop behavior are covered.
- AC-28: deferred outside this scoped seed PR. The source ceiling remains unchanged, and this PR does not claim raised-limit/deep-body acceptance.
- Repository-wide matrix: the original 323-untagged-criteria FAIL is retained from the review baseline and was not recomputed for this delta. It is not evidence that this scoped PR introduces 323 gaps.

## Current-head audit record

Exact clean head `fc150dc6aa02e022ab63d4097b3eeb78c1adc1eb` was audited against base `701ac65f8b8360236b1b48ca3f549d726a180100`; production change scope is the function schedule generator and its integration/unit coverage. The computed Quoin matrix has 738 criteria: 368 tagged, 318 untagged, 11 tagged-by-ignored-test, and 41 method-without-symbol. This aggregate remains a repository-wide FAIL, with many untagged criteria outside the changed feature. The historical FND-001 count of 323 and its disposition above are preserved as the PR #367 baseline record.

**Current changed-scope assessment: PASS for the reviewed FR-021 schedule slice.** The current computed matrix tags FR-021-AC-7, AC-15, AC-24, AC-25, AC-26, AC-27, AC-28, AC-29, and AC-34. AC-18 is still untagged, but its QSL-owned agreement is explicitly PLANNED and this diff does not claim that leg. The new public integration test directly exercises AC-7/AC-28 with an authentic checked graph, a 512 KiB generation worker, default source-limit refusal, caller-raised exact source limit, exact/one-less limits, module artifact ceilings, and compilation assertions. Static review confirms the tagged test builds the graph through strict admission and invokes the public API.

The original changed-scope AC-26 disposition remains fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`; AC-28's prior deferred disposition is superseded for this current head by the new AC-28 coverage in this changed scope. The global matrix result is still FAIL; current changed-scope gaps identified: none. Reverse-gap source review: new public limits/API and module output behavior are owned by FR-021; changed source/test stubs found: 0.

- Reconciliation evidence: `quoin matrix --repo /home/peter/dev/worktrees/ir496-deep-schedule --json` (captured as `/tmp/ir496-review-fc150dc/quoin-matrix.json`) and `quire coverage --scope /home/peter/dev/worktrees/ir496-deep-schedule --json` (captured as `/tmp/ir496-review-fc150dc/quire-coverage.json`). Quoin reports no run evidence.
- Reverse-gap inventory was bounded to the changed public API, implementation and test paths; no unowned changed behavior or stubs were identified.
- Plan completion: not assessed
- Semantic review: skipped; not requested.
- No Cargo/Kani/tests/full gates were run.

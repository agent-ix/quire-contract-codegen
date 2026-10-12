---
id: SR-8640
title: "IR-496 PR 367 code review and Rust lane"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@fc150dc6aa02e022ab63d4097b3eeb78c1adc1eb; src/lib.rs, src/oracle/function/mod.rs, src/oracle/function/schedule.rs, tests/checked_package_support/rekey.rs, tests/it/exact_function_generation.rs; FR-021-AC-24..27, AC-34"
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Reviewed public PR #367 at `0ce345ff1fb66f6bd9c5a445aeb6411cb9720a4b` against `origin/main`, including the Rust lane and the changed test bindings. The flat schedule seed has bounded iterative traversal and useful public-chain coverage, but its public path miscounts generation work and omits or loses parts of the stated schedule contract.

## Verdict

**CONDITIONAL** — six medium findings affect work-limit semantics, accepted body forms, schedule information, and claim ordering.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Public generation charges every node entry and edge during both projection and schedule traversal | src/oracle/function/mod.rs:557 |
| FND-002 | medium | Literal leaves are refused, so admitted scalar and call arguments with literal operands cannot be scheduled | src/oracle/function/mod.rs:654 |
| FND-003 | medium | Boolean and integer equality operation identities collapse to the same emitted operation | src/oracle/function/schedule.rs:55 |
| FND-004 | medium | Claims sort by function name before the declaring node id required by FR-021-AC-24 | src/oracle/function/mod.rs:518 |
| FND-005 | medium | Repeated request items produce duplicate claims instead of one DuplicateRequest disposition | src/oracle/function/mod.rs:455 |
| FND-006 | medium | The emitted operation set has no branch/stop instructions for charge-boundary control flow | src/oracle/function/schedule.rs:108 |

## Dispositions

- FND-001: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`.
- FND-002: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`.
- FND-003: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`.
- FND-004: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`.
- FND-005: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`.
- FND-006: fixed at `6b61567cea79b1e246d6a87eb5941979c6e650a`.

The same-reviewer delta review of `6b61567cea79b1e246d6a87eb5941979c6e650a` found all six findings fixed. This scoped seed does not claim FR-021-AC-7's 100K schedule acceptance or AC-28.

## Current-head review record

Reviewed exact clean head `fc150dc6aa02e022ab63d4097b3eeb78c1adc1eb` against `origin/main` at `701ac65f8b8360236b1b48ca3f549d726a180100`. The diff contains `src/lib.rs`, `src/oracle/function/mod.rs`, `src/oracle/function/schedule.rs`, and `tests/it/exact_function_generation.rs`. The merged IR-764 checked-package support is base context, not a changed file.

**Current-head verdict: PASS — no new code or Rust findings.** The original FND-001..006 rows and their fixed dispositions above are historical and remain unchanged. Source review confirms the current public path meters projection node/edge work before scheduling; preserves scalar literal values and equality operation identities; sorts claims by declaration identity before names and deduplicates requests; and includes explicit charge-boundary control-flow operations. The new source-limit path checks total root-plus-module bytes before returning a generation result, enforces per-artifact and bundle ceilings, and partitions large schedules into deterministic ordinal modules. The 100,000-call tests include exact and one-less work/query/source controls, a 512 KiB worker stack, and generated-crate compilation assertions.

Current acceptance criteria examined: FR-021-AC-7 (deep 100K schedule, raised source limit and compile), AC-15 (constant-size location records and charged path query), AC-24 (claim ordering/deduplication), AC-25 (pre-action charges and all-or-nothing schedule), AC-26 (left-stop charge order), AC-27 (bound calls/cycle/shared-node refusals), AC-28 (source ceiling and raised-limit outputs), AC-29 (linear parent-chain path), and AC-34 (strict-reader chain control). AC-18 was read as context only; its QSL agreement leg remains outside this diff.

No Cargo, Kani, integration test, or other gate was run, per dispatch. This is source review only; assertions in the test source were inspected but not executed.

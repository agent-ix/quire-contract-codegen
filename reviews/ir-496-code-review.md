---
id: SR-8640
title: "IR-496 PR 367 code review and Rust lane"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@0ce345ff1fb66f6bd9c5a445aeb6411cb9720a4b; src/lib.rs, src/oracle/function/mod.rs, src/oracle/function/schedule.rs, tests/checked_package_support/rekey.rs, tests/it/exact_function_generation.rs; FR-021-AC-24..27, AC-34"
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

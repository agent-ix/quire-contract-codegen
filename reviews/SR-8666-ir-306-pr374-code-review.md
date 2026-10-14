---
id: SR-8666
title: "Code review — nested integer operand inference"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/oracle/scalar/mod.rs, tests/it/routed_generation.rs, FR-014-AC-46, FR-015-AC-104, TC-024, TC-033"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: references
---

## Summary

Reviewed the IR-306 Rust implementation and routed tests at frozen source head `7f31d612d79cbc96330729a5656e09a51f6f5666`, against FR-014-AC-46, FR-015-AC-104, TC-024 and TC-033. Four boundary cases were defective: inline nested literals, the serialized node-limit kind, memoized paths crossing the depth ceiling, and malformed reference classification.

## Verdict

**CONDITIONAL** — four medium/low findings require correction before the implementation satisfies the stated inference contract.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Recursive inference rejects inline canonical integer literals inside nested applications as `OperandUnsupported`; FR-014 requires them as singleton leaves. | src/oracle/scalar/mod.rs:1947 | implementation-bug-despite-evidence |
| FND-002 | medium | `OperandRangeLimitKind::Work` serializes as `work`, while FR-014 names this ceiling kind `nodes`; the routed test asserts the mismatched value. | src/oracle/scalar/mod.rs:603 | implementation-bug-despite-evidence |
| FND-003 | medium | A memo hit returns before checking application depth, so a reused leaf can bypass the 64-level path ceiling. | src/oracle/scalar/mod.rs:1835 | implementation-bug-despite-evidence |
| FND-004 | low | A malformed nested `reference` term is classified as `OperandUnsupported` instead of `InvalidBody` at the inference seam. | src/oracle/scalar/mod.rs:1947 | implementation-bug-despite-evidence |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Inline canonical literal arguments now produce checked singleton intervals. |
| FND-002 | fixed | The public limit kind is now `Nodes`, serialized as `nodes`. |
| FND-003 | fixed | Application depth is checked before memo lookup and counted by application node. |
| FND-004 | fixed | A malformed reference term now returns `InvalidBody` naming the enclosing malformed graph node. |
| FND-001 | fixed | Round 2 at ff62ddc: inline canonical literals still enter the recursive inference path and produce checked singleton ranges (`src/oracle/scalar/mod.rs:1953-1968`). |
| FND-002 | fixed | Round 2 at ff62ddc: `OperandRangeLimitKind` serializes snake_case and has the `Nodes` variant; the refusal carries it as `resource_kind` (`src/oracle/scalar/mod.rs:501-509,600-610`). |
| FND-003 | fixed | Round 2 at ff62ddc: application depth is checked before memo reuse (`src/oracle/scalar/mod.rs:1840-1855`). |
| FND-004 | fixed | Round 2 at ff62ddc: a malformed reference term returns `InvalidBody` with the enclosing body node id (`src/oracle/scalar/mod.rs:1970-1975`). |

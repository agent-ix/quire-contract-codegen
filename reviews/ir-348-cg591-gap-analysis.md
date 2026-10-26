---
id: SR-586008
title: "IR-348 CG591 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #591 head 5ab576da; AD-004 L-8 and affected FR-021 function paths"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Ticket: IR-348. Planless inspection of the AD-004 L-8 function slice, its computed test trace, and its source guard at PR #591 head `5ab576da`. The changed behavior remains backed by existing function tests, while the newly added architectural regression test is bypassable.

## Verdict

**CONDITIONAL** — the L-8 guard leaves a real test gap for direct raw identity reads with another binding name.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The scoped L-8 regression guard does not reject all direct function-body identity reads outside `core/ir`. | src/core/ir/frame.rs:133 |

## Coverage

Plan completion: not assessed. Examined AD-004 L-8, AD-004 step 3, FR-021 function schedule/refusal context, both changed files and the existing function tests. `quire coverage --scope . --format tsv` reported existing repository-wide status and trace gaps; no new trace tag or criterion was introduced by this PR. `quire matrix --scope . --format json` computed 40 FR-021 criteria, with 30 having at least one binder; the unbound criteria are pre-existing planned scope, not introduced by this slice. Reverse inspection found existing FR-021 ownership for the five moved reads. The new source guard checks only `operation.get(`, `operation[`, a leading `Application{operation` pattern, and `serde_json::Value::as_str`; `op.get("identity")` passes all four. Optional semantic review was not requested. Full gates were not run during this review.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At `5a6ecec4`, the scoped L-8 guard bans `.get("` independently of variable name; its positive control appends `op.get("identity")` and asserts detection. The original test gap is closed. |

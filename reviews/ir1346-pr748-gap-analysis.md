---
id: SR-2026110283874
title: "CG PR 748 inline Integer operand gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen; IR-1346; FR-018-AC-47; TC-029; src/oracle/equality/mod.rs; tests/it/inline_integer_equality.rs; tests/inline_integer_equality_support/agreement_cases.rs"
---

# SR-2026110283874: CG PR 748 gap analysis

## Summary

Ticket IR-1346; frozen PR #748 head 93649a9ba4d62124497adf6f3bfee4bdf63e5626. Planless, diff-scoped review of FR-018-AC-47, its computed trace matrix row, the new source-backed tests, and the changed generator branch. The matrix reports FR-018-AC-47 tagged by `tests/it/inline_integer_equality.rs:18`. FR-018-AC-15 is context for the unchanged conversion-reference path.

## Verdict

**CONDITIONAL** — authentic source admission, generated value agreement, and typed mismatch are exercised. The AC-47 refusal control does not inspect emitted artifacts for the criterion's required absence of a refused symbol or unchanged healthy emitted code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Refused descriptor test checks claim disposition but not absence of generated symbol or preservation of healthy emitted code | tests/it/inline_integer_equality.rs:113 |

## Coverage

- FR-018-AC-47 (examined): source-authenticated literal type, generated values, mismatch position and found ID, no refused symbol and healthy sibling.
- TC-029 inline Integer literal controls (examined): native generated test and fixed-package wrong-descriptor control.
- FR-018-AC-15 (context only): nested conversion-reference typing remains a separate existing control.
- `src/oracle/equality/mod.rs` (examined): direct `Literal.type_id` branch; reference/convert traversal preserved.
- `tests/it/inline_integer_equality.rs` (examined): actual source compilation, IR admission, generator and wrong-descriptor disposition.
- `tests/inline_integer_equality_support/agreement_cases.rs` (examined): generated native value agreement and independent Boolean expectations.
- Plan completion: not assessed.
- Optional semantic review: not requested; criterion/test/code alignment was inspected as part of the required PR code review.

## Analysis

The test identifies one generated claim and one refused claim, then compares the healthy claim's `result` with the solo baseline. A generator regression could still emit an unclaimed function for the refused descriptor, or alter the healthy function body while preserving its claim-map metadata. The AC explicitly requires no generated symbol for the refused item and an unchanged healthy sibling. The test should inspect the emitted source or compiled artifact for exactly the healthy oracle and constructor, and compare the healthy emitted function body with its solo generation where the differing context-root list permits a stable comparison. A direct native value assertion remains present and should stay.

No generated fixture or source schema was copied between repositories. The new test support file is an authored native driver that compiles the generated CG artifact in a scratch crate.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The test asserts one oracle and one environment function in emitted src/lib.rs and exact equality with the healthy baseline source. |

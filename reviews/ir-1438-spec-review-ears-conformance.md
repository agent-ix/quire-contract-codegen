---
id: SR-17940486709564847047579
title: "spec-review/ears-conformance of IR-1438 PR 764"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-049-enum-scalar-harnesses.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-1438. PR #764 at dec7b1331330face200854102bcb755ce11f9aad. EARS review of FR-049's
statement and Behavior section. `quire validate --scope .` reports two grammar warnings at line 67,
`ears:unclassifiable` and `ears:missing-subject`; both are real and stem from one compound sentence.

## Verdict

**PASS with findings.** The Description statement and the "When asserting" and "When emitting"
clauses fit the event pattern. Two low findings: the compound sentence the linter flags, and the
refusal paragraphs, which are normative but carry no SHALL and use the event form for unwanted
behaviour.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Lines 65-68 join a descriptive mapping clause, a SHALL and a SHALL NOT in one sentence; the SHALL NOT clause at line 67 has no subject of its own, which is what the two linter warnings report. Split into "The harness shall build each operand through the owner's checked member identity for its index." and "The harness shall not mint, rename or reorder members." | spec/kani/functional/FR-049-enum-scalar-harnesses.md:65-68 |
| FND-002 | low | The refusal paragraphs ("the item is refused ...", "the item is `unsupported` ...") state required behaviour with no SHALL and use "When" for unwanted conditions; EARS unwanted form is "If <condition>, then CG shall refuse ...". | spec/kani/functional/FR-049-enum-scalar-harnesses.md:78-85 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-049 | examined | When an admitted request selects an `enum` scalar comparison node ... CG SHALL generate a bounded Kani row proving that the generated enum oracle of FR-014 agrees with a native expression of the same comparison over every pair of declared members. |
| FR-049 Behavior, operand draw | examined | When the comparison is representable, CG SHALL draw each operand as one symbolic unsigned index and constrain it to `0..member_count` ... the harness SHALL build each operand through the owner's checked member identity for that index and SHALL NOT mint, rename or reorder members. |
| FR-049 Behavior, assertion | examined | When asserting, the harness SHALL compare the oracle's `Outcome<bool>` against a native expression of the operator over the two indices ... The assertion SHALL require `Completed` with the native Boolean. |
| FR-049 Behavior, refusals | examined | When two operands are drawn from different enums, or a node's operand is not an admitted member of its enum, the item is refused with a typed reason before any harness. |
| FR-049 Behavior, cover | examined | When emitting an enum harness, CG SHALL include exactly one final completion cover after all assertions. |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Sentence split into separate "The harness SHALL build ..." and "The harness SHALL NOT mint, rename or reorder members." statements; `quire validate` no longer reports the line-67 EARS warnings. |
| FND-002 | fixed | Refusals rewritten in unwanted form: "If an ordering operator is requested over an unordered declaration ..., then CG SHALL refuse the item ...", and likewise for the IllTyped and incomplete-member-list cases. |

---
id: SR-575003
title: "IR-364 PR #575 spec-review/spec-ears-analysis review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #575 head 212ca3e1b710a104ab1de9b92036646378cb5654 on base d6f09905; Behavior statements of spec/strategy/functional/FR-011-numeric-harness-campaigns.md and spec/strategy/functional/FR-002-tristate-proptest.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
---

## Summary

EARS sub-analysis of the Behavior statements that PR #575 adds or edits, at frozen head 212ca3e1.
Ticket: IR-364. `quire validate` reports 0 grammar findings over `spec/strategy/**/*.md`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-002 event statement is compound and miscounts its subject: it says the generator "shall call the planned Contract IR FR-058 accessor for each admitted package, checked clause id, and authentic `claim` occurrence tuple and take only that tuple's accepted QSL binding point". There are two responses (call, take), and "each admitted package" suggests several packages where Inputs fixes exactly one. FR-011 phrases the same rule as "with the admitted package, checked clause id, and authentic `claim` occurrence" | spec/strategy/functional/FR-002-tristate-proptest.md:89 |

## Verdict

EARS: mergeable after a low fix. Every new refusal has an If-then form: FR-011 for
`MissingClaimOccurrence`, `MissingExecutionPoint` and `ExecutionPointMismatch`, and FR-002 the same
with "Planned (IR-815):". Both no-substitution rules are well-formed ubiquitous "shall not"
statements. The FR-011 event statement ("When a V2 claim is selected, the generator shall call ...
and use only ...") forms a single call-and-use response, which is acceptable. No requirement relies
on an unbounded "etc." or on an undefined term.

## Dispositions

Round 1, checked against the repository at head 1462d1197a0287bfae9766a9aec9aeecdfa1348f (Contract IR #558 merged at f1079483; CG #574 at 39b212d7).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Split into single-response statements: 'shall call the planned Contract IR FR-058 accessor for the one admitted package and each selected checked clause id and `claim` occurrence tuple' and 'shall take each execution point only from the accepted QSL binding point returned for that exact selection'. |

## Coverage

```yaml
scope:
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "When a V2 precondition and postcondition are selected, the V2 harness generator shall call the planned Contract IR FR-058 accessor for each admitted package, checked clause id, and authentic `claim` occurrence tuple and take only that tuple's accepted QSL binding point (planned, IR-815)."
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "If the selected occurrence is not an authentic `claim` of that checked clause in that package, then the generator shall return `StrategyErrorCode::MissingClaimOccurrence` and no artifact. The generator shall not substitute another occurrence of the same checked clause node for the selected occurrence."
```

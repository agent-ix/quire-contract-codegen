---
id: SR-578003
title: "IR-364 PR #578 spec-review/spec-ears-analysis review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #578 head 33116e82f60b1f9e0affff3f6bad89530df8859a on base b60b18ef; Behavior statements of spec/strategy/functional/FR-002-tristate-proptest.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
---

## Summary

EARS sub-analysis of the one FR-002 Behavior statement that PR #578 rewrites, at frozen head
33116e82. Ticket: IR-364.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The rewritten unwanted-behaviour bullet is compound. It packs three subjects into one bullet: the V2 harness, the generated campaign runner, and "the supplied runtime report", which is a data object rather than a system. It makes six or more responses and ends with a descriptive non-requirement sentence ("Its `attempted` count continues to describe ..."). Split it into single If-then and shall-not statements, one subject each | spec/strategy/functional/FR-002-tristate-proptest.md:88-96 |

## Verdict

EARS: mergeable after a low fix. The leading clause keeps the If-then unwanted-behaviour form
("if an embedded V2 clause oracle returns an outcome that does not complete, then the V2 harness
shall return ..."). The negative responses are well-formed "shall not" clauses, and no term is
unbounded. The untouched AC-17/18 Behavior bullet ("shall neither invoke the subject nor evaluate the
postcondition oracle") is unchanged.

## Dispositions

Round 1, checked against the repository at head 6c4f1f1623d94d49744d14bdc2129cbcd5435bd4 (merge-base main b60b18ef). FR-002-AC-17, AC-18 and AC-19 rows are byte-identical to main; only AC-16 changed, and it is not narrowed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The bullet is split into six single-subject statements: two If-then statements for the harness (`Ok` stops, `Err` stops), one If-then statement for the campaign runner's `NonCompleted` result, and three "for a non-completed case ... shall not / shall retain" statements. The descriptive `attempted` sentence moved to Outputs. |

## Coverage

```yaml
scope:
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "if an embedded V2 clause oracle returns an outcome that does not complete, then the V2 harness shall return `V2HarnessCaseResult::NonCompleted` ... The generated campaign runner shall terminate with this non-success result; it shall not construct or record a runtime verdict for the stopped case, emit a pass artifact, count the case as passed, or convert it into a precondition rejection, postcondition failure, explicit discard, or framework exhaustion. The supplied runtime report shall retain its prior verdict counters. Its `attempted` count continues to describe recorded verdicts and explicit discards only"
```

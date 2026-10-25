---
id: SR-575002
title: "IR-364 PR #575 spec-review/spec-integrity-analysis review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #575 head 212ca3e1b710a104ab1de9b92036646378cb5654 on base d6f09905; spec/strategy/functional/FR-011-numeric-harness-campaigns.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/TC-020-numeric-harness-campaigns.md, spec/strategy/matrix/TC-004-proptest-strategies.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
---

## Summary

Integrity sub-analysis of PR #575 (consistency, completeness and atomicity) at frozen head
212ca3e1, diffed word by word against main d6f09905. Ticket: IR-364. The PR body asks for a check
of the FR-002-AC-12 identity interpretation; that check is FND-001. `quire validate` over
`spec/strategy/**/*.md` passes, with 16/16 docs grammar-clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-002 Inputs redefines "selected clause identity" to include the authentic claim occurrence so that FR-002-AC-12 can stay verbatim. The same term is in FR-002-AC-10 ("A selected clause identity that is unknown ... yields a typed, located diagnostic"), so an unknown occurrence now falls under both AC-10's diagnostic and AC-20's `MissingClaimOccurrence`, and neither says which wins. For AC-12 the narrowing is defensible, because a clause id alone no longer picks one point. It is still carried by Inputs prose, outside any AC, and per SR-575001 FND-001 it does not change pre or post points. State the definition once (or limit it to AC-12) and give AC-10's unknown identity and AC-20's wrong occurrence a precedence | spec/strategy/functional/FR-002-tristate-proptest.md:27-28; spec/strategy/functional/FR-002-tristate-proptest.md:129; spec/strategy/functional/FR-002-tristate-proptest.md:131; spec/strategy/functional/FR-002-tristate-proptest.md:139 |
| FND-002 | low | FR-011-AC-6 changed "rendered from that node's accepted QSL `ClauseBinding.execution_point`" to "rendered from the accepted QSL binding of that exact tuple", which drops the named member. The verdict point is rendered from the binding's `execution_point`, not from the binding as a whole, and FR-002-AC-19 in this same PR keeps `ClauseBinding.execution_point` | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:145 |

## Verdict

Integrity alone: mergeable after FND-001. Checked clean: FR-002-AC-12 is byte-identical to main.
AC-19, AC-20, AC-6 and AC-7 are strengthened or partitioned, never narrowed. The old single
"absent point" case is now split exactly between `MissingClaimOccurrence` (inauthentic occurrence)
and `MissingExecutionPoint` (authentic tuple with no point), with no gap between them. The Outputs
refusal lists in FR-002 and FR-011 match their Behavior and ACs. Every new AC clause has a TC step
and an expected result. The Dependencies sections cite FR-058 and IR-1013 as planned and claim no
implementation credit.

## Dispositions

Round 1, checked against the repository at head 1462d1197a0287bfae9766a9aec9aeecdfa1348f (Contract IR #558 merged at f1079483; CG #574 at 39b212d7).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Inputs now limits the occurrence reading to FR-002-AC-12 and says AC-10's clause identity checks retain their original scope; a new Behavior bullet and AC-20 ('After FR-002-AC-10's clause id, kind, and distinctness checks') make AC-10 precede MissingClaimOccurrence; TC-004 combines unknown/wrong-kind ids with wrong occurrences. AC-10 and AC-12 rows byte-identical to main d6f09905. |
| FND-002 | fixed | FR-011-AC-6 reads 'rendered from the accepted QSL `ClauseBinding.execution_point` of that exact tuple'. |

## Coverage

```yaml
scope:
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "A selected clause identity in the V2 harness includes its authentic claim occurrence, so repeated selection in FR-002-AC-12 names the same full tuple."
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or the same for both selections yields a typed, located diagnostic naming the selection and no artifact."
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities and campaign policy is byte-identical. The requirement and revision identity that the generated header, `RequirementId` and symbols carry is not an input here and waits on OQ-1."
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Each V2 selection preserves the accepted QSL `ClauseBinding.execution_point` for its exact admitted package, checked clause id, and authentic `claim` occurrence through the injective runtime rendering"
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "a wrong-role, wrong-ordinal, or other-node `claim` occurrence yields `HarnessErrorCode::MissingClaimOccurrence` and no artifact even if a sibling occurrence has a point; an authentic selected tuple with no accepted point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact"
- id: FR-011-AC-6
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "every constructed verdict carries the runtime point rendered from the accepted QSL binding of that exact tuple, with variant and anchor name preserved."
- id: FR-011-AC-7
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "A wrong-role, wrong-ordinal, or other-node `claim` occurrence returns `StrategyErrorCode::MissingClaimOccurrence` and no artifact, even when another authentic occurrence of the selected node has a point."
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: "Repeating either full selection with the same package and policy is byte-identical (FR-002-AC-12)."
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: examined
  excerpt: "The same checked node's two authentic claim occurrences produce distinct points with no node-level reuse."
```

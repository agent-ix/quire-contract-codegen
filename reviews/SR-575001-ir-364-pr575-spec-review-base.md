---
id: SR-575001
title: "IR-364 PR #575 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #575 head 212ca3e1b710a104ab1de9b92036646378cb5654 on base d6f09905; spec/strategy/functional/FR-011-numeric-harness-campaigns.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/TC-020-numeric-harness-campaigns.md, spec/strategy/matrix/TC-004-proptest-strategies.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
---

## Summary

Non-author base spec review of draft PR #575 at frozen head 212ca3e1, scoped to its diff. Ticket:
IR-364. Compared word by word with merged main d6f09905 and checked against open Contract IR PR
#558 (FR-058 at 095a9bdc), open CG PR #574 (FR-004 at b4876d1f) and QSpec FR-341 on
quire-specification main (state clause body: a precondition's anchor is its operation's
`state`/`operation_anchor` node and is part of the body).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-002 two-claim control needs one precondition node whose two authentic claims bind distinct `Pre` operation names. Under QSpec FR-341 a precondition's anchor (its operation's `operation_anchor` node) is a body argument, so preconditions on different operations have different bodies and node ids; a QSL-produced package cannot hold that fixture, and FR-058-AC-1 gives a synthetic package no producer-origin credit. The new AC-19 clause and the TC-004 expectation of distinct points and artifacts on one precondition node cannot be met by an admitted producer package. Same-node distinct points belong to invariant and constraint claims (FR-011, FR-058-AC-2). For pre and post nodes, use two authentic claims sharing one `Pre`/`Post` point to exercise occurrence selection, refusals and AC-12 byte identity | spec/strategy/functional/FR-002-tristate-proptest.md:26-28; spec/strategy/functional/FR-002-tristate-proptest.md:138; spec/strategy/matrix/TC-004-proptest-strategies.md:41-46; spec/strategy/matrix/TC-004-proptest-strategies.md:79-80 |
| FND-002 | medium | The no-sibling-substitution rule is observed only with inauthentic occurrences, which FR-058 already refuses upstream as `MissingClaimOccurrence`. The case where substitution would actually happen, an authentic claim with no accepted point beside an authentic sibling that has one, is not required: AC-7, AC-20, TC-020 step 8 and TC-004 supply "an authentic selected tuple lacking its accepted binding point" without a pointed sibling. A generator that falls back to the node's other claim passes. Require that fixture and `MissingExecutionPoint` (as FR-058-AC-4 does) | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:146; spec/strategy/functional/FR-002-tristate-proptest.md:139; spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:45-49; spec/strategy/matrix/TC-004-proptest-strategies.md:47-49 |
| FND-003 | medium | Conflict with open CG #574 FR-004 at b4876d1f. FR-004-AC-25 and its Behavior refuse only `MissingExecutionPoint` and `ExecutionPointMismatch` and never name `MissingClaimOccurrence`, yet AC-25 cites TC-020 steps 7-8, which this PR rewrites to expect `MissingClaimOccurrence` for wrong occurrences. #574 also says `CheckedNodeId` where #575 says "checked clause id". Whichever lands second must add `MissingClaimOccurrence` to FR-004 and align the terms | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:146; spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:45-49 |
| FND-004 | low | FR-058's closed refusal set is `UnknownClause`, `NotStateClause`, `MissingClaimOccurrence` and `MissingExecutionPoint`. FR-011 maps only the last two to `StrategyErrorCode` and does not say that `UnknownClause` and `NotStateClause` are unreachable because of earlier selection checks, or which CG code they become. FR-002 covers this through AC-10; FR-011 does not | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:65-67; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:78-87 |

## Verdict

NOT MERGEABLE (one high, two medium). What holds: FR-002-AC-12 is byte-identical to main. FR-011
and FR-002 name FR-058's key (admitted package, checked clause id, authentic `claim`
`CheckedOccurrence`). `MissingClaimOccurrence` matches FR-058's refusal of the same name and its
wrong-role, wrong-ordinal and other-node triad (FR-058-AC-3). Substituting a sibling is forbidden,
and there is still no caller point, no clause-kind default and no inferred path. No QSL or IR type
is copied: `ClauseBinding`, `CheckedOccurrence` and `ExecutionPoint` are named with their owners.
No compatibility layer is added. FR-011-AC-6 and TC-020 step 7 do observe the two-claim invariant
node, matching FR-058-AC-2. The AC-12 identity question is covered in SR-575002.

## Dispositions

Round 1, checked against the repository at head 1462d1197a0287bfae9766a9aec9aeecdfa1348f (Contract IR #558 merged at f1079483; CG #574 at 39b212d7).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-004 now selects two authentic claims on one precondition node sharing its same `Pre` point and instruments the accessor per occurrence; FR-002-AC-19 says both retain that node's accepted `Pre` point; FR-002 Inputs states shared Pre/Post points per QSpec FR-341. Same-node distinct points remain only for invariants (FR-011-AC-6, TC-020 step 7). |
| FND-002 | fixed | FR-011-AC-7 and FR-002-AC-20 add 'even when another authentic occurrence of that node has a point' to MissingExecutionPoint; TC-020 step 8 and TC-004 select an authentic claim lacking a point beside a pointed authentic sibling and require MissingExecutionPoint with no substitution. |
| FND-003 | fixed | Resolved on the #574 side at 39b212d7: FR-004-AC-25 now uses 'checked clause id' and delegates wrong-role/ordinal/node, no-point and mismatch refusals to 'the corresponding typed no-artifact refusal of FR-011-AC-7', so MissingClaimOccurrence is covered; #574 touches TC-020 frontmatter only, no textual conflict with #575. |
| FND-004 | fixed | FR-011 Behavior and AC-7 map FR-058 UnknownClause to the existing StrategyErrorCode::UnknownClause and NotStateClause to StrategyErrorCode::UnsupportedClauseKind before occurrence checks, no artifact; TC-020 step 8 and its expected results observe both. Order matches merged FR-058-AC-3 (f1079483). |

## Coverage

```yaml
scope:
- id: FR-011-AC-6
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "For a selected checked clause id and authentic `claim` occurrence in one admitted V2 package, every constructed verdict carries the runtime point rendered from the accepted QSL binding of that exact tuple, with variant and anchor name preserved. ... Two authentic claim occurrences on one invariant node with identical clause kind and body anchor but different accepted points yield their distinct rendered points."
- id: FR-011-AC-7
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "A wrong-role, wrong-ordinal, or other-node `claim` occurrence returns `StrategyErrorCode::MissingClaimOccurrence` and no artifact, even when another authentic occurrence of the selected node has a point. An authentic selected tuple lacking an accepted binding point returns `StrategyErrorCode::MissingExecutionPoint` and no artifact."
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "When a V2 claim is selected, the generator shall call the planned Contract IR FR-058 typed accessor with the admitted package, checked clause id, and authentic `claim` occurrence, and use only the returned accepted QSL binding point."
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities and campaign policy is byte-identical."
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Each V2 selection preserves the accepted QSL `ClauseBinding.execution_point` for its exact admitted package, checked clause id, and authentic `claim` occurrence through the injective runtime rendering; precondition and postcondition points may differ, and two requests selecting different authentic occurrences of the same checked clause node preserve their distinct points."
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "For either selected V2 clause, a wrong-role, wrong-ordinal, or other-node `claim` occurrence yields `HarnessErrorCode::MissingClaimOccurrence` and no artifact even if a sibling occurrence has a point; an authentic selected tuple with no accepted point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact"
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: examined
  excerpt: "Independently select a wrong-role, wrong-ordinal, and other-node occurrence against a checked clause with another authentic claim that has a point; verify that the sibling point cannot satisfy the lookup. Supply an authentic selected tuple with its accepted binding point absent and inspect the generation refusal."
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: "For two separate requests, select the same precondition checked clause node with two authentic `claim` occurrences bound to distinct `Pre` operation names, keeping the postcondition selection fixed"
- id: FR-058
  path: "agent-ix/quire-contract-ir PR #558 spec/checked_package/functional/FR-058-admitted-clause-execution-point.md"
  role: context_only
  excerpt: "On failure, `CheckedExecutionPointError` with a stable, closed `CheckedExecutionPointRefusal` code: `UnknownClause`, `NotStateClause`, `MissingClaimOccurrence`, or `MissingExecutionPoint`."
- id: FR-004-AC-25
  path: "agent-ix/quire-contract-codegen PR #574 spec/evidence/functional/FR-004-vacuity-evidence.md"
  role: context_only
  excerpt: "An absent point for the selected tuple returns `StrategyErrorCode::MissingExecutionPoint`; a rendered runner point differing from that selected binding returns `StrategyErrorCode::ExecutionPointMismatch`."
- id: FR-341
  path: "agent-ix/quire-specification spec/objects/interfaces/FR-341-checked-package-state-clause-body.md"
  role: context_only
  excerpt: "the anchor — a reference to the node the clause is anchored at: ... the operation's state/operation_anchor node (FR-342) for a precondition or postcondition"
```

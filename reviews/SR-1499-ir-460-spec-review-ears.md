---
id: SR-1499
title: "EARS conformance review of quire-contract-codegen PR 276 (IR-460 state-clause replay spec)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@d22cc5d849db3edd7bba68c3638bb9c4995ac723; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md (PR #276 diff vs origin/main 7345463; QSL qsl-replay locked at c8f0c28)"
review_set: subset
---

## Summary

Ticket: IR-460. EARS check of the nine new FR-024 Behavior bullets and the edited Description sentence. The event-driven trigger ("When a counterexample is a postcondition state-clause counterexample"), the two unwanted-behaviour bullets ("If the generator cannot supply...", "If the generator reports Incomplete...") and the ubiquitous "return ... unchanged" and "give each snapshot every declared field" bullets conform. Two statements do not.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "...and shall repeat neither in the payload (QSL FR-122)" cannot be violated. StateClauseCounterexample at c8f0c28 has only clause, observation and witness, so the type cannot carry a node or occurrence. The second shall is a vacuous requirement. Reword it as rationale, or drop it. | FR-024 Behavior line 99, FR-024-AC-12 |
| FND-002 | low | Three bullets are compound (payload clause plus observation; envelope clause_node and occurrence_key plus the payload clause; pre snapshot plus post snapshot from two different sources). "Preconditions (PreCall) and invariants (Current) are not specified by this requirement." is a scope note placed in the Behavior list as if it were a requirement. Split the compounds, and move the scope note to the Description or a Scope paragraph. | FR-024 Behavior lines 97-104, 109 |

## Dispositions

Round 1, reviewed at 02a5f67616fed74c0c014d546e2938e555791968.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: the occurrence_key bullet now ends "The payload carries neither identity (QSL FR-122), so there is no second copy to keep equal." That is rationale, not a second shall. |
| FND-002 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: the clause/observation, clause_node/occurrence_key and pre/post bullets are each split into single-shall bullets, and the PreCall/Current note moves to a new Scope section. |

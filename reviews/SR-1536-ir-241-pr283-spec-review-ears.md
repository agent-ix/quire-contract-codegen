---
id: SR-1536
title: "EARS conformance review of quire-contract-codegen PR #283"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@1f3025ea99c3fe1fa804ed43fce116e339f9c96d; spec/kani/functional/FR-028-bounded-proof-ceilings.md (Behavior), spec/kani/functional/FR-015-bounded-kani-obligations.md (Behavior, IR-264), spec/oracle/functional/FR-018-composite-equality-oracles.md (Behavior, Planned bullets)"
review_set: subset
---
# EARS conformance review of PR #283

## Summary

Ticket: IR-241 (also IR-264). This review covers the Behavior statements the PR adds: 16 bullets in
FR-028, 7 in FR-015 and 2 in FR-018. quire's EARS lint finds nothing new (6 warnings at base, the
same 6 at head, all in FR-024). Reading each statement by hand, almost all are well-formed
ubiquitous, event (`When`), unwanted (`If ... then`) or optional (`Where`) statements with the
generator as subject. Two are not.

## Scope examined

- FR-028 Behavior bullets referencing FR-028-AC-13 to AC-23 (examined).
- FR-015 IR-264 Behavior bullets referencing FR-015-AC-69 to AC-76 (examined).
- FR-018 Planned bullets referencing FR-018-AC-21 and AC-22 (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-015 statement "When the installed backend runs the corpus case's harness, the generator shall classify it `Verified` ... and shall classify each seeded mutant of the shadow `Falsified`" has two `shall`s. It also requires an outcome that an external tool produces, which is a test expectation and not a response of the system. It should be split into the classification rule and one statement per expected outcome, or the expected outcomes left to the ACs. | spec/kani/functional/FR-015-bounded-kani-obligations.md, IR-264 Behavior bullet (FR-015-AC-74, FR-015-AC-75) |
| FND-002 | low | The FR-018 Planned bullet "each emitted oracle shall complete every pair ..." makes the generated artifact the subject, not the generator. It also states a universal property ("every pair of operand values its declared types admit") as a requirement. EARS form would be "The generator shall emit oracles that ...", with the universal domain narrowed as SR-1537 FND-003 describes. | spec/oracle/functional/FR-018-composite-equality-oracles.md, Planned bullet (FR-018-AC-22) |

## Verdict

EARS conformance is good across the diff. The two low findings are wording.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 3e0bc7c | Split into two single-`shall` generator statements (corpus harness Verified with an exhaustive refinement; Falsified for each shadow mutant). |
| FND-002 | fixed 3e0bc7c | The subject is now the generator ('the generator shall emit ... an oracle that completes'), over the refinement domain, not a universal domain. |

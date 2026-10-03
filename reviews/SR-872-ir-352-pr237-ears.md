---
id: "SR-872"
title: "CG PR 237 spec review (EARS conformance): FR-021 new Behavior bullets"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@6d0e02a4770c6b77bc4b08ea46a3978015c6c1bc; spec/oracle/functional/FR-021-function-application-oracles.md:176-187 (two new Behavior bullets), compared with the existing FR-021 bullets (When/If ... the generator shall ...)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

# SR-872: CG PR 237 spec review, EARS conformance

## Summary

Ticket: IR-352. Existing FR-021 Behavior bullets use event-driven ("When ..., the generator shall
...") or unwanted-behaviour ("If ..., then the generator shall refuse ...") form, one obligation
each, with rationale kept short. The new bullets are checked against that house form.

## Verdict

Both bullets are readable and correct in intent, but each packs two obligations and one of them
carries a normative claim without "shall". Low severity; fix in the same edit as SR-870 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The first bullet's only "shall" is the macro ban. The catch-all obligation ("matches the four variants ... and returns any other variant ... as `Outcome::Refused(Refusal::CheckedInvariant)`") sits inside a rationale sentence without "shall", so the bullet is compound and the second requirement is unstated as one. Split it: a ubiquitous ban, then "If a runtime operator returns an `Outcome` variant the generator does not know, then the lowered body shall return `Outcome::Refused(Refusal::CheckedInvariant)`", with the reason after it. | spec/oracle/functional/FR-021-function-application-oracles.md:176-183 |
| FND-002 | low | The second bullet joins an unwanted-behaviour trigger ("If lowering a function body meets an integer operator that is not ... `Add`, `Subtract` or `Multiply`") to a constraint on the generator's own source that does not depend on the trigger. Make it two statements, and name the refusal (`ExactFunctionRefusal::UnsupportedOperator`) rather than "the typed generation refusal", which no other FR-021 text defines. | spec/oracle/functional/FR-021-function-application-oracles.md:184-187 |

## Dispositions

Round 1, reviewed at cac5002cc137ad6297def98b225b9394b0673fef.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (split into a ubiquitous emitted-source ban, a When-shaped arm-mapping bullet and a When-shaped unknown-variant bullet, each with its own shall) |
| FND-002 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (If-shaped Negate bullet names ExactFunctionRefusal::UnsupportedOperator and omission from checked_package(); the source-text constraint is its own ubiquitous bullet) |

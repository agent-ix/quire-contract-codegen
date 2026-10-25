---
id: SR-580003
title: "IR-595 PR 580 checked-closure facade re-point EARS analysis"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@08c68e3ac71b47e1fedae1c017891ac150053a89; spec/oracle/functional/FR-021-function-application-oracles.md, spec/assurance/AD-004-cg-crate-layout.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

## Summary

Ticket: IR-595. PR: quire-contract-codegen#580 at 08c68e3a. This pass checks requirement-grammar
conformance of the edited FR-021 statements and the re-pointed acceptance criteria. It also
examines the author-reported grammar-shape warning.

## Examined scope

- FR-021 Description edit ("CG shall use only the FR-485 facade ..."): examined. It is a
  ubiquitous `shall` with an unchanged obligation.
- New FR-021 Behavior bullet ("For a matching `CallableSelection`, the concrete oracle
  shall ..."): examined (FND-003).
- FR-021-AC-32: examined. A direct assertion.
- FR-021-AC-35: examined. A direct assertion.
- FR-021-AC-36: examined. This is where `quire validate` reports the warning (FND-002).
- FR-021-AC-37: examined (FND-001).
- AD-004 seam paragraph and TC-031 12b: context only, because they are not requirement
  statements.

## Verdict

PASS WITH LOW FINDINGS. The author reported the `ac:non-canonical-shape` warning as AC-37.
`quire validate` puts it at line 330, which is AC-36. It fires on AC-36's "... to
`ApplicationRegistry::register`, then passes ...", which the heuristic reads as
given-when-then. AC-36 is a direct assertion of an ordered mapping, so this is not a
correctness or testability defect. It is a low wording nit that one reword clears. AC-37's real
grammar problem is its "between ... respectively" construction, which the validator does not
flag.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-37 reads "or between `CallableSelection::owner_package_id()` and read-only check-minted key bytes and the selected function's declaring admitted V2 package id and function node digest respectively". That is four nouns chained by "and" inside "between ... and", resolved only by "respectively". The pairing (owner to declaring package id, key bytes to function node digest) is recoverable but easy to misread. Stating two explicit pairs removes the ambiguity. | spec/oracle/functional/FR-021-function-application-oracles.md:331 |
| FND-002 | low | `quire validate` warns `ac:non-canonical-shape` (given-when-then-shaped) at line 330, which is AC-36, not AC-37 as the PR body says. The cause is the procedural "... to `ApplicationRegistry::register`, then passes that same selection ... to `begin_call`". This is a heuristic hit on a sequence, not a semantic defect. Phrasing the order as an outcome (for example "registers ... and passes that same selection ... to `begin_call` only after registration is `Accepted`") clears it. | spec/oracle/functional/FR-021-function-application-oracles.md:330 |
| FND-003 | low | The new Behavior bullet joins four obligations: create and register, require `Accepted`, pass to `begin_call`, and step or run. It adds a negative ("CG shall not dispatch") and a factual coda, and it opens with "For a matching `CallableSelection`" rather than the EARS "When"/"If" trigger used by the neighbouring bullet ("If the FR-485 acquisition's ... differs ..."). As one compound statement it cannot be cited one obligation at a time. | spec/oracle/functional/FR-021-function-application-oracles.md:277-289 |

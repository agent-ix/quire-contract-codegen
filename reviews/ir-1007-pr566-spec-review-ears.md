---
id: SR-35013
title: "EARS conformance review of IR-1007 CG PR 566 (FR-021 changed statements)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-021-function-application-oracles.md"
review_set: subset
---

## Summary

Ticket: IR-1007. PR agent-ix/quire-contract-codegen#566 at fd2a03a69a75, diff
only. Engine check: `quire validate --scope . <FR-021> --summary` reports
"1/1 docs grammar-clean (100%); 0 grammar finding(s)". Semantic pass over the
changed statements (Description paragraph, new Inputs bullet, Outputs,
Constraints, Out of Scope).

Clean: "CG shall use only the FR-485 facade for checked-closure acquisition
and application" (ubiquitous, singular, concrete), the Outputs and
Constraints re-wordings, and the Out of Scope bullet.

Defect: the new Inputs bullet packs three obligations with three different
subjects into one statement, and one subject is a ticket rather than the
system.

## Scope examined

- FR-021 Description, re-pointed paragraph (lines 40-52): examined; it is status prose with one ubiquitous SHALL, which is clean.
- FR-021 Inputs, new bullet (lines 79-86): examined.
- FR-021 Outputs bullets (schedule-only output, claim map): examined; clean.
- FR-021 Constraints bullets (schedule crate, concrete package admission): examined; clean.
- FR-021 Out of Scope, first bullet: examined; clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new FR-021 Inputs bullet is non-singular. It holds three `shall` obligations with three subjects: "the caller shall supply ...", "CG shall not reconstruct ...", and "The IR-595 consumer shall bind ...". The last names a ticket as the system under requirement, and its unwanted-behaviour clause ("a mismatch refuses that item") should be an `If <mismatch>, then the generator shall ...` statement. Split it into atomic statements and give each its own AC (see SR-35012 FND-001). | spec/oracle/functional/FR-021-function-application-oracles.md:79-86 |

## Verdict

One medium wording finding. Fix it together with SR-35012 FND-001.

## Dispositions

### Round 1 — exact `221c5514399aba606d0c383e433bbc4e8f61309e` (fix diff `fd2a03a6..221c5514`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Inputs bullet no longer holds three SHALLs. The caller inputs and the identity mapping are now descriptive Inputs text backed by AC-35 and AC-36, and the ticket ("The IR-595 consumer shall") is no longer the subject. The unwanted-behaviour clause is a singular If-then requirement in Constraints with the system as subject: "If the FR-485 acquisition's checked owner package id or selected callable id differs from the requested admitted V2 package/function identity, the generator shall refuse that item as `OwnerIdentityMismatch` before body registration, Begin or any application charge." The engine EARS check reports FR-021 grammar-clean. |

No new findings this round.

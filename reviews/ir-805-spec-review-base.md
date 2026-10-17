---
id: "SR-12100"
title: "IR-805 spec-review/base review of reference-graph reachability Kani claims"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@edbe6aabdd756a864482c2b73b1c0c113b101894; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-12100: IR-805 spec-review/base review

## Summary

Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.
Method: spec-review/base (checklist, ID formats, coverage rules, hash/pin antipattern).
Diff reviewed: `git diff origin/main...HEAD`. Ground truth read via `git show` at
quire-spec-language 46aa650be and quire-contract-ir 64f884e7 / 299a126.

## Scope

- FR-015 description :109-148 (examined): graph-half paragraph, claim forms, typed IR inputs, pin.
- FR-015 Inputs :204-210 (examined): symbolic graph item inputs.
- FR-015-AC-98 (examined), FR-015-AC-99 to FR-015-AC-102 graph clauses (examined), FR-015-AC-105 to FR-015-AC-107 (examined).
- FR-015-AC-96, FR-015-AC-97 (context_only): IR-268 collection half, unchanged.
- TC-025 graph-family paragraph and "Open upstream inputs" (examined).
- tests.md :94 and :95 TC-025 rows (examined).

ID formats are consistent (AC-105..107 follow AC-104 with no gap or duplicate); every new AC is
listed on the tests.md :95 row; :94 does not list any graph AC and is consistent; quire validate
passes on the three files. Open PRs #403/#406/#409/#410/#412 edit other tests.md rows only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Revision pins in requirement prose: FR-015 :140-148 pins the CG lock's quire-contract-ir 38f3581, 64f884e7, 299a126, qsl lock 23f9f34, #411's 2a316c0 and 46aa650be ("the code phase needs ... at or after"); TC-025 :104 repeats a "Code-phase pin"; tests.md :95 status says "awaits a CG pin of quire-contract-ir at or after 64f884e7 and qsl-* at or after 46aa650be". Checklist hash/digest/pin antipattern: package revisions live in Cargo.toml/Cargo.lock only. Fix is removal; state the capability needed (IR reaches admission of multi-valued ends, FR-053 select_population) instead of commits. | spec/kani/functional/FR-015-bounded-kani-obligations.md:140, spec/kani/matrix/TC-025-bounded-kani-obligations.md:104, spec/kani/matrix/tests.md:95 |

## Verdict

Revise before accept. The pinned facts are themselves accurate (38f3581 is an ancestor of
64f884e7, 64f884e7 is in 299a126, 23f9f34 and 2a316c0 predate 46aa650be), but the checklist rule
is removal regardless of accuracy. All node identities and IR names the spec binds were checked
and exist as written (see the failure-domain review SR-12101 for the substantive defects).

## Dispositions

Reviewed at e1873b156ea042a0b76fedf2a804becf17ac89e0 (round 1; fix commits d088e9c, e1873b1).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d088e9c..e1873b1: every commit pin removed from FR-015 :109-148, TC-025 and tests.md :95; `grep` over spec/ finds none of the six revisions. |

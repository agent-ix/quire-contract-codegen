---
id: SR-794003
title: "IR-1518 PR #794 spec-review of status and evidence edits"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #794 at 1da7a721; IR-1518; spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-48, AC-135, AC-136), spec/kani/matrix/TC-025-bounded-kani-obligations.md (step 14 note, steps 52/53 heading), spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1518. The PR changes status and evidence text only: AC-135 and AC-136 move from PLANNED
to Implemented; AC-48's residual-gap sentence now leaves only AC-133 (IR-1517) open and states
AC-134/135/136 implemented; TC-025 step 14 and the steps 52/53 heading mark 53 implemented and 52
planned; the matrix index row for AC-38..49 says the same. The edits agree with each other and with
the code and tests (see SR-794001/SR-794002). The normative text of AC-135/136 and step 53 is
unchanged. `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 with
the same 6 warnings as main (none on a changed line). Sub-analyses (EARS, integrity, dependency) not
run: no requirement statement, structure or relationship edge changed.

## Verdict

**PASS (mergeable).** One low staleness finding in the AC-135 row.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-135 now reads "Implemented" but keeps the forward-looking fallout sentence ("are refused once this lands and must carry the clause node's resolved key", naming `fixture_obligation`, which still builds an arbitrary key for two call sites that bypass classification) and names no evidence test, unlike the AC-134 row | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-135 | examined | Implemented (IR-1518; spec IR-1515; refs FR-015-AC-48, FR-015-AC-136 and AD-003 E-1). A V2 clause claim whose ObligationIdentity NodeKey is not equal ... is refused with UnsupportedObligation::V2ClauseNodeKeyMismatch ... |
| FR-015-AC-136 | examined | Implemented (IR-1518; spec IR-1515; refs FR-015-AC-135 and FR-015-AC-48). ... refused as V2ClauseNodeKeyMismatch even when its record also differs ... |
| FR-015-AC-48 | examined | ... One gap stays open and is specified by FR-015-AC-133 (element domain of a one-level sequence field; code IR-1517). The unread-drawn-field gap is closed: FR-015-AC-134 is implemented, and so are FR-015-AC-135 and FR-015-AC-136 ... |
| TC-025 step 14 / steps 52-53 heading | examined | The sequence-field element domain and the NodeKey binding are specified by steps 52 (planned) and 53 (implemented, IR-1518) below. |
| spec/kani/matrix/tests.md AC-38..49 row | examined | ... leaves only AC-133 (code IR-1517) open, AC-135 and AC-136 being implemented by IR-1518 ... |

---
id: "SR-1544"
title: "IR-624 EARS conformance analysis (CG #285)"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@82ba95ff370dc5b6caac85d1623f7764a0f2ac1d; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md (PR #285 diff against main 13fc2d0)"
---

# SR-1544: IR-624 EARS conformance analysis

## Summary

Ticket: IR-624. Checked the new and edited requirement statements in FR-015's IR-624 section
and in FR-024's Inputs and Behaviour. `quire validate` reports no new EARS warning: head and
main have the same 6, all pre-existing (FR-017:152, FR-024's MissingField and UndeclaredField
bullets). The new "When ... shall", "If ... then ... shall", "Where ... shall" and ubiquitous
"shall" bullets conform.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Three normative behaviours in FR-015's IR-624 section are prose bullets with no `shall`, so they are not EARS requirements and the lint cannot see them: a non-clause field with no range is listed, has no `domains` entry and is drawn without an assumption; the field set and draw order are the request's `state_fields`; and an unread field is listed without a range. Restate each as a requirement, for example "Where a state field other than the clause's has no range, the generator shall list it in state_fields and shall give it no domains entry". | spec/kani/functional/FR-015-bounded-kani-obligations.md:463-474 |

## Dispositions

Round 1, reviewed at 2fac5ac6394887b1d617c64350d7886e057f2e4c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2fac5ac: The unranged-field, field-set and draw-order behaviours are now If/Where ... shall statements. quire validate still reports 6 warnings, all pre-existing. |

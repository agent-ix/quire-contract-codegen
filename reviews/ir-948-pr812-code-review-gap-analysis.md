---
id: SR-17940486709564847047902
title: "IR-948 PR #812 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #812 at e1e3ec92; IR-948; FR-015-AC-38, FR-015-AC-126, FR-015-AC-129, FR-017-AC-63, FR-024-AC-63, FR-024-AC-64; tests/it/kani_obligations_v2_profiles.rs, src/kani/generate/clause/v2.rs, docs/qualification/ir948-kani-profiles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-948. Plan completion: not assessed. Computed matrix (`quoin matrix --json` at
e1e3ec92, quoin 0.29.1): FR-015-AC-129 is bound by all six tests of
`tests/it/kani_obligations_v2_profiles.rs` (five `ignored`); FR-015-AC-126 is bound by those five
Kani tests plus its existing TC-027/IR-917 binders. Both rows read "no run evidence", as expected
for ignored Kani-lane tests; the run evidence is the qualification record, which the reviewer
reproduced (all five pass; see SR-17940486709564847047901).

Status check: neither AC is marked Implemented. FR-015-AC-129 moves PLANNED to PARTIAL and lists
exactly the members executed; FR-015-AC-126 keeps PARTIAL. Every member claimed executed has an
installed-Kani verdict at unwind 8 under the 24 GiB scope: pre `Verified` + cover, impossible pre
`CoverUnsatisfied`, invariant `Verified` + cover, two invariant mutants `Falsified` from the
witness phase with replay `ReproducedWithEvaluatedWitness` / `Violation` at balance 0 and -1.

The postcondition blocker the PARTIAL states was checked against code: `lower_with_census`
refuses a Postcondition whose oracle input has a `state_field` (only `parameter_*` or
`result_result` inputs are accepted), returning `V2ClauseConditionNotRendered`, so a
`self.balance` postcondition cannot render today. The QSL refusal is tracked as QSL-1901
(Backlog, handed to terra:QSL:code, "blocks IR-948").

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | The two blockers the PARTIAL rests on are not pinned by any test: no default-lane test asserts that `Debited` (state-reading postcondition) negotiates to `Unsupported(V2ClauseConditionNotRendered)`, and none records QSL's `unproved-range` refusal of the `=` form. When the CG renderer or QSL-1901 lands nothing flips, so the PARTIAL can go stale silently; the default-lane test's doc comment ("checks what generation yields for the profile package") covers only two of the three authored clauses. Add a generation assertion for `Debited` (refused today, expected to flip) | tests/it/kani_obligations_v2_profiles.rs:1-7, tests/it/kani_obligations_v2_profiles.rs:64-80, docs/qualification/ir948-kani-profiles.md:37-44 |

## Verdict

PASS for the slice. The executed members of FR-015-AC-126 and FR-015-AC-129 are backed by
reproducible installed-Kani verdicts from tests tagged to those criteria; no AC is claimed
Implemented without an executed verdict; the unexecuted members (postcondition, frame, the
four-kind run over one package) are stated as not established. The healthy `withdraw` subject
named by AC-129 is not exercised, which is correct for this slice: the precondition and invariant
harnesses do not call a subject (`render_clause`), so it only matters for the postcondition and
frame members still planned. Spec wording issues are in SR-17940486709564847047903.

## Dispositions

Round 1, checked against 4630b054.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | new default-lane test `the_postcondition_member_is_blocked_by_the_generator_and_by_qsl` asserts `Debited` negotiates to `Unsupported(V2ClauseConditionNotRendered)` between two `Supported` records, and that QSL's `clause_site` refuses the `=` text with `UndefinedExpression` / `unproved-range`; it passes at 4630b054 |

---
id: SR-17940486709564847047734
title: "IR-1509 PR #805 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #805 at 48809302 (base main 63cf3ca0); IR-1509; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1509-kani-clause-controls.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1509. The spec edits are status and evidence edits: FR-015-AC-132 and FR-017-AC-64
flip from PLANNED to Implemented, FR-015-AC-151 is rewritten from its list of unmet items into a
description of the executed runs and marked Implemented, FR-015-AC-41, AC-119 and FR-017-AC-60,
AC-63 have their IR-1509 pointers replaced by AC-151 references, TC-025 gains step 56 and the
matrix index flips the AC-132, AC-64 rows and adds an AC-151 row. No new requirement statement
was added, so EARS and integrity sub-analyses were folded in here. `quire validate` on the four
edited spec files: the same warning count as main (1, 3, 0, 0), none new.

The Kani numbers in the qualification record and AC-151 match the coder's run logs
(`scratchpad/ir1509/*.log`, `summary.txt`): healthy rc 0, `Discriminant(0)`, proof_phase false,
44.0 s in-test / 44.13 s whole test, peak 2,803,191,808 B; exact-999 129.16 s, 11.61 GB;
plus-one 134.97 s, 11.73 GB; left-only 125.29 s, 11.19 GB; helper 116.77 s, 10.65 GB; every
mutant `Discriminant(1)` (Falsified) with proof_phase true. The individual playback values
(2, 0, 1000) and the 2458 successful checks are not in any retained log (see SR code-review
FND-002).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-015-AC-151 reads `Implemented` (and its matrix row `Covered`) although one of the four items the criterion listed as unmet on main, "a subject compiled by the pipeline", is not delivered: the subject is the test-written `crate::subject` module. The PR rewrote the criterion body into a description of what ran and moved this gap into "Scope notes", so the obligation disappears from the spec with no owner. The planner's IR-1509 scope (Linear comment 28a4d257) names "pipeline-compiled subject with QSL regeneration" as part of AC-151. AC-151 must be split or read Partial: Implemented for the runner two-phase verdicts, kernel mutants, regenerated package and the host 999 playback; the pipeline-compiled subject PLANNED with an owner. No CG or QSL step emits a subject (the QSL operation declares no body), so whether this item is withdrawn or ticketed is a lead decision (new tickets go to the lead first) | spec/kani/functional/FR-015-bounded-kani-obligations.md:966, spec/kani/matrix/tests.md:57 |
| FND-002 | medium | FR-015-AC-132 is marked Implemented but still says the subject mutants are applied "in the compiled subject source built from the QSL source"; the subject is written by the test (the qualification record says so). The criterion text claims a provenance the test does not have | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 |
| FND-003 | medium | The FR-017-AC-63 matrix row still reads "Unmet: the installed-Kani controls of FR-015-AC-41, AC-119 and AC-126 ... IR-1022 stays open ... upgrade IR-1509", contradicting the AC-63 text this PR rewrote (those controls are now FR-015-AC-151) and the new AC-151 row | spec/kani/matrix/tests.md:61 |
| FND-004 | low | FR-017-AC-64 still states in the present tense that "the healthy clause and the exact-999 mutant were each `Inconclusive` with `TimedOut`" as their classification; after AC-151 the same clause is `Verified` and the mutant `Falsified` through CG's runner. The sentence should be marked as the earlier 16 GiB / 600 s measurement | spec/kani/functional/FR-017-kani-execution-evidence.md:916 |
| FND-005 | low | Healthy wall time is stated as 44.0 s in FR-015-AC-151 (the in-test `Instant` wall) and 44.1 s in the qualification record (whole test), while every mutant figure in both is the whole-test wall; the record also says "all seven default-lane tests" where the module has eight | spec/kani/functional/FR-015-bounded-kani-obligations.md:966, docs/qualification/ir1509-kani-clause-controls.md:46,69 |

## Verdict

Not mergeable at 48809302 until FND-001 to FND-003 are fixed in this PR. FND-001 needs the lead's
ruling on the pipeline-compiled-subject item before the status can be written (withdraw it with
the decider named, or keep it PLANNED under a ticket someone is building). Checked with no
defect: the host-run 999 playback of the sealed artifact is an honest reading of AC-151's
"deterministic `amount_current = 999` playback of the sealed artifact" (it runs the unchanged
generated arithmetic assertion against the mutated owner), and the Kani-backed playback stays
with FR-017-AC-60 (IR-917), which is unchanged and PLANNED; the AC-151 caveats are disclosed in
the AC, the matrix row and the record; FR-017-AC-64's flip is backed by tests that print and check
the record and the matrix wording; TC-025 step 56 matches the five installed tests and the
playback test; FR-015-AC-41 and AC-119 correctly stay PLANNED under IR-917; no hashes, pins or
compatibility layers were added.

## Dispositions

Round 1, head bc809502. `quire validate` on the edited spec files adds no warning.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | FR-015-AC-151 reads PARTIAL: implemented items listed; the pipeline-compiled subject PLANNED, blocked on QSL operation bodies (planner ruling 2026-11-12, no ticket); the Kani-backed 999 playback PLANNED under IR-917 |
| FND-002 | fixed | FR-015-AC-132 now says "in the test-written subject source (no CG or QSL step emits a subject, see FR-015-AC-151)" |
| FND-003 | fixed | The FR-017-AC-63 matrix row no longer says Unmet; it points the installed controls at FR-015-AC-151 (partial there) and marks the 16 GiB run as earlier |
| FND-004 | fixed | FR-017-AC-64 now reads "The earlier measured symbolic runs (16 GiB, 600 s, before the IR-1022 tractability work) keep their own classification" |
| FND-005 | fixed | AC-151 and the record both use whole-test wall (43.5 s healthy, in-run 43.4 s stated) and the record says eight default-lane tests |

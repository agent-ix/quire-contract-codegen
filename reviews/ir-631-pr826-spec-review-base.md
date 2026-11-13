---
id: SR-17940486709564847047966
title: "IR-631 PR #826 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #826 at 25030e08; spec/replay/functional/FR-032-routed-scalar-replay-binding.md (AC-7, AC-8, status paragraph), with sibling AC-3/AC-4/AC-5 and spec/replay/matrix/TC-047 read for consistency; IR-631"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

## Summary

Ticket: IR-631. This review checks the spec status edits and the IR-631 allowance they rely on: "If CG has no
Kani lane that can falsify the mutant, this is IR-610 driver evidence instead." `quire validate` on FR-032
exits 0 (module-level warnings only). The integrity and consistency checks are folded in here. No EARS
statement changed, so the EARS sub-analysis does not apply.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-032-AC-7 (and AC-8 and the status paragraph) hand the real-Kani half to "IR-610 driver evidence", but the ticket's escape clause does not apply and the new owner cannot do the work. The clause applies when CG has no lane that can *falsify* the mutant. This PR shows CG's lane does falsify it, in 29 s. What is missing is the join, which is a different condition. IR-610 is an umbrella (Specced). Its driver criteria (FR-009-AC-9: report conversion; IR-773: execute the same *proved* artifact) do not include a falsified mutant run, and the driver cannot seal a mutant or get a native receipt for a falsified proof. The result is an ownerless PLANNED half. Fix: either build the crate-private join (re-seal the mutant with body and package bound, mutate the oracle and harness consistently, `plan` the real playback, observe the same staged bytes natively, then genuine QSL gives `Diverged` and `Failed`), or get an explicit owner ruling that retires the real-Kani half in FR-032, rather than pointing at IR-610. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:401,402,510 |
| FND-002 | medium | Sibling statuses still name the lane this PR says does not exist. AC-5 says "a QSL `Diverged` result from a changed generated artifact is not asserted (PLANNED, real-Kani lane)". AC-4 and AC-3 keep "real Kani run ... PLANNED". TC-047 still says "a real falsified Kani run, the IR-631 code slice" (lines 25, 239) and step 2 "In the named real-Kani lane". Once IR-631 closes, these PLANNED halves cite a closed ticket and a lane FR-032-AC-7 now denies. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:397-399, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:25,34,239 |

## Verdict

Not mergeable as written. The status text is honest that AC-7 and AC-8 stay PARTIAL, but it gives the
remainder to an owner that will not execute it, and it leaves AC-3/AC-4/AC-5 and TC-047 saying the opposite.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-003 | low | The re-seal caveat is stated only in the AC-7 row and the evidence doc. The AC-3, AC-4, AC-5 and AC-8 rows cite the same test as IMPLEMENTED evidence without saying that its artifact is a crate-private re-sealed mutant. TC-047 and the doc's step 5 say "the driver observes the same staged bytes", but the observer is the `#[cfg(test)]` CG helper `observe_staged_native_for_test`, not the driver. The claims are true; the provenance of the evidence should read the same everywhere. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:397-399,402, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:24-30, docs/qualification/ir631-scalar-replay-real-kani-evidence.md:24 |

## Dispositions

Round 1, checked against 40124b5f. `quire validate` on FR-032 exits 0.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The join is built in CG. The FR-032 status paragraph now reads: "AC-1 to AC-8, AC-12, AC-14 to AC-17 and AC-20 to AC-25 are **Implemented** (IR-631; AC-3 to AC-5, AC-7 and AC-8 including the real-Kani half) ... the remaining gate for the whole route is the driver's same-proved-artifact execution (the driver's E9 run, IR-610)". No half is handed to IR-610 any more. |
| FND-002 | fixed | AC-5 now reads "a QSL `Diverged` result from a changed generated artifact is asserted by `a_real_falsified_native_mutant_plans_observes_and_settles_diverged_failed`"; AC-3 and AC-4 cite the same test. TC-047 line 239 now names only "the driver's E9 run, IR-610", and step 2 is reworded to the real-Kani lane that exists. |

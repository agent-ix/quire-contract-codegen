---
id: SR-17940486709564847047738
title: "IR-1448 PR #806 spec-review evidence"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen PR #806 at c2a8f9e0; IR-1448; spec/kani/functional/FR-055-option-clause-harnesses.md, spec/kani/matrix/TC-069-option-clause-harnesses.md, spec/kani/matrix/tests.md, spec/spec.md, docs/qualification/ir1448-kani-option-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-069
    type: reviews
---

## Summary

Ticket: IR-1448. Checked that each lowered construct has a healthy proof and a mutant that can
fail, naming the distinguishing input and the failing step, and that the probe record is
honestly scoped. Worked each mutant by hand against the claim's expected value
(`present implies value <= 1000` is true on every admitted draw).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-055-AC-2's mutant "returns the negated presence" is ambiguous and, on the subject reading, names the wrong distinguishing input. If the subject returns `!present`: absent gives `true` = expected (agrees), present gives `false` != `true`, so the counterexample is a present draw, not "playback absent (presence false, payload 0)". Only if the mutant is the `option.present` lowering does the absent draw fail (via the `value` of absent stop, the AC-4 mechanism). State that it is a lowering mutant and its site, or change the input. The probe ran no such mutant (its `mut_present` is AC-3's `< 1000` subject). | spec/kani/functional/FR-055-option-clause-harnesses.md:133; TC-069 step 2; docs/qualification/ir1448-kani-option-probe.md results table |
| FND-002 | low | FR-055-AC-1's three mutants share "fail the enumeration or the shape inspection" with no per-mutant step or distinguishing draw (e.g. absent with payload 1 for the free-absent mutant, present with 1001 for the widened bound), and FR-055-AC-9's "changes when the encoding changes" names no concrete encoding change. | FR-055-AC-1, FR-055-AC-9 |
| FND-003 | low | Probe record: the time column is Kani verification time, and four rows say "well under 1 s" rather than a number; no wall time is recorded, though AC-10 asks for wall time and the brief asked for wall caveats. The memory caveat (cgroup `memory.peak` lower bound, 24G scope, 16 GiB cap not exercised) is stated. | docs/qualification/ir1448-kani-option-probe.md:27-40, 52-54 |

## Verdict

AC-3, AC-5 and AC-6 mutants are correct (distinguishing inputs present/1000, absent/0 and
present/n=1000 check out by hand), and AC-5's guard-order lowering mutant is a real failing
control. The probe is clearly marked as not CG evidence, not CG's runner, not scaled and with
`option.value` of absent modelled by assumption; it carries no proof credit. AC-4's evidence
problem (inadmissible input) is recorded as base SR FND-003.

## Dispositions

Round 1, reviewed head a9adfb39b95bb563fd3e4f64c8f277c9fe0cc0b3.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | AC-3: "the claim `result = option.present(x)` verifies healthy ... Mutant at the site where the oracle reads the presence leaf: returning its negation fails the produced-result step at every draw, including presence 0 with payload 0 and presence 1 with payload 0". Checked by hand: correct. The probe now has present_ok and present_lowering_mut (playback presence 0, payload 0), plus guarded_lowering_mut for AC-6. |
| FND-002 | fixed | AC-2 mutants now name inputs ("the enumeration finds the absent value twice, at payload 1"; "finds payload 1001"). AC-18 names the encoding change ("changing the absent rule from canonical payload to free payload"). |
| FND-003 | fixed | The probe table now gives numeric Kani time, wall time (0.105-0.160 s) and peak (31.9-33.9 MB) per row, defines each column, and keeps the lower-bound memory caveat. The numbers are consistent with the FR's "under one second" and the probe's "under 0.05 s of Kani time". |

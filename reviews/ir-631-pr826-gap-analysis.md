---
id: SR-17940486709564847047965
title: "IR-631 PR #826 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #826 at 25030e08; IR-631 acceptance checks 1-5; FR-032 criteria in the computed matrix (quire 0.36.2, quire matrix --scope .)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

## Summary

Ticket: IR-631. Planless gap analysis of the criteria named by IR-631's remaining acceptance checks.
Plan completion: not assessed. `quire matrix --scope .` on the PR head shows every live FR-032
criterion `tagged`; AC-9, AC-10, AC-11 and AC-13 are retired Inspection rows (`method-without-symbol`).

- Check 2: AC-1 (4 binders), AC-4 (2), AC-5 (1), AC-15 (4 or more), AC-16 (2) are tagged. Met.
- Check 3: AC-2, AC-6, AC-17 and AC-18 are tagged; AC-2, AC-6 and AC-17 are IMPLEMENTED. AC-18 is PARTIAL,
  but its absent-kind and absent-operand-identity residuals are tagged in `scalar_replay_acceptance.rs`.
  Met.
- Check 4: AC-12 (`scalar_replay_acceptance.rs:555`) and AC-14 (5 binders) are tagged and IMPLEMENTED. Met.
- Check 5: AC-19 is IMPLEMENTED (IR-304 slice (b)) with 4 binders. Met.
- Check 1: the real-Kani mutant is falsified and its playback decoded (in an ignored test tagged
  FR-015-AC-126/TC-025, not FR-032). No test drives a real falsified run through `plan`, `complete`, QSL and
  the converter to `Parity(Diverged)` and `Failed`. Not met; this PR reassigns it (finding below).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Acceptance check 1 has no test and, after this PR, no executing owner. The AC-7 and AC-8 `scalar_settlement.rs` binders use synthetic falsified evidence (`falsified()` returns a fixed playback) and genuine QSL reports, not a real Kani run. The only real-Kani test is tagged FR-015-AC-126 and stops at decode. The PR marks the rest "IR-610 driver evidence", but no driver criterion executes it: quire-driver FR-009-AC-9 (Planned, IR-610) covers converting a QSL report, and IR-773 (Done) covers the same *proved* artifact. Per the doc's own argument, the driver cannot seal a mutant (`generated` is `pub(crate)`) and gets no native receipt for a falsified proof. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:401-402, tests/it/scalar_settlement.rs:25-29, tests/it/kani_scalar_native_private.rs:66-70 |

## Verdict

Checks 2 to 5 are met by code already on main (#803 and earlier). Check 1 is not met. The PR reroutes it
to a ticket that does not own it. IR-631 should not be closed on this basis.

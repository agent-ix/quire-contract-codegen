---
id: SR-1946
title: "IR-652 lifecycle spec review (base, manual criterion strength)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

# SR-1946: IR-652 lifecycle spec review, base (manual criterion strength)

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080.
Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run
5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. The Jev client is unavailable: `command -v jev` finds
nothing, and the installed quoin spec-criterion-strength-analysis skill says its Jev client does
not exist yet. This is therefore a manual judgement under `analysis: base`, not a calibrated run,
and it does not use the criterion-strength archetype. One medium and one low finding.

## Method

For each new or changed criterion I asked what observable result separates a correct
implementation from a wrong one, and whether the criterion defines that result or only names it.
Scope units examined: FR-017-AC-19, FR-034-AC-2, 5, 7, 12, 17 and 31 to 34. Clean:
- AC-2, AC-5 and AC-7 have defined refusal and termination oracles.
- AC-17's argv bytes are exact.
- AC-31 requires the merged #295 control to fail its named teardown assertion, which is a genuine
  adverse case against current behaviour.

## Verdict

**FAIL: one medium and one low finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The resource-charge oracle is undefined. FR-034-AC-32 requires "actual pipe-capacity/kernel-backing charge ... against the original ceiling", and AC-34 refuses on "missing conservative backing accounting". Neither says what quantity is measured: memfd i_size, st_blocks or shmem pages, and F_GETPIPE_SZ for the pipe. Nor do they give the comparison (RSS plus pipe capacity plus backing against the ceiling?) or say when accounting counts as "missing". Two implementers would build different checks, and a test cannot tell a correct charge from an undercount. | spec/kani/functional/FR-034-caller-death-ownership.md:535, spec/kani/functional/FR-034-caller-death-ownership.md:537, spec/kani/functional/FR-034-caller-death-ownership.md:479-484 |
| FND-002 | low | FR-034-AC-31, 32 and 33 put delivery status inside the criterion ("presently UNRUN", "These gates are UNRUN"). The criterion becomes false prose the moment CODE lands, and forces a criterion edit to record evidence. Status belongs in TC-049 and the computed matrix. | spec/kani/functional/FR-034-caller-death-ownership.md:534-536 |

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Charge oracle defined: owned_RSS + caller_run_buffers + page-rounded F_GETPIPE_SZ reserve (resize authority excluded or maximum pre-reserved) + page-rounded 16 MiB+1 memfd reserve, checked arithmetic, refusal on missing quantity. |
| FND-002 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Status words removed from the criteria; UNRUN status kept in TC-049/TC-027 and the computed untagged matrix. |

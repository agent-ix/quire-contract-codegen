---
id: "SR-11996"
title: "IR-808 manual acceptance check: kani_gate trace tags still prove their criteria"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@3254cf45489e87955a3a375fa660bf3623f47a9d; tests/it/kani_gate.rs, spec/kani/functional/FR-041-claimed-module-proof-gate.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/matrix/TC-045-real-kani-lane-gate.md"
---

# SR-11996: IR-808 manual acceptance check

## Summary

Ticket: IR-808. PR: agent-ix/quire-contract-codegen#408. The change is test-only and changes no
FR, so I did a manual check in place of quoin gap-analysis. For each traced test I asked whether
it can still fail when its criterion is violated. Plan completion: not assessed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean.

- FR-041-AC-5 ("A stubbed executor ... cannot pass the gate"), NFR-006-AC-19 and TC-056 are
  traced by `gate_rejects_fake_cargo_and_kani_with_printed_proof_and_reports`. The test still
  shows a stand-in that prints the proof line and writes nonempty reports being rejected. It is
  now deterministic: 8 of 8 passes, against 2 of 8 failures on main. The oracle-precision nit is
  SR-11995 FND-002.
- `gate_runs_once_under_lock_and_refuses_missing_tests_or_failures` (NFR-006-AC-5/6/7, TC-045)
  and `gate_refuses_missing_launcher_dirty_or_moving_tree_and_stale_main` (NFR-006-AC-8/9/10/14,
  TC-045) keep their assertions. The only change is the gate's cwd, and the stand-in git ignores
  cwd.
- `gate_waits_for_the_existing_lock_holder` (NFR-006-AC-5, TC-045) still needs a positive ps
  observation of the waiting flock command, which now uses the absolute script path. It also
  still asserts that the lane does not start before the lock is released.
- `make_targets_share_one_real_lane_and_ci_excludes_it` (NFR-006-AC-4/12) is unchanged and passed,
  which confirms the Makefile recipe text is identical.

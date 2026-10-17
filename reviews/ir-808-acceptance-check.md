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


## Follow-up review (disposition pass 3)

Provisional local review only. Frozen candidate: 80378693a29f745dfb7dc1d6b4eddb07ce294a1c;
new delta: e2f6c1b..8037869, `src/kani/run/memory.rs`. The locally cached
`origin/main...HEAD` diff was inspected as context, including the gate script and tests;
remote freshness was not verified. No spec/plan or CI workflow changes appear in that diff.
No applicable AssuranceProfile was found under `spec/`.

All prior finding text and disposition rows above are preserved. SR-11995 FND-001,
FND-002 and FND-003 already have terminal `fixed` outcomes; no redundant outcome rows
are added. This round introduces no new findings. This does not renew historical test-run
claims: no Cargo or Kani command ran in this review. The leader reports that the required
systemd user scope was denied before Cargo launch; this reviewer did not retry it or run
unscoped commands. Formatting, clippy, execution tests and the aggregate gate remain UNRUN.

Tracker team-key validation, private comment dedup/publication/read-back and label state
are unavailable in this environment. No tracker or GitHub comment was posted. No remote
state, mergeability or merge approval is asserted. This scratch artifact requires custody
by the leader; the frozen branch was not edited.


### Scoped manual acceptance and computed trace evidence

Provisional verdict: changed behavior is consistent with FR-028-AC-21 by inspection;
runtime acceptance remains UNRUN. This continues the existing manual acceptance method
under SR-11996; it is not a new full repository gap-analysis pass. The earlier summary's
"test-only" scope applies only to its original candidate; this follow-up also examines the
production RSS change. Full repository gap-analysis and its reverse-gap/stub inventory
were not rerun in this focused disposition pass. Optional full semantic analysis was skipped.
Plan completion: not assessed.

Examined: FR-028-AC-21, `spec/kani/functional/FR-028-bounded-proof-ceilings.md:220`:

> The backend process tree of every run is held to the harness identity's memory ceiling by a mechanism that observes or limits the tree's memory (the choice is the code change's, Open Questions); every execution evidence records the mechanism and, where it observes memory, the tree's peak resident memory; a tree that exceeds the ceiling is killed whole and settles as FR-028-AC-3 states, with `KaniInconclusiveReason::MemoryExhausted`, serialized `memory_exhausted`, a variant distinct from every existing reason; a FR-017 batch's launcher process tree is held to the ceiling as one group, killed whole when it exceeds it and refused as memory-exhausted with no member classified (the report is written only at the end); and on a platform where the generator has no mechanism that observes or limits that memory, the run is refused with a typed reason before the backend starts. PARTIAL: current contract, scalar and state-frame execution entries; startup caller-death supervision (IR-639), refinement runs and the limit-only stand-in remain planned (IR-241; TC-039 Status).

The observation clause is satisfied by recording a value already read from the owned,
identity-verified init while its gate is held. This prevents `None` when the backend exits
before polling. The code still refuses unavailable observation, retains later larger tree
samples and does not estimate an unobserved intersample peak. The broader ceiling kill,
batch refusal, and platform refusal clauses were read as context and are unchanged.

The computed matrix command `quire matrix --scope /home/peter/dev/worktrees/ir808 --format json`
completed successfully. FR-028-AC-21 computes `tagged`; the full output is `matrix.json`
and the selected row is `fr028-ac21-matrix.json` beside this artifact. This is trace presence,
not proof that tests ran or the whole repository passed strict coverage. Reviewed bindings:
`identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory` →
FR-028-AC-21 (correct for successful observed evidence), and
`available_observer_does_not_invent_a_peak_before_observing_a_tree` → FR-028-AC-21
(correct for absence before tree observation). The latter calls `prepare`, not `bind_root`,
so the fix does not invalidate its expected `None`. No tests were added in the RSS delta.

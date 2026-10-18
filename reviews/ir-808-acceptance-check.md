---
id: "SR-11996"
title: "IR-808 manual acceptance check: kani_gate trace tags still prove their criteria"
type: SpecReview
scope: "agent-ix/quire-contract-codegen; IR-808; scripts/kani_gate.sh, tests/it/kani_gate.rs, src/kani/run/memory.rs, src/kani/run/namespace.rs, src/kani/run/execute.rs, spec/kani/"
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


## Follow-up review (disposition pass 4)

Formal follow-up at frozen pushed candidate 715d41da048b2e5c20d7c68b72ec8249d4e7d961,
base bacde1b; actual diff `origin/main...HEAD` examined. The earlier pass-3 limitations
remain historical statements, superseded by the evidence in this section. No original
finding or disposition text was edited. All SR-11995 findings retain terminal fixed
outcomes; no unresolved finding requires a fresh outcome row and no new code defect was found.

Code-review, Rust lane and shell lane: PASS for this PR delta. No applicable
AssuranceProfile exists under spec/. No spec, plan, dependency or CI workflow change.
The initial resident_bytes value is obtained with the owned init identity before the
startup gate is released. Recording that measured RSS fixes missing successful-run
observations without inventing an estimate or changing prepare-only None. Later samples
retain their existing maximum. Identity/read failures propagate before peak mutation.
No new unsafe/panic, conversion, wire field, lock, unbounded work or duplicate helper.
The symlink test uses canonical ix_trace_rs::trace on a #[test] only; existing trace
cleanup outside this delta stays in its owning migration work. Prior symlink, oracle
precision and trace findings are still fixed by inspection.

Focused validation evidence was read directly from the coder's restored logs at
/home/peter/dev/worktrees/kani-lead-codex/ir808/: fmt-check and locked all-target
clippy passed; kani_gate 8/8, memory observer 8/8, RSS 1/1 and cancellation 1/1 passed.
These are coder execution results, not fresh reviewer runs. No reason to repeat them
was found; this reviewer ran no Cargo/Kani job and no aggregate gate.

The production change requires genuine gap-analysis: SR-12730 records the full audit,
including unchanged baseline gaps and explicit out-of-scope dispositions. SR-11996's
original manual check remains historical evidence, not a substitute for that audit.
Cancellation behavior is unchanged and one focused passing test does not establish
elimination of its historical race. IR-808 must remain open for residual cancellation scope.
Merge eligibility for the reviewed delta depends on committing the latest review artifacts
and the leader's successful aggregate gate on the exact final head.

### Round 4 examined scope

```yaml
scope:
- id: FR-028-AC-21
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The backend process tree of every run is held to the harness identity's memory ceiling by a mechanism
    that observes or limits the tree's memory (the choice is the code change's, Open Questions); every execution
    evidence records the mechanism and, where it observes memory, the tree's peak resident memory; a tree that exceeds
    the ceiling is killed whole and settles as FR-028-AC-3 states, with `KaniInconclusiveReason::MemoryExhausted`,
    serialized `memory_exhausted`, a variant distinct from every existing reason; a FR-017 batch's launcher process
    tree is held to the ceiling as one group, killed wh
- id: src/kani/run/memory.rs::MemoryObserver::bind_root
  path: src/kani/run/memory.rs
  role: examined
  excerpt: "let bound = self.resident_bytes(pid, start)?.ok_or_else(|| {\n    io::Error::new(\n        io::ErrorKind::InvalidData,\n\
    \        \"owned init RSS identity changed\",\n    )\n})?;"
- id: src/kani/run/namespace.rs::NamespaceOwner::dispatch
  path: src/kani/run/namespace.rs
  role: examined
  excerpt: 'self.init = Some((info.child, start, handle));

    observer.bind_root(info.child, start)?;'
- id: src/kani/run/memory.rs::MemoryObserver::observe
  path: src/kani/run/memory.rs
  role: examined
  excerpt: "self.observation.peak_resident_bytes =\n    Some(self.observation.peak_resident_bytes.unwrap_or(0).max(total));"
- id: src/kani/run/execute.rs::identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory
  path: src/kani/run/execute.rs
  role: examined
  excerpt: assert!(evidence.memory.peak_resident_bytes.is_some());
- id: src/kani/run/memory.rs::available_observer_does_not_invent_a_peak_before_observing_a_tree
  path: src/kani/run/memory.rs
  role: examined
  excerpt: assert_eq!(observer.observation().peak_resident_bytes, None);
- id: scripts/kani_gate.sh
  path: scripts/kani_gate.sh
  role: examined
  excerpt: 'script_dir=$(dirname -- "$(/usr/bin/readlink -f -- "$0")")

    if ! freshness=$(/usr/bin/bash "$script_dir/kani_scope.sh" --main-freshness); then'
- id: tests/it/kani_gate.rs::gate_script
  path: tests/it/kani_gate.rs
  role: examined
  excerpt: command.current_dir(&self.root);
- id: tests/it/kani_gate.rs::gate_rejects_fake_cargo_and_kani_with_printed_proof_and_reports
  path: tests/it/kani_gate.rs
  role: examined
  excerpt: "result.starts_with(\"kani-gate: result=failed ran=3 expected=3 elapsed=\")\n    && result.ends_with(\"\
    s kani=Kani 1.0 tree=clean\")"
- id: tests/it/kani_gate.rs::gate_invoked_through_a_symlink_finds_its_scope_script
  path: tests/it/kani_gate.rs
  role: examined
  excerpt: '#[trace("TC-045")]

    #[test]

    fn gate_invoked_through_a_symlink_finds_its_scope_script() {'
- id: tests/it/kani_gate.rs::gate_waits_for_the_existing_lock_holder
  path: tests/it/kani_gate.rs
  role: examined
  excerpt: '"flock -x {} /usr/bin/bash {}/scripts/kani_gate.sh"'
- id: Makefile::kani-gate
  path: Makefile
  role: examined
  excerpt: $(BASH) scripts/kani_gate.sh $(KANI_LOCK) -- $(KANI_TEST)
- id: scripts/kani_scope.sh
  path: scripts/kani_scope.sh
  role: context_only
  excerpt: base=$(git merge-base HEAD origin/main)
- id: FR-028-AC-2
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its wall-clock ceiling is stopped and classified KaniRunOutcome::Inconclusive
    with the timed-out reason naming the ceiling, never verified or falsified. FR-029 maps this source outcome to
    final QSL Incomplete(TimedOut); the backend classification is not the terminal category.
- id: FR-028-AC-4
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the bounds of each symbolic argument and the two ceilings the run was
    held to.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and
    cannot pass the gate.
- id: NFR-006-AC-5
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` starts the lane''s `cargo test` once with `--test-threads=1`, holds the lock named
    by `KANI_LOCK` while it runs, and does not start it while another process holds that lock.'
- id: NFR-006-AC-8
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: When the tree state or the head commit read after the run differs from the one read before the build,
    `make kani-gate` prints `result=failed` and exits non-zero.
- id: NFR-006-AC-14
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Given Kani-reaching paths changed on `origin/main` since the candidate's merge base and main absent from
    the candidate, `make kani-gate` refuses before running tests and names those paths. Given only non-touching
    main changes, it may run. The merger updates a stale candidate from current main and the full gate passes on
    the updated clean candidate before merge.
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's module census and real mutation controls
    in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero.
    An optional mutation target or saved earlier result cannot substitute.
bindings:
- test_id: gate_invoked_through_a_symlink_finds_its_scope_script
  ac_id: TC-045
  trace: correct
- test_id: identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory
  ac_id: FR-028-AC-21
  trace: correct
- test_id: available_observer_does_not_invent_a_peak_before_observing_a_tree
  ac_id: FR-028-AC-21
  trace: correct
```


## Formal publication and read-back

IR-808 already carries has-review. GitHub PR title/body already references IR-808;
no findings were posted to GitHub. Optional spec/plan review methods were not applicable
(no changed spec or plan), and optional deep semantic gap review was not requested.
The required code-review includes its Rust lane; the genuine gap-analysis is separate.

The first attempted full gap comment was rejected before creation by Linear's confirmed
100000-character body limit. On dispatching lead instruction, the same logical method
record was split into eight parts with identical SR/method/run markers and explicit part
numbers. Every part was read back; YAML parsed, content matched after trailing-newline
normalization, and the union exactly matched all 327 finding IDs and 817 scope units.
The single dispositions comment read back with 327/327 deferred outcomes and nonempty
reasons. Prior methods' round-4 dispositions lists are empty because their findings already
had terminal outcomes before this round. No prior review text or comments were edited.

Comment IDs:
- code-dispositions-comment.md: b1c43bc8-e8e9-4543-b140-4a40d5d17331 — read-back passed.
- acceptance-dispositions-comment.md: 54926bb5-52ab-4929-8217-cd503eed468f — read-back passed.
- gap-review-part-01.md: 02205a17-263c-4b64-8c20-649cea3cfb25 — read-back passed.
- gap-review-part-02.md: f6f5c645-7b70-4e06-85aa-1def489466f5 — read-back passed.
- gap-review-part-03.md: 425834e3-036d-40af-81a5-7addd006b541 — read-back passed.
- gap-review-part-04.md: 04b40552-3850-490c-b8fe-f0e5e3bb33cc — read-back passed.
- gap-review-part-05.md: 2bd5a56b-d13d-4d0d-8556-2f1d067dc734 — read-back passed.
- gap-review-part-06.md: 755bac97-9648-497e-8d81-99945ca8590c — read-back passed.
- gap-review-part-07.md: 8cccc445-e443-4d5d-860a-0729a06ede79 — read-back passed.
- gap-review-part-08.md: ad333a01-92b3-4ad3-902e-3e5149e0b068 — read-back passed.
- gap-dispositions-comment.md: 85139f49-30e0-4e4a-a7ee-f4cdd6891d9e — read-back passed.

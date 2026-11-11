---
id: TC-039
title: "Verify bounded proof ceilings, their inconclusive reasons and the proof subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: verifies
---
# TC-039: Verify bounded proof ceilings, their inconclusive reasons and the proof subject

## Description

Verify that every harness identity records its ceilings, that a run exceeding its wall-clock ceiling settles
inconclusive with the timed-out reason and an environment memory kill is classified from the backend's
report (steps 23 to 26), that evidence records the bounds and ceilings used, that every
identity, refusal and evidence names its family, and that the proof subject and a narrowed bound are
recorded as FR-028 states. Verify the shadow contract (steps 11 to 22): a shadow's independence from
the code it stands for, its refinement obligation and the class and strength of its run, the
abstractions it owes evidence for, its refused shapes, the memory ceiling and the seeded mutants it
must catch.

## Test Procedure

1. Generate one harness at two memory ceilings and at two wall-clock ceilings.
2. Run a harness whose run cannot finish inside a small wall-clock ceiling.
3. Run a harness whose backend the caller's environment ends for exceeding a small memory
   ceiling (step 23; FR-028-AC-3 is retired and FR-028-AC-25 replaces it).
4. Read the evidence of a verified run and of each run above.
5. Read the family named in a harness identity, a generation refusal and an execution evidence.
6. Generate a family with no supplied shadow.
7. Generate a family with a supplied shadow.
8. Narrow one argument inside its declared domain.
9. Narrow one argument outside its declared domain.
10. Run a batch of N harnesses sharing one wall-clock budget T with a report in which one member's
    entry reads Failure, no checks, exit status `timeout` beside finished members; a stand-in that
    is still running at N times T; a budget whose product overflows; and a T above 4294967295
    seconds whose product with N fits, and a `Duration::MAX` request, reading each launch's
    arguments and outer bound.

### The shadow contract (planned, IR-241)

Steps 11 to 22 use a stand-in family, not composite equality: a two-leaf shadow (two bounded
integers compared for equality) with a stand-in production subject that the test can mutate, so
that the machinery of FR-028-AC-13 to FR-028-AC-24 is asserted without the composite family's
generator. The composite family's own run is TC-025 steps 29 to 36.

11. Parse the source of the stand-in shadow harness and read its imports, stubs and assumptions;
    generate it again with a mutated generator that imports a runtime type, one that emits a
    `kani::stub`, and one that assumes the two drawn values are equal.
12. Read the result's refinement obligation: the production subject's symbol, the shadow, the
    bounded domain, the abstraction function and the comparison's observables; locate the
    abstraction function's module and the imports of the shadow, the expectation and the harness
    (only `core`, `alloc`, `std` and `kani`).
13. Run the refinement over the stand-in subject unmutated, over one that charges one extra
    `equality.pair`, over one that refuses a legal pair, over one that returns the other
    verdict, and over one whose environment refuses to construct a value the shadow draws, reading
    each case outcome.
14. Run the refinement over a domain of exactly the case cap, one case larger, and one whose case
    count overflows `u128`, twice each with equal inputs and one run with another seed, reading the
    class, the case count, the cases run, the seed, that a sampled run executed the boundary cases
    (one leaf at a time, plus all-lower and all-upper, not a cross product) and exactly the
    request's sample size of seeded cases, and that the same seed repeats them.
15. Read the proof strength of a verified `production` harness; of a verified `bounded_shadow`
    harness with an exhaustive refinement, a sampled one, none and one stopped at a ceiling; and of
    a verified shadow harness beside a refinement disagreement.
16. Read the identity's abstractions, unexercised behaviours and shared helpers with their
    `value_bearing` or `order_only` marks; generate a shadow holding an abstraction no evidence
    discharges; mutate each `value_bearing` shared helper (a bound, a presence) and run the shadow
    harness, the refinement and the family's independent check together; permute an `order_only`
    helper and read that no check is required to fail.
17. Request a refinement domain narrower than the shadow's drawn domain on one argument.
18. Request items whose shape is unbounded, has no shadow, or exceeds the size budget, beside a
    supported item.
19. SUPERSEDED by steps 23 to 26 (ADR-007). It drove a runner-side memory mechanism (a resident-
    memory observer, a limit-only mechanism, a platform with none and a batch killed by the
    runner), which the minimal design removes.
20. Run the seeded mutants of the stand-in: a shadow that always answers `true`, a production
    subject that never compares one leaf, a mutated `value_bearing` shared helper that the
    production subject reads independently, and one it reads through the same helper.
21. Read the evidence of a shadow run and of a refinement run for the Kani, Rust and solver
    versions, the unwind bound and the option vector of each phase the run launched; for a
    falsified run read both the proof vector and the witness vector bound to one harness
    identity, and for a verified run only the proof vector.
22. Run the refinement with a stand-in subject that outruns its wall-clock ceiling and one that
    exceeds its memory ceiling; read the refinement evidence (class, cases, seed, ceilings,
    mechanism, outcome, versions) beside the shadow run's.

### The environment-owned memory cap (IR-1510, ADR-007)

23. Run a launcher stand-in whose report states the harness entry with exit status `out_of_memory`
    (the shape Kani writes when CBMC is killed by SIGKILL), and a stand-in that is killed by SIGKILL
    and writes no report. Read each run's outcome.
24. Run a launcher stand-in that exits with a verified report, through the `PeakMemorySource` seam
    (FR-028-AC-21) injected with a source named `test_peak_source` that returns 123456789 bytes, and
    read the evidence. Inject a source that reports 1048576 KiB and read the evidence. Inject a source
    that returns nothing and read the evidence and the outcome.
25. Run a launcher stand-in with an identity memory ceiling of 1 MiB that allocates 64 MiB and exits
    with a verified report, and read the outcome. Run a run on a stand-in environment that offers no
    memory interface at all, and read that it is not refused.
26. Run the structural scan of TC-043 step 17, with its one token list and its one file set (the
    non-test, non-comment source files of `src/kani/run/`, `#[cfg(test)]` modules, comments and
    `#[doc]` attributes excluded).

## Expected Results

1. Each ceiling change changes the identity (FR-028-AC-1).
2. The run is stopped and is `inconclusive` with the timed-out reason naming the ceiling, never
   verified or falsified (FR-028-AC-2).
3. See step 23: the environment's memory stop is classified from the backend's report, naming the
   ceiling, and never as `timed_out` or as a verdict (FR-028-AC-25; FR-028-AC-3 is retired).
4. Each evidence records every argument's bounds and both ceilings (FR-028-AC-4).
5. Each names the family (FR-028-AC-5).
6. The harness records proof subject `production` (FR-028-AC-6).
7. The result holds a `bounded_shadow` harness together with its refinement obligation
   (FR-028-AC-7).
8. The identity and the evidence record the declared domain, the narrowed bound and that the
   harness covers only the narrowed bound (FR-028-AC-8).
9. The obligation is refused with a typed reason and no harness (FR-028-AC-9).
10. The timed-out member is `inconclusive` timed-out naming T and the finished members keep their
    results; the second batch is killed, refused as timed out and no member is classified; the
    third never elapses and does not panic; each oversized T launches without `--harness-timeout`
    and the batch runs, its outer bound elapsing at N times T where the product fits and never where
    it does not (FR-028-AC-12).
11. The unmutated shadow source has no runtime path, no stub and no assumption beyond the domain
    bounds, and each mutated generator fails the check it breaks (FR-028-AC-13).
12. The obligation names the subject, shadow, domain, abstraction function and comparison; the
    abstraction function's module is imported by neither the shadow, the expectation nor the
    harness (FR-028-AC-14); the shadow source names no crate but `core`, `alloc`, `std` and `kani`
    (FR-028-AC-13).
13. The unmutated subject agrees on every case; the extra charge, the refused legal pair, the
    other verdict and the construction refusal are each a recorded disagreement carrying the case and both results, none
    skipped (FR-028-AC-15).
14. The domain of exactly the cap is `exhaustive` with every case run; the larger and the
    overflowing domains are `sampled` with the boundary cases plus exactly the sample size of
    seeded cases, the seed and the cases run recorded, and equal inputs run equal cases
    (FR-028-AC-16).
15. The proof strengths read as FR-028-AC-17 states, and no `bounded_shadow` result reads
    `production_proved`.
16. The identity lists each abstraction with its evidence, each unexercised behaviour and each
    shared helper with its mark; the undischarged abstraction is refused naming it with no
    harness; each `value_bearing` helper's mutation fails the shadow harness, the refinement or
    the independent check, and an `order_only` permutation is exempt (FR-028-AC-18).
17. The narrower refinement domain is refused naming the argument, with no harness (FR-028-AC-19).
18. Each of the three shapes is refused with its disposition and reason, none emits a harness or a
    `production` fallback, and the supported item settles as it does alone (FR-028-AC-20).
19. SUPERSEDED by steps 23 to 26 (ADR-007); nothing to read.
20. The always-true shadow is `Falsified`, the never-comparing subject settles `refinement_failed`,
    the independently read helper mutant settles `refinement_failed` and the same-helper mutant is
    caught by the independent check; the unmutated stand-in verifies and agrees (FR-028-AC-22).
21. Each evidence records the versions, the unwind bound and the complete option vector of the run
    that produced it, and for a two-phase run of both phases (FR-028-AC-23, FR-017-AC-63).
22. Each stopped refinement is stopped at its ceiling, records it, and yields
    `shadow_proved_refinement_inconclusive`; the refinement evidence is recorded beside the shadow
    run's (FR-028-AC-24).

23. The run with the `out_of_memory` report is `FailedWithoutCounterexample` with `failureCause`
    `out_of_memory`, and its terminal value is `Incomplete(ResourceExhausted)`; it is not `TimedOut`
    and not a verdict. The SIGKILLed run with no report is `NoVerdict` with terminal value `Failed`,
    not `Incomplete(ResourceExhausted)` (FR-028-AC-25, FR-029-AC-34). The classification half passes
    on main as of this specification; the control that the runner measures nothing is step 25.
24. With the first source, the evidence records exactly 123456789 bytes and the name
    `test_peak_source`; with the KiB source it records exactly 1073741824 bytes (a mutant that stores
    1048576 fails); with the source returning nothing it records no peak (the member is absent or
    null, not zero), and the run is not refused and classifies as it otherwise would. A peak taken
    from the launcher process's own resident size, an estimate, or a zero fails this step
    (FR-028-AC-21).
25. The over-allocating stand-in is `Verified`: the runner measured nothing and killed nothing, so
    the environment, not the runner, is the only thing that could stop it. The stand-in environment
    with no memory interface runs and is not refused for lack of a memory mechanism (FR-028-AC-21).
26. The scan finds none of the tokens. Before IR-1510 it found `procfs` and
    `pidfd` in `memory.rs`; they are deleted (FR-028-AC-21, FR-017-AC-67). The
    macOS property is verified when the suite runs on a macOS host and is not claimed by the Linux
    CI.

## Status

Partially implemented. Steps 1 to 4 (FR-028-AC-1 to FR-028-AC-4) have required request ceilings
on the generated contract, scalar and state-frame identities. The runner reads those identity
ceilings, and its evidence records them together with each symbolic argument's bounds. The V1
bundle proof graph and the canonical corpus identity also record required ceilings; this does not
claim execution coverage for the bundle and corpus tests that invoke Kani directly.

The focused tests backing the ceiling slice are:

- `changing_either_request_ceiling_changes_the_generated_proof_identity`
- `routed_scalar_identity_records_both_required_ceilings`
- `both_state_obligation_identities_change_with_either_required_ceiling`
- `bundle_proof_record_requires_and_records_each_request_ceiling`
- `either_ceiling_changes_the_canonical_corpus_identity`
- `identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory`
- `zero_wall_ceiling_cannot_accept_an_already_exited_launcher`
- `expired_dispatch_deadline_starts_no_helper`
- `child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree`
- `batch_memory_overage_refuses_every_member_without_classifying_a_partial_report`
- `unequal_identity_memory_ceilings_run_in_separate_backend_processes`
- `unavailable_tree_memory_observation_refuses_before_spawn`
- `available_observer_does_not_invent_a_peak_before_observing_a_tree`
- `resident_memory_of_a_reused_pid_is_excluded_from_the_backend_sample`
- `released_address_space_is_observed_before_zombie_status_but_missing_rss_is_refused`
- `released_leader_mm_uses_live_worker_rss_or_refuses_observation`
- `missing_task_ancestry_refuses_a_zombie_leader_with_live_workers`
- `unavailable_children_observation_refuses_and_only_disappeared_tasks_are_skipped`
- `startup_cap_refuses_without_claiming_the_identity_wall_ceiling_elapsed`
- `unranged_state_draws_record_the_full_i64_domain`
- `gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend`
- `completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample`
- `observation_failure_after_spawn_refuses_a_valid_report_and_stops_the_run`

Step 10 (FR-028-AC-12, the batch wall-clock rule) remains implemented (IR-277), backed by the
`tc_043_*` batch tests listed in TC-043. Batching now also requires equal identity memory and
wall-clock ceilings.

The process-tree part of step 19 (the former FR-028-AC-21) once ran with a Linux PID-namespace owner
and a procfs resident-memory observer. ADR-007 withdrew that design and IR-1510 deleted it and its
memory-overage, observer, procfs, ancestry and gated-startup fixtures: FR-028-AC-21 is now
environmental. The identity-ceiling and expired-deadline fixtures above are unaffected. Steps 23 to 26 are implemented under
IR-1510: `execute.rs` (`identity_ceilings_govern_execution_and_the_runner_enforces_no_memory_cap`, `the_evidence_records_the_named_peak_the_environment_provides_in_bytes`, `an_environment_memory_kill_is_classified_from_the_report_only`), `two_phase_tests.rs` (`a_witness_phase_killed_by_the_environment_is_no_verdict`, `a_witness_report_stating_out_of_memory_is_failed_without_counterexample`) and `tests/it/kani_run_scan.rs`.

Steps 5 to 9 and steps 11 to 18 and 20 to 22 remain planned. The ceiling slice supplies no shadow,
refinement obligation, proof strength, family/proof-subject field or tool-version evidence. In
particular, process-tree memory enforcement does not complete the shadow contract or IR-241's
stand-in refinement and seeded-mutant obligations.

# IR-917 Kani-backed concrete playback at 999

This file is the durable record behind FR-017-AC-60 and the playback part of FR-015-AC-41. The tests are
`tc_027_kani_playback_at_999_*` of `tests/it/kani_obligations_v2_clause_controls.rs`; the public entry is
`execute_kani_concrete_playback` (FR-017-AC-57 to AC-59), whose refusal and bound controls run in the default
lane (`concrete_playback_refuses_unsealed_sources_and_bounds_the_launch` in `src/kani/run/execute.rs` and
`the_playback_entry_launches_only_for_the_sealed_artifact_with_one_appended_test`).

## What is exercised

- Fixture, harness, subject and owners are those of `ir1509-kani-clause-controls.md`: the clause
  `result = (amount < 1000 implies amount + 1 <= 1000)` over `amount: Int[0, 1000]` compiled from the QSL source
  of `Twin::with_bounded_amount_implication`, the sealed harness `negotiate_kani_native_obligations` generates,
  and the test-written `crate::subject` calling `quire_contract_runtime::operators::implies_short_circuit`.
  The plus-one mutant is the isolated copy of the locked quire-exact checkout with
  `left.0.checked_add(right.0)?.checked_add(1)`.
- The playback test is appended inside the generated module immediately before its closing brace. The entry
  refuses unless removing exactly those bytes restores the sealed artifact in `src/lib.rs`, then runs
  `cargo kani playback --lib -Z function-contracts -Z concrete-playback -- <module>::<test> --exact --nocapture`
  through the proof launcher. The test body is `kani::concrete_playback_run(vec![999_i64.to_le_bytes().to_vec()], prove_clause)`,
  so Kani's own replay feeds 999 through the unchanged `kani::any` draw of the sealed harness.
- Memory: the installation carries a test-side `PeakMemorySource` reading the `memory.peak` of the cgroup scope
  the test process runs in, so each outcome holds an observation (source `cgroup-memory.peak`). The figure is
  cumulative over the scope, so for the mutant test it includes the preceding two-phase proof run.

## Executed runs (2026-11-12)

Kani 0.68.0, rustc nightly-2026-08-21 for the backend, test binary built with 1.98.1, each test one at a time under
the cargo slot lock in

    systemd-run --user --scope -q -p MemoryMax=24G -p MemorySwapMax=0 timeout 1800 <it test binary> --ignored --exact <module>::<test> --nocapture --test-threads=1

| Test | Playback outcome | Playback test line | Playback wall | cgroup peak at the end of the playback | Test wall |
| --- | --- | --- | --- | --- | --- |
| `..._passes_the_healthy_clause` | `Completed`, `succeeded` true, exit 0 | `playback_999_healthy ... ok`, `1 passed; 0 failed` | 0.3 s | 232,689,664 B | 0.4 s |
| `..._fails_the_arithmetic_assertion_under_the_plus_one_owner_mutant` | `Completed`, `succeeded` true, exit 0 | `playback_999_arithmetic - should panic ... ok`, `1 passed; 0 failed`; the panic names `arithmetic subterm agrees with independent i128 expectation` | 1.8 s | 11,735,691,264 B (scope total with the proof run) | 134.0 s |
| `..._controls_do_not_satisfy_the_arithmetic_expectation`, unmutated owner | `Completed`, `succeeded` false, exit 1 | `test did not panic as expected` | 0.2 s | 184,827,904 B | 0.6 s |
| same test, compiled-subject `amount + 2 <= 1000` and no preamble | `Completed`, `succeeded` false, exit 1 | panic at the produced-result comparison, `panic did not contain expected string` | 0.2 s | 185,163,776 B | |

The mutant test runs first the two-phase `execute_kani_obligation` run of the same sealed artifact in the same
scratch crate (FR-017-AC-63): `Falsified` from the witness phase in 131.9 s, failing check 74 (the generated
arithmetic assertion), in-domain playback 2, 2457 successful checks of 2485, proof phase `Inconclusive` with
`FailedWithoutCounterexample`. The backend chose 2 for the unconstrained run; the deterministic 999 control is the
playback above. The first cold run of the healthy playback took 4.3 s for the playback build.

The symbolic run of the `amount + 2` subject (playback exactly 999) is FR-015-AC-151.

Rerun with the current test code (same command, 24 GiB scope, one test at a time): both installed playbacks passed
again (healthy 4.1 s, scope peak 800,899,072 B; mutant two-phase `Falsified` 134.2 s then 999 playback
`should panic ... ok` 2.5 s, scope peak 11,749,707,776 B), and the two control playbacks each exited 1 (0.2 s). Owned teardown (FR-017-AC-60) is asserted after every installed playback by
`assert_no_process_of_playback`: `pgrep -f` finds no live process whose command line contains the run's target
directory (cargo, rustc and the playback test binary carry it in their paths) or the exact playback test name (the test
binary's arguments). The scratch crate directory is only a working directory and is deliberately not the needle.
`the_teardown_check_sees_a_live_process` runs the check against a live process carrying both needles and requires it to
fail, then to pass after the process is stopped.
It passed in 0.03 s, the check reporting the live `sh` process before it was stopped.

## Impossible-precondition control (FR-015-AC-126)

`tc_025_real_kani_runner_reports_an_impossible_precondition_as_cover_unsatisfied` lowers the QSL precondition
`other > 1000` over `other: Int[0, 1000]` and runs the installed Kani: `CoverUnsatisfied { satisfied: 0, total: 1 }`
in 6.2 s, 1536 successful checks of 1558, no failed check, under the 24 GiB scope. It is neither resource exhaustion
nor a vacuous `Verified`.

## Produced-result domain (FR-025-AC-13)

The QSL postcondition `result >= 0` over the declared result `Int[0, 1000]` is lowered to a result-bearing harness
whose subject `check() -> i64` draws nothing. Installed Kani 0.68.0, 24 GiB scope, one at a time:

| Test | Subject returns | Outcome | Wall |
| --- | --- | --- | --- |
| `tc_036_real_kani_runner_verifies_an_in_domain_produced_result` | 1000 | `Verified`, 1544 of 1566 checks successful, no failure | 6.0 s (earlier run; the test body is unchanged) |
| `tc_036_real_kani_runner_falsifies_an_out_of_domain_produced_result` | 1001 (satisfies the clause) | `Falsified`, exactly one failed check, whose counterexample names "the produced V2 result lies in its admitted domain" and not the clause assertion, 1528 successful of 1566 | 5.2 s (rerun with the current assertion) |

## Not established

- The 16 GiB cap was not exercised; the runs used the 24 GiB cgroup.
- A subject compiled by the pipeline from QSL source (owner IR-1509, parked behind QSL-1899, which waits on IR-1528).
- The IR-948 V2 pre/post/invariant profiles of FR-015-AC-126 (owner IR-948).
- The memory ceiling is not applied by the runner (ADR-007), so the playback outcome records none.
- The playback peak is a scope-cumulative figure, not a per-process maximum.

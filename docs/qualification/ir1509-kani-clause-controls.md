# IR-1509 clause controls: exhaustive enumeration and installed Kani runs

This file is the durable record behind FR-015-AC-132, FR-015-AC-151 and FR-017-AC-64. It extends
`ir1022-kani-clause-measurements.md`, whose raw `cargo kani` runs of a hand-written subject it
supersedes for the controls it names. The tests are in `tests/it/kani_obligations_v2_clause_controls.rs`.

## What is exercised

- Fixture: clause `result = (amount < 1000 implies amount + 1 <= 1000)` over `amount: Int[0, 1000]`,
  compiled from the QSL source of `Twin::with_bounded_amount_implication` and generated through
  `negotiate_kani_native_obligations` (subject path `crate::subject::check`, unwind 3).
- Subject: the module `crate::subject` appended beside the sealed artifact; it calls
  `quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 1 <= 1000)`.
  The runner's cargo build compiles it. CG and QSL have no step that emits a subject from QSL source
  (the QSL operation declares no body), so the subject is written by the test.
- Owners: quire-exact and quire-contract-runtime at the commits CG's `Cargo.lock` records. A mutant copies
  the locked checkout into the scratch crate (`owner-mutant/`), edits the copy, and a `[patch]` entry
  resolves the dependency to it; the test asserts `cargo metadata` resolves the owner to that copy. The
  shared checkouts are never edited.

## Exhaustive host enumeration (FR-015-AC-132, FR-017-AC-64)

No Kani launch. The sealed artifact has the enumeration test appended inside the generated module
(removing those bytes restores the sealed artifact exactly), only the `#[cfg(kani)]` and
`#[kani::proof]` lines are removed, and a host stand-in for `kani` replays the queued value through the
unchanged `kani::any` draw (`assume(false)` panics, `cover!` does nothing). Each value 0, 1, ..., 1000 runs the
harness body under `catch_unwind` and then compares the compiled subject with the independent `i128`
expectation `!(a < 1000) || (a + 1 <= 1000)`; nothing stops early. The printed record states
`evidence_kind` `exhaustive_concrete_enumeration`, `symbolic_proof` false, `domain_size` 1001, `sequence_exact`,
`checked`, `passed`, `failing_inputs` and the clause identity, and the wording check rejects the listed
tokens.

| Run | Site | `failing_inputs` | `checked` | `sequence_exact` |
| --- | --- | --- | --- | --- |
| healthy | none | empty | 1001 | true |
| subject consequent `amount + 2 <= 1000` | compiled subject | {999} | 1001 | true |
| subject consequent `amount + 1 < 1000` | compiled subject | {999} | 1001 | true |
| subject antecedent `amount <= 1000` | compiled subject | {1000} | 1001 | true |
| helper body returns `false` | isolated quire-contract-runtime copy, `src/operators.rs` | 0 through 1000 | 1001 | true |
| `left.0.checked_add(right.0)?.checked_add(1)` | isolated quire-exact copy, `src/native_integer.rs` | 0 through 999 | 1001 | true |
| `Some(left.0)` | isolated quire-exact copy, `src/native_integer.rs` | 0 through 999 | 1001 | true |

Enumeration mutants: omitting 999 gives `checked` 1000 and `sequence_exact` false; visiting 998 twice and
omitting 999 gives `checked` 1001 and `sequence_exact` false; a descending visit is not exact. A record
renderer that prints `Proved`, `proved`, `Verified`, `verified` or `Kani proof`, or sets `symbolic_proof` true,
fails the wording check. The eight default-lane tests of the module ran in 11.42 s.

## Sealed playback at 999

The playback test is appended inside the generated module before its closing brace and expects the panic
message of the generated arithmetic assertion (`arithmetic subterm agrees with independent i128 expectation`).
Against the plus-one kernel mutant it passes (`playback_999 - should panic ... ok`). Against the unmutated
owner it fails (`test did not panic as expected`). The compiled-subject `amount + 2` mutant, run without the
preamble that first calls the subject at 999, reaches the sealed harness body and fails with the
produced-result message (`the produced V2 Boolean result agrees with the independent checked condition`) and
`panic did not contain expected string`, so it does not satisfy the control. It runs on the host through the
stand-in above, not through Kani's `concrete_playback_run`; the Kani-backed playback via
`execute_kani_concrete_playback` is FR-017-AC-60, delivered by IR-917 (`ir917-kani-playback.md`).

## Installed Kani runs through CG's runner (FR-015-AC-151)

`execute_kani_obligation` (the FR-017-AC-63 two-phase run) with Kani 0.68.0 (CBMC 6.11.0), rustc
nightly-2026-08-21, `-Z function-contracts --exact --unwind 3 --solver cadical`. The `Makefile` kani target
applies no memory cap and the runner enforces none (ADR-007); the cap below was applied by the harness script
that ran each test, one run at a time under the cargo slot lock:

    flock /tmp/cargo-slot.lock systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0 \
        timeout 1800 <it test binary> --ignored --exact <module>::<test> --nocapture --test-threads=1

The test binary was built first, outside the scope. Wall is the whole test process after the build, both phases
(the in-test wall is a second or two lower). Peak is the scope's cgroup `memory.peak` sampled every 2 s; the
runner records no memory observation without an environment source, so the evidence has none. Recorded run of
2026-11-12 (the first run of 2026-11-11 gave the same verdicts and playbacks within 5 s and 0.2 GB).

| Run | Verdict | Failing check (id) | Playback | Successful checks of total | Wall | cgroup peak |
| --- | --- | --- | --- | --- | --- | --- |
| healthy | `Verified`, one proof-phase launch, cover satisfied | none | none | 2458 of 2485 | 43.5 s | 2.80 GB |
| exact-999 subject mutant `amount + 2 <= 1000` | `Falsified` (witness phase) | produced V2 Boolean result agrees with the independent checked condition (350) | 999 | 2457 of 2485 | 129.8 s | 11.60 GB |
| kernel `left.0.checked_add(right.0)?.checked_add(1)` | `Falsified` (witness phase) | arithmetic subterm agrees with independent i128 expectation (74) | 2 | 2457 of 2485 | 131.1 s | 11.64 GB |
| kernel `Some(left.0)` | `Falsified` (witness phase) | arithmetic subterm agrees with independent i128 expectation (71) | 0 | 2457 of 2485 | 125.1 s | 11.19 GB |
| helper body returns `false` | `Falsified` (witness phase) | produced V2 Boolean result agrees with the independent checked condition (131) | 1000 | 2449 of 2478 | 119.6 s | 10.45 GB |

Each test logs one `IR1509` line with the outcome, the successful and total check counts, the failed check
ids and the proof-phase outcome, and one line with the playback values; the table is taken from those lines.
For each `Falsified` run the test asserts: the proof phase nested in the evidence with outcome `Inconclusive`
`FailedWithoutCounterexample` (the proof vector prints no playback), `concrete-playback` only in the witness
vector, equal ceilings in both phases and equal to the identity's, one harness path, the named failing check in
the counterexample, and every playback value within 0..=1000 (exactly 999 for the exact-999 mutant, below 1000
for the kernel mutants). The backend picks the in-domain counterexample (2 and 0 for the kernel mutants, 1000 for
the helper); the deterministic 999 control is the sealed playback. The healthy test asserts `Verified`, no
proof-phase evidence and a satisfied cover. The check counts and the helper playback are logged, not asserted.

## Not established

- The 16 GiB cap itself was not exercised; the runs used the 24 GiB cgroup the planner and owner allowed.
- Program-step counts are not part of the runner's evidence and were not recorded.
- PLANNED, blocked on QSL operation bodies: a subject compiled by the pipeline. The subject is test-written;
  no CG or QSL step emits one because a QSL operation cannot carry a body (planner ruling 2026-11-12).
- The 999 playback here is a host stand-in run; the Kani-backed one is `ir917-kani-playback.md` (IR-917).

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
fails the wording check. All seven default-lane tests of the module ran in 10.07 s.

## Sealed playback at 999

The playback test is appended inside the generated module before its closing brace and expects the panic
message of the generated arithmetic assertion (`arithmetic subterm agrees with independent i128 expectation`).
Against the plus-one kernel mutant it passes (`playback_999 - should panic ... ok`). Against the unmutated
owner it fails (`test did not panic as expected`), and against the `amount + 2` subject mutant it fails too (that
mutant fails the produced-result comparison, not the arithmetic assertion). It runs on the host through the stand-in
above, not through Kani's `concrete_playback_run`; the Kani-backed playback via `execute_kani_concrete_playback`
is FR-017-AC-60 (IR-917).

## Installed Kani runs through CG's runner (FR-015-AC-151)

`execute_kani_obligation` (the FR-017-AC-63 two-phase run) with Kani 0.68.0 (CBMC 6.11.0), rustc
nightly-2026-08-21, `-Z function-contracts --exact --unwind 3 --solver cadical`, one run at a time under the
cargo slot lock, each in its own `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0` with a
`timeout 1800` cap. Wall is the whole test (both phases) after the test binary was built. Peak is the scope's
cgroup `memory.peak` sampled every 2 s; the runner records no memory observation without an environment source,
so the evidence has none.

| Run | Verdict | Failing check | Playback | Wall | cgroup peak |
| --- | --- | --- | --- | --- | --- |
| healthy | `Verified`, one proof-phase launch, 2458 successful checks | none | none | 44.1 s | 2.80 GB |
| exact-999 subject mutant `amount + 2 <= 1000` | `Falsified` (witness phase) | produced V2 Boolean result agrees with the independent checked condition | 999 | 129.2 s | 11.61 GB |
| kernel `left.0.checked_add(right.0)?.checked_add(1)` | `Falsified` (witness phase) | arithmetic subterm agrees with independent i128 expectation | 2 | 135.0 s | 11.73 GB |
| kernel `Some(left.0)` | `Falsified` (witness phase) | arithmetic subterm agrees with independent i128 expectation | 0 | 125.3 s | 11.19 GB |
| helper body returns `false` | `Falsified` (witness phase) | produced V2 Boolean result agrees with the independent checked condition | 1000 | 116.8 s | 10.65 GB |

For each `Falsified` run the test requires: the proof phase nested in the evidence (its outcome is
`Inconclusive` `FailedWithoutCounterexample`, since the proof vector prints no playback), `concrete-playback`
only in the witness vector, equal ceilings in both phases and equal to the identity's, one harness path, the named
failing check in the counterexample, and every playback value within 0..=1000. The backend picks the
in-domain counterexample (2 and 0 for the kernel mutants); the deterministic 999 control is the sealed playback.

## Not established

- The 16 GiB cap itself was not exercised; the runs used the 24 GiB cgroup the planner and owner allowed.
- Program-step counts are not part of the runner's evidence and were not recorded.
- The subject is test-written, not emitted from QSL source (see above).
- The 999 playback is a host stand-in run (see above), not a Kani playback.

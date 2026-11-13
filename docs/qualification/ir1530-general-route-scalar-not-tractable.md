# IR-1530 general-route (`rt::Integer`) scalar harness: measured not tractable

This records why the real-Kani scalar lane tests moved from the general route to the native-integer
route, and what each ends with there. It follows the IR-1022 recipe (`ir1022-kani-clause-measurements.md`)
and the native guarded harness record (`ir304-kani-guard-native-harness.md`, which carries the
earlier statement of these timeouts).

## Measured row: general integer scalar route

Row in the shape of the measured tractability table (FR-056 "Measured table"):

| Field | Value |
| --- | --- |
| operation_ids | `quire.op.integer.add`, `quire.op.integer.sub` |
| scalar form | routed exact-scalar harness from `generate_routed` (operands and oracle result as `rt::Integer`), `x + 1` over `Int[0, 9]` and `x - y` over two `Int[0, 9]` |
| max_operand_bits | 4 (every operand bound is at most 9) |
| collection_bound | none |
| verdict | `not_tractable` |
| wall_seconds | 600 (the lane budget; `Inconclusive { TimedOut }`) |
| peak_bytes | about 14.5e9 at 8 min (add); about 15.0e9 at 600 s (sub) |
| Kani / CBMC / solver | 0.68.0 / 6.11.0 / cadical, `--unwind 3` |
| generator version | quire-contract-codegen 0.1.0 at main bb3e6176 |
| kernel version | quire-exact e82db487 (Cargo.lock) |
| host, date | titan, 2026-11-12 |

Evidence for the row:

- Base, nothing forgotten: `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`,
  `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` and
  `kani_generation_real_v2_scalar_verifies` end `Inconclusive { TimedOut }` at 600 s on titan
  (`ir304-kani-guard-native-harness.md`; also reproduced by the IR-1530 reporter and, for the
  violating-bound test, at 600.24 s by an IR-304 reviewer). The subtraction receipt test
  `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` ended
  `ProofNotVerified(Inconclusive { TimedOut })` at 600.33 s, cgroup peak 15.0 GB, in the IR-1530
  review. These base timeouts were not re-run by the IR-1530 coder.
- IR-1022 recipe applied to the general route (probe, this PR). Edit to the generated tail in
  `src/kani/generate/scalar.rs`: the guard `if lowered.native_integer_selection.is_some() {` around
  `Statement::Forget("meter")` and `Statement::Forget("outcome")` was replaced by a bare block, so
  the general harness also ends `let cover_reached: bool = completed;`, `core::mem::forget(meter);`,
  `core::mem::forget(outcome);`, `kani::cover!(cover_reached, ..)`. The lane test
  `kani_generation_real_v2_scalar_verifies` was pointed at the general fixture
  (`routed_scalar_increment`) for the probe. Run: foreground, `flock /tmp/cargo-slot.lock`,
  `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, Kani 0.68.0, CBMC 6.11.0. The
  scope was killed by the kernel OOM killer after 57 s of wall time (`cbmc` anon-rss 25.0 GB; the
  user journal reports `p1530-run6-<pid>.scope: Failed with result 'oom-kill'`, `24G memory peak`).
  Both edits were reverted afterwards. An earlier probe of the same edit showed the same peak
  (`MemoryPeak` equal to `MemoryMax`) at 50 s but kept no test output; this run supersedes it.

The cost is the `rt::Integer` bignum heap shape, which no forget placement removes, so the general
integer scalar route is not tractable at unwind 3, 600 s and 24 GiB.

## Refusal of the general route: built at the run level

The row above is shipped as data in `ir1530-general-route-scalar-table.json` (the FR-056 table
shape, with `"route": "general_scalar"` so it never applies to V2 clauses or the native route). The
refusal is FR-056-AC-14: `execute_kani_obligation_decided` and `execute_kani_obligations_decided`
(`src/kani/terminal/tractable.rs`) ask the one `TractabilityDecider` seam, with features on the
`general_scalar` route, before any Kani process starts, and return
`DecidedRunRefusal::NotTractable` carrying `UnsupportedObligation::V2ClauseNotTractable` (node,
operation id, width, the row with its Kani, CBMC, solver, generator and kernel versions). It is
decided at the run, not at generation: a generation-level refusal broke the host tests that
exercise this route (below), so generation still emits the harness and the wall-clock ceiling
stays the backstop. The native-route harness is not refused.

The refusal is opt-in: the caller passes the table to the decided wrappers, and with none passed
the harness runs. CG ships the table in-crate (`TractabilityTable::shipped_general_route_scalar`,
embedded from the JSON file, which is the single source; a test pins the equality). The production
caller is quire-driver (`src/provider.rs` ~308, `src/upstream.rs` ~191, which call
`execute_kani_obligation` on `generate_routed` harnesses); it will be wired to the decided wrapper
in a quire-driver PR under IR-1530. Operand widths below the row's 4 bits on this route are
unmeasured: nothing narrower was probed, so such a harness is not refused and is left to the
wall-clock backstop.

The `solver_version` of the row, `cadical-3.0.0`, was not captured in the #816 probe logs. It is the
CaDiCaL that the same Kani 0.68.0 / CBMC 6.11.0 installation reports ("Solving with CaDiCaL 3.0.0",
TC-043 and the FR-015-AC-99 close-out run).

Blast-radius evidence for the rejected generation-level alternative. A new `UnsupportedObligation` variant (`GeneralIntegerScalarNotTractable`, a
stable serde code, carrying the node, the operation identity and `use_instead:
"generate_routed_native"`) replaced the Add, Subtract, Multiply and Negate scalars in
`negotiate_integer_policy` whenever the integer policy was not native (Implies untouched). It
compiled, then whole `--lib` and `--test it` gave 3 failed in `--lib` (551 passed) and 81 failed in
`--test it` (880 passed); the change was reverted. Approximate failing tests per file (some
shared-fixture cascade is possible):

| File | Failing tests |
| --- | --- |
| `tests/it/routed_generation.rs` | 17 |
| `tests/it/kani_obligations.rs` | 14 |
| `tests/it/scalar_playback.rs` | 10 |
| `tests/it/scalar_settlement.rs` | 9 |
| `tests/it/requirement_records.rs` | 8 |
| `tests/it/scalar_replay_acceptance.rs` | 7 |
| `tests/it/scalar_replay_binding.rs` | 4 |
| `tests/it/scalar_replay_residuals.rs` | 3 |
| `tests/it/scalar_literal_operands.rs` | 2 |
| `tests/it/cover_last.rs`, `no_generation_panics.rs`, `kani_generation.rs` | 1 each |
| `--lib`: `routed/contract_package_tests.rs`, two `scalar_native_private` tests, one `routed::smt` test | 3 |

About 50 test call sites use `generate_routed`; the crate has no other production caller of it
outside its own routing code.

What IR-1530 delivers: the typed run-level refusal above; the real-Kani scalar lane tests below run on the native route with
executed verdicts, and the general route is recorded not tractable in the table-shaped row above.

## Tests moved to the native route

Each test runs the same assertions as before on the harness `generate_routed_native` produces for
the same package. Runs: one at a time, foreground, inside `flock /tmp/cargo-slot.lock`,
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, the lane's 600 s budget; wall is
the `cargo test` run (build excluded), peak is the scope's cgroup `memory.peak`.

| Test | Verdict as asserted | Wall | Peak |
| --- | --- | --- | --- |
| `kani_generation_real_v2_scalar_verifies` | `Verified` | 7 s | 0.64 GB |
| `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` | `Falsified` | 28 s | 2.98 GB |
| `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` | healthy `Verified`, `Add` to `Subtract` mutant `Falsified` on the generated `sound` assertion | 31 s | 2.37 GB |
| `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` | two `Verified` proofs with typed native receipts | 14 s | 0.65 GB |
| `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` (both orders `[4, 1]`, `[1, 4]`) | both `Verified`, receipts `Completed(3)` and the swapped order | 16 s | 0.80 GB |
| private lib `cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal` | as asserted | 21 s | 0.98 GB |
| private lib `seeded_false_bound_harness_produces_a_paired_completion_refusal` | `Falsified`, paired `CompletionRefusal` | 28 s | 2.97 GB |
| private lib `bounded_paired_runner_propagates_the_native_helpers_oracle_stop` | as asserted | 11 s | 0.77 GB |

The three private lib controls are `#[ignore]` and outside `make kani`; they used the general
`x + 1` fixture and now use `native_fixture` in `tests/it/kani_scalar_native_private.rs`. The
test-only `false_bound_harness_for_test` now carries the original harness's native selection so the
seeded false bound is rendered on the same route as the harness it narrows.

The arithmetic mutation edits `nx::NativeIntegerOperation::Add {` to `Subtract {` in the native
oracle, since the native oracle has no `rt::IntegerArithmetic` call.

`kani_generation_real_v2_scalar_verifies` previously never reached its argument-vector assertion
(it timed out first). That assertion counted one launcher argument after the options, but the
launcher always appends `-Z unstable-options --export-json <path>`; it now states the exact vector.

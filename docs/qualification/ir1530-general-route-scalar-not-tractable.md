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
integer scalar route is not tractable at unwind 3, 600 s and 24 GiB. The refusal owner for such
scalars is pending a planner ruling (FR-056 covers V2 clauses only); no refusal mechanism is added
here.

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

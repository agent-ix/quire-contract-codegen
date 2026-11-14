# IR-1530 general-route (`rt::Integer`) scalar harness: measured not tractable

This records why the real-Kani scalar lane tests moved from the general route to the native-integer
route, and what each ends with there. It follows the IR-1022 recipe (`ir1022-kani-clause-measurements.md`)
and the native guarded harness record (`ir304-kani-guard-native-harness.md`, which carries the
earlier statement of these timeouts).

## Measured row: general integer scalar route

Rows in the shape of the measured tractability table (FR-056 "Measured table"). The shipped table
holds two rows, `quire.op.integer.add` from 2 bits and `quire.op.integer.sub` from 4 bits, the
smallest width at which each was observed to time out:

| Field | add row | sub row |
| --- | --- | --- |
| operation_ids | `quire.op.integer.add` | `quire.op.integer.sub` |
| scalar form | routed exact-scalar harness from `generate_routed` (operands and oracle result as `rt::Integer`), `x + 1` | `x - y` over two `Int[0, 9]` |
| max_operand_bits | 2 (smallest observed, below) | 4 (every operand bound is at most 9) |
| collection_bound | none | none |
| verdict | `not_tractable` | `not_tractable` |
| wall_seconds | 600 (the lane budget; `Inconclusive { TimedOut }`) | 600 (same) |
| peak_bytes | 14602888806 (13.6 GiB, the systemd scope peak of the 2-bit probe, build included; not the runner's) | about 15.0e9 at 600 s |
| Kani / CBMC / solver | 0.68.0 / 6.11.0 / cadical-3.0.0, `--unwind 3` | same |
| generator version | quire-contract-codegen 0.1.0 | same |
| kernel version | quire-exact `KERNEL_VERSION` 1 | same |
| host, date | titan, 2026-11-12 (4 bits); 2026-11-13 (2 bits) | titan, 2026-11-12 |

The add row's width was first 4 bits (`Int[0, 9]`). The driver's Kani-lane test
`successful_native_receipt_is_authenticated_against_retained_limits` (quire-driver `drive.rs`) on the
general route with `quire.op.integer.add` over `Int[4, 4]` (`interval_bits(4, 4)` = 3 bits) ended
`ProofNotVerified` at 600 s in its coder's measured run, while the table let it run. One further probe
under this ticket (follow-up to #821) ran the general-route `x + 1` over `Int[2, 2]` (operands
`left` = `Int[2, 2]`, 2 bits, and `right` = `Int[1, 1]`, so the largest width is 2 bits) through
`execute_kani_obligation` and the lane budget: foreground, `flock /tmp/cargo-slot.lock`,
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, Kani 0.68.0, CBMC 6.11.0. It ended
`Inconclusive { reason: TimedOut }` at 600.04 s; the user journal reported the scope's `13.6G memory peak`
(systemd's G is GiB, so about 1.46e10 bytes, build included; the runner reports no peak). The driver's
3-bit scope reported 13.8G. The probe is the ignored test `ir1530_probe_general_route_two_bit_add`
(`tests/it/kani_obligations.rs`; fixture `left` `Int[2, 2]`, `right` `Int[1, 1]`). The add row is therefore at
2 bits. Subtraction was measured only at 4 bits and keeps its 4-bit row. One bit (`Int[1, 1]` operands)
and anything else narrower is unmeasured: it is not refused and is left to the wall-clock backstop.

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
shape, with `"route": "general"`, the FR-056 route of the `rt::Integer` carrier, so it never applies to the native route). The
refusal is FR-056-AC-15: `execute_kani_obligation_decided` and `execute_kani_obligations_decided`
(`src/kani/terminal/tractable.rs`) ask the one `TractabilityDecider` seam, with features on the
`general` route, before any Kani process starts, and return
`DecidedRunRefusal::NotTractable` carrying `UnsupportedObligation::V2ClauseNotTractable` (node,
operation id, width, the row with its Kani, CBMC, solver, generator and kernel versions). It is
decided at the run, not at generation: a generation-level refusal broke the host tests that
exercise this route (below), so generation still emits the harness and the wall-clock ceiling
stays the backstop. The native-route harness is not refused only because the shipped table holds no
`native` row: the decided wrappers ask the decider on the harness's own route, so a `native` row
would refuse it.

Rows apply per route across both shapes, a V2 clause at generation and a routed scalar harness at
the run. What the row measures is the `rt::Integer` carrier, not the shape, so the same
`general` add row would refuse a general-route V2 clause. The shipped scalar row is therefore
scoped by use, not by schema: it is embedded for the decided run wrappers and must not be passed as
a generation `tractability_table`.

The refusal is opt-in: the caller passes the table to the decided wrappers, and with none passed
the harness runs. CG ships the table in-crate (`TractabilityTable::shipped_general_route_scalar`,
embedded from the JSON file, which is the single source; a test pins the equality). The production
caller is quire-driver (`src/provider.rs` ~308, `src/upstream.rs` ~191, which call
`execute_kani_obligation` on `generate_routed` harnesses); it will be wired to the decided wrapper
in a quire-driver PR under IR-1530. Operand widths below a row's bits on this route (below 2 bits for
add, below 4 for sub) are unmeasured: nothing narrower was probed, so such a harness is not refused
and is left to the wall-clock backstop.

The `solver_version` of the row, `cadical-3.0.0`, was not captured in the #816 probe logs. It is the
CaDiCaL that the same Kani 0.68.0 / CBMC 6.11.0 installation reports ("Solving with CaDiCaL 3.0.0"
the FR-015-AC-99 close-out run is the real run record; the TC-043 mention is fixture text, not a run log).

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

## IEEE float arithmetic rows (IR-260)

The table also holds five rows for the IEEE float arithmetic identities, with `max_operand_bits` 32
(the `binary32` width, so both widths are covered): `add` and `sub` together, `mul`, `div`, `sqrt`
and `fma`, each at `float32` and `float64`, `not_tractable`, 600 s, no peak recorded. The basis is
quire-exact FR-405's table rows (no `exact` or directed-spelling harness, and no `div`, `sqrt` or
`fma` harness, reached a verdict within 600 s) and is detailed in
`ir260-float-scalar-kani-measurements.md`. Kani's symbolic `f32` division is bit-wrong at
`0x800000A9 / 0x9C873333` (the correct result is `0x1BA00000`), which is why the `div` row has no
host-division reference. The float clause carries no integer operand width, so the decider's width
match does not select these rows; the refusal of a float arithmetic node is made at generation by
the float row (FR-053), with a named capability and no harness, and these rows are its measured
record.

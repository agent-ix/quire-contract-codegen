# IR-1530 general-route (`rt::Integer`) scalar harness: measured not tractable

This records why four real-Kani scalar lane tests moved from the general route to the native-integer
route, and what each ends with there. It follows the IR-1022 recipe (`ir1022-kani-clause-measurements.md`)
and the native guarded harness record (`ir304-kani-guard-native-harness.md`).

## Measured row: general integer scalar route

The general routed scalar harness (ordinary `generate_routed`, operands and oracle result as
`rt::Integer`) for `x + 1` over `Int[0, 9]`, unwind 3, `--solver cadical`, Kani 0.68.0:

| Variant | Verdict | Wall | Peak |
| --- | --- | --- | --- |
| base (cover read directly, nothing forgotten) | `Inconclusive { TimedOut }` at the 600 s budget | 600 s | 14.5 GB at 8 min (ir304 record) |
| IR-1022 recipe applied to the general route (`meter` and `outcome` forgotten, cover read into a `bool` first) | killed at the 24 GiB cgroup limit | 50 s | 24 GiB (limit) |

The cost is the `rt::Integer` bignum heap shape, which no forget placement removes: the general
route is therefore recorded as not tractable at this bound. Typed refusal of integer scalars that
need the general route belongs to the IR-1526 seam; no second refusal mechanism is added here.

## Tests moved to the native route

Each test runs the same assertions as before on the harness `generate_routed_native` produces for
the same package (`native_scalar_increment`). Runs: one at a time inside `flock /tmp/cargo-slot.lock`,
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, the lane's 600 s budget; wall is the
`cargo test` run (build excluded), peak is the scope's cgroup `memory.peak`.

| Test | Verdict as asserted | Wall | Peak |
| --- | --- | --- | --- |
| `kani_generation_real_v2_scalar_verifies` | `Verified` | 7 s | 0.64 GB |
| `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` | `Falsified` | 28 s | 2.98 GB |
| `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` | healthy `Verified`, `Add` to `Subtract` mutant `Falsified` on the generated `sound` assertion | 31 s | 2.37 GB |
| `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` | two `Verified` proofs with typed native receipts | 14 s | 0.65 GB |

The arithmetic mutation edits `nx::NativeIntegerOperation::Add {` to `Subtract {` in the native
oracle, since the native oracle has no `rt::IntegerArithmetic` call.

`kani_generation_real_v2_scalar_verifies` previously never reached its argument-vector assertion
(it timed out first). That assertion counted one launcher argument after the options, but the
launcher always appends `-Z unstable-options --export-json <path>`; it now states the exact vector.

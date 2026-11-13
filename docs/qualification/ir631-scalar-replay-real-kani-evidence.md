# IR-631 routed scalar replay: real-Kani evidence and the remaining IR-610 half

This records the real falsified Kani run behind FR-032-AC-7 and FR-032-AC-8 (TC-047) and says which
half of the check has no CG lane.

## Real run (2026-11-13)

Native-integer routed scalar route (`generate_routed_native`; the general route is refused as not
tractable, FR-056-AC-15, `ir1530-general-route-scalar-not-tractable.md`). Kani 0.68.0, cadical, one job in
`flock /tmp/cargo-slot.lock` inside `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`,
foreground, `timeout 600`.

| Test | Verdict | Wall | Peak (scope `memory.peak`) |
| --- | --- | --- | --- |
| `routed::scalar_native_private::native_tc025_arithmetic_mutant_falsifies` (`--ignored`) | `Falsified` on the generated `sound` assertion; playback parsed by `parse_assertion_playback` and decoded by `decode_scalar_falsification` against the original intervals: two draws, `left` in `[0, 9]`, `right == 1` | 29 s (test 29.17 s) | 2.49 GB (2,494,185,472 B) |

The mutant edits `nx::NativeIntegerOperation::Add {` to `Subtract {` in the sealed `x + 1` native harness
(`x: Int[0, 9]`, result `Int[0, 10]`). The playback decoder is the one `OriginalScalarContext::plan` uses.

## What has no CG lane

`plan` to `Diverged` to `Failed` from that same falsified run cannot be produced inside CG:

- A harness that Kani falsifies is not a CG-generated harness. `has_valid_provenance`, `scalar_replay_body`
  and `execute_kani_scalar_native::validate_request` all refuse a harness whose bytes differ from the
  generator's seal (an oracle that differs from the emitted package, `GeneratedArtifactChanged`), and
  `ScalarReplayPlan::complete` refuses any observation of a changed staged byte (FR-032-AC-4, tagged
  `scalar_replay_binding.rs`, which observes a compiled arithmetic mutant and asserts `Observation`).
- The native receipt exists only for a `Verified` proof (`ProofNotVerified` otherwise), so CG has no native
  observation of a falsified artifact.
- CG's generator emits no falsifiable scalar harness; `Diverged` therefore arises only if the driver's
  same-proved-artifact execution (IR-610 E9) observes a real lowering defect.

The QSL half is already asserted over genuine reports (`tests/it/scalar_settlement.rs`, FR-032-AC-7, AC-20 to
AC-24): `Diverged` and `GeneratedFault` give `Failed`, `Agrees` gives `Inconclusive(ScalarAgrees)`. The
falsified-run to `Diverged` join is IR-610 driver evidence.

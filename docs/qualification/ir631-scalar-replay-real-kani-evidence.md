# IR-631 routed scalar replay: real falsified Kani run joined to the settlement

Record behind FR-032-AC-3, AC-4, AC-5, AC-7 and AC-8 (TC-047, real-Kani half). The ignored Kani-lane test
`routed::scalar_replay_real_kani_private::a_real_falsified_native_mutant_plans_observes_and_settles_diverged_failed`
(`tests/it/scalar_replay_real_kani_private.rs`, tagged AC-3, AC-4, AC-5, AC-7, AC-8) runs the whole join.

## What the test does

1. Generates the QSL-emitted `x + 1` (`x: Int[0, 9]`, result `Int[0, 10]`) through the native routed route
   (`generate_routed_native`; the general route is refused as not tractable, FR-056-AC-15,
   `ir1530-general-route-scalar-not-tractable.md`).
2. Builds the mutant crate-privately. The generator never emits a falsifiable harness, and `generated(...)` alone
   binds no replay body or original package, so `plan` would refuse it with `Authority(Unavailable)`. The test
   therefore re-seals with `generated_with_scalar_limits`, then binds the original replay body
   (`bind_scalar_replay_body`) and the mutated artifact package (`bind_artifact_package`). `nx::NativeIntegerOperation::Add {`
   becomes `Subtract {` in both the harness and the emitted oracle `src/lib.rs`, so `validate_request`'s
   oracle-equality check holds. Production callers cannot do this.
3. Real Kani falsifies the mutant (`execute_kani_obligation`, the `sound` assertion).
4. `OriginalScalarContext::plan` decodes that real playback (entry limit `arguments.len() + guard_inputs.len()`)
   against the original identity and body; the vector is `[x, 1]` with `x` in `[0, 9]`.
5. The observer is the cfg(test) CG helper, not the driver. It observes the same staged bytes natively: `observe_staged_native_for_test` stages the plan's
   package, builds the observer and runs the staged oracle once at the decoded vector, with no proof. The
   observation (`x - 1`, differing from exact `x + 1`) completes the plan.
6. The genuine `qsl_replay::replay_operator_parity` returns `Diverged`; `CompletedScalarReplay::settlement` gives
   `ScalarSettlement::Parity(Diverged)`; `scalar_run_terminal_value` over the real falsified evidence gives
   `TerminalValue::Failed`.

Scalar replay proves lowering parity only: `Failed` here is a lowering fault, never a source `Refuted`.

## Reproduction

Commit: the head of PR #826 (the commit carrying this file). Toolchain: rustc 1.99.1, Kani 0.68.0 (cargo
plugin, backend CBMC 6.11.0, `kani-0.68.0` install), cadical. One job at a time, foreground:

    export CARGO_TARGET_DIR=$PWD/target
    flock /tmp/cargo-slot.lock systemd-run --user --scope -q -p MemoryMax=24G -p MemorySwapMax=0 \
      timeout 600 cargo test --lib -- --ignored --exact \
      routed::scalar_replay_real_kani_private::a_real_falsified_native_mutant_plans_observes_and_settles_diverged_failed \
      --nocapture --test-threads=1

## Runs (2026-11-13)

Peak is the scope's cgroup `memory.peak` (includes the build when the scope compiles). Output retained in
`ir631-scalar-replay-real-kani-run.txt`.

| Test | Verdict | Test time | Scope wall | Peak |
| --- | --- | --- | --- | --- |
| `a_real_falsified_native_mutant_plans_observes_and_settles_diverged_failed` | `Falsified`, plan, native observation, `Diverged`, `Failed` | 25.12 s | 25 s | 2.36 GB (2,360,565,760 B) |
| `native_tc025_arithmetic_mutant_falsifies` (decode only, FR-015-AC-126) | `Falsified`, witness decodes against original intervals | 25.24 s | 31 s | 3.59 GB (3,586,723,840 B) |

Earlier identical-source runs: 32.59 s and 2.50 GB for the join test; 29.17 s and 2.49 GB for the decode test.

## Remaining

The driver's own E9 run (IR-610) executes the artifact it proved and calls QSL between the builder and
converter. CG's contract and this test do not depend on it.

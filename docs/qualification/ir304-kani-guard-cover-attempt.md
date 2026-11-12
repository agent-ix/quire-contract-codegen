# IR-304 installed-Kani attempt on the guarded scalar harness (FR-022-AC-35)

This records a run that produced no verdict. FR-022-AC-35 and TC-033 step 28 stay Planned.

## What was run

- Harness: the routed scalar harness CG generates for `if x < y then x + 1 else 0` with
  `x: Int[5, 9]`, `y: Int[0, 4]` into `Int[0, 10]`, guarded by `(x < y, holds=true)`. It carries
  `kani::assume(left_native < guard_0);` after the operand and guard-input assumptions, so no
  input satisfies the assumptions and the `completed` cover cannot be reached. The generated
  source was copied unchanged into a scratch crate whose only dependency is quire-exact `main`
  at the commit CG's `Cargo.lock` records (the harness is self-contained; no subject is called).
- Command (Kani 0.68.0, CBMC 6.11.0), one run under the shared cargo slot lock, inside
  `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, `timeout 1800`:

      cargo kani -Z function-contracts --harness kob_scalar_integer_add_module::kob_scalar_integer_add_proof \
          --exact --unwind 3 --solver cadical --output-format regular

## Result

| Run | Verdict | Wall | cgroup memory.peak |
| --- | --- | --- | --- |
| infeasible guard `x < y` | none: killed by the 1800 s cap (exit 124) while CBMC was still in symbolic execution | 1800.7 s | 25.29 GB (at the 24 GiB scope limit) |
| feasible guard `x > y` | not run | | |

The proof body evaluates `oracle_integer_add` on `rt::Integer` (arbitrary precision) values with
unlimited meter limits, so symbolic execution unwinds the bignum allocation and drop glue of
quire-exact; the step count was never printed. The IR-1022 clause harness, which computes on
native machine integers, finished in under 100 s; this harness shape does not.

## What this does not establish

- Neither the `CoverUnsatisfied` nor the `Verified` verdict of FR-022-AC-35 was observed.
- The feasible-guard run was not started: it has the same program shape and the infeasible run
  had already exhausted the cap.

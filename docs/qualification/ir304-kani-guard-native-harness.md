# IR-304 installed-Kani verdicts on the native guarded scalar harness (FR-022-AC-35)

This is the durable record behind FR-022-AC-35 and TC-033 step 28. It supersedes the no-verdict
attempt in `ir304-kani-guard-cover-attempt.md`, which ran the same scenario on the general
(`rt::Integer`) harness.

## What changed in the harness

The routed scalar harness for integer arithmetic now follows the IR-1022 recipe (FR-015-AC-7,
`ir1022-kani-clause-measurements.md`):

- Native integer arithmetic. The routed native route (`generate_routed_native`) holds each operand
  as `nx::NativeInteger` and the oracle is the exact kernel's `evaluate_native_integer_arithmetic`
  on an admitted plan, so there is no bignum allocation or drop glue.
- The `meter` and the `outcome` (whose `Refused` arm holds a heap value) are never dropped:
  `core::mem::forget(meter); core::mem::forget(outcome);`.
- The cover stays the last and only `kani::cover!`. Its condition is read into
  `let cover_reached: bool = completed;` before the forgets, then `kani::cover!(cover_reached, ..)`.

## What was run

- Harness: the source CG generates through `generate_routed_native` for the QSL function
  `f(x: Int[5, 9], y: Int[0, 4]): Int[0, 10] { if x OP y then x + 1 else 0 }`, routed item `x + 1`
  with path condition `[(x OP y, holds=true)]`, unwind 3. `OP` is `<` (infeasible: every `x >= 5`
  exceeds every `y <= 4`) or `>` (feasible). The generated file was copied unchanged to a scratch
  crate whose only dependency is quire-exact `main` at the commit CG's `Cargo.lock` records; the
  harness is self-contained and calls no subject. Obtained by a throwaway test (not committed)
  that wrote `RoutedGeneration` harness contents to disk.
- Command (Kani 0.68.0, CBMC 6.11.0), one run at a time inside `flock /tmp/cargo-slot.lock`,
  `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, `timeout 1800`:

      cargo kani -Z function-contracts \
          --harness kob_scalar_integer_add_module::kob_scalar_integer_add_proof \
          --exact --unwind 3 --solver cadical --output-format regular

  Wall is the whole `cargo kani` invocation (compilation included) measured after the lock was
  taken; peak is the cgroup `memory.peak` of the scope, sampled every second.
- Mutants edit the scratch copy of the generated source at the named site.

## Results

| Run | Kani verdict | FR-017 verdict | Steps | Symex | Verification | Wall | cgroup memory.peak |
| --- | --- | --- | --- | --- | --- | --- | --- |
| infeasible guard `x < y` | VERIFICATION SUCCESSFUL, 0 of 1 cover properties satisfied (1 unreachable) | `CoverUnsatisfied` | 95,300 | 3.07 s | 4.27 s | 8.0 s | 0.604 GB |
| feasible guard `x > y` | VERIFICATION SUCCESSFUL, 1 of 1 cover properties satisfied | `Verified` | 95,300 | 3.08 s | 5.56 s | 9.0 s | 0.606 GB |
| mutant A: oracle plan `Add` changed to `Multiply` (feasible case) | VERIFICATION FAILED | proof failure | 97,223 | 3.16 s | 5.84 s | 9.0 s | 0.610 GB |
| mutant B: `exact` computed as `left + right + 1` (feasible case) | VERIFICATION FAILED | proof failure | 95,317 | 3.10 s | 5.57 s | 9.0 s | 0.605 GB |
| mutant C: guard not assumed, `kani::assume(left_native < guard_0);` deleted (infeasible case) | VERIFICATION SUCCESSFUL, 1 of 1 cover satisfied | `Verified`, not `CoverUnsatisfied` | 95,296 | 3.15 s | 5.68 s | 9.0 s | 0.605 GB |

Failing step of mutants A and B: the assertion "the oracle must complete with exactly the native
quire.op.integer.add result when it lies in the checked domain, and refuse otherwise". Mutant C
fails the expected verdict: the unassumed guard makes the `completed` cover reachable, so the
infeasible harness no longer reports `CoverUnsatisfied`.

Before the change, the general harness for the same infeasible scenario produced no verdict in the
1800 s cap, with 25.3 GB at the 24 GiB scope limit (`ir304-kani-guard-cover-attempt.md`). A
hand-written probe of the target shape ran first: 95,330 steps, 3.09 s symex, 4.28 s verification
for the infeasible case (0.585 GB), and the feasible case 6.0 s in total (0.606 GB).

## What this does not establish

- Not CG's runner: no `run_kani`-style launch, no FR-017-AC-63 two-phase run and no
  classification code ran. The FR-017 column maps Kani's cover result onto the verdict names by
  the rule FR-017 states (a successful verification with an unsatisfied cover is
  `CoverUnsatisfied`).
- The subject is not called: the harness checks the oracle against the independent native
  expectation.
- Division, modulo and the other scalar families are not measured.

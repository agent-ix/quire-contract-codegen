---
id: TC-034
title: "Verify the claimed-module proof gate"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-023
    type: verifies
---
# TC-034: Verify the claimed-module proof gate

## Description

Verify that the gate is green only when the prover discharged a check inside
every claimed module, that a module the prover never reached fails it, and that
a violation injected inside each claimed module turns it red.

## Test Procedure

1. Apply the gate to synthetic transcripts holding a `SUCCESS` check in one
   module, a `SATISFIED` cover in a second, an `UNREACHABLE` check in a third,
   and nothing for a fourth. Claim a module whose name is a prefix of another's,
   claim nothing, and apply it to a falsified outcome.
2. Run the spine's healthy harness under the pinned prover and apply the gate to
   the two checked-in claimed modules, then to those two plus a module the crate
   compiles and never calls.
3. Run the same harness with the subject's operation changed to credit rather
   than debit, and with the generated ensures bound tightened past what the
   subject satisfies, and apply the gate to each run. Decode the credit run's
   counterexample and replay it through QSL.

The input is a hand-built `BoundPackage` projection: no contract is compiled, so
the contract-to-Contract-IR step does not run. It passes through CG's Kani
obligation generation and the pinned prover, and the decoded counterexample
through `qsl_replay::replay`. The native twin is hand-mirrored QSL source tied to
the Rust side only by the clause and its argument names, and the request's limits
are unlimited stand-ins.

## Expected Results

Only the module with a `SUCCESS` check is discharged; the cover-only,
unreachable-only and absent modules are unreached; a prefix does not match a
longer segment; an empty claim list and a falsified outcome fail. The healthy run
is verified with a discharged check in both claimed modules, and the never-called
module is unreached. Each injected violation falsifies the run and turns the gate
red.

## Status

Covered. Step 1 is in the default suite (`src/kani_module_gate.rs` unit tests);
steps 2 and 3 run in the ignored Kani lane
(`tests/it/skeleton_spine.rs`, `make kani`), which `make ci` does not include.

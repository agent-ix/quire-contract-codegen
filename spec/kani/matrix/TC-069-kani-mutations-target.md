---
id: TC-069
title: "Verify the kani-mutations target"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: verifies
---
# TC-069: Verify the kani-mutations target

## Description

Verify [FR-055](../functional/FR-055-kani-mutations-target.md): the `make kani-mutations` target
runs every generated harness's mutant and fails on a surviving mutant. This case is planned. Steps
1 to 6 use stand-in `cargo-kani` executables on `PATH` and need no installed Kani; step 7 uses
installed Kani (0.68.0 on terra) and runs through the Kani lane, one run at a time.

## Test Procedure

1. With a stand-in launcher that reports a failed check for every mutant, run the target; assert
   exit zero, every mutant `pass`, and an unchanged working tree (FR-055-AC-1).
2. Assert one output line per mutant run with harness, mutant and outcome, and a total; mutate the
   target to omit a line and require failure (FR-055-AC-2).
3. Run `make -n ci` and assert it holds `kani-mutations` (FR-055-AC-3).
4. With a stand-in that verifies one mutant, assert that mutant is `fail` and the exit is non-zero
   (FR-055-AC-4).
5. Remove an anchor from a copy of the declaration, duplicate one, supply an uncompilable mutant,
   and a stand-in that times out; assert a non-`pass` outcome and a non-zero exit each
   (FR-055-AC-5). With no `cargo-kani` on `PATH`, assert all `unavailable` and non-zero
   (FR-055-AC-6).
6. Inspect NFR-006-AC-12 and TC-045 step 12 for the amendment (FR-055-AC-8).
7. In the Kani lane with installed Kani, weaken one generated harness's proposition to trivially
   true, run the target, and require that harness's mutant `fail` and a non-zero exit; restore it
   and require zero (FR-055-AC-7).

## Expected Results

A defect that a harness accepts is reported by name and fails the target; an absent toolchain,
broken anchor, timeout or compile error never reads as success; every run mutant is listed.

## Status

PLANNED (IR-1437): no FR-055 criterion is implemented or tagged.

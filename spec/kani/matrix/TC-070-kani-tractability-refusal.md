---
id: TC-070
title: "Verify measured-table tractability refusal and decider corpus"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: verifies
---
# TC-070: Verify measured-table tractability refusal and decider corpus

## Description

Verify [FR-056](../functional/FR-056-kani-tractability-refusal.md) through admitted V2 clause
packages, public CG generation, a small test table and corpus, and, for the last step only,
installed Kani through CG's runner. This case is planned. Every mutant fails at its named step.

## Test Procedure

1. Implement the table decider and a trivial always-`Run` decider against the one interface; assert
   both compile behind it and the interface parameter is the feature value alone, by inspection of
   the signature (FR-056-AC-1).
2. Admit `a * b + c` at three widths and compare the exposed per-operation widths with those the
   oracle rendering computes. Mutate to the declared width; the multiply-then-add case must fail
   (FR-056-AC-2).
3. Load a valid table, an empty table and tables with an unknown verdict, a missing field and a
   fractional width; assert the load results and the failing row index. Add a row for a fresh
   operation id and assert the decision changes with no source change (FR-056-AC-3).
4. With a `not_tractable` row for `quire.op.integer.mul` at 64 bits, request a clause multiplying
   at width 64 and assert `V2ClauseNotTractable` with operation id, width, the row, the versions and
   no harness, beside a supported sibling that keeps its harness (FR-056-AC-4).
5. Probe `Int[0, 2^32]` (33 bits), `Int[0, 2^64 - 1]` (64 bits) and `Int[0, 2^64]` (65 bits) against that row, a clause holding only another operation, and a
   clause holding both (FR-056-AC-5).
6. Request a clause no row qualifies and diff its harness and identity record against the
   empty-table output; byte-identical required. Add a `tractable` row at the same width and repeat
   (FR-056-AC-6).
7. Repeat step 4 with the row's generator version, then its kernel version, changed: the clause
   runs. Change only the Kani, CBMC or solver version: still refused (FR-056-AC-7).
8. Add two qualifying rows in both file orders and assert the reported one is unchanged
   (FR-056-AC-8).
9. With a fake backend that exceeds the wall-clock ceiling on a clause the decider runs, assert the
   `KaniRunOutcome::Inconclusive` timed-out reason of FR-028-AC-2 and no tractability refusal (FR-056-AC-9).
10. Inspect the corpus file for the fields, the seed families and a measurement source on every
    row (FR-056-AC-10).
11. Run the scoring test. Assert it prints false-refuse and false-accept, passes at the recorded
    baseline, and fails for an always-`Refuse` decider and an always-`Run` decider
    (FR-056-AC-11).
12. Append one corpus row and assert a count changes with no source change; score the
    always-`Run` decider from step 1 against the same file (FR-056-AC-12).
13. Execute one clause the table lets through via CG's runner under installed Kani, one at a time,
    and record verdict, wall time, Kani version and generator/kernel version. Request integer
    multiply at the measured-bad wide width and assert it is refused before any Kani launch; the
    Decimal multiply row is used only once IR-1444 lets it reach the decider (FR-056-AC-13).

## Expected Results

Steps 1 to 12 pass on the host with no Kani. Step 13 records one executed verdict and one
pre-launch refusal. Every refusal keeps its own reason and no harness; a clause with no bad row is
unchanged.

## Status

Steps 1 to 12 are implemented and tagged (IR-1526). Step 13 is coded as an ignored kani-lane test and
needs an installed Kani run.

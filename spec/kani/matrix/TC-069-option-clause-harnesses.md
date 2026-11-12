---
id: TC-069
title: "Verify option presence and value clause harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: verifies
---
# TC-069: Verify option presence and value clause harnesses

## Description

Verify [FR-055](../functional/FR-055-option-clause-harnesses.md) through admitted V2 clause
packages over option fields, public CG generation and, one run at a time, installed Kani
through CG's runner. This case is planned. The scratch probe in
`docs/qualification/ir1448-kani-option-probe.md` measured the encoding only.

## Test Procedure

1. Generate the harness for `x: Option<Int[0, 1000]>` and inspect it: one presence `bool`, one
   `i64` payload, no allocation or heap enum arm, payload `0` when absent and inside `0..=1000`
   when present. Enumerate the admitted values on the host and assert 1002 draws, each once.
   Mutate by leaving an absent payload free, widening the bound and carrying the value in an
   enum arm; each must fail (FR-055-AC-1).
2. For the claim `result = (option.present(x) implies option.value(x) <= 1000)` run the healthy
   proof and require `Verified` with its cover satisfied. Run the mutant returning the negated
   presence and require `Falsified` at "produced result agrees with the independent checked
   condition" with playback absent (FR-055-AC-2).
3. Run the mutant subject with consequent `x.value < 1000` and require `Falsified` at the same
   step with playback present, payload 1000 (FR-055-AC-3).
4. Run the unguarded claim `result = (option.value(x) <= 1000)` and require the failing step
   with playback absent. Mutate the lowering to a default payload `0`, then to an assumption of
   presence; each must make the claim verify and so fail this step (FR-055-AC-4).
5. Run the guarded claim healthy and the subject `present and value <= 1000`; require
   `Falsified` with playback absent. Mutate the lowering to evaluate the value before the guard
   and require the guarded claim to fail as in step 4 (FR-055-AC-5).
6. Repeat steps 2 and 3 for `y: Option<R>` with `R` a record of `n: Int[0, 1000]` and a Boolean
   (FR-055-AC-6).
7. Request one clause per refusal reason (a text payload and a collection payload, an option of
   an option, a field needing `null`, an unrecognised option operation) with a supported sibling
   and an unbounded-integer option; assert the typed reason, the node, no harness, the sibling's
   harness and `requires_bound` for the unbounded integer (FR-055-AC-7, FR-055-AC-8).
8. Compare the identity record before and after changing the encoding, decode a present and an
   absent playback, and refuse an absent playback with a non-zero payload (FR-055-AC-9).
9. Run steps 2 to 6 through the two-phase runner in a named cgroup, one run at a time, recording
   verdict, wall time and peak memory (FR-055-AC-10).

## Expected Results

Healthy clauses verify with a satisfied cover; every mutant fails at its named step with its
distinguishing input; every refusal keeps its own reason and no harness.

## Status

PLANNED (IR-1448): no FR-055 criterion is implemented or tagged.

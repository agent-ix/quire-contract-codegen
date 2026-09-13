---
id: TC-021
title: "Verify shrinking preserves numeric constraints"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-012
    type: verifies
---
# TC-021: Verify shrinking preserves numeric constraints

## Description

Verify that every candidate proptest visits while shrinking a bound population stays in domain and
keeps its original tag, that `Broad` never shrinks across sides, and that shrink replays are counted.

## Test Procedure

1. For every operator and domain of 1 to 4 members from TC-018 step 3, create `Satisfying`,
   `Violating`, and `Broad` value trees from every reachable seed state and walk every path of
   `simplify` and `complicate` calls to exhaustion, recording each visited value.
2. For 1,000 seeded `VersionUnchanged` value trees of each population over 0..=1000, follow the greedy
   shrink path: call `simplify` until it returns false, calling `complicate` after each failing
   candidate.
3. Run a `Violating` `VersionUnchanged` campaign whose oracle is replaced by
   `post == pre || post == pre + 1`, so only `Violated` cases with post = pre + 1 mismatch, and
   capture the minimal counterexample.
4. In step 3's campaign, count the replaced oracle's evaluations and read `attempted`.

## Expected Results

- Every visited candidate in steps 1 and 2 lies in its domain and keeps its original `Holds` or
  `Violated` tag, and no `Broad` candidate changes side.
- The minimal counterexample has post = pre + 1, both values in 0..=1000, tagged `Violated`.
- `attempted` equals the oracle evaluation count, including shrink replays.

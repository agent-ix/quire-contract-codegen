---
id: TC-021
title: "Verify shrinking preserves numeric constraints"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-012
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: verifies
---
# TC-021: Verify shrinking preserves numeric constraints

## Description

Verify that every candidate proptest visits while shrinking stays in domain and on its original
relation side, and that residual candidates are counted as rejections.

## Test Procedure

1. For 1,000 seeded `Satisfying` and 1,000 seeded `Violating` `VersionUnchanged` value trees, walk the
   complete shrink tree. At each step, call `simplify` until it returns false, and `complicate` after
   each accepted simplification. Record every visited value.
2. Repeat the walk for every operator and domain pair in TC-018 step 3.
3. Run a campaign against a subject that fails only when post = pre + 1, and capture the minimal
   counterexample.
4. Run a residual fixture whose shrink candidates can leave the shaped constraint, and read the
   campaign summary.

## Expected Results

- Every visited candidate lies in its domain and keeps its original `Holds` or `Violated` tag.
- The minimal counterexample lies in 0..=1000, has post = pre + 1, and is tagged `Violated`.
- Every residual candidate outside the shaped constraint is counted in `rejected`, and none is counted
  as accepted or passed.

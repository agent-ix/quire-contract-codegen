---
id: TC-018
title: "Verify constructive satisfying and violating populations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: verifies
---
# TC-018: Verify constructive satisfying and violating populations

## Description

Verify that generated numeric populations are exactly the satisfying or violating set over the
declared domains, are constructed without filtering, and are reproducible.

## Test Procedure

1. Generate the `Satisfying`, `Violating`, and `Broad` populations for `VersionUnchanged` and
   `amount < 7`. Compile them in a generated-crate fixture.
2. Run 10,000 seeded cases per population with a pinned RNG. Record the proptest global reject count
   and the campaign discard count.
3. For each of the six operators, and for every pair of domains drawn from
   {[0,0], [0,1], [0,3], [2,5], [4,7], [i64::MAX-1, i64::MAX], [i64::MIN, i64::MIN+1]}, enumerate the
   population's value tree exhaustively and compare it with a brute-force satisfying or violating set.
4. Scan the generated source for `prop_filter`, `prop_filter_map`, `prop_assume`, `reject`, and
   discard constructors.
5. Generate an empty-side fixture `amount < 0` over 0..=1000.
6. Generate every population twice and compare the bytes.

## Expected Results

- Every case lies in domain and matches its tag. The `VersionUnchanged` `Satisfying` cases have
  post = pre, and its `Violating` cases have post ≠ pre.
- The enumerated sets equal the brute-force sets for every operator and domain pair.
- The global reject count is 0 and the discard count is 0, and no forbidden call appears in the source.
- `amount < 0` refuses `Satisfying` and `Broad` with `EmptyPopulation` naming `Satisfying`, and
  generates `Violating`.
- The extreme domains complete without overflow or panic.
- Repeated generation is byte-identical.

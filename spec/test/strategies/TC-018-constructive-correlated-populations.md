---
id: TC-018
title: "Verify constructive satisfying and violating populations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: verifies
---
# TC-018: Verify constructive satisfying and violating populations

## Description

Verify that bound populations are constructed rather than filtered, generate exactly the true value
sets, refuse empty sides explicitly, and handle `i64` extremes without overflow.

## Test Procedure

1. Generate `Satisfying`, `Violating`, and `Broad` populations for `VersionUnchanged` and draw 10,000
   seeded cases from each.
2. Scan each generated population source for `prop_filter`, `prop_filter_map`, `prop_assume`,
   `TestCaseError::reject`, and discard constructors.
3. For each of the six operators, each domain of 1, 2, 3, and 4 members, and each relation shape (two
   reads; one read against a literal at each domain member), enumerate every value the generated
   strategy can produce by exhaustively walking its value-tree space, and compare it with an
   independent enumeration of the satisfying and violating sets.
4. Request all three populations for `amount < 0` and for `amount >= 0` over 0..=1000.
5. Generate, compile, and draw 10,000 seeded cases for `NotEqual`, `Less`, and `Equal` over
   `i64::MIN..=i64::MAX`.
6. Generate each population twice and compare bytes.

## Expected Results

- Every `Satisfying` case has post = pre, every `Violating` case has post ≠ pre, and all values lie in
  0..=1000.
- The source scan finds no match, and the step 1 runs record zero discards and zero global rejects.
- Generated and enumerated sets are equal for every combination; every combination with an empty side
  is refused with `EmptyPopulation` instead of producing a strategy.
- `amount < 0` refuses `Satisfying` and `Broad` naming `Satisfying` and generates `Violating`;
  `amount >= 0` refuses `Violating` and `Broad` naming `Violating` and generates `Satisfying`; no
  refused request emits an artifact or attestation.
- The extreme-domain runs complete without overflow or panic, with every case in domain and on its
  side.
- Repeated generation is byte-identical.

---
id: TC-019
title: "Verify domain and relation boundary censuses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: verifies
---
# TC-019: Verify domain and relation boundary censuses

## Description

Verify that the generated in-domain census, out-of-domain array, and unrepresentable-edge array hold
exactly the cases FR-010 defines, in order, with correct tags and bounded size.

## Test Procedure

1. Generate the `Boundary` bundle for `amount < 7`, `amount < 1`, and `VersionUnchanged` over
   0..=1000, compile each, and read the three generated arrays.
2. Generate and compile the `Boundary` bundle for `amount < 7` with the declaration widened to
   `i64::MIN..=i64::MAX`, and for `x == y` over that domain.
3. Request `Boundary` and `Satisfying` for `amount <= 1000` over 0..=1000.
4. For every in-domain census case, evaluate the relation with an independent evaluator; for every
   out-of-domain case, check each value against the domain.
5. Count the in-domain and out-of-domain cases for every admitted fixture in TC-017, TC-018, and this
   case.
6. Generate each bundle twice and compare bytes.

## Expected Results

- The `amount < 7`, `amount < 1`, and `VersionUnchanged` arrays equal FR-010-AC-1 and FR-010-AC-2
  exactly, in order, with no duplicates.
- The extreme-domain bundles compile without overflow or panic, hold no out-of-domain case for either
  outer edge, and list both outer edges as unrepresentable.
- `amount <= 1000` refuses `Boundary` with `UnsupportedCampaignConstraint` and emits no artifact or
  attestation for it, and its `Satisfying` bundle
  carries the out-of-domain array [-1, 1001].
- Every tag equals the independent evaluation, and every out-of-domain case has at least one
  out-of-domain value.
- No fixture exceeds 20 census cases, and an exhaustive sweep reaches exactly 20.
- Repeated generation is byte-identical.

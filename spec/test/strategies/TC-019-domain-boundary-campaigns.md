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

Verify that the `Boundary` population is the exact census of just-inside and just-outside domain
values plus the relation edges, tagged correctly, with unrepresentable edges recorded rather than
dropped.

## Test Procedure

1. Generate `Boundary` for `amount < 7` and for `VersionUnchanged` over 0..=1000. Extract the census
   from the generated source.
2. Generate `Boundary` for a `Less` clause over `i64::MIN..=i64::MAX` against literal 0, and read the
   unrepresentable-edge record.
3. Generate `Boundary` for a relation whose representable census is single-tagged: `amount <= 1000`
   over 0..=1000 (Holds only).
4. For every census case in every admitted fixture, recompute the tag independently: first the domain
   check, then the relation.
5. Count the census size for each admitted fixture.

## Expected Results

- `amount < 7` yields exactly the nine tagged cases in FR-010-AC-1.
- `VersionUnchanged` yields the pairs in FR-010-AC-2.
- The full-`i64` domain emits no `OutOfDomain` edge case, lists both outer edges as unrepresentable,
  and does not panic.
- `amount <= 1000` is refused with `UnsupportedCampaignConstraint`.
- Every generated tag equals the independent recomputation.
- Every two-read census has at most 64 cases.

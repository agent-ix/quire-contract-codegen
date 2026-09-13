---
id: FR-010
title: "Generate domain and relation boundary campaigns"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-010: Generate domain and relation boundary campaigns

## Description

When a caller requests the `Boundary` population for an admitted bound clause, the generator shall
emit a finite, deterministic census. The census holds the values just inside and just outside each
declared domain edge, plus the values on either side of the relation's own edge. The generator tags
each case with its expected classification.

## Inputs

- An admitted relation and its domain census from
  [FR-008](./FR-008-bound-domain-strategy-admission.md).

## Outputs

- A generated `proptest::sample::select` population over the census. Each case carries an
  expectation tag of `Holds`, `Violated`, or `OutOfDomain`.
- A generated census record listing every unrepresentable edge value that was omitted.

## Behavior

- For each read with domain `min..=max`, the census shall include:
  - `min` and `max`;
  - `min + 1` and `max - 1` when they lie in the domain;
  - `min - 1` and `max + 1` as `OutOfDomain` cases.
- For a read compared with literal `k`, the census shall include `k - 1`, `k`, and `k + 1`.
- The generator shall tag each in-domain literal-edge value `Holds` or `Violated` by evaluating the
  relation.
- For two correlated reads, the census shall pair each in-domain edge value of the first read with:
  - the partner values at the relation edge (first value, first value + 1, first value - 1) that lie
    in the second domain;
  - both `OutOfDomain` partners just outside the second domain (`min - 1` and `max + 1`).
- The census shall pair each out-of-domain edge value of the first read with the nearest in-domain
  value of the second read.
- When any value of a case lies outside its declaration's domain, the generator shall tag the case
  `OutOfDomain`.
- When every value of a case lies in domain, the generator shall tag the case `Holds` or `Violated`
  by evaluating the relation.
- The generator shall compute every edge value with checked `i64` arithmetic.
- If an edge value such as `i64::MIN - 1` is unrepresentable, then the generator shall list it in the
  census record by read, edge, and direction instead of clamping, wrapping, or silently dropping it.
- If the representable census has no `Holds` case or no `Violated` case, then the generator shall
  refuse the census with `UnsupportedCampaignConstraint`.
- The single-tag refusal matches the existing integer boundary rule.
- Checked-arithmetic overflow edges of `Numeric` expressions are out of this slice, because FR-008
  refuses arithmetic nodes with `UnsupportedRelation` until the arithmetic slice is specified.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-010-AC-1 | For `amount < 7` over 0..=1000, the census is exactly {-1 OutOfDomain, 0 Holds, 1 Holds, 6 Holds, 7 Violated, 8 Violated, 999 Violated, 1000 Violated, 1001 OutOfDomain}. | Test (TC-019) |
| FR-010-AC-2 | For `VersionUnchanged` over 0..=1000, for each pre in {0, 1, 999, 1000} the census contains post = pre tagged Holds, each in-domain post = pre ± 1 tagged Violated, and post -1 and post 1001 tagged OutOfDomain; it also contains (pre -1, post 0) and (pre 1001, post 1000) tagged OutOfDomain. | Test (TC-019) |
| FR-010-AC-3 | A domain `i64::MIN..=i64::MAX` generates without overflow or panic, emits no OutOfDomain case for either edge, and lists both outer edges as unrepresentable in the census record. | Test (TC-019) |
| FR-010-AC-4 | A relation whose representable census holds only `Holds` cases or only `Violated` cases is refused with `UnsupportedCampaignConstraint`. | Test (TC-019) |
| FR-010-AC-5 | Every census case's tag equals an independent evaluation of the domain check followed by the relation. | Test (TC-019) |

## Dependencies

- **Upstream**: [FR-008](./FR-008-bound-domain-strategy-admission.md); existing integer
  `Boundary` campaign in [FR-002](../FR-002-tristate-proptest.md).
- **Downstream**: [FR-011](./FR-011-numeric-harness-campaigns.md),
  [TC-019](../../test/strategies/TC-019-domain-boundary-campaigns.md).

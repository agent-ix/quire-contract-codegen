---
id: FR-010
title: "Generate domain and relation boundary censuses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-010: Generate domain and relation boundary censuses

## Description

When a caller requests the `Boundary` population for an admitted bound clause, the generator shall
emit a finite, deterministic census of in-domain cases at the domain edges and the relation's own
edge. Every bound strategy bundle also carries the out-of-domain edge values, so a consumer can prove
that it refuses them at domain admission. Each in-domain case is tagged with the value the clause's
generated oracle returns for it.

## Inputs

- An admitted relation and its shared domain `min..=max` from
  [FR-008](./FR-008-bound-domain-strategy-admission.md).

## Outputs

- A generated constant array holding the in-domain census, each case tagged `Holds` or `Violated`.
- A generated constant array holding the out-of-domain edge cases, which carry no truth tag.
- A generated constant array listing every unrepresentable edge value by read, edge, and direction.

## Behavior

- For the primary read, the in-domain edge values shall be `min`, `max`, and `min + 1` and `max - 1`
  where they lie in the domain.
- For a primary read compared with literal `k`, the in-domain edge values shall also include `k - 1`,
  `k`, and `k + 1` where they lie in the domain.
- For a primary read with no partner read, the in-domain census shall hold one case per in-domain edge
  value.
- For a primary read with a partner read, the in-domain census shall pair each in-domain primary edge
  value `v` with each of `v - 1`, `v`, and `v + 1` that lies in the domain.
- The generator shall tag each in-domain census case `Holds` or `Violated` by evaluating the relation.
- For a primary read with no partner read, the out-of-domain cases shall be `min - 1`, `max + 1`, and
  each of `k - 1` and `k + 1` that lies outside the domain.
- For a primary read with a partner read, the out-of-domain cases shall pair each in-domain primary
  edge value with partner `min - 1` and with partner `max + 1`.
- For a primary read with a partner read, the out-of-domain cases shall also pair primary `min - 1`
  with partner `min`, and primary `max + 1` with partner `max`.
- The generator shall order both arrays by primary value, then partner value, ascending, with
  duplicates removed.
- The generator shall compute every edge value with checked 128-bit integer arithmetic.
- If an edge value such as `i64::MIN - 1` is outside the `i64` range, then the generator shall list it
  in the unrepresentable-edge array instead of clamping, wrapping, or silently dropping it.
- The generator shall test representability only for values that would be out-of-domain cases:
  `min - 1`, `max + 1`, and each of `k - 1` and `k + 1` outside the domain, for the primary and the
  partner read separately.
- The generator shall omit an out-of-domain case whose value is unrepresentable, and list that edge in
  the unrepresentable-edge array instead.
- The generator shall list a literal edge and a domain edge as separate unrepresentable entries, even
  when their values coincide.
- If the in-domain census has no `Holds` case or no `Violated` case, then the generator shall refuse
  the `Boundary` population with `UnsupportedCampaignConstraint`, matching the existing integer
  boundary rule.
- The generator shall emit the out-of-domain and unrepresentable-edge arrays in every bundle it
  generates for a non-refused population request, including a `Satisfying`, `Violating`, or `Broad`
  request for a clause whose `Boundary` population is refused.
- A refused population request shall emit no artifact and no attestation.
- Generated Rust shall be the only carrier of the census arrays; the generator emits no serialized
  census file.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-010-AC-1 | For `amount < 7` over 0..=1000, the in-domain census is exactly [0 Holds, 1 Holds, 6 Holds, 7 Violated, 8 Violated, 999 Violated, 1000 Violated], and the out-of-domain array is exactly [-1, 1001]; for `amount < 1`, where the literal edges coincide with the domain edges, the in-domain census is exactly [0 Holds, 1 Violated, 2 Violated, 999 Violated, 1000 Violated] with no duplicates, and the out-of-domain array is exactly [-1, 1001]. | Test (TC-019) |
| FR-010-AC-2 | For `VersionUnchanged` (primary post, partner pre) over 0..=1000, the in-domain census is exactly the 10 pairs (post, pre): (0,0 H), (0,1 V), (1,0 V), (1,1 H), (1,2 V), (999,998 V), (999,999 H), (999,1000 V), (1000,999 V), (1000,1000 H); the out-of-domain array is exactly the 10 pairs (-1,0), (0,-1), (0,1001), (1,-1), (1,1001), (999,-1), (999,1001), (1000,-1), (1000,1001), (1001,1000). | Test (TC-019) |
| FR-010-AC-3 | A domain `i64::MIN..=i64::MAX` compiles as generated Rust without overflow or panic, its out-of-domain array holds no case for either outer edge, and its unrepresentable-edge array lists both. | Test (TC-019) |
| FR-010-AC-4 | `amount <= 1000` over 0..=1000 refuses `Boundary` with `UnsupportedCampaignConstraint` and emits no artifact or attestation for that request, while its `Satisfying` bundle still carries the out-of-domain array [-1, 1001]. | Test (TC-019) |
| FR-010-AC-5 | Every in-domain census tag equals an independent evaluation of the relation, every out-of-domain case has at least one value outside the domain, and repeated generation produces byte-identical arrays. | Test (TC-019) |

## Dependencies

- **Upstream**: [FR-008](./FR-008-bound-domain-strategy-admission.md); existing integer
  `Boundary` campaign in [FR-002](../FR-002-tristate-proptest.md).
- **Constrained by**:
  [NFR-004](../../nonfunctional/strategies/NFR-004-constructive-generation-efficiency.md).
- **Downstream**: [FR-011](./FR-011-numeric-harness-campaigns.md),
  [FR-013](./FR-013-it010-consumable-output.md),
  [TC-019](../../test/strategies/TC-019-domain-boundary-campaigns.md).

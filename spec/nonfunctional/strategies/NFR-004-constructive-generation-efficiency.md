---
id: NFR-004
title: "Constructive numeric generation efficiency"
type: NFR
quality_attribute: performance_efficiency
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: constrains
---
# NFR-004: Constructive numeric generation efficiency

## Statement

The generated numeric strategies shall produce every requested case without discards or global
rejects, while a campaign runs at the proptest default of 256 cases per property and at a
10,000-case stress setting.

## Scope

- Applies to: `Satisfying`, `Violating`, `Broad`, and `Boundary` populations generated under
  [FR-009](../../functional/strategies/FR-009-constructive-correlated-populations.md) and
  [FR-010](../../functional/strategies/FR-010-domain-boundary-campaigns.md).
- Excludes residual populations, whose rejection is intentional and is reported under
  [FR-011](../../functional/strategies/FR-011-numeric-harness-campaigns.md).

## Rationale

A strategy that filters would reach proptest's global-reject limit on narrow correlated relations,
such as post = pre over 0..=1000, where a uniform pair has a 1 in 1001 chance of satisfying the
relation. The resulting `Exhausted` outcome would hide the missing coverage behind a framework
reason. Constructive generation removes that failure mode and makes the discard rate a checkable
zero, rather than a tuning parameter.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Discarded / attempted on constructive populations (10,000-case seeded run) | 0 | 0 | Seeded campaign accounting |
| Global rejects on constructive populations (10,000-case seeded run) | 0 | 0 | proptest `TestRunner` reject count |
| Campaigns ending `Exhausted` on constructive populations | 0 | 0 | Seeded campaign accounting |
| Generated `Boundary` census size per admitted clause of two reads | ≤ 40 cases | ≤ 64 cases | Census count |

## Verification

TC-018 and TC-020 run 10,000-case seeded campaigns over `VersionUnchanged` and `amount < 7`, asserting
the discard count, the global reject count, and the `Exhausted` count are each zero. TC-019 counts the
`Boundary` census for every admitted fixture.

## Dependencies

- **Upstream**: [FR-009](../../functional/strategies/FR-009-constructive-correlated-populations.md),
  [FR-011](../../functional/strategies/FR-011-numeric-harness-campaigns.md).

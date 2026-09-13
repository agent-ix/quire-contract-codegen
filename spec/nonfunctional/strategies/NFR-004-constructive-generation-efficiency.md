---
id: NFR-004
title: "Constructive numeric generation efficiency"
type: NFR
quality_attribute: performance_efficiency
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: constrains
---
# NFR-004: Constructive numeric generation efficiency

## Statement

The generated bound strategies and conformance runner shall produce every requested case without
discards, proptest global rejects, or framework exhaustion, and the boundary census shall stay small
enough to evaluate exhaustively.

## Scope

- Applies to: `Satisfying`, `Violating`, and `Broad` populations under
  [FR-009](../../functional/strategies/FR-009-constructive-correlated-populations.md), the census under
  [FR-010](../../functional/strategies/FR-010-domain-boundary-campaigns.md), and the runner under
  [FR-011](../../functional/strategies/FR-011-numeric-harness-campaigns.md).
- Excludes caller-supplied `ResidualExclusion` constraints of the existing integer strategy API, whose
  rejection is intentional under [FR-002](../../functional/FR-002-tristate-proptest.md).

## Rationale

A strategy that filters would reach proptest's global-reject limit on narrow correlated relations,
such as post = pre over 0..=1000, where a uniform pair has a 1 in 1001 chance of satisfying the
relation. The resulting `Exhausted` outcome would hide the missing coverage behind a framework
reason. Constructive generation, and a runner that records rejected preconditions instead of
returning them as global rejects, remove that failure mode and make the discard rate a checkable zero.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Discarded / attempted on a fresh 10,000-case campaign per population and fixture | 0 | 0 | Generated summary `discard_rate()` |
| Proptest global rejects on the same campaigns | 0 | 0 | `TestRunner` reject count |
| Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config campaigns | 0 | 0 | Generated campaign conclusion |
| In-domain plus out-of-domain census cases per admitted clause | ≤ 22 | ≤ 22 | Census array lengths |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-004-AC-1 | Fresh 10,000-case and 256-case campaigns of every population over `VersionUnchanged`, `amount < 7`, and a `Precondition` fixture record zero discards, zero proptest global rejects, and no `Exhausted` conclusion. | Test (TC-020) |
| NFR-004-AC-2 | For every admitted fixture in TC-019, the in-domain census plus out-of-domain array holds at most 22 cases. | Test (TC-019) |

## Verification

TC-020 runs the campaigns and reads the summary, the runner's reject count, and the conclusion. TC-019
counts the census arrays for every admitted fixture; 22 is the largest census the FR-010 rules can
produce (12 in-domain plus 10 out-of-domain cases for two reads).

## Dependencies

- **Upstream**: [FR-009](../../functional/strategies/FR-009-constructive-correlated-populations.md),
  [FR-010](../../functional/strategies/FR-010-domain-boundary-campaigns.md),
  [FR-011](../../functional/strategies/FR-011-numeric-harness-campaigns.md).

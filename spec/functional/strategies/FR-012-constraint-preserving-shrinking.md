---
id: FR-012
title: "Preserve numeric constraints while shrinking"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-012: Preserve numeric constraints while shrinking

## Description

When proptest shrinks a failing case drawn from a bound `Satisfying`, `Violating`, or `Broad`
population, every candidate it visits shall stay inside the shared domain and keep the expectation tag
of the case it came from. This refines [FR-002](../FR-002-tristate-proptest.md) FR-002-AC-4 for bound
populations; FR-002-AC-4 itself stays verified by TC-004.

## Inputs

- A failing case drawn from a `Satisfying`, `Violating`, or `Broad` population of
  [FR-009](./FR-009-constructive-correlated-populations.md).

## Outputs

- A minimal counterexample that lies in the domain and keeps the original case's expectation tag.

## Behavior

- For a primary read with a partner read, the generated value tree shall derive the partner value from
  the current primary value at every shrink step, so no candidate leaves the partner set.
- The generated value tree shall keep the expectation tag of the case it was drawn from on every
  `simplify` and `complicate` step.
- A shrink-path walk shall call `complicate` only after at least one successful `simplify`, because
  the proptest `ValueTree` protocol does not require a value tree to handle `complicate` before its
  first `simplify`.
- The `Broad` population shall use a value tree that never shrinks a case from one side into the other
  side, including when proptest's union shrinking would switch to an earlier alternative.
- The census runner of [FR-011](./FR-011-numeric-harness-campaigns.md) shall not shrink, because it
  reports each failing census case exactly as enumerated.
- The runner shall count every shrink replay in `attempted`, as interface-001 `accounting_unit`
  states.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-012-AC-1 | For every operator and every domain of 1 to 4 members from FR-009-AC-2, an exhaustive walk of every valid `simplify` and `complicate` path of `Satisfying`, `Violating`, and `Broad` value trees visits only in-domain candidates that keep their original tag; a valid path calls `complicate` only after at least one successful `simplify`. | Test (TC-021) |
| FR-012-AC-2 | For 1,000 seeded `VersionUnchanged` value trees of each population over 0..=1000, every candidate on the greedy shrink path stays in domain and keeps its tag. | Test (TC-021) |
| FR-012-AC-3 | A `Violating` `VersionUnchanged` campaign whose oracle is replaced by `post == pre || post == pre + 1` (so only `Violated` cases with post = pre + 1 mismatch) reports a minimal counterexample with post = pre + 1, both in 0..=1000, tagged `Violated`. | Test (TC-021) |
| FR-012-AC-4 | After a shrinking campaign, `attempted` equals the number of oracle evaluations the runner performed, including shrink replays. | Test (TC-021) |

## Dependencies

- **Upstream**: [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-011](./FR-011-numeric-harness-campaigns.md); FR-002-AC-4 in
  [FR-002](../FR-002-tristate-proptest.md).
- **Downstream**: [TC-021](../../test/strategies/TC-021-constraint-preserving-shrinking.md).

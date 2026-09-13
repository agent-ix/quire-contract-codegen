---
id: FR-012
title: "Preserve generated constraints while shrinking"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
---
# FR-012: Preserve generated constraints while shrinking

## Description

When proptest shrinks a failing numeric or state-scalar case, every candidate it tries shall stay
inside the declared domains and on the same side of the relation as the original case. The one
exception is a residual population, where the runner records a candidate that leaves the shaped
constraint as a rejection and never reports it as a pass.

This closes [FR-002](../FR-002-tristate-proptest.md)-AC-4 for the numeric/state slice.

## Inputs

- A failing case drawn from a `Satisfying`, `Violating`, `Broad`, or `Boundary` population.

## Outputs

- A minimal counterexample that keeps the original case's expectation tag, or a residual rejection
  recorded in campaign accounting.

## Behavior

- For correlated reads, the generated shrink tree shall derive the partner value from the current
  first value at every step, so a shrink of the first value can never leave the partner outside its
  partner set.
- A shrunk case shall keep the expectation tag of the case it was shrunk from. The generator shall
  not let a `Violated` case shrink into a `Holds` case, or the reverse.
- Each branch of the `Broad` union shall shrink only within its own branch.
- `Boundary` census populations shall shrink only to other census members that have the same tag.
- If a population is residual (a clause form that is shaped only in part), then the runner shall
  count every shrink candidate outside the shaped constraint in `rejected` through the explicit
  rejection path.
- The runner shall not count a residual shrink candidate outside the shaped constraint as accepted
  or passed.
- The runner shall count shrink replays in `attempted`, as interface-001 `accounting_unit` states.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-012-AC-1 | Walking the complete shrink tree (every `simplify`/`complicate` step) from 1,000 seeded `Satisfying` and `Violating` `VersionUnchanged` cases, every visited candidate lies in 0..=1000 and keeps its original tag. | Test (TC-021) |
| FR-012-AC-2 | For every operator over the exhaustive small-domain census of FR-009-AC-2, every visited shrink candidate satisfies its population's domain and relation side. | Test (TC-021) |
| FR-012-AC-3 | A deliberately failing subject that fails only when post = pre + 1 shrinks to a counterexample whose values are in domain and whose tag is `Violated`. | Test (TC-021) |
| FR-012-AC-4 | A residual fixture whose shrink candidates can leave the shaped constraint records each such candidate in `rejected`, and no such candidate is counted as accepted or passed. | Test (TC-021) |

## Dependencies

- **Upstream**: [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-002](../FR-002-tristate-proptest.md).
- **Downstream**: [TC-021](../../test/strategies/TC-021-constraint-preserving-shrinking.md).

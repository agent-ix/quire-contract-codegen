---
id: FR-004
title: "Produce vacuity and rejection evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-004: Produce vacuity and rejection evidence

## Description

When LLVM/cargo coverage export and runtime campaign counts are supplied, the generator shall combine
consequent execution, rejection, discard, and test outcome into a per-requirement vacuity report.

## Inputs

- Generated consequent source map, LLVM coverage export, runtime campaign counts, and test outcomes.

## Outputs

- Per-requirement vacuity, coverage, rejection, discard, and outcome evidence.

## Behavior

- The generator shall report an always-false implication antecedent as vacuous despite true oracle returns.
- Unexecuted control flow shall remain distinct from vacuous satisfaction.
- Exercised coverage shall require an observed consequent source region.
- The generator shall retain coverage tool/version and source-map digests.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | An unexecuted implication consequent yields a vacuity finding. | Test (TC-006) |
| FR-004-AC-2 | Unexecuted control flow and vacuous satisfaction have distinct classifications. | Test (TC-006) |
| FR-004-AC-3 | No report claims exercised coverage without an observed consequent region. | Test (TC-006) |
| FR-004-AC-4 | Coverage and source-map identities are present in every report. | Inspection |

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md) and [FR-002](./FR-002-tristate-proptest.md).

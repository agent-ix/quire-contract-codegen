---
id: FR-001
title: "Generate deterministic Rust oracles (retired)"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-001: Generate deterministic Rust oracles (retired)

## Description

FR-001 is retired with the V1 `BoundPackage` model it read.
[FR-014](./complete-v1/FR-014-exact-scalar-oracles.md) is the one oracle generator for every family
it covers, the Boolean connectives and the bounded-integer comparisons included, and it reads only
an admitted `quire.checked-package/v2` package
([ADR-001](../decisions/ADR-001-overlapping-generators-and-input-models.md) Q2 and Q3). FR-001
generated a Boolean oracle for each executable V1 clause in the Boolean and bounded-integer
comparison grammar, with its source map.

Every FR-001 behaviour that FR-014 did not already state is an FR-014 criterion.

## Inputs

None. The requirement is retired.

## Outputs

None. The requirement is retired.

## Behavior

None. The criteria below keep their numbers, and each names the criterion that carries it.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Retired. Byte-identical regeneration is carried by FR-014-AC-4. | Test (TC-001) |
| FR-001-AC-2 | Retired. The differential corpus over every Boolean connective and the integer comparisons is carried by FR-014-AC-35 and FR-014-AC-37. | Test (TC-002) |
| FR-001-AC-3 | Retired. Identities in names and refusals are carried by FR-014-AC-5. | Test (TC-001) |
| FR-001-AC-4 | Retired. One typed disposition per item with nothing dropped is carried by FR-014-AC-1. | Test (TC-003) |
| FR-001-AC-5 | Retired. The consequent census and its probes are carried by FR-014-AC-36. | Test (TC-001, TC-006) |
| FR-001-AC-6 | Retired. The V1 bound-package consumer has no successor, because the V1 model is retired. | Test (TC-001, TC-002) |
| FR-001-AC-7 | Retired. Package identity in names is carried by FR-014-AC-38. | Test (TC-001) |
| FR-001-AC-8 | Retired. The Boolean and bounded-integer comparison oracles are carried by FR-014-AC-35. | Test (TC-002) |

## Dependencies

- **Carried by**: [FR-014](./complete-v1/FR-014-exact-scalar-oracles.md).

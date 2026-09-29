---
id: FR-003
title: "Generate Kani obligations and proof dependencies (retired)"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-003: Generate Kani obligations and proof dependencies (retired)

## Description

FR-003 is retired. [FR-015](./complete-v1/FR-015-bounded-kani-obligations.md) is the Kani backend's
one generator, and it reads only an admitted `quire.checked-package/v2` package
([ADR-001](../decisions/ADR-001-overlapping-generators-and-input-models.md) Q1 and Q3). FR-003
lowered clauses of the retired V1 `BoundPackage` model into Kani function contracts, harnesses and a
proof dependency graph.

Every FR-003 behaviour that FR-015 did not already state is an FR-015 criterion. FR-003's
caller-supplied solver and its optional stubbing are not carried: FR-015 lowers against `cadical`,
and FR-015-AC-9 bans every stubbing option.

## Inputs

None. The requirement is retired.

## Outputs

None. The requirement is retired.

## Behavior

None. The criteria below keep their numbers, and each names the criterion that carries it.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | Retired. Proof-dependency readiness and `not_run` execution are carried by FR-015-AC-25. | Test (TC-005) |
| FR-003-AC-2 | Retired. Embedding the byte-identical oracle and agreeing with its verdicts are carried by FR-015-AC-20. | Test (TC-014) |
| FR-003-AC-3 | Retired. Typed refusals with no harness are carried by FR-015-AC-3, FR-015-AC-21 and FR-015-AC-23. | Test (TC-003, TC-014) |
| FR-003-AC-4 | Retired. The option vector and bounds in the identity are carried by FR-015-AC-2 and FR-015-AC-9. | Test (TC-005, TC-014) |
| FR-003-AC-5 | Retired. Exact inclusive domain assumptions are carried by FR-015-AC-11. | Test (TC-014) |
| FR-003-AC-6 | Retired. The bounded state transition is carried by FR-015-AC-19, with the state binding ADR-004 Q2 decides in FR-025-AC-8. | Test (TC-014) |
| FR-003-AC-7 | Retired. The plain comparison falsified through concrete playback is carried by FR-015-AC-24. | Test (TC-014) |
| FR-003-AC-8 | Retired. The Boolean argument is carried by FR-025-AC-3, and byte-identical regeneration by FR-015-AC-10. | Test (TC-014) |

## Dependencies

- **Carried by**: [FR-015](./complete-v1/FR-015-bounded-kani-obligations.md).

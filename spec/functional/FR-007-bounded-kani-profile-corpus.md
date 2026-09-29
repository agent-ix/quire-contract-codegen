---
id: FR-007
title: "Generate the bounded Kani profile corpus (retired)"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: references
---
# FR-007: Generate the bounded Kani profile corpus (retired)

## Description

FR-007 is retired. [FR-015](./complete-v1/FR-015-bounded-kani-obligations.md) is the Kani backend's
one generator, and it reads only an admitted `quire.checked-package/v2` package
([ADR-001](../decisions/ADR-001-overlapping-generators-and-input-models.md) Q1 and Q3). FR-007
derived a cross-backend corpus from Contract IR's `kani-bounded/1` profile and replayed its
counterexamples through a caller-supplied native evaluator.

Every FR-007 behaviour that FR-015 did not already state is an FR-015 criterion. Its replay through
a caller-supplied evaluator is not carried: a counterexample replays only through QSL's replay
facade, and a counterexample that no backend run produced takes the `Input` arm
([FR-024](./complete-v1/FR-024-counterexample-envelope-intake.md)).

## Inputs

None. The requirement is retired.

## Outputs

None. The requirement is retired.

## Behavior

None. The constraints and criteria below keep their numbers, and each names what carries it.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-007-CON-1 | Retired. Reading Contract IR through its public interface only is carried by FR-015's `CheckedPackageV2` input. | Architecture | Inspection |
| FR-007-CON-2 | Retired. Not carried as a criterion: the dependency direction is AD-001's IR → CG seam. | Architecture | Inspection |
| FR-007-CON-3 | Retired. No assumption erasing an invalid, refused or incomplete outcome is carried by FR-015-AC-4. | Integrity | Test |
| FR-007-CON-4 | Retired. Not carried. | Compatibility | Inspection |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-007-AC-1 | Retired. One disposition per construct is carried by FR-015-AC-23. | Test (TC-023) |
| FR-007-AC-2 | Retired. Deterministic artifacts from one validated input are carried by FR-015-AC-10 and FR-015-AC-25. | Test (TC-023) |
| FR-007-AC-3 | Retired. A typed result with no partial artifact is carried by FR-015-AC-23, and the timed-out and memory-exhausted run outcomes by FR-028-AC-2 and FR-028-AC-3. | Test (TC-023) |
| FR-007-AC-4 | Retired. Replay of a counterexample no backend run produced is carried by FR-024-AC-5. | Test (TC-023) |
| FR-007-AC-5 | Retired. Not carried as a criterion: the dependency direction is AD-001's IR → CG seam. | Inspection (TC-023) |
| FR-007-AC-6 | Retired. Not carried. | Test (TC-023) |
| FR-007-AC-7 | Retired. Declared-census validation is carried by FR-015-AC-22 and FR-015-AC-25. | Test (TC-023) |

## Dependencies

- **Carried by**: [FR-015](./complete-v1/FR-015-bounded-kani-obligations.md),
  [FR-024](./complete-v1/FR-024-counterexample-envelope-intake.md).

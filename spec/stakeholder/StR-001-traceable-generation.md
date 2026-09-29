---
id: StR-001
title: "Traceable multi-backend contract generation"
type: StR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-007
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-012
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-013
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: satisfied_by
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: satisfied_by
---
# StR-001: Traceable multi-backend contract generation

## Stakeholder Need

Assurance engineers require that the generator shall derive executable, property-test, proof, and
coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,
vacuity, rejection, or assumptions.

## Rationale

Independently authored tests and proofs can drift from requirements and from each other. Deterministic
generation traced to requirement identities makes semantic alignment inspectable while keeping every
inconclusive or unsupported state visible to human decision makers.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-001-VC-1 | Repeated generation of one package produces byte-identical bundles whose artifacts name the requirement and clause they were generated from. | Demonstration |
| StR-001-VC-2 | Executable, proptest, Kani, and vacuity outputs retain the same requirement identity and agree on the shared bounded corpus. | Demonstration |
| StR-001-VC-3 | A proof counterexample is reported as a contract failure only after native replay of that counterexample reproduces it, and a malformed, out-of-domain or disagreeing counterexample is reported as exactly that. | Demonstration |
| StR-001-VC-4 | A proof result is claimed only for an obligation that was settled once, generated for its routed backend, and observed verifying with its non-vacuity cover satisfied. | Demonstration |

## Dependencies

The governing compatibility, evidence, and qualification policy is PGM-01 at
`ix://agent-ix/quire-contract-ir/PGM-01`.

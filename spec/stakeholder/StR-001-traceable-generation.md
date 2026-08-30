---
id: StR-001
title: "Traceable multi-backend contract generation"
type: StR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: satisfied_by
---
# StR-001: Traceable multi-backend contract generation

## Stakeholder Need

Assurance engineers require that the generator shall derive executable, property-test, proof, and
coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,
vacuity, rejection, assumptions, or tool identity.

## Rationale

Independently authored tests and proofs can drift from requirements and from each other. Deterministic
generation with explicit provenance makes semantic alignment inspectable while keeping every
inconclusive or unsupported state visible to human decision makers.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-001-VC-1 | Repeated generation of one pinned package produces byte-identical bundles with complete input, tool, backend, and output identities. | Demonstration |
| StR-001-VC-2 | Executable, proptest, Kani, and vacuity outputs retain the same requirement identity and agree on the shared bounded corpus. | Demonstration |

## Dependencies

The governing compatibility, provenance, evidence, and qualification policy is PGM-01 at
`ix://agent-ix/quire-contract-ir/PGM-01`.

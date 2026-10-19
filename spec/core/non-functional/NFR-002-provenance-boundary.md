---
id: NFR-002
title: "Traceability, licensing, and qualification boundary"
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: constrains
---
# NFR-002: Traceability, licensing, and qualification boundary

## Statement

- Every generated Rust file shall name `quire-contract-codegen` and its input-carried generation
  subject identity: the obligation or source node when present, otherwise the strategy or
  population identity. Its header shall also name the requirement, revision and clause when those
  identifiers are present in that input.
- Generated Rust shall carry `MIT OR Apache-2.0` SPDX identity.
- No automated output shall claim project-specific validation, accreditation, certification, or human release approval.

## Scope

Generated source, diagnostics, source maps, proof graphs, and reports. Generated Rust families are
strategy harnesses; integer, enum and bound strategies and bound populations; Boolean, exact
scalar, composite equality and function-application oracles; and V1 and V2 contract, exact-scalar,
collection and state-frame Kani harnesses.

## Rationale

Opaque licensing prevents independent audit, while automated qualification claims
would exceed the tool's authority and hide consuming-project responsibilities.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Generated Rust files missing generator or input-carried subject identity | 0 | 0 | contract-testing |
| Generated files missing dual-license SPDX header | 0 | 0 | inspection |
| Silent unsupported/inconclusive states | 0 | 0 | negative-abuse-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Every generated Rust file in a family named in Scope has a header naming `quire-contract-codegen`, its input-carried subject identity (obligation or source node when present, otherwise strategy or population), and every requirement, revision or clause identifier present in the generation input. | Test (TC-001) |
| NFR-002-AC-2 | Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity. | Test (TC-001) |
| NFR-002-AC-3 | Unsupported and inconclusive states remain explicit rather than silently succeeding. | Test (TC-003) |
| NFR-002-AC-4 | Generated output makes no project-specific validation, accreditation, certification, or release-approval claim. | Inspection |

## Verification

TC-001 validates the generated header identity and licensing; TC-003 validates explicit diagnostic
states.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md).

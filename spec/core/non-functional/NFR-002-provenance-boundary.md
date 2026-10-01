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

- Every generated Rust file shall name the generator and the requirement, revision and clause it was
  generated from.
- Generated Rust shall carry `MIT OR Apache-2.0` SPDX identity.
- No automated output shall claim project-specific validation, accreditation, certification, or human release approval.

## Scope

Generated source, diagnostics, source maps, proof graphs, and reports.

## Rationale

Opaque licensing prevents independent audit, while automated qualification claims
would exceed the tool's authority and hide consuming-project responsibilities.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Generated files missing requirement and clause identity | 0 | 0 | contract-testing |
| Generated files missing dual-license SPDX header | 0 | 0 | inspection |
| Silent unsupported/inconclusive states | 0 | 0 | negative-abuse-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Every generated Rust file's header names the generator and the requirement, revision and clause it was generated from. | Test (TC-001) |
| NFR-002-AC-2 | Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity. | Test (TC-001) |
| NFR-002-AC-3 | Unsupported and inconclusive states remain explicit rather than silently succeeding. | Test (TC-003) |
| NFR-002-AC-4 | Generated output makes no project-specific validation, accreditation, certification, or release-approval claim. | Inspection |

## Verification

TC-001 validates the generated header identity and licensing; TC-003 validates explicit diagnostic
states.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md).

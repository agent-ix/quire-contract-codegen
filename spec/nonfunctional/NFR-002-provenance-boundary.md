---
id: NFR-002
title: "Provenance, licensing, and qualification boundary"
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: constrains
---
# NFR-002: Provenance, licensing, and qualification boundary

## Statement

- Every emitted artifact shall identify its tool, input, schema, backend, configuration, output, and digest.
- Generated Rust shall carry `MIT OR Apache-2.0` SPDX identity.
- No automated output shall claim project-specific validation, accreditation, certification, or human release approval.

## Scope

Generated source, manifests, diagnostics, source maps, proof graphs, reports, and retained evidence.

## Rationale

Opaque derivation or licensing prevents independent audit, while automated qualification claims
would exceed the tool's authority and hide consuming-project responsibilities.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Missing mandatory identity fields | 0 | 0 | contract-testing |
| Generated files missing dual-license SPDX header | 0 | 0 | inspection |
| Silent unsupported/inconclusive states | 0 | 0 | negative-abuse-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Every emitted artifact records the required tool, input, schema, backend, configuration, output, and digest identity. | Test (TC-001) |
| NFR-002-AC-2 | Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity. | Test (TC-001) |
| NFR-002-AC-3 | Unsupported and inconclusive states remain explicit rather than silently succeeding. | Test (TC-003) |
| NFR-002-AC-4 | Generated output makes no project-specific validation, accreditation, certification, or release-approval claim. | Inspection |

## Verification

TC-001 and TC-003 validate identities, licensing, and explicit diagnostic states. The Measurement Plan
retains the exact draft or released upstream revisions used by each candidate.

## Dependencies

- **Upstream**: PGM-01 and [FR-001](../functional/FR-001-deterministic-oracles.md).

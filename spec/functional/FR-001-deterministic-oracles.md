---
id: FR-001
title: "Generate deterministic Rust oracles and attestations"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-001: Generate deterministic Rust oracles and attestations

## Description

When a validated contract package is supplied, the generator shall emit a separately named Boolean
oracle for every executable clause plus source maps and, for each generated artifact, a shared proof
attestation with complete identity.

## Inputs

- A versioned serialized contract package conforming to the pinned IR schema and corpus revision.
- Backend configuration and declared customer type bindings.

## Outputs

- Rust oracle source, source-region map, diagnostics, and one shared proof attestation per generated
  artifact.

## Behavior

- Generated signatures shall contain exactly the clause dependency set in deterministic order,
  preserving current, pre-state, and post-state observations in caller-facing parameter names.
- An implication consequent shall occupy its own coverable source region.
- The generator shall retain every input, schema, tool, backend, configuration, output, and digest identity;
  consuming packages shall supply only the sealed record digest and the candidate revision their
  attestations bind to.
- The generator shall identify its exact source revision, dirty state, and lowering-implementation digest.
- The generator shall render source in linear space and reject it before exceeding 1,048,576 bytes per clause.
- Distinct requirement and clause identities shall produce bounded, fixed-digest-disambiguated Rust,
  source-map, and attestation paths whose individual filename components do not exceed 255 bytes.
- Unsupported constructs shall produce diagnostics and no falsely complete artifact.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Repeated generation from identical inputs is byte-identical. | Test (TC-001) |
| FR-001-AC-2 | A differential corpus covering every supported operator compiles with only the runtime and matches an independent evaluator. | Test (TC-002) |
| FR-001-AC-3 | Requirement IDs and revisions appear in symbols, failures, source maps, and attestations. | Test (TC-001) |
| FR-001-AC-4 | No construct, obligation, name collision, or bounded-resource failure is silently dropped, approximated, or marked complete after a diagnostic. | Test (TC-003) |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-traceable-generation.md), IR issue #10, and runtime issue #3.

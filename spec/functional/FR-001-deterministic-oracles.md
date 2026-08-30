---
id: FR-001
title: "Generate deterministic Rust oracles and manifests"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-001: Generate deterministic Rust oracles and manifests

## Description

When a validated contract package is supplied, the generator shall emit a separately named Boolean
oracle for every executable clause plus source maps and a derivation manifest with complete identity.

## Inputs

- A versioned serialized contract package conforming to the pinned IR schema and corpus revision.
- Backend configuration and declared customer type bindings.

## Outputs

- Rust oracle source, source-region map, diagnostics, and derivation manifest.

## Behavior

- Generated signatures shall contain exactly the clause dependency set in deterministic order.
- An implication consequent shall occupy its own coverable source region.
- The generator shall retain every input, schema, tool, backend, configuration, output, and digest identity.
- Unsupported constructs shall produce diagnostics and no falsely complete artifact.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Repeated generation from identical inputs is byte-identical. | Test (TC-001) |
| FR-001-AC-2 | Generated code compiles with only the runtime and declared customer types. | Test (TC-002) |
| FR-001-AC-3 | Requirement IDs and revisions appear in symbols, failures, source maps, and manifests. | Test (TC-001) |
| FR-001-AC-4 | No construct is silently dropped, approximated, or marked complete after a diagnostic. | Test (TC-003) |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-traceable-generation.md), IR issue #10, and runtime issue #3.

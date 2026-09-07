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

- An immutable public IR `BoundPackage` decoded from the versioned derived executable projection.
  Normal authored sources remain the validated frontend/model pipeline; synthetic projections used
  in tests do not establish frontend coverage. The low-level single-clause API remains available
  but does not establish complete package binding.
- Backend configuration and declared customer type bindings.

## Outputs

- Rust oracle source, source-region map, diagnostics, and one shared proof attestation per generated
  artifact.

## Behavior

- Generated signatures shall contain exactly the clause dependency set in deterministic order,
  preserving current, pre-state, and post-state observations in caller-facing parameter names.
- An implication consequent shall occupy its own coverable source region.
- Each oracle shall carry exactly one evaluation-entry probe on its function-entry line, disjoint
  from every consequent region. Every consequent shall carry a single-line entry-token probe inside
  its exact expression region. Probe columns are one-based byte offsets with an exclusive end.
- The clause envelope shall declare its expected consequent count derived from the typed expression,
  independent of the emitted region list. Dropped or duplicate regions cannot redefine that count.
- The generator shall retain every input, schema, tool, backend, configuration, output, and digest identity;
  consuming packages shall supply only the sealed record digest and the candidate revision their
  attestations bind to.
- The generator shall identify its exact source revision, dirty state, and lowering-implementation digest.
- The generator shall render source in linear space and reject it before exceeding 1,048,576 bytes per clause.
- Distinct requirement and clause identities shall produce bounded, fixed-digest-disambiguated Rust,
  source-map, and attestation paths whose individual filename components do not exceed 255 bytes.
- Unsupported constructs shall produce diagnostics and no falsely complete artifact.
- `generate_bound_oracles` shall consume every executable clause through public `BoundPackage` and
  `BoundClause` accessors, without private wire structures, a codegen-owned input schema, clause
  selection, or inferred pre/post pairing. An unsupported executable clause shall fail the entire
  batch, preserving its complete ClauseRef and diagnostics and returning no publishable artifact.
- An empty or informational-only executable population shall return `NoExecutable`, preserving
  the bound semantic digest and informational references. It is neither invalid IR nor an
  unsupported clause, and it shall carry no publishable artifact or generation attestation.
- Every source-map row shall retain the complete ClauseRef through required `packageId`,
  `requirementId`, `requirementRevision`, and `clauseId` fields. Oracle naming shall hash all four
  identity components so equal requirement/revision/clause names in different packages cannot alias.
- A generated bound batch shall retain its bound semantic digest, ordered clause identities,
  declaration/expression digests and informational references. Per-artifact attestations shall bind
  that input through the existing input-digest field, with the separately versioned IR bound-identity
  profile. Their command remains a description of a library operation, not a fabricated executable
  CLI invocation. Generation success shall not imply campaign execution or coverage sufficiency.
- Batch collection shall preflight the publisher's artifact-count limit and generated names, and
  enforce its byte limits incrementally before constructing the final `ArtifactBundle`. Existing
  per-clause source-size and stack guards remain active. Publication is a separate explicit call.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | Repeated generation from identical inputs is byte-identical. | Test (TC-001) |
| FR-001-AC-2 | A differential corpus covering every supported operator compiles with only the runtime and matches an independent evaluator. | Test (TC-002) |
| FR-001-AC-3 | Requirement IDs and revisions appear in symbols, failures, source maps, and attestations. | Test (TC-001) |
| FR-001-AC-4 | No construct, obligation, name collision, or bounded-resource failure is silently dropped, approximated, or marked complete after a diagnostic. | Test (TC-003) |
| FR-001-AC-5 | Every oracle map declares the typed-expression consequent census, one disjoint evaluation-entry probe, and an exact entry-token probe for every consequent; generated-oracle LLVM controls distinguish observed entry from zero consequent execution. | Test (TC-001, TC-006) |
| FR-001-AC-6 | The public bound-package consumer returns every executable clause exactly once, preserves information separately, fails the whole batch for unsupported executable content, and gives an explicit non-publishable NoExecutable result for empty populations. | Test (TC-001, TC-002) |
| FR-001-AC-7 | Full ClauseRef package identity distinguishes source/map names and rows across packages, and source-generation attestations bind the exact bound semantic input without claiming execution or coverage. | Test (TC-001) |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-traceable-generation.md), IR issue #10, and runtime issue #3.

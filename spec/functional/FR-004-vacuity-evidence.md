---
id: FR-004
title: "Produce vacuity and rejection evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-004: Produce vacuity and rejection evidence

## Description

When LLVM/cargo coverage export and runtime campaign counts are supplied, the generator shall combine
oracle evaluation, consequent execution, rejection, discard, and test outcome into a deterministic
per-requirement vacuity report without executing a coverage producer itself.

## Inputs

- Generated source-map artifact with explicit oracle-evaluation and implication-consequent regions.
- LLVM coverage JSON export `2.0.1` bytes, their producer name/version and source root, an exact
  quire-contract-runtime `CampaignReport` for the same requirement revision, a test outcome, and an
  attestation context.

## Outputs

- A versioned per-requirement report plus one packaged ProofAttestationV1 body that binds it.

## Behavior

- The analyzer shall consume an LLVM coverage JSON export without implementing instrumentation,
  profile merging, or a coverage engine.
- The analyzer shall classify a mapped implication consequent as `vacuous` only when the owning
  oracle-evaluation region has a positive execution count and that consequent has none.
- The analyzer shall classify a mapped clause as `unexecuted` when its oracle-evaluation region has
  no positive execution count, regardless of its test outcome or campaign counts.
- The analyzer shall classify a requirement as `partially_exercised` when some, but not all, mapped
  implication consequents were observed; only observation of every mapped consequent may produce
  `exercised` for an implication-bearing clause.
- A clause with no implication consequents shall be `exercised` only when its oracle-evaluation
  region is observed; vacuity shall not be inferred when there is no implication.
- The analyzer shall preserve accepted, rejected, failed, and discarded campaign counts and the
  supplied test outcome as separate facts rather than folding them into coverage classification.
- Source-map requirement/revision identity shall equal the runtime campaign identity, and duplicate,
  missing, ambiguous, malformed, summary-only, or unsupported-version coverage input shall fail
  without a partial report.
- Coverage filenames shall match source-map artifact paths only after stripping the caller-declared
  source root and applying lexical normalization that rejects parent traversal.
- Every report shall retain the coverage producer name/version, LLVM export format version, coverage
  export digest, source-map digest, generated-report schema identity, and exact requirement revision.
- The v0.1 qualified producer profile shall be cargo-llvm-cov 0.9.0 emitting LLVM coverage JSON
  export format 2.0.1.
- Every other coverage producer or export format version shall fail as unsupported.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | An evaluated implication whose consequent is unobserved yields a vacuity finding even when every oracle return was true. | Test (TC-006) |
| FR-004-AC-2 | A clause whose oracle-evaluation region was not observed is unexecuted, not vacuous. | Test (TC-006) |
| FR-004-AC-3 | An implication-bearing requirement is exercised only when every mapped consequent region was observed. | Test (TC-006) |
| FR-004-AC-4 | Coverage and source-map identities are present in every report. | Inspection |
| FR-004-AC-5 | Malformed, summary-only, identity-mismatched, path-ambiguous, or unsupported coverage inputs return structured diagnostics and no partial report. | Test (TC-006) |
| FR-004-AC-6 | Campaign counts and test outcome remain complete facts independent of coverage classification. | Test (TC-006) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md) and [FR-002](./FR-002-tristate-proptest.md).

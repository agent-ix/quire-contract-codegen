---
id: TC-006
title: "Distinguish vacuity and unexecuted control flow"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: verifies
---
# TC-006: Distinguish vacuity and unexecuted control flow

## Description

Verify coverage export and runtime counts classify consequent execution, vacuity, rejection, discard,
ordinary unexecuted flow, and test outcome distinctly per requirement.

## Test Procedure

Generate always-false, always-true, mixed, implication-free, rejected, discarded, and unexecuted
fixtures. Produce LLVM JSON with the pinned cargo-llvm-cov command outside the analyzer, then pass the
retained bytes, tool identity, source root, generated source-map artifact, matching runtime campaign
report, test outcome, and attestation context to `analyze_coverage`.

Exercise exact positive and zero segment counts for oracle-evaluation and consequent regions,
multiple consequents with mixed observation, and complete campaign counters. Mutate each input class
independently: export type/version, absent segments from summary-only output, segment tuple shape,
missing/duplicate filenames, parent-traversing or ambiguous paths, source-map digest, requirement or
revision identity, and invalid attestation context. Use the pinned quire-contract-runtime
`CampaignReport` type rather than constructing a caller-owned counter lookalike.

## Expected Results

An evaluated always-false implication is vacuous; a never-evaluated clause is unexecuted; a mixed
multi-implication clause is partially exercised; and only complete consequent observation is
exercised. An implication-free clause is never labeled vacuous. Accepted, rejected, failed, and
discarded counts plus test outcome remain unchanged in every report. Each valid report contains exact
tool, export-format, export-digest, source-map-digest, schema, requirement, and revision identity.
Every malformed or mismatched input returns a stable diagnostic and no report or attestation.

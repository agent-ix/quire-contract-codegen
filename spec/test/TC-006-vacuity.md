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

Verify actual generated-oracle probes against native LLVM export, and ultimately keep measured
coverage, runtime accounting, native execution outcome, and obligation discharge distinct.

## Test Procedure

The implemented primitive fixture generates Rust and maps through `generate_boolean_oracle`, then
executes the pinned cargo-llvm-cov producer outside the analyzer for vacuous, mixed,
implication-free, and never-called generated functions. Parse its actual full JSON and observe the
generated entry probes. Synthetic export controls independently remove or corrupt exact fields;
they test refusal behavior but do not masquerade as native coverage evidence.

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
Every malformed or mismatched input retains a stable non-success diagnostic without inventing a
measured-zero observation or passed coverage attestation. The primitive fixture emits no report or
attestation and makes no native campaign binding claim.

## Remaining aggregate controls

Once IR #50 supplies the immutable executable population, remove one clause, consequent, evaluation
probe, and the entire population independently. Rebind source/maps, requirement revisions, binary,
profiles, toolchain, target, campaign and candidate independently; every mismatch must prevent
coverage discharge. Exercise unavailable producer/evaluator and implication-free positive evaluation
as different cases. Feed vacuous, unexecuted, partial, failed, aborted and unavailable outcomes
through the consuming obligation gate: each must deny success even if its diagnostic report can be
serialized. Check accepted/rejected/discarded accounting independently and reject passed execution
with failed postconditions. Use the native runtime CampaignReport, never a private counter type.

TC-006 and FR-004 matrix rows remain planned: primitive controls are partial implementation, not
completion of bound analysis, native-run provenance, versioned report output or shared consumption.

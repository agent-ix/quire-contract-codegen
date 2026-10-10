---
id: TC-006
title: "Distinguish vacuity and unexecuted control flow"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: verifies
  - target: ix://agent-ix/quire-contract-ir/FR-045
    type: references
---
# TC-006: Distinguish vacuity and unexecuted control flow

## Description

Verify actual generated-oracle probes against native LLVM export, and ultimately keep measured
coverage, runtime accounting, generated-campaign execution, QSL clause-run outcome, and obligation
discharge distinct.

## Test Procedure

The implemented primitive fixture generates Rust and maps through `generate_boolean_oracle`, then
executes cargo-llvm-cov outside the analyzer for vacuous, mixed,
implication-free, and never-called generated functions. Parse its actual full JSON and observe the
generated entry probes. Synthetic export controls independently remove or corrupt exact fields;
they test refusal behavior but do not masquerade as native coverage evidence.

Exercise exact positive and zero segment counts for oracle-evaluation and consequent regions,
multiple consequents with mixed observation, and complete campaign counters. Mutate each input class
independently: export type/version, absent segments from summary-only output, segment tuple shape,
missing/duplicate filenames, parent-traversing or ambiguous paths, and requirement or revision
identity. Use the quire-contract-runtime
`CampaignReport` type rather than constructing a caller-owned counter lookalike.

## Expected Results

An evaluated always-false implication is vacuous; a never-evaluated clause is unexecuted; a mixed
multi-implication clause is partially exercised; and only complete consequent observation is
exercised. An implication-free clause is never labeled vacuous. Accepted, rejected, failed, and
discarded counts plus test outcome remain unchanged in every report.
Every malformed or mismatched input retains a stable non-success diagnostic without inventing a
measured-zero observation or passed coverage result. The primitive fixture emits no report and makes
no native campaign binding claim.

The planned aggregate case also consumes a native campaign whose oracle returns all succeed while
one implication consequent never executes. Its report must classify that clause as `vacuous` and
the consuming coverage obligation must deny success. IR FR-045 may consume the observation but
does not count the probe or successful native test as a completed proof.

## Remaining aggregate controls

Bank complete-package native controls through `generate_bound_oracles` and independently corrupt
whole-population, source/map, full package identity and typed implication census bindings.
Assert that entirely exercised observations without a native run are still reported as observations
only, and retain informational references for valid no-executable input. Missing campaign transport
must never become a passing coverage result. These controls are not implemented.

Once Contract IR supplies the immutable executable population, remove one clause, consequent, evaluation
probe, and the entire population independently. Rebind source/maps, requirement revisions and
campaign independently; every mismatch must prevent coverage discharge. Exercise unavailable producer/evaluator and implication-free positive evaluation
as different cases. Feed vacuous, unexecuted, partial, failed, aborted and unavailable outcomes
through the consuming obligation gate: each must deny success even if its diagnostic report can be
serialized. Check accepted/rejected/discarded accounting independently and reject passed execution
with failed postconditions. Use the native runtime CampaignReport, never a private counter type.

For the native-run binding control, record the generated source and source map actually handed to
the coverage producer. Analyze those same artifacts with the resulting export and campaign run;
then replace the map, source, revision or run independently while preserving the other inputs.
Also omit or change the LLVM producer tool/version identity. The successful case reports that
identity and the run's requirement/revision; each mismatch or omission is a structured non-success
result and cannot discharge coverage.

## Planned native-run-result/2 consumer controls

1. Obtain QSL FR-267's genuine `native-run-result/2` documents for `AllBelow` over `high`
   (decisive counterexample with witness), `AllBelow` over `low` (closed scope without witness),
   and exhausted work (unavailable without witness). Feed each through QSL's strict reader, then
   the planned CG `analyze_coverage` input. Compare typed stage, category, truth, basis and each
   assigned witness component with independently authored expectations; do not compare the reader
   output to itself as the oracle.
2. In separate documents change the `format` to `/1` and `/3`, omit `format`, omit or change
   `basis`, omit a required witness component, add an unknown witness member, place a witness on
   closed scope, or remove one from a decisive basis. The QSL reader must return a located refusal
   before CG classifies a clause. Feed the QSL FR-267 `unknown_edition` command-error envelope
   through that same strict reader; its distinct typed variant must retain stage `profile`, code
   `unknown_edition`, `basis: unavailable`, no witness and the defined message/details as an
   error, never become a successful clause run. Remove its required member, add a clause-run-only
   member or witness, change its basis, violate code-specific cause/details, or change its format:
   each must yield a located reader refusal before CG receipt binding.
3. Give CG a genuine QSL semantic-success document with a decisive witness but no execution of
   its generated Rust campaign or no producer-authenticated source/map binding. Conversely,
   run a generated campaign and measure its probes while QSL's clause run reports violation,
   refusal, undefined, incomplete, unsupported or internal failure; exercise a QSpec cancellation
   form separately only when QSL provides such a producer case. Check that each
   fact stays typed and independent, none is inferred from the other, and no missing generated-run
   evidence or adverse result can discharge coverage or grant IR FR-045 proof credit.
4. For each valid decoded clause-run or command-error variant, omit the CG receipt, omit each
   required association independently, or supply only caller-declared matching paths, QSL
   `package_id`, or self-declared producer metadata. Check a structured absent/unauthenticated
   binding diagnostic retains the decoded QSL outcome and available input identities, with no
   campaign qualification or measured classification. Supply an authenticated CG receipt
   associating the exact result, producer execution, generated source and source map; check the
   binding succeeds without upgrading the QSL outcome. Then independently replace each of those
   four inputs while keeping the receipt, including replacements with matching path or QSL
   `package_id`; each mismatch must be identified and refuse binding. A matching receipt never
   upgrades a command error or adverse clause-run result. The LLVM producer identity and actual
   probe observations remain independent CG coverage inputs.

These controls are planned. QSL-520 must deliver the `/2` producer/reader; QSL-688 allocated the
separate authenticated result/execution/source/map receipt to CG, whose trusted authority and
verifier still need implementation. The existing `analyze_bound_coverage` primitive does not
consume a QSL result and is not evidence for these steps.

TC-006 and FR-004 matrix rows remain planned: primitive controls are partial implementation, not
completion of bound analysis, native-run binding, or a consuming obligation.

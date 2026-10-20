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

For the native-run binding control, give the campaign the generated source and source map
owned by the same CG runner invocation. Start it with fresh private profile output, then start
the LLVM export child over only that output and bind bytes read directly from that child's
output stream before the invocation ends. Retain the linked campaign and export child handles
through binding; reject a pre-existing or foreign profile output location.
Use a controlled internal producer seam to present malformed same-run map, source, revision, run
or export associations independently before binding while preserving the other inputs. Inject
a foreign run's export bytes with the same source/map and LLVM tool/version but different probe
counts through a detached buffer, then through a stream from that foreign export child. Both
must refuse as `producer_run_mismatch` before classification because neither came from the
current runner-owned campaign/profile/export chain. Also replace just the profile output with a
foreign run's profile while leaving the current invocation's export child in place; the fresh
private profile binding must refuse it. These controls must inspect the actual child/profile
association rather than assume that an owned channel authenticates whichever bytes it returns.
Also omit or change the LLVM producer tool/version identity. The successful case reports that
identity and the run's requirement/revision; each mismatch or omission is a structured non-success
result without measured classification or coverage discharge.

For live-run authority, pass the original `/2` wire bytes through QSL's strict reader exactly once
and consume that returned typed result directly. Inspect that the analyzer neither reserializes the
typed value nor constructs a second provenance record. Assert callers cannot construct, clone,
serialize or deserialize the private run context, or attach a foreign-run value to its private
invocation lifetime. Try a context from a completed run and a copied report in a later invocation:
neither may grant qualification. Feed raw inputs through the allowed
wire boundary in a fresh CG run; qualification must be recomputed there rather than inherited.
At the exact caller-supplied maximum encoded envelope size, admission succeeds; one byte above it
refuses before allocation, naming that limit. Repeat at the maximum decoded working allocation
with a compact encoded input that expands during parsing; the first over-limit allocation refuses
before growth and names that limit. No partial run context or measured classification is emitted.

## Planned V2 population controls (IR-815)

Admit a `CheckedPackageV2` holding a vacuous implication, a never-evaluated clause, a
partially exercised multi-implication clause and an exercised clause. Independently derive each
clause's expected population entry and implication count from its body, then compare the analyzer's
expectation. Remove one clause, one consequent, one evaluation probe and every clause from the
generated artifacts, each alone. Alter one source-map implication count against its body. Repeat the
FR-004-AC-5 malformed-input mutations. Inspect the analyzer source for any `BoundPackage` or
`ClauseRef` use and for any body opened outside `core/ir/`. The expected population equals the
independent derivation (FR-004-AC-16); each count mismatch yields a binding non-success naming the
clause with no classification, and the clause is never treated as implication-free (AC-18); the
four classifications hold (AC-19 to AC-22); each removal prevents success (AC-23); each malformed
input gives non-success with no invented classification (AC-24); and neither `BoundPackage` nor
`ClauseRef` is named (AC-17). These controls are planned.

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
   each must yield a located reader refusal before CG in-run binding.
3. Give CG a genuine QSL semantic-success document with a decisive witness but no execution of
   its generated Rust campaign or no live CG-run source/map binding. Conversely,
   run a generated campaign and measure its probes while QSL's clause run reports violation,
   refusal, undefined, incomplete, unsupported or internal failure; exercise a QSpec cancellation
   form separately only when QSL provides such a producer case. Check that each
   fact stays typed and independent, none is inferred from the other, and no missing generated-run
   evidence or adverse result can discharge coverage or grant IR FR-045 proof credit.
4. For each valid decoded clause-run or command-error variant, omit the live CG run context, omit
   each required association independently through the controlled producer seam, or supply only
   caller-declared matching paths, QSL `package_id`, or producer metadata. Check a structured
   absent, foreign or mismatched binding diagnostic retains the typed QSL outcome and available
   input identities, with no campaign qualification or measured classification. In one live CG
   run, bind the original QSL wire/typed-result pair, producer execution, generated source, source
   map and exact LLVM export before returning; the QSL outcome remains unchanged. Independently
   present a malformed same-run association before binding, including one with matching path or
   QSL `package_id`; each observable mismatch must be named and refuse measured classification.
   Try to attach a foreign-run export with the same source/map and LLVM tool/version but different
   probe counts: the private invocation lifetime must prevent constructing that binding request.
   A completed run's context or forwarded report cannot be replayed to
   qualify another run. A bound live run never upgrades a command error or adverse clause-run
   result. The LLVM producer identity and actual probe observations remain CG coverage inputs,
   not QSL result members.

These controls are planned. QSL-520 must deliver the `/2` producer/reader; QSL-688 allocated the
result/execution/source/map/export association to CG, whose live runner binding still needs
implementation. No stored or wire receipt is specified. For V2 identity, compare the selected
QSL `RequirementItem` occurrence accepted by `export_bindings` (node, role and ordinal) and the
admitted `CheckedPackageV2::package_id()` with the campaign, generated map and analyzed population
using the FR-011 canonical rendering or exact typed fields. Corrupt one occurrence part and one
package-id field at a time in a controlled generated-map/campaign input, and refuse a noncanonical
rendering without normalizing it. Neither identity alone authenticates the same-run
producer/artifact association.
The existing `analyze_bound_coverage` primitive does not
consume a QSL result and is not evidence for these steps.

TC-006 and FR-004 matrix rows remain planned: primitive controls are partial implementation, not
completion of bound analysis, native-run binding, or a consuming obligation.

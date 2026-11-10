---
id: SR-17940486709564847047606
title: "IR-1022 PR #771 two-phase Kani run: EARS conformance"
type: SpecReview
analysis: ears-conformance
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #771 @ 36dc18d1; IR-1022; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-039-bounded-proof-ceilings.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
---

## Summary

Ticket: IR-1022 (design decision D1 on IR-1436). Spec-only PR #771 (head 36dc18d1, one commit on origin/main 67739cf5) specifying the two-phase Kani run: a proof phase under a proof option vector with no concrete playback, then, only after a failed non-unwinding check, a witness phase under the witness vector. Edits FR-015 (Outputs, AC-9, AC-41, AC-119), FR-017 (Description, Inputs, Behavior, AC-5, AC-21, AC-60, new AC-63), FR-028 (Behavior, AC-23), FR-029 (witness-disagreed row), FR-032 (replay input), TC-025, TC-027, TC-039 and tests.md. Reviewed against CG origin/main 67739cf5 src/kani/abi.rs:108-134 (adapter_options), src/kani/generate/spec.rs:1286-1311 (HarnessSpec::new_constrained), src/kani/identity.rs:393/624/810 (has_valid_provenance), src/kani/run/harness.rs:91-99, src/kani/run/execute.rs:400-440/834-890, src/kani/classify.rs:26-53/166-260, src/kani/output/report.rs:231-240/624-637, src/kani/output/playback.rs, src/kani/run/scalar.rs:347-373, src/kani/terminal.rs; Contract IR 43ef764 src/kani/outcome.rs (KaniOutcomeKind); quire-exact origin/main TC-931 Status and Linear IR-844 (19,671,392,256 B traced 40 GiB probe vs 8,183,332,864 B healthy sliced); quire matrix on the PR head.

## Verdict

CHANGES REQUIRED (low only). quire raises no EARS warning on the new lines. Manual check: the launch, falsified, batch and argv bullets are well-formed event, unwanted and optional statements with the generator as subject, and FR-028's bullet is ubiquitous. Two bullets mix normative and declarative sentences.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Only the first sentence uses 'shall'. 'If the witness phase reports the matching failed check but prints no playback, the run is inconclusive ...' and 'If the witness phase stops on its wall-clock or memory ceiling, the run is inconclusive with TimedOut or MemoryExhausted' state required behaviour with no subject and no shall. Fix: rewrite each as 'If ..., then the generator shall classify the run as inconclusive with ...', one bullet each. | spec/kani/functional/FR-017-kani-execution-evidence.md:235 |
| FND-002 | low | One bullet holds two requirements with opposite triggers (When <failed non-unwinding> shall launch; When <success, ...> shall not launch) and a declarative sentence ('A verified result never needs a playback.'). Fix: split into a launch rule and a no-launch rule, and move the declarative sentence to Rationale. | spec/kani/functional/FR-017-kani-execution-evidence.md:225 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-017-description-two-phase | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:44 | A run of one harness is two-phase (IR-1022): a proof phase under the identity's proof option vector, then, only after a failed non-unwinding check, one witness phase under its witness option vector (the rules after the verified, cover-unsatisfied and falsified rules in Behavior; FR-017-AC-63). "The run" below is the two-phase run; a statement about "the proof phase" or "the witness phase" is about that launch alone. |
| FR-017-behavior-two-phases | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:216 | - The generator shall run one harness in two phases (IR-1022, decision D1 of IR-1436). Phase 1, the proof phase, launches the harness identity's proof option vector: the function-contracts option (and the stubbing option when the identity records one), the exact fully qualified harness, `--exact`, the explicit unwind, the explicit solver and `--output-format regular`, with neither `-Z concrete-playback` nor `--concrete-playback`, so the backend keeps CBMC's `--slice-formula`. Phase 2, the witness phase, launches the same harness once with the identity's witness option vector, which is the p... |
| FR-017-behavior-witness-launch | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:225 | - When the proof phase reported a failed check that is not an unwinding assertion, the generator shall launch the witness phase under the identity's same `ProofCeilings`, the same crate directory, target directory and installation, and not classify the run from the proof phase alone. When the proof phase reported success, a cover-unsatisfied result, only failed unwinding assertions (`UnwindBoundExhausted`), no verdict (`NoVerdict`), or a timeout, memory stop or cancellation, the generator shall not launch the witness phase. A verified result never needs a playback. |
| FR-017-behavior-witness-falsified | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:230 | - If the witness phase reported a failed check that is not an unwinding assertion, whose class and source location the proof phase also reported failed, and printed a concrete playback for it, then the generator shall classify the run as falsified and retain that playback verbatim as the counterexample. A playback printed for a satisfied cover witnesses reachability and shall never be taken as a counterexample, and the proof phase's failure alone is never a falsification. |
| FR-017-behavior-witness-disagreed | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:235 | - If the witness phase verifies, or reports failed non-unwinding checks none of whose class and location the proof phase reported failed, then the generator shall classify the run as inconclusive with the reason `WitnessDisagreed` (serialized `witness_disagreed`, a new variant of `KaniInconclusiveReason` distinct from every existing reason). If the witness phase reports the matching failed check but prints no playback, the run is inconclusive with the existing `FailedWithoutCounterexample` reason. If the witness phase stops on its wall-clock or memory ceiling, the run is inconclusive with `... |
| FR-017-behavior-witness-batch | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:243 | - Where a batch member's proof phase reported a failed non-unwinding check, the generator shall run that member's witness phase singly, or together with other failing members of equal witness option vector and equal identity `ProofCeilings` under the grouping rule of FR-017-AC-21; the refusal rules for a refused group apply to a witness group as they do to a proof group, and a witness phase is not a retry of a refused group. |
| FR-017-behavior-argv | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:415 | - The generator shall retain, in the evidence of a single run, the obligation kind when the harness carries one, the harness path, the invoked launcher path, the complete argument vector, the unwind bound, the solver, the process exit code, the outcome and the count of checks the report lists as holding. The generator shall make each launched phase's argument vector hold the `kani` subcommand, that phase's identity option vector (proof or witness) unchanged and the report-export flags, in that order and nothing else, so the evidence cannot claim an invocation the harness did not specify. A ... |
| FR-017-behavior-playback-payload | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:445 | - The report carries no concrete playback. The generator shall take a falsifying playback from the witness phase's console as a payload only, after the witness report names a failed property check, and pass it through verbatim as the counterexample, which FR-016 decodes. A playback printed for a cover is never the counterexample. |
| FR-028-behavior-versions | examined | spec/kani/functional/FR-028-bounded-proof-ceilings.md:177 | - The generator shall record in the execution evidence of every run the Kani, Rust and solver versions the run used, and every option of each launched phase's option vector (FR-028-AC-23). |
| FR-029-witness-rationale | examined | spec/kani/functional/FR-029-run-outcome-terminal-record.md:172 | - A witness-disagreed run maps to `Failed`: the proof phase and the witness phase of one harness contradicted each other, so neither a proof nor a counterexample exists and the tool, not the property, is at fault (FR-017-AC-63). This row is new with IR-1022 and keeps the map total; the matching IR-side reason and FR-030 row are not specified here. |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at cf7a86af. Verified in the fix commit: - If the witness phase's failed non-unwinding checks equal the proof phase's and Kani printed no counterexample block headed for the harness path, then the generator shall classify the run as inconclusive with the existing `FailedWithoutCounterexample` reason. - If the witness phase stops on its wal |
| FND-002 | fixed | Round 1 at cf7a86af. Verified in the fix commit: - When the proof phase reported a failed property check that is not an unwinding assertion and no failed unwinding assertion, the generator shall launch the witness phase under the identity's same `ProofCeilings`, crate directory, target directory and installation, and shall classify the run only af |

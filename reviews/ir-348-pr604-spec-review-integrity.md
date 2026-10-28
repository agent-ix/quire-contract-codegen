---
id: SR-601007
title: "IR-348 CG #604 spec-review/integrity review"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md; spec/core/functional/interface-001-codegen-api.md; spec/replay/functional/FR-024-counterexample-envelope-intake.md"
---

# IR-348 CG #604 spec-review/integrity review

## Summary

Ticket: IR-348. Independent Claude Opus review of frozen head dfffa4994653b36ef2b701c74fddd48d21e5d338. Diff is spec-only. No applicable AssuranceProfile found under spec/. `make spec` passed with inherited warnings in FR-017 and FR-014. No Cargo tests or builds ran.

## Scope

- examined: AD-004:run-output — | Witness decode | `replay/witness.rs` | `decode_falsification` and the V2 clause entry take typed playback entries from `kani/output/playback.rs`; the state-clause entry is the next typed `Playback` boundary; public replay entries in `function` and `frame` still take transcript text and call the output parser before decoding (L-6 residual below) |
- examined: AD-004:parse-boundary — The step 5 end state remains one report parse per Kani process and one `parse_assertion_playback` call per replay. The classifier's counterexample selection is separate. The state-clause L-6 slice makes `kani/output/playback.rs` parse the selected falsifying block once and pass its `Playback` to `StateClauseReplayInputs::decode_falsification`; that method does not inspect the run outcome or parse transcript text. Function and frame replay still have their separate L-6 residuals below.
- examined: AD-004:L-6 — - L-6. Kani output is read in `kani/output/` only, and the classifier, the terminal maps and the witness decode take typed values. Test: a grep for Kani's banner, check and playback wording outside that directory, over non-test source: the test modules that embed real Kani captures as fixtures (`kani/classify.rs`, the decode tests) are not scanned. Step 5 removed the former console-text classifier input and moved decoded-value comment parsing into `kani/output/playback.rs`; witness decoding still compares the resulting typed `RenderedValue`. The output reader supplies `CounterexamplePlayback` 
- examined: AD-004:state-exception — | State-clause replay entry re-parses `KaniRunOutcome::Falsified` text before typed decoding | `replay/state_clause.rs`, `StateClauseReplayInputs::decode_falsification` | IR-348 step 5 state-clause slice: pass parsed `Playback` from `kani/output/playback.rs`; the method retains the smaller of caller and run decode limits and all harness, clause, property, assertion and field-order checks |
- examined: AD-004:function-exception — | Public function replay entries and their internal helpers take transcript text, then call the output parser before typed decoding | `replay/function.rs`, `replay_counterexample`, `BoundedCorpusCase::replay_counterexample`, `replay_counterexample_through`, `replay_harness_counterexample` and `replay_harness_counterexample_through` | IR-348 step 5 completion: pass typed `Playback` at the public entries and through their helpers |
- examined: AD-004:frame-exception — | The public frame replay input carries transcript text, which `FrameReplay::new` passes to the parser through `decoded_state` | `replay/frame.rs`, `FrameReplayInputs::playback`, `FrameReplay::new` and `decoded_state` | IR-348 step 5 completion: pass typed `Playback` at `FrameReplayInputs::playback` and through its helpers |
- examined: interface-001:decode_falsification — - name: StateClauseReplayInputs::decode_falsification status: planned typed-playback boundary under AD-004 step 5 L-6 inputs: [mutable StateClauseReplayInputs whose playback values are empty, the falsified postcondition harness's StateFrameIdentity, parsed Playback selected by kani/output/playback.rs, WitnessDecodeLimits] output: Result<(), StateClauseReplayError (PlaybackAlreadySupplied | NotPostconditionHarness | HarnessMismatch{member} | Document(DocumentError::Clause) | Decode(DecodeFailure) | AssertionMismatch)>; on success the borrowed StateClauseReplayInputs holds ordered named i64 pre-
- examined: FR-024:implementation-status — - The state-clause adapter in `src/replay/state_clause.rs` decodes a falsified postcondition harness's Kani playback into the named `i64` pre-state values that `StateClauseReplayInputs::playback` takes. The remaining AD-004 step 5 L-6 boundary change makes `kani/output/playback.rs` supply its already-parsed `Playback` to `StateClauseReplayInputs::decode_falsification`, rather than having that method read the `KaniRunOutcome` text and parse it again. The method derives ordered bindings from the harness's state fields, uses the smaller caller and run decode limits in each dimension, and retains 

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The current-state lists omit state_clause from transcript-taking entries while the interim exception and source still show StateClauseReplayInputs::decode_falsification taking KaniRunOutcome and parsing text. Mark the new lists as planned or retain state_clause until the code slice lands. | spec/assurance/AD-004-cg-crate-layout.md:765,940-941,1474 |
| FND-002 | low | The typed method drops the current NotFalsified refusal without allocating the pre-parse falsified-run check or saying whether the error variant remains. Assign that check to the output reader/caller and account for the existing test and error. | spec/core/functional/interface-001-codegen-api.md:243-244 |

## Verdict

FINDINGS: 2 defect(s); correct before merge.

## Dispositions

Round 1 at 4c420b7bdf81716d5b961e0c4cc36abd17535167. Original Findings rows remain unchanged.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed at 4c420b7b |
| FND-002 | fixed | Fixed at 4c420b7b |

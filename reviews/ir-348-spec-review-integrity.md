---
id: SR-585008
title: 'IR-348 CG #587 spec-review/integrity review'
type: SpecReview
analysis: integrity
review_set: subset
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md
---

# IR-348 CG #587 spec-review/integrity review

## Summary

Ticket: IR-348. Independent Opus review of the spec-only diff at e18e3385b8ae9ec99ffe1836f2bc7c719bf1804b against origin/main source. Targeted Quire validation passed; no applicable AssuranceProfile exists under spec/. L-6 end state and its test are retained. No Cargo, Kani, or full gate was run.

## Scope

- examined: AD-004:dependency-direction — - The classifier's former L-6 exception is closed by step 5: `classify_kani_run` takes `Option<CounterexamplePlayback>` beside the typed report. `kani/output/playback.rs` extracts that value from console text; `kani/classify.rs` does not read the text. The edge from `classify` to `output` remains in the allowed direction.
- examined: AD-004:run-output — | Run classification and vacuity | `kani/classify.rs` | the typed report and `Option<CounterexamplePlayback>` extracted by `kani/output/playback.rs`; step 5 removed the console-text input | | Witness decode | `replay/witness.rs` | `decode_falsification` takes typed playback entries from `kani/output/playback.rs`; the public V2 clause convenience entry still takes transcript text and calls the output parser before decoding |
- examined: AD-004:witness-boundary — The former `kani_witness_join` text scan now lives in `kani/output/playback.rs`. Step 2f moved the scan there, step 2g moved witness decoding to `replay/witness.rs`, and step 5 made the output reader return typed entries and a typed refusal. `replay/witness.rs` checks those entries against persisted argument bindings and builds `qsl_replay::WitnessValue`. Replay imports `kani`; `kani` never imports `replay`. There is one report parse and one playback parse.
- examined: AD-004:L-6 — - L-6. Kani output is read in `kani/output/` only, and the classifier, the terminal maps and the witness decode take typed values. Test: a grep for Kani's banner, check and playback wording outside that directory, over non-test source: the test modules that embed real Kani captures as fixtures (`kani/classify.rs`, the decode tests) are not scanned. Step 5 removed the former console-text classifier input and witness comment comparison. `kani/output/playback.rs` now supplies `CounterexamplePlayback` for classification and typed entries for witness decoding. The public `decode_v2_clause_falsifica
- examined: AD-004:interim-exception — #### Interim exceptions  Each is a place where the target of this AD is not yet true, stated so that no step stops for it.  | Exception | Where | Removed by | | --- | --- | --- | | The public V2 clause witness entry takes transcript text, then calls the output parser before typed decoding | `replay/witness.rs`, `decode_v2_clause_falsification` | move parsing to the caller and pass `Playback` directly; L-6 remains partial |
- examined: AD-004:layout-tree — output/                   the one reader of Kani output diagnostics.rs          the bounded NoVerdict record: failure phase from the console markers and the redacted stream tails (FR-017-AC-32 to AC-36) report.rs               typed report parse (--export-json) playback.rs             the concrete-playback block: the console-text extraction (`counterexample_playback`) and, from step 2f, the block scan `kani_witness_join` held, with `DecodeFailure`; typed entries from step 5
- context_only: AD-004:step-5 — 5. **One output reader and the terminal map.** The playback scanning already sits in `kani/output/playback.rs` (step 2f moved it verbatim); this step changes the interface: `playback.rs` returns typed entries and its own typed refusal, `replay/witness.rs` takes them and `DecodeFailure` is built from that refusal, and `classify_kani_run` takes the extracted playback in place of the console text, which removes the interim edge from `kani/classify.rs` to `counterexample_playback`. Batching follows, in

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | L-6 says it remains partial until this V2 entry takes parsed playback, and the exception table lists only that entry. Main also has transcript-taking replay paths in function.rs:1300-1345,1402-1428, scalar.rs:23-35, state_clause.rs:455-545, and frame.rs:340-369. Under the stated typed-input standard, converting the V2 entry alone cannot close L-6. | spec/assurance/AD-004-cg-crate-layout.md:929-931 |
| FND-002 | low | The new residual row names a change but no owning step or ticket. Step 5 at lines 1134-1139 describes the typed core decoder and classifier but does not assign the remaining public-wrapper conversion. | spec/assurance/AD-004-cg-crate-layout.md:1421 |

## Verdict

FINDINGS: 2 defect(s) in this method; the changed AD needs correction before this review is clean.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The function row omits public transcript-taking wrappers replay_counterexample and BoundedCorpusCase::replay_counterexample (src/replay/function.rs:1283-1291,983-997). Converting only the listed helpers could leave text parsing inside these public boundaries. | spec/assurance/AD-004-cg-crate-layout.md:1427 |
| FND-004 | low | The frame row names private decoded_state, while the public boundary is FrameReplayInputs::playback: String, consumed by FrameReplay::new (src/replay/frame.rs:66,559-579). Converting only the helper could leave a public raw-text boundary. | spec/assurance/AD-004-cg-crate-layout.md:1430 |

## Dispositions

Round 1 at 306b8d4a523dde0f1c10e1da03d3a0ba6e84479a. The original Findings rows above remain unchanged.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | AD-004:931-934 now names all five replay files; the exception table at 1426-1430 covers every production replay call to parse_assertion_playback. L-6 end state remains intact. |
| FND-002 | fixed | AD-004:1141-1144 assigns all transcript-taking replay entries to IR-348 step 5 completion; each exception row repeats that owner. |

FND-003 and FND-004 are new low-severity findings requiring a later disposition. The L-6 end state and test remain unchanged.

### Round 2

Reviewed at f956d7fea961d81b6963e4afa18c866089f94f42. FND-003 and FND-004 are fixed. The single-parse target at AD-004:783-785 retains distinct report-per-Kani-process and playback-per-replay units; L-6 remains partial as stated.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-003 | fixed | AD-004:1430 now names replay_counterexample and BoundedCorpusCase::replay_counterexample as public text-taking entries and requires typed Playback at those entries and their helpers; source replay/function.rs:983-997,1283-1291 confirms both wrappers. |
| FND-004 | fixed | AD-004:1433 now names FrameReplayInputs::playback, FrameReplay::new and decoded_state, and requires typed Playback at the public field; source replay/frame.rs:66,559-579 confirms the boundary. |

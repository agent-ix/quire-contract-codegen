---
id: SR-585006
title: 'IR-348 CG #587 code-review review'
type: SpecReview
analysis: code-review
review_set: subset
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md
---

# IR-348 CG #587 code-review review

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
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS: no defects in this method.

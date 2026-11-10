---
id: SR-17940486709564847047607
title: "IR-1022 PR #771 two-phase Kani run: dependency analysis"
type: SpecReview
analysis: dependency
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

CHANGES REQUIRED (one medium, one low). The FR-032 and FR-017-AC-60 edges follow from the decision. FR-029 is the right owner of the CG-side map. Defects: AC-63 lists a prerequisite it does not need, and FR-029 announces an IR/FR-030 follow-up that its siblings show is not needed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-63 is 'PLANNED (IR-1022; prerequisites IR-844 and IR-1436 decision D1)', but every case in it runs 'over launcher stand-ins that return real-shaped reports'. IR-844 (quire-exact u64 Meter encoding to fit 16 GiB) is enablement for the installed-Kani controls of FR-015-AC-41/119/126, not for this stand-in criterion. Listing it gates IR-1022's code on an unrelated In Progress ticket. Fix: drop IR-844 from AC-63's prerequisites and keep it on the installed controls (TC-025 caveat). | spec/kani/functional/FR-017-kani-execution-evidence.md:879 |
| FND-002 | low | 'the matching IR-side reason and FR-030 row are not specified here' suggests a missing IR/FR-030 counterpart. Contract IR's KaniOutcomeKind (43ef764 outcome.rs) has no representation for any Failed-mapped CG reason. Contract IR refuses every Inconclusive cause except kani_vacuous_proof before FR-030 maps (FR-030:114). NoVerdict, MissingCoverSummary and FailedWithoutCounterexample/properties_failed have no IR reason or FR-030 row either. Following them, witness_disagreed needs none, and no IR ticket is owed. Fix: say that, like the sibling Failed reasons, it does not reach FR-030. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:172 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-017-AC-63 | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:879 | PLANNED (IR-1022; prerequisites IR-844 and IR-1436 decision D1). Over launcher stand-ins that return real-shaped reports, the run of one harness is two-phase. (a) Proof report with a failed assertion check, then a witness report that fails the same check class and location and prints a playback: `Falsified` carrying that playback verbatim, with two launches whose argument vectors are the identity's proof vector and witness vector respectively and whose ceilings and crate are equal. (b) The proof run fails an assertion and the witness run verifies, and a second case where the witness run fai... |
| FR-029-witness-rationale | examined | spec/kani/functional/FR-029-run-outcome-terminal-record.md:172 | - A witness-disagreed run maps to `Failed`: the proof phase and the witness phase of one harness contradicted each other, so neither a proof nor a counterexample exists and the tool, not the property, is at fault (FR-017-AC-63). This row is new with IR-1022 and keeps the map total; the matching IR-side reason and FR-030 row are not specified here. |
| FR-029-witness-row | examined | spec/kani/functional/FR-029-run-outcome-terminal-record.md:159 | / `inconclusive` with the witness-disagreed reason (PLANNED, IR-1022) / `Failed` / |
| FR-032-inputs-playback | examined | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:64 | - The falsified run's retained assertion playback, which is the witness-phase playback of FR-017's two-phase run (FR-017-AC-63); a proof-phase failure without a reproducing witness phase is not a falsified run and supplies none. The scalar builder owns its decode byte |
| FR-017-AC-60 | examined | spec/kani/functional/FR-017-kani-execution-evidence.md:876 | PLANNED (IR-917). Through the public `execute_kani_concrete_playback` entry and an installed Kani backend, a healthy V2 clause playback test reports its selected test as passed, and the plus-one owner mutant's `amount_current = 999` playback executes the same sealed generated arithmetic assertion, observes its unique assertion message or source location and reports the selected expected-panic test as passed. Both installed runs use the same `-Z function-contracts` compilation option as the sealed proof, retain their bounded stdout/stderr and memory observations, and confirm owned teardown. ... |

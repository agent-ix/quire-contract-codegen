---
id: SR-611028
title: IR-348 CG PR 605 L-6 state-clause code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR 605; src/kani/classify.rs, src/kani/output/playback.rs, src/replay/state_clause.rs, src/replay/witness.rs, src/replay/frame.rs, src/replay/function.rs, src/lib.rs, tests/it/kani_obligations_state_clause_replay.rs"
review_set: subset
---

## Summary

Ticket: IR-348. Reviewed frozen head 8e16ae149b7774bbab9a03e37c0ff2ec00af4b45 against base b2e08e08. Applied the Rust lane of code-review, including AD-004 L-6, interface-001, FR-016-AC-6, and FR-024-AC-11 to FR-024-AC-19. Claude CLI Opus review and independent source inspection found the production boundary sound. No applicable AssuranceProfile was found under spec/.

## Verdict

CONDITIONAL: one test gap remains. The focused coder checks passed, but final aggregate gates were not run in this review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Post-parse minimum bounds are only half-tested: requested byte limit and run value limit can be ignored independently without failing the new cases. Add the two swapped limit cases to establish both inputs constrain both dimensions. | tests/it/kani_obligations_state_clause_replay.rs:128 |

## Coverage

Examined AD-004 L-6, interface-001 parse_state_clause_falsification and StateClauseReplayInputs::decode_falsification entries, FR-016-AC-6, and FR-024-AC-11 to FR-024-AC-19. The classifier rejects non-falsified runs before parsing, computes both minima, and parses the complete text once. The replay entry accepts parsed Playback, recomputes both minima, validates selected clause/property/field order, and delegates typed harness identity and source length checks to decode_playback. The single WitnessDecodeLimits definition is in kani/output/playback.rs. The V1-facing function and frame files only change the type import. No vendoring, compatibility wrapper, or gate narrowing was found in the diff.

For FND-001, the added table has a run byte limit paired with unbounded requested bytes, and requested value limit paired with unbounded run values. A decoder changed to use only the run byte limit and only the requested value limit would pass both cases, despite interface-001 requiring the per-dimension minimum. This is a test sufficiency finding; no production defect was identified.

Focused checks supplied by coder: state-clause integration 15 passed, 2 Kani-lane ignored; playback unit 4 passed; witness unit 15 passed; clippy completed. No full gate was run by reviewer.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix head 18a0b72ada24bd13e2b81d1800a4d1ded568d897 adds requested-byte and run-value refusal cases to the same direct typed-decoder test. The four cases now constrain each minimum operand in each dimension; no new defect appears in the fix diff. |

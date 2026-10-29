---
id: SR-619007
title: "IR-1085 PR 621 acceptance and trace gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #621 at bdaac83aedc4dbbce0da7e933d56a8a4e74ee26e; IR-1085 AC1-4, FR-016-AC-1/2/3/4/5/6/8/25; src/replay/function.rs, src/kani/output/playback.rs, tests/it/skeleton_spine.rs, tests/it/bounded_kani_corpus.rs"
review_set: subset
---

## Summary

Ticket: IR-1085. The computed Quoin matrix tags all examined FR-016 criteria. The changed TC-026 tests directly check typed replay refusals and outcomes. A caller-owned bounded parse is not demonstrated by an active function replay caller.

## Verdict

**CONDITIONAL** — IR-1085 AC1's caller-owned bounded parse needs a production call path or a narrower, explicit external-caller contract with test evidence. Plan completion: not assessed.

## Coverage

- `quoin matrix --repo . --json` at the reviewed head lists FR-016-AC-1/2/3/4/5/6/8/25 as tagged; AC-25's three tags are in `tests/it/kani_stdout_capture.rs`, and the installed proof remains planned under IR-604.
- IR-1085 AC1-4 examined against the diff. Existing FR-016-AC-8 V2 producer-to-schema and AC-25 installed Kani gaps were already disclosed before this PR and are not attributed to it.
- Reverse trace over changed production code found the typed replay API under FR-016 and AD-004. No copied cross-repository source or fixture was added.
- Plan completion: not assessed. Optional repository-wide semantic audit was not run; this review examined the PR's changed AC bindings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No production function caller performs the required bounded parse of complete retained playback: the changed function and corpus callers are tests and their `typed_playback` helpers pass `usize::MAX`; replay's later bound check is sound but does not prove AC1's pre-parse bounded intake. | tests/it/skeleton_spine.rs:1134; tests/it/bounded_kani_corpus.rs:31; src/replay/function.rs:1290 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | At c04c77ef, AD-004 and interface-001 explicitly preserve the production caller integration as an L-6 residual. quire-driver main c0b2f312 has no `parse_assertion_playback` or `replay_counterexample` caller in src/drive.rs, src/upstream.rs, or src/. IR-1085 AC1 still requires a bounded parse of the complete retained counterexample at the caller boundary. |

### Round 2

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | rejected | The original finding incorrectly made a production quire-driver caller a condition of the CG boundary. IR-1085 AC1 requires the CG replay entries to receive typed `Playback` and their callers to parse bounded, complete retained text. At 6d83f3c1, the function and corpus test callers parse once with finite byte/value limits, including the two in-crate callers (`src/replay/function.rs:1779` and `src/kani/generate/corpus/bounded_kani_corpus.rs:925`); the replay entries independently recheck limits. AD-004 truthfully states that quire-driver production intake is separate IR-1106 and does not claim it exists. |

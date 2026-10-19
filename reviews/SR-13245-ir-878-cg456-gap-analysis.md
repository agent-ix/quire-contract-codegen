---
id: SR-13245
title: "IR-878 CG PR 456 gap analysis: FR-024 intake criteria against spec, matrix and code"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@943f4c268ba9288ee8a71f3677e6afcb55b2c6f8 vs origin/main fb380d29; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md, src/replay/"
review_set: subset
---

## Summary

Ticket: IR-878. Plan completion: not assessed. The reviewer ran `quire coverage --scope . --strict --json` (quire 0.36.2, engine 0.50.2) on origin/main fb380d29 and on the head:

| | origin/main fb380d29 | PR head |
| --- | --- | --- |
| Backed | 551/909 | 555/909 |
| Unbacked rows | 232 | 232 |
| Contradicted statuses | 0 | 0 |

The four newly backed targets are exactly FR-024-AC-2, AC-3, AC-4 and AC-6, and no target lost backing. The two `unbacked_rows` sets are identical row for row, and neither contains an FR-024 row.

Why the unbacked-row footer stays at 232:

- `unbacked_rows` is a per-row diagnostic, keyed on the row's verification reference. Each FR-024 AC row's verification cell is `Test (TC-035)`.
- TC-035 already had tagged tests on main, so the walk from AC to TC to code resolves for every FR-024 row. FR-024-AC-1 to AC-9 were therefore never among the 232, even while `backed: false`. FR-024-AC-1 and AC-5 are still `backed: false` at the head and still absent from `unbacked_rows`.
- FR-024's own "Current state" paragraph already documents this masking.

The PR's explanation is right: the two counts measure different things, and only the backed total could move. No matrix row or verification reference needs to change to fix the count. The one spec fix this PR owes is the stale prose in FND-001.

The PR leaves NFR-002-AC-2 untagged. The reviewer confirmed the coder's reason: the generated equality-oracle module source starts with `use super::*;` and has no SPDX line (src/oracle/equality/mod.rs:1776). That defect predates this PR and is out of scope here, and it needs its own ticket.

## Examined criteria

- FR-024-AC-2 (examined): "Every backend-witness transcript the generator passes to `Witness::parse` is produced by the one adapter rendering function from decoded values, and that function takes no backend-native text as input."
- FR-024-AC-3 (examined): "A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not."
- FR-024-AC-4 (examined): "For each `WitnessPacket` member in turn, a packet missing that member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present with the value none is admitted."
- FR-024-AC-6 (examined): "A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged."
- FR-024 Current state (examined): "At this revision the generator meets none of FR-024-AC-1 to FR-024-AC-9 in full."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024's "Current state" section now contradicts the matrix and the code. It says the generator "meets none of FR-024-AC-1 to FR-024-AC-9" (line 410). It says "No domain check runs before replay, and no `WitnessEnvelope` is built" (line 419), yet `first_out_of_domain` runs at src/replay/function.rs:1063,1114 and `WitnessEnvelope::reconstruct` runs at src/replay/state_clause.rs:743. It says "`FR-024-AC-1` to `FR-024-AC-9` are planned and have no test of their own" (line 475), while the PR marks AC-2 to AC-4 and AC-6 Covered. Fix: name the four backed criteria and their tests, delete the false domain/envelope sentence, and restate the masking paragraph with the measured 232 rows | spec/replay/functional/FR-024-counterexample-envelope-intake.md:410,419,475-482 |

## Verdict

**PASS with one medium finding.** The four criteria are backed by tests that exercise them. The coverage delta is exactly +4 and is correctly explained. The FR-024 "Current state" prose must be updated in this PR to match.

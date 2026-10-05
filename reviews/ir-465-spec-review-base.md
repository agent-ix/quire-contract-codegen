---
id: "SR-1548"
title: "CG PR 286 spec review: FR-029-AC-10 widening and FR-029-AC-18"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@d16a37d59d8501751749eb1ff8918df66589daaa; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, spec/kani/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md; diff origin/main...HEAD, base 13fc2d0"
---

# SR-1548: CG PR 286 spec review: FR-029-AC-10 widening and FR-029-AC-18

## Summary

Ticket: IR-465 (also IR-460). PR: agent-ix/quire-contract-codegen#286 at d16a37d.

**The AC-10 widening is legitimate.** On main, FR-029's Description already read "fault: an `InternalFault` anywhere in the error the replay path returned". AD-003 link 7 read "the map classifies by walking the whole error, not by its top variant". FR-029-AC-16 already read the `StateClauseReplayError` wrappers' fault readings as AC-10's.

The old AC-10 listed `ReplayRefusal::Fault` inside `StateClauseReplayError::Refused` but not inside the Spine and Frame `Refused` wrappers, so it was narrower than the Behavior text it backed. The widened AC adds assertions and drops none. It is still a direct, enumerated assertion, and it is strengthened, not inflated. The Behavior bullet was widened in step with it.

No other AC contradicts it:

- AC-9 covers non-fault `ReplayRefusal`.
- AC-13 covers non-fault `CallSiteRefusal`.
- AC-16 defers its fault readings to AC-10.
- FR-030-AC-10 follows AC-10 by reference.

**Failed is the right settlement.** It is what FR-029's text and QSL's `from_replay_refusal` both give.

**The FR-029-AC-18, Status, FR-024 and TC-040 step 17 texts were checked against the planner's relayed ruling, for consistency only.** The ruling says inputs are refused, outputs are evidence, an out-of-range post state is admitted as the witness, the replay settles reproduced or violated naming the field, range and observed value, and CG maps that to its ordinary violated terminal. The texts agree with it. They state today's behaviour truthfully: `Inconclusive(ReplayRefused(InvalidRuntimeInput))`, the code QSL bcca433's admission gives. They mark the work as pending, PLANNED and unbuilt. FR-029-AC-18 is numbered without collision.

## Verdict

Approve with two medium precision fixes to the AC texts.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-029-AC-10 (and TC-040 step 9) call `ReplaySettlement::Fault` "the replay-result fault", but that settlement is the call-site fault's reading (terminal.rs:37, function.rs:560); no QSL replay result carries a fault (faults arrive as `Err(ReplayRefusal::Fault)`), so the term names nothing and a reader would look for a fault arm in `ReplayResult` | spec/kani/functional/FR-029-run-outcome-terminal-record.md:228 |
| FND-002 | medium | FR-029-AC-18 is conditioned on "whose replay QSL settles as a reproduced `violation`": if QSL-634 delivers a shape CG's `From<&StateClauseReplayResult>` does not read as reproduced, the run maps to `Failed` and the AC holds vacuously; CG's own obligation (post snapshot built with the exact out-of-range value, not refused as `OutOfDomain`/`CgDefect`) is unstated | spec/kani/functional/FR-029-run-outcome-terminal-record.md:236 |

## Dispositions

Round 1 was reviewed at 87918933f8ff9be4d3d4fa0b2003089565ff61aa (delta d16a37d..8791893).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8791893: FR-029-AC-10, TC-040 step 9 and the `for_each_fault` doc comment now say "the call-site fault's own reading (`ReplaySettlement::Fault`, which QSL's replay result does not carry: QSL reports a fault as an error)". |
| FND-002 | fixed | 8791893: FR-029-AC-18 is now stated unconditionally over the `deposit_debiting` run. That run's real playback is the floor, and its native post state is -1 against `balance`'s range (0, 1000), measured in tests/state_frame_support. The AC names CG's own obligation: build the post snapshot with the exact unclamped value and refuse nothing itself (not `OutOfDomain` or any other CG defect). It targets `Refuted` and excludes `Inconclusive(ReplayRefused)` and `Failed`. It is still PLANNED, pending QSL-634, unbuilt and unbacked. TC-040 steps 17 and the tests.md row agree. |

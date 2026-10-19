---
id: "SR-13152"
title: "CG PR 452 spec review (integrity): FR-024 matrix split versus FR-024 status prose, and the FR-016-AC-12 conflict"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@88af6e6ea8fe51647515e5cd5e7d2c941e65a7ed; spec/replay/matrix/tests.md, spec/tests.md (changed); spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/functional/FR-016-witness-native-replay.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md (read for consistency) (diff origin/main...HEAD, base 87db2f7)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
---

# SR-13152: CG PR 452 spec review (integrity)

## Summary

Ticket: IR-878. PR: agent-ix/quire-contract-codegen#452 at 88af6e6. The PR edits no
requirement text. It splits the FR-024 matrix row (AC-1..AC-9 Planned, AC-10 Covered) and
edits the Replay line of `spec/tests.md`. That is a status change, so this pass checks status
consistency only. The EARS, object and dependency sub-analyses do not apply.

The PR does not touch the FR-024 document. Its "Current state" section still says the
generator "meets none of FR-024-AC-1 to FR-024-AC-10 in full" (line 410). It also says
"`FR-024-AC-1` to `FR-024-AC-10` are planned and have no test of their own" (line 475). Both
now contradict the matrix row this PR adds (FND-001).

Two pre-existing defects in the same documents were read to judge the PR's untagged list:

- FR-016-AC-12 conflicts with FR-029 and with QSL's own mapping (FND-002).
- FR-024's status prose was already stale on `main`, and FR-024-AC-5 is a Test criterion the
  spec says will not be met (FND-003, FND-004).

## Verdict

FND-001 was introduced by this PR and should be fixed in it. Update FR-024's "Current state"
bullets so AC-10 reads as covered by the source-layout test, with the same scope caveat the
matrix row ends up with.

FND-002 to FND-004 are pre-existing and need their own ticket. FND-002 is the one that
matters: FR-016-AC-12 cannot be backed honestly by any test while it contradicts FR-029-AC-10.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-024 "Current state" prose says the generator meets none of AC-1..AC-10, and that AC-1..AC-10 are planned with no test of their own. This contradicts the matrix row this PR adds marking FR-024-AC-10 "✅ Covered" by `layout::qsl_replay_types_are_neither_defined_nor_imported_from_contract_ir`. Introduced by this PR (status changed in the matrix only) | spec/replay/functional/FR-024-counterexample-envelope-intake.md:410-411, spec/replay/functional/FR-024-counterexample-envelope-intake.md:475-482, spec/replay/matrix/tests.md:16-17 |
| FND-002 | medium | Spec defect. FR-016-AC-12 (and the Behavior row at line 83) says a `ReplayRefusal::Fault` yields "a typed unavailable result, and never a mismatch, a refusal or a failure". FR-029-AC-10 says a replay fault, bare or as the cause of `SpineReplayError::Refused`, maps to `Failed`. That follows QSL's `TerminalValue::from_replay_refusal`, which CG reads (src/kani/terminal.rs:34-37, 234, 238), and FR-029-AC-10 is backed by tests/it/terminal_map.rs:535. At the replay layer the code returns `SpineReplayError::Refused(Box<ReplayRefusal>)` for every refusal, a fault included (src/replay/function.rs:100, 270). No "unavailable" type exists. The code follows the newer, QSL-owned FR-029, so this is neither a test nor a code defect. Restate or retire FR-016-AC-12 and the line-83 row to defer to FR-029. Pre-existing | spec/replay/functional/FR-016-witness-native-replay.md:83, spec/replay/functional/FR-016-witness-native-replay.md:132, spec/kani/functional/FR-029-run-outcome-terminal-record.md:284, spec/kani/functional/FR-029-run-outcome-terminal-record.md:370 |
| FND-003 | low | The FR-024 "Current state" says "No domain check runs before replay, and no `WitnessEnvelope` is built". This was already false on `main`. The function path checks domains before replay (src/replay/function.rs:1063, 1114), and the frame and state-clause paths build envelopes with `WitnessEnvelope::reconstruct` (src/replay/frame.rs:679, src/replay/state_clause.rs:743). Pre-existing | spec/replay/functional/FR-024-counterexample-envelope-intake.md:419 |
| FND-004 | low | FR-024-AC-5 carries `Test (TC-035)`, but the same document says the corpus path "is to be removed, not built on" and that AC-5 "stays unbacked". A criterion the spec plans never to meet should be retired or superseded by the FR-015-AC-109..111 removal, not kept Planned. Pre-existing | spec/replay/functional/FR-024-counterexample-envelope-intake.md:358, spec/replay/functional/FR-024-counterexample-envelope-intake.md:415-418 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The AC-10 sentences were inserted into the bullet that goes on to say "`quire coverage --strict` does not count them as unbacked … and the criteria stay planned". "Them" and "the criteria" now read as including FR-024-AC-10, which the matrix marks Covered. Move the AC-10 sentences into their own bullet, or say "AC-1 to AC-9" in the continuation | spec/replay/functional/FR-024-counterexample-envelope-intake.md:475-483 |

## Dispositions

Round 1, reviewed at 2acbf09a9e1ecd76cbef094e44214bd53317ca08. The "Current state" lead-in and the AC-1..AC-10 bullet were rewritten. Both are now true at this head, apart from the continuation wording in FND-005. Line 419 is unchanged and is owned by IR-890.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The lead-in now says none of AC-1 to AC-9 is met in full and that AC-10 has a source-scan test over `src/`; the bullet states the scan's scope and limits as the matrix row does |
| FND-002 | deferred | IR-890 (Backlog) owns the FR-016-AC-12 versus FR-029-AC-10 conflict; pre-existing |
| FND-003 | deferred | IR-890 (Backlog) owns the stale FR-024:419 prose; pre-existing |
| FND-004 | deferred | IR-890 (Backlog) owns retiring FR-024-AC-5; pre-existing |
| FND-005 | still-open | the continuation sentence still reads as covering AC-10 |

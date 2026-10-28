---
id: SR-611031
title: "Specification integrity review of IR-1086 PR 611"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md (FR-024-AC-30, Inputs, Current state); spec/replay/matrix/TC-035-counterexample-envelope-intake.md (step 27); spec/core/functional/interface-001-codegen-api.md (FrameReplay::new, read-only context); spec/assurance/AD-004-cg-crate-layout.md (L-6, Interim exceptions, read-only context)"
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-024
  type: references
- target: ix://agent-ix/quire-contract-codegen/AD-004
  type: references
---

## Summary

Ticket: IR-1086. PR: quire-contract-codegen#611 (draft), exact head
fda70b9fd76841a15a0f91681984a5ca48b48379, merge base and origin/main fb50acdc.
This lens checks the restated FR-024-AC-30 for consistency with the rest of FR-024, TC-035,
interface-001 and AD-004 L-6 at that head, and against the frame replay code at the merge base
(`src/replay/frame.rs`, `src/kani/classify.rs`, `src/kani/output/playback.rs`,
`tests/it/kani_obligations_state_frame.rs`), read-only. No Cargo, Kani or test run was made;
the installed-Kani outcome was checked as wording only (AC-30 and TC-035 expected result 27 are
unchanged).

## Verdict

**CHANGES REQUESTED (owner decision item).** AC-30 and TC-035 step 27 agree with each other, and
both agree with AD-004 L-6 and its Interim exception for `FrameReplayInputs::playback`. The
amendment leaves the documents around AC-30 stating the old boundary: FR-024's Current state
still claims AC-30 is met, and the Inputs list AC-30 incorporates, and interface-001's
`FrameReplay::new` inputs, still name playback text.

## Scope

```yaml
scope:
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: "With the installed backend, the real playback of the falsified frame harness of a subject that writes a forbidden field is replayed through `FrameReplay::new` and `replay`, the caller supplying the harness's `StateFrameIdentity`, the parsed playback produced from the complete retained backend output by the bounded Kani output reader, and the inputs of FR-024's Inputs list but no obligation identity and no transcript, and settles a reproduced violation naming the written field."
- id: FR-024-inputs-frame
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: "For a frame counterexample (IR-459): the falsified harness's `StateFrameIdentity` and the falsified run's playback text, both read by the generator, and the proving run's members, the operation, the invocation document reference, the pre, post and invocation documents and the claimed change, which the caller supplies as `FrameReplayInputs` does today. The caller supplies no obligation identity and no witness transcript."
- id: FR-024-current-state
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: "FR-024-AC-11 to FR-024-AC-30 are met: ... The frame path is implemented (IR-459): `FrameReplayInputs` carries the falsified harness's `StateFrameIdentity` and its playback text and no obligation identity or transcript."
- id: TC-035-step-27
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: examined
  excerpt: "With the installed backend (`make kani`), replay the real playback of the falsified frame harness of a subject that writes a forbidden field, supplying the harness's identity, the parsed playback produced from the complete retained backend output by the bounded Kani output reader and the Inputs list, but no obligation identity and no transcript."
- id: interface-001-FrameReplay-new
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "inputs: [FrameReplayInputs (proving-run ReplayInputs, domain package ProvidedDocuments, invocation and snapshot ProvidedDocuments, qsl-replay OperationName, invocation DocumentRef, ClaimedChange, the falsified harness's StateFrameIdentity, the playback text; no obligation identity and no transcript)]"
- id: AD-004-interim-frame
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "The public frame replay input carries transcript text, which `FrameReplay::new` passes to the parser through `decoded_state` | `replay/frame.rs`, `FrameReplayInputs::playback`, `FrameReplay::new` and `decoded_state` | IR-348 step 5 completion: pass typed `Playback` at `FrameReplayInputs::playback` and through its helpers"
- id: FR-024-AC-25
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: "The frame witness is built from the real playback: the playback text decodes through the one decoder of `src/replay/witness.rs` against bindings built from the harness's `state_fields` (each `i64`, in draw order) ..."
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-024 Current state still claims AC-30 met and AC-30 carries no PLANNED marker, but the code and test at this head pass playback text that `FrameReplay::new` parses | spec/replay/functional/FR-024-counterexample-envelope-intake.md:481; spec/replay/functional/FR-024-counterexample-envelope-intake.md:522; spec/replay/functional/FR-024-counterexample-envelope-intake.md:568-569; src/replay/frame.rs:65-66; tests/it/kani_obligations_state_frame.rs:5268-5271 |
| FND-002 | medium | AC-30 incorporates "the inputs of FR-024's Inputs list", whose frame bullet, like interface-001's `FrameReplay::new` inputs, still names "playback text", so the AC reads as both parsed playback and text | spec/replay/functional/FR-024-counterexample-envelope-intake.md:178-182; spec/core/functional/interface-001-codegen-api.md:233 |

## Detail

FND-001. After the restatement, AC-30 requires the caller to supply parsed playback.
`FrameReplayInputs::playback` is `String` ("The falsified run's concrete-playback text"),
`decoded_state` calls `parse_assertion_playback`, and the tagged test builds
`Run { playback: counterexample }` from the raw `String`. FR-024's Current state line
"FR-024-AC-11 to FR-024-AC-30 are met" and its IR-459 bullet "`FrameReplayInputs` carries ... its
playback text" are now false. Sibling ACs awaiting code use a `PLANNED (IR-624)` prefix; AC-30 has
none. TC-035's narrative (line 334) already calls AC-30 planned with IR-624, which also disagrees
with FR-024 Current state; that disagreement predates this PR. Suggested fix: prefix AC-30 with
PLANNED (IR-1086) and remove it from the met range in Current state, or amend Current state in
the same PR.

FND-002. The Inputs bullet says the generator reads "the falsified run's playback text", and
interface-001 lists "the playback text" among `FrameReplayInputs`. AC-30 says the caller supplies
parsed playback *and* those inputs. A reader can take that as supplying both. AC-25's "the
playback text decodes through the one decoder" is context: it stays true if the caller does the
parse, but it reads more cleanly if aligned. Suggested fix: restate the Inputs frame bullet and
interface-001's `FrameReplay::new` inputs to the parsed playback in this PR, or have AC-30 cite
the Inputs list "other than the playback".

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The fix marks AC-30 `PLANNED (IR-1086)` and Current state says only "FR-024-AC-30's typed frame intake" remains planned, but AC-30 also requires the emitted-package harness without `Twin::aligned` (FR-024-AC-32, IR-624), and TC-035's narrative still plans AC-30 under IR-624 | spec/replay/functional/FR-024-counterexample-envelope-intake.md:483; spec/replay/functional/FR-024-counterexample-envelope-intake.md:525; spec/replay/matrix/TC-035-counterexample-envelope-intake.md:337-339 |
| FND-004 | low | The FR-024 Inputs frame bullet now lists the requested decode limits among inputs "the caller supplies as `FrameReplayInputs` does today", but `FrameReplayInputs` carries no requested limits at this head (`FrameReplay::new` decodes under `run.decode_limits` only) | spec/replay/functional/FR-024-counterexample-envelope-intake.md:178-184; src/replay/frame.rs:559-579 |

FND-003. Neither the AC row nor Current state is wrong about IR-1086, but each names one owner
for a criterion with two pending parts. A reader taking Current state at face value would expect
AC-30 met once IR-1086 lands, while the emitted-package harness is still IR-624 work. Suggested
fix: mark the row `PLANNED (IR-1086, IR-624)` and drop "typed frame intake" from the Current state
sentence, or say both parts there.

FND-004. The parenthetical "typed intake planned in IR-1086" signals the change, but "does today"
now asserts something false of one listed member. Suggested fix: "which the caller supplies as
`FrameReplayInputs`" without "does today", or name the requested limits as IR-1086 work.

## Dispositions

Disposition pass 1 at exact head 2ea5e10663fa0ca7164b02f8a1381e80ca150ddc (fix commit
2ea5e106 over fda70b9f).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | AC-30 now carries `PLANNED (IR-1086).`; Current state no longer claims AC-30 met (range is AC-11 and AC-13 to AC-29, with AC-12 and AC-30 planned), and the IR-459 bullet says IR-1086 replaces the text input. Carving out AC-12 (IR-965) agrees with TC-035 step 10 and tests.md; it is not a narrowing. Residual owner attribution is new FND-003. |
| FND-002 | fixed | The FR-024 Inputs frame bullet and interface-001 `FrameReplay::new` inputs now name parsed `Playback` from the falsified outcome's complete counterexample text plus requested decode limits; no input still names playback text. interface-001 semantics add the minimum-limit recheck. Residual "does today" wording is new FND-004. |

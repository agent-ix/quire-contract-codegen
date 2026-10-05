---
id: SR-1518
title: "base review of quire-contract-codegen#280 (frame envelope obligation identity, FR-024-AC-20..30)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@70f5e08ac399d0880b50c1610592220c15298e09; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md (diff against main 6a6e721 only)"
review_set: subset
---

## Summary

Ticket: IR-459. PR: quire-contract-codegen#280, spec only, base 6a6e721 (the merge-base equals
main). This is the base checklist over the PR diff: ID format, numbering, matrix and TC-035
consistency, and the strict coverage delta measured on main and on the head.

Examined: FR-024-AC-1 (edited), FR-024-AC-20 to FR-024-AC-30 (new), the new FR-024 statements,
Inputs, Current state and Open questions, TC-035 steps and expected results 18 to 27 and its
Status note, the new tests.md row and the TC-035 summary row, and the AD-002 and AD-003 prose
edits.

Measured facts, re-checked against main 6a6e721:

- `src/replay/frame.rs` takes a caller `obligation_identity: [u8; 32]` (:50), sends
  `declared_domains: Some(Vec::new())` (:184) and the fixed `Witness::parse("<<<assertion|{operation}|frame|>>>")` (:150).
  No playback is read.
- The twin passes `[1; 32]` (`tests/state_frame_support/native_twin.rs:374,472`).
- `StateClauseReplayInputs::obligation_identity: [u8; 32]` (`src/replay/state_clause.rs:111`).
- The function path's identity is `function_contract_identity` in `src/replay/obligation.rs`,
  digested by `core::canonical::content_digest`, with members
  `function`/`declaration`/`kind`/`arguments`.
- `StateFrameIdentity` (`src/kani/identity.rs:257`) has no `state_fields` member. `domains`
  holds ranged fields only, and `Frame{granted, checked}` is unordered. Every measured claim
  holds.

Numbering: FR-024-AC-20 to FR-024-AC-30 follow the merged FR-024-AC-11 to FR-024-AC-19 with no
gap and no collision. The FR-031/TC-044 work is already on main and touches nothing in FR-024 or
TC-035. No other open PR edits these files: #222 touches AD-003, but it is a stale layout PR and
is not in this scope. TC-035 steps 18 to 27 map one to one onto AC-20 to AC-30, and the tests.md
row and summary row list exactly AC-20 to AC-30.

`quire validate --scope <tree> "spec/**/*.md"` exits 0 on both trees, with the same six EARS
warnings shifted by line. `quire coverage --strict` reports 44 unbacked rows on main and 45 at
the head; FND-001 is the cause.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-024-AC-25's matrix row holds the unescaped text `<<<assertion\|{operation}\|frame\|>>>`. The pipes inside the code span split the table row, so the parsed criterion is cut off at `<<<assertion` and its Verification cell reads `{operation}`. At the head, `quire coverage --strict` reports a new `uncatalogued-verification-method '{operation}'` and a new `FR-024-AC-25 ... has no backing symbol`, taking the count from 44 to 45 unbacked. The coverage delta is therefore not "only new planned rows". | spec/replay/functional/FR-024-counterexample-envelope-intake.md:291 |
| FND-002 | high | The second half of FR-024-AC-30 cannot be met. A subject that writes only a granted field leaves its frame harness `Verified`; main's `tc_025_real_kani_frame_counterexamples_replay_natively_through_qsl` asserts this at :2311. A verified run produces no counterexample and no playback, so there is no "playback of a subject that writes only a granted field" to replay "from the harness's StateFrameIdentity and the playback text alone". Today's inconclusive case uses a hand-built invocation instead, which AC-27's pre-state tie would now refuse. TC-035 step 27 and expected result 27 inherit the same impossibility. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:296; spec/replay/matrix/TC-035-counterexample-envelope-intake.md |
| FND-003 | low | FR-024-AC-30 replays "from the harness's `StateFrameIdentity` and the playback text alone". The new Inputs bullet, however, has the caller supply the run lock, packages, invocation reference, pre, post and invocation documents, and the claimed change. "Alone" contradicts those inputs; it should say the caller supplies no identity and no transcript. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:296 |

## Dispositions

Round 1, reviewed at b094aaca23d924831ee362d8865d31b74acbb753.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b094aac |
| FND-002 | fixed | b094aac |
| FND-003 | fixed | b094aac |

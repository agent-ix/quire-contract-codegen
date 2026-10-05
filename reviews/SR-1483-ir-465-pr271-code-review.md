---
id: "SR-1483"
title: "CG PR 271 code review (with rust-review lane and test-oracle strength): the FR-029 terminal map, the replay settlement conversions, the Duplicate and backend_manifest deletions and the QSL 02530e7 lock move"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@06075846dcd8f0ce3710c174bb4b80f29f638b4f; Cargo.lock, src/kani/{mod,terminal}.rs, src/lib.rs, src/replay/{function,frame}.rs, tests/it/{terminal_map,skeleton_spine,layout,main}.rs, tests/state_frame_support/native_twin.rs (diff 250dc84...HEAD, merge base 250dc84)"
---

# SR-1483: CG PR 271 code review

## Summary

Ticket: IR-465 (code half), with IR-611. PR: agent-ix/quire-contract-codegen#271 at 0607584,
merge base 250dc84. `origin/main` has one newer commit (f08b61a, PR 270, spec only), which does
not touch this diff. This review runs the code-review and rust-review lanes. It also measures
test-oracle strength with eleven hand-applied mutants, each reverted afterwards, and the
worktree was clean after the run.

What I measured myself rather than taking from the PR text:

- Gates. `make ci` on the head, with its own target dir, exited 0. That covers fmt-check, spec,
  lint, msrv, deny, audit-unsafe, rustdoc and test: 988 passed, 0 failed, 46 ignored summed over
  every test binary. `#[test]` attributes go from 566 on the merge base to 579 on the head
  (+13, all in `tests/it/terminal_map.rs`). No test is removed, and `#[ignore]` stays at 29 on
  both sides. The real-Kani lane (`make kani`) on the head ran 23 tests: 23 passed, 0 failed.
- Lock. Only first-party git sources move. The ten QSL crates go from 47dd209 to 02530e7, which
  is QSL `main` HEAD (compare: identical). `quire-contract-ir` and `quire-contract-model` go from
  6fb6e97 to 7e4dc54, which is one commit behind IR `main` (3be6ff7, IR-605). The lock holds
  one entry each for `quire-contract-ir` and `quire-contract-model`. No crates.io package
  changes. `Cargo.toml` and `deny.toml` are unchanged.
- `run_terminal_value` is one `match` over `(outcome, settlement)` with no wildcard arm. Every
  `{ .. }` ignores fields only, never a variant. `settled` and all seven `From` impls are also
  exhaustive with no `_` arm. The new non-test code has no `unwrap`, `expect`, `panic!`, indexing
  or `as` cast (NFR-005). The deleted pre-check removes the only indexing (`pair[0]`) from
  `admit`.
- A refusal from `qsl_replay::replay` goes through QSL's `TerminalValue::from_replay_refusal`
  (read at 02530e7: `Fault` and `Admission(AdmissionFailure::Fault)` map to `Failed`, and every
  other refusal to `Inconclusive(ReplayRefused(refusal.code()))`). The code adds no fault walk of
  its own. `CallSiteRefusal::Fault` maps to `ReplaySettlement::Fault`, bare and through both
  wrappers.
- `backend_manifest` is gone from `src/`, `tests/` and the FR, interface and matrix docs. The
  wire `backend` is `BACKEND_IDENTITY = "kani"`. QSL 02530e7's `ReplayRequestWire::backend` is a
  `String` that QSL refuses only when empty (`request.rs:540`), so this is the form QSL requires.
  The remaining mentions are in AD-004 and in old review files (see SR-1485).
- `DependencyLockError` is now `Input` only, and its `Display` and every `From` match follow.
  `ReplayInputs::admit` still sorts, so the request's ascending-identity order is kept: mutant
  M11 (no sort) is killed by `tc_026_the_request_package_reference_carries_the_lock_dependencies`.

Mutants (killed means at least one test failed):

| Mutant | Result | Killed by |
| --- | --- | --- |
| M1: swap the `TimedOut` and unwind causes | killed | `tc_040_a_timeout_and_an_exhausted_unwind_bound_are_incomplete` |
| M2: `ReplaySettlement::Fault` to `Inconclusive(ReplayRefused(runtime_invariant))` | **survived** | none |
| M3: `CallSiteRefusal::{UnknownOperation, UnknownClause}` to `CgDefect` | **survived** | none |
| M4: `DependencyLockError::Input` to a constant `invalid_package` code | **survived** | none |
| M5: `Verdict { disagreement: None }` to `Reproduced` | killed | `tc_040_only_a_reproduced_replay_refutes` |
| M6: `verdict_of` drops QSL's disagreement | killed | two `tc_026` replay tests |
| M7: `Disagreement` to `Failed` | killed | `tc_040_a_replay_disagreement_carries_its_cause` |
| M8: vacuous proof to `Proved { success_checks }` | killed | `tc_040_a_vacuous_proof_...` |
| M9: `SetupRefused` to `Failed` | killed | the AC-13 and AC-14 tests |
| M10: `FrameReplayError::Name` to `SetupRefused` | killed | the AC-11 test |
| M11: no sort in `admit` | killed | `tc_026_the_request_package_reference_...` |

Rust-review lane: the new module reads well. It has typed input, no string causes and a
hand-written `Display`/`Error` (the repo uses no `thiserror` in this area). The `kani` to
`replay` direction is kept: `kani/terminal.rs` imports only `qsl_replay` and `kani::classify`,
and `replay/` owns the conversions. The one design point, `TerminalPairError`, is a spec and
domain question and is recorded once, in SR-1484 FND-002.

## Verdict

Request changes, small. The map, the conversions, the deletions and the lock move are correct.
The gates pass, and 8 of 11 mutants are killed. Three survivors are real test-oracle gaps
against criteria the PR marks backed (FR-029-AC-13), or against a map arm that could be asserted
today (the fault reading). None is a defect in the shipped behaviour.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-029-AC-13 requires `DependencyLockError::Input` to carry "`DependencyInputRefusal::code()` of that refusal". But both refusals the tagged test builds, `duplicate_identity()` and `shared_owner()`, have code `invalid_package`, so a conversion that ignores the refusal and hard-codes `invalid_package` passes. Mutant M4 survived. Add QSL's `EmptyIdentity` refusal (`DependencyInput::new` with an empty identity, code `invalid_identifier`), bare and through both wrappers. | tests/it/terminal_map.rs:430-438, src/replay/function.rs:542-548 |
| FND-002 | low | FR-029-AC-13 covers "a non-fault `CallSiteRefusal`", but the tagged test builds six of the eight non-fault variants and omits `UnknownOperation` and `UnknownClause`. A conversion that sends those two to `CgDefect` (`Failed`) passes. Mutant M3 survived. Add both, each paired with a `DigestRecord` package, as `UnknownFunction` is. | tests/it/terminal_map.rs:389-417, src/replay/function.rs:526-540 |
| FND-003 | low | The map arm `ReplaySettlement::Fault => Failed` is not asserted anywhere. `with_settlements` passes `Fault` only to `!= Refuted` (AC-12) and `!= Tested` (AC-6), so mapping a fault to `Inconclusive(ReplayRefused(runtime_invariant))` passes. Mutant M2 survived. QSL's `InternalFault` cannot be built here, but `ReplaySettlement::Fault` is a unit variant this crate owns. Assert `map_settled(ReplaySettlement::Fault) == Failed` in an untagged TC-040 test, as the AC-3 half is. FR-029-AC-10 can stay planned. | tests/it/terminal_map.rs:369-380, src/kani/terminal.rs:132 |

## Dispositions

Round 1, reviewed at d58e94085841930693a14b565a6268843465fce1. That head is the fix commit
d58e940 on top of 1ebbf8f, a merge of `origin/main` f08b61a (PR 270). I checked the merge
myself: `diff 0607584..1ebbf8f` equals `diff 250dc84..f08b61a` except for hunk-context lines in
`spec/kani/matrix/tests.md`, so the merge brings in PR 270 and nothing else. The fix commit
changes only `tests/it/terminal_map.rs`, `tests/it/skeleton_spine.rs` and spec prose.
`src/`, `Cargo.lock` and `Cargo.toml` are unchanged since 0607584.

Re-measured on this head:
- `make ci` exited 0: 990 passed, 0 failed, 46 ignored.
- All 11 mutants are now killed, including M2, M3 and M4, which survived in the review pass.
- The real-Kani lane was not re-run. It passed 23 of 23 on 0607584, and the code it exercises is
  byte-identical here.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d58e940: the AC-13 test adds QSL's `empty_identity()` refusal, asserts its code is `invalid_identifier`, and maps it bare and through both wrappers; mutant M4 (constant `invalid_package`) is now killed |
| FND-002 | fixed | d58e940: `CallSiteRefusal::UnknownOperation` and `UnknownClause` are added to the AC-13 list; mutant M3 is now killed |
| FND-003 | fixed | d58e940: `tc_040_a_fault_settlement_is_failed` (tagged TC-040 only) asserts `ReplaySettlement::Fault` gives `Failed`; mutant M2 is now killed, and FR-029-AC-10 stays untagged |

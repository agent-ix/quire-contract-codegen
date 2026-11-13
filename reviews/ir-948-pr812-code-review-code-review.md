---
id: SR-17940486709564847047901
title: "IR-948 PR #812 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #812 at e1e3ec92; IR-948; tests/it/kani_obligations_v2_profiles.rs, tests/it/kani_obligations_v2_clause_controls.rs, tests/it/main.rs, tests/state_frame_support/native_twin.rs, docs/qualification/ir948-kani-profiles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-948. Test-and-docs-only PR (no `src/` change): a new `kani_obligations_v2_profiles`
module (one default-lane generation test, five `#[ignore]` Kani-lane tests), a
`Twin::with_withdraw_profile` fixture building the `test/bank` package (`balance: Int[lo, hi]`,
`withdraw(amount: Int[0, 100])`, `CanAfford` / `Debited` / `NonNegative`), an `invariant_inputs`
anchor that follows the declared operation (`withdraw` vs `deposit`), `pub(crate)` on four
helpers of `kani_obligations_v2_clause_controls` for reuse, and a qualification record.

Reproduction (reviewer, worktree at e1e3ec92, `CARGO_TARGET_DIR=<worktree>/target`, each step
under `flock /tmp/cargo-slot.lock`): `cargo test --test it kani_obligations_v2_profiles` 1
passed / 5 ignored; each Kani test run alone as
`systemd-run --user --scope -q -p MemoryMax=24G -p MemorySwapMax=0 timeout 900 <it> --ignored --exact ... --test-threads=1`:
pre-healthy `Verified` 6.9 s 633,466,880 B 1537/1558; pre-impossible
`CoverUnsatisfied { satisfied: 0, total: 1 }` 2.5 s 239,976,448 B; inv-healthy `Verified` 2.5 s
249,905,152 B; inv-strengthened `Falsified` two_phase=true 10.3 s 1,261,326,336 B; inv-widened
`Falsified` two_phase=true 10.4 s 1,262,522,368 B. All five pass and match the record's outcomes,
with wall and peak within run-to-run noise. `cargo fmt --check` clean; `cargo clippy --test it
-- -D warnings` clean.

Harness substance (read from `src/kani/generate/clause/v2.rs::render_clause`): the precondition
harness draws each input in its declared domain, calls the embedded oracle and covers
`Completed(true)`; it neither calls the subject nor asserts, so `subject_module("true")` is only a
compile stub and does not make the verdict vacuous; the cover is the meaningful check, and the
impossible control proves it can fail. The invariant harness asserts the oracle's
`Completed(true)` and covers domain reachability; both mutants falsify it, and the replay checks
that the decoded snapshot `balance` equals the expected in-domain value (0, -1) and settles
`ReproducedWithEvaluatedWitness` / `Violation`. The `pub(crate)` widening is the minimum needed
(`subject_module`, `OwnerMutation`, `ScratchCrate` and its `directory`, `ScopePeak`) and stays
crate-private.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | `cover_satisfied` passes when any check's `Debug` text equals `"Satisfied"`, not when the named V2 cover ("the V2 precondition is satisfiable within admitted input bounds" / "...admitted state domain is reachable") is satisfied. Correct today only because each harness has one cover; match the cover by its message and the status enum variant rather than a `Debug` string | tests/it/kani_obligations_v2_profiles.rs:115-120 |
| FND-002 | low | The impossible-precondition test asserts only `matches!(.., CoverUnsatisfied { .. })`. FR-015-AC-129 requires the verdict "from the proof phase" and the record states `{ satisfied: 0, total: 1 }`; add `assert!(evidence.proof_phase.is_none())` and the exact counts, as the healthy tests do for their phase | tests/it/kani_obligations_v2_profiles.rs:135-145 |
| FND-003 | low | `Twin::handler` picks the anchor by substring search for `/withdraw"` in the serialized domain JSON. Any future fixture with both operations, or a field/type identity ending in `/withdraw`, silently anchors wrongly. Record the operation name on the `Twin` when `with_withdraw_profile` renames it, or read `operations[].name` from the parsed document | tests/state_frame_support/native_twin.rs:631-637 |
| FND-004 | low | The Kani tests do not check that they ran under the 24 GiB cap the record and FR-015-AC-126 cite: `make kani` runs them with no `systemd-run` scope, where `ScopePeak` reads the session cgroup's cumulative `memory.peak`, and no test asserts `evidence.memory.peak_resident_bytes.is_some()` or a ceiling. The cap is only reproducible from the hand command in the record (same pattern as IR-917's controls) | tests/it/kani_obligations_v2_profiles.rs:82-113, Makefile:111-116 |
| FND-005 | low | Trace: the strengthened-mutant test is tagged `FR-024-AC-63` and the widened one `FR-024-AC-64`, but FR-024-AC-64 names both playbacks (strengthened and widened), and FR-024-AC-63's substantive claim (the transcript's operation name is the selected anchor's name, which is what the `handler()` fix changes) is not asserted by either test. Tag both with FR-024-AC-64 and either assert the transcript operation `withdraw` or drop the AC-63 tag | tests/it/kani_obligations_v2_profiles.rs:209-243 |

## Verdict

PASS with low findings only. The recorded verdicts are real and reproduce from the repository's
Kani-lane tests at e1e3ec92; the precondition verdict is not a vacuous cover (the impossible
control shows the cover can fail) and both invariant mutants falsify from the witness phase with
in-domain counterexamples that replay as reproduced violations. The `pub(crate)` changes and the
new fixture/anchor are sound. No code finding blocks merge; FND-001 to FND-005 are cheap to fix in
the same PR.

## Dispositions

Round 1, checked against 4630b054 (fix commit 4630b054 on e1e3ec92). The five Kani-lane tests
were rerun one at a time under `flock /tmp/cargo-slot.lock` in a 24G `systemd-run` scope: all pass
(pre Verified 6.0 s 617,893,888 B; impossible `CoverUnsatisfied { 0, 1 }` 2.5 s; inv Verified
2.5 s; both mutants Falsified two-phase 10.2 s / 10.3 s, ~1.26 GB). Default lane
`kani_obligations_v2_profiles`, `kani_obligations_state_clause_replay`,
`kani_obligations_state_frame` 128 passed; `state_clause_invariant_replay` 11 passed; fmt and
clippy `-D warnings` clean.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | `named_cover_satisfied` requires exactly one `KaniCheckClass::Cover`, at the line of the named cover message, with `KaniCheckStatus::Satisfied` |
| FND-002 | fixed | impossible test asserts `CoverUnsatisfied { satisfied: 0, total: 1 }`, `proof_phase.is_none()` and the named cover not satisfied |
| FND-003 | fixed | `Twin::handler` reads `operations[0].name` of the `Account` type from the parsed domain document |
| FND-004 | fixed | every Kani test calls `assert_within_cap` (identity ceiling 24 GiB, supplied peak below it); the record's new Run conditions section states the scope is the caller's under `make kani` |
| FND-005 | fixed | both mutant tests carry FR-024-AC-63 and FR-024-AC-64, and `falsified_and_replayed` asserts the transcript `<<<assertion\|withdraw\|NonNegative\|balance={expected}>>>` |

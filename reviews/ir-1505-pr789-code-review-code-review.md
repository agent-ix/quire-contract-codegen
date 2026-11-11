---
id: SR-789001
title: "IR-1505 PR #789 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #789 at 8f6517e1; IR-1505; src/replay/state_clause/invariant.rs, src/replay/state_clause.rs, src/kani/identity.rs, src/kani/generate/clause/v2.rs, src/lib.rs, tests/it/state_clause_invariant_replay.rs, tests/it/main.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1505. PR #789 (one commit 8f6517e1 on origin/main 3a4fa795, branch code/ir-1505-impl) adds `StateClauseReplay::new_invariant(InvariantReplayInputs)` and `InvariantReplayInputs::decode_falsification(StateClauseHarness, &Playback, WitnessDecodeLimits)` in a new child module `src/replay/state_clause/invariant.rs`. It adds `StateClauseReplayError::NotInvariantHarness`, mapped to CG defect in the settlement conversion, and moves the invariant assertion message into `kani/identity.rs` as `V2_INVARIANT_ASSERTION_MESSAGE`, shared with `v2.rs`. `state_clause.rs` gains a shared `populations()`, and `state_fields()` now takes the object node instead of the `ClauseShape`; the postcondition `new` is otherwise unchanged. Reviewed read-only in /home/peter/dev/worktrees/IR-1505-review, scoped to `git diff origin/main...HEAD`. The rust-review lane is folded in here. One focused baseline job (`--test it state_clause_invariant_replay`: 7 passed), then four mutation runs, one at a time. No Kani, no lint, no full suite; the coder reports it 878 and lib 536.

## Verdict

**MERGEABLE after the SR-789002 and SR-789003 spec-text edits; no code change required.** The code does what FR-024-AC-57 to AC-64 say, `StateClauseReplay::new` and its Invocation path are untouched apart from the `state_fields(object)` signature, and all four reviewer mutation controls are caught. The three code findings are low.

Rust surface: no new `unsafe`, `unwrap`, `expect` or `panic!` outside `#[cfg(test)]` and test files. The `i128` to `i64` playback conversion is checked with `try_from` into a typed `Decode`. The decode limits are the component-wise minimum of the requested and the proving-run limits. `ReplayPackage::read(.., u64::MAX)` repeats the postcondition path's existing call. Public API: `InvariantReplayInputs` and `StateClauseHarness` are re-exported only at the crate root (`lib.rs:301`), which satisfies layout L-2 (only the crate root re-exports), and the names fit the `StateClause*` family. `StateClauseHarness` is an enum over borrowed harness identities, which is a reasonable shape for a closed "which harness" input. Trace tags use `#[trace(..)]` via ix-trace-rs, and no `/// Trace:` line was added. No hash, digest or pin was added. `make spec` gives 6 warnings, the same set as main, none in an edited file. Overlap: #788 (IR-1507) changes `ObligationItem::V2Clause` (adds a required `claim`) and the `kani/identity.rs` and `clause/v2.rs` import blocks next to this PR's hunks. Whichever lands second needs a compile-level rebase of `generate_with` in the new test file and a trivial import merge. #583 (IR-917) touches `native_twin.rs`, `identity.rs`, `v2.rs` and `lib.rs`, a textual merge only. #553 also touches `native_twin.rs`; #535, #474, #413 and #381 share `lib.rs` and `tests/it/main.rs` lines only. #787 touches only `tests.md`, on a different row.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `new_invariant` takes `object_node` and `object` (the snapshot's `object_type`, population and key) from the caller, and never ties either to the invariant's declaring object. The postcondition path reads the object from `ClauseShape` (`shape.scope.object`). On the invariant path, only `decode_falsification` checks `object_node` against the harness's `declaring_node_id`. `new_invariant` and every field of `InvariantReplayInputs` (including `playback`) are public, and the tests themselves fill `playback` directly and call `new_invariant`. A caller can then pass another object's node, and CG reads the field set and the `OutOfDomain` range from that object, which is the only range check on this path. Impact is bounded. A reviewer probe shows QSL's admission refuses a snapshot value outside the domain package's declared type (`invalid_runtime_input`/`invalid-value`), so a wrong value cannot settle a verdict. A wrong `object_node` produces a wrong or missing CG-typed refusal (`OutOfDomain` against the wrong range, or a valid value refused), not a wrong verdict. Fix: in `new_invariant`, check `object_node` against the object the invariant is declared `on` (from the admitted package or QSL's `ClauseSite`), and return `HarnessMismatch { Object }` or a new typed refusal. | src/replay/state_clause/invariant.rs:214-256; src/replay/state_clause.rs:634 |
| FND-002 | low | The caller's `obligation_identity` is copied into the request and packet without a check against the harness, as on the postcondition path. Open PR #788 (IR-1507) puts QSL's `ObligationIdentity` on `KaniV2ClauseObligationHarness`, after which `decode_falsification` can compare the two. Follow-up once #788 lands; not blocking here. | src/replay/state_clause/invariant.rs:303-311 |
| FND-003 | low | Gap 4, untested branches. `decode_falsification`'s refusals for a collection-precondition or collection-operation invariant harness (`NotInvariantHarness`), an invariant harness with no drawn state field (`HarnessMismatch { StateFields }`), a provenance mismatch (`Decode` `cg_witness_harness_provenance_mismatch`), a non-integer value and an `i64` overflow are not exercised by any test. M3 shows the kind check is held only by the postcondition and precondition cases. The behaviour is consistent with AC-62's closed set (ruling (ii) in SR-789003). Add one test per refusal, or name the uncovered ones in the TC-035 status. | src/replay/state_clause/invariant.rs:116-152, 184-195 |

## Mutation controls (reviewer)

| Mutant | Change | Outcome |
| --- | --- | --- |
| M1 | `observation` built as `ClauseSelectionInput::Invocation { invocation: snapshot_label.reference() }` | killed, 6 of 7 tests fail, including step 39's observation-form assertion (state_clause_invariant_replay.rs:220) |
| M2 | `call_site` given the first `invariant` declared in the unit source instead of the requested name; `ClauseSelectionMismatch` check disabled | killed, step 41 fails on the replay target (line 327) and step 44's `CallSite` case fails (line 680) |
| M3 | `identity.kind != ObligationKind::Invariant` dropped from the `NotInvariantHarness` guard | killed, step 44 fails on the `NotInvariantHarness` assertion (line 524); only the postcondition and precondition cases hold it |
| M4 | invariant path binds with `Side::PostState`, skipping CG's `OutOfDomain` range check | killed, step 43 (`OutOfDomain`, line 430) and step 45's widened-playback check (line 789) fail. A reviewer probe under M4 shows that QSL itself then refuses `balance = -1` against `Int[0, 100]` at admission (`invalid_runtime_input`/`invalid-value`, field `balance`) |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| src/replay/state_clause/invariant.rs | examined | src/replay/state_clause/invariant.rs:1-388 | StateClauseHarness, InvariantReplayInputs, decode_falsification, new_invariant, unit test |
| src/replay/state_clause.rs | examined | src/replay/state_clause.rs:161-1060 | NotInvariantHarness, settlement mapping, populations(), state_fields(object), StateClauseReplay::new |
| src/kani/identity.rs | examined | src/kani/identity.rs:359-361 | V2_INVARIANT_ASSERTION_MESSAGE |
| src/kani/generate/clause/v2.rs | examined | src/kani/generate/clause/v2.rs:40, 742 | shared message constant |
| src/lib.rs | examined | src/lib.rs:301 | pub use replay::state_clause::invariant::{InvariantReplayInputs, StateClauseHarness} |
| tests/it/state_clause_invariant_replay.rs | examined | tests/it/state_clause_invariant_replay.rs:1-792 | seven tagged tests |
| tests/state_frame_support/native_twin.rs | examined | tests/state_frame_support/native_twin.rs:440-518 | with_bank_invariants, invariant_inputs |

## Dispositions

Round 1, head 4c1b0509 (fix commit 72867ce8, rebased on main 0f3bc428 after #788). Focused job at the new head: lib unit test and 8 integration tests pass. quire matrix shows FR-024-AC-57..64 tagged; make spec gives 6 warnings, the same as main, none in edited files. Reviewer mutations: D1, removing ensure_unique_documents from request_documents, is caught by the unit test (invariant.rs:378). D2, having new_invariant bypass the seam and build documents inline without the guard, is not caught; it is an equivalent mutant on public behaviour, because no public input yields a duplicate (AC-62 now says so). The two unreachable decode branches (non-integer, i64 overflow) were folded into one guard with a source comment; that is acceptable, because the decoder's I64 binding yields WitnessValue::Integer(i128::from(i64)) only (replay/witness.rs:375-380).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Not fixed at 4c1b0509. new_invariant has no check against the invariant's declaring object. The only object handling is the pre-existing state_fields(&graph, object_node, &requested), which refuses ModelFields only for a node that is not a model object or does not declare the drawn field. The new test passes the clause node (not a model object) and a 'ghost' field, and both were already refused before this round. Another model object that declares the same field name (with a different range), and an object.object_type address that disagrees with object_node, still pass. Close it by comparing object_node with the invariant's on-object (from the admitted package or the ClauseSite) and adding a test with a second model object declaring balance. Or, since the impact is bounded (QSL admission refuses out-of-range values; never a wrong verdict), record it as deferred to a ticket. AC-62's new ModelFields wording accurately describes what the code does. |
| FND-002 | deferred | IR-1516 (Backlog) owns cross-checking the caller's obligation_identity against the harness identity, now carried on the harness after #788. FR-024's Current-state paragraph names IR-1516. |
| FND-003 | fixed | after: tc_035_the_remaining_invariant_refusals_are_typed: collection_precondition / collection_operation set on an invariant harness -> NotInvariantHarness; edited subject_path -> Decode(cg_witness_harness_provenance_mismatch); invariant 'Constant' (1 >= 0) with no drawn field -> HarnessMismatch { StateFields }. invariant.rs: // The `i64` binding the decoder types yields only an integer within `i64`, so this refusal is the decoder contract's guard, not a reachable input. |

Round 2, head 850fea91 (round-2 fix commit 850fea91; rebased on main 9b79e19d). Focused job: lib unit test plus 8 integration tests pass; quire matrix shows FR-024-AC-57..64 tagged; make spec gives 6 warnings, same as main. Reviewer mutations: R1, dropping the object-type address check, is caught by case (b) (Account node + Vault address). R2, dropping the read-field declaring-node check, is caught by case (a) (Vault node + Vault address). Both fail through the test's refusal expectation at tests/it/state_clause_invariant_replay.rs:936. On the 'every field the clause reads' notion: it ties object_node to the declaring node of each state read, and read_clause gives every state read Some(declaring_node_id). It is vacuous for an invariant that reads no field. That case is refused earlier on the decode path (HarnessMismatch StateFields) and cannot affect a verdict, because the clause reads nothing, so it is acceptable. A stronger form would compare against the invariant's on-object declaration directly.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | after: ensure_declaring_object, run by new_invariant before state_fields, refuses HarnessMismatch{Object} when a read field's declaring node is not object_node or object_type is not object_node's model address |

---
id: SR-13244
title: "IR-878 CG PR 456 code review (with Rust lane): FR-024 intake tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@943f4c268ba9288ee8a71f3677e6afcb55b2c6f8 vs origin/main fb380d29; src/replay/function.rs, tests/it/layout.rs, tests/it/kani_obligations_state_clause_replay.rs, tests/it/kani_obligations_state_frame.rs"
review_set: subset
---

## Summary

Ticket: IR-878. PR agent-ix/quire-contract-codegen#456. This review covers the four new or retagged tests for FR-024-AC-2, AC-3, AC-4 and AC-6, with the Rust lane folded in. Tags use `#[trace(...)]` from ix_trace_rs. No `/// Trace:` lines were added. The diff adds no hash, pin, digest, vendored copy or compatibility layer, and it does not touch `Cargo.toml` or `Cargo.lock`.

The reviewer measured the following at the head:

- The focused tests pass (6 of 6).
- `make fmt-check` passes.
- `make lint` passes.

Reviewer mutants:

- **AC-3:** bypassing `first_out_of_domain` in `replay_harness_counterexample_through` fails the new test at `a=-1`.
- **AC-6:** setting the frame packet's `clause_node` to the anchor fails the new test.
- **AC-2:** a fully qualified `qsl_replay::Witness::parse(..)` call outside `render_witness` passes the new source scan (FND-001).

AC-4 covers all 13 `WitnessPacket` members of the pinned `qsl-replay`. `StateClauseReplay::replay_through` calls `WitnessEnvelope::reconstruct` before the executor (`src/replay/state_clause.rs:743`).

## Examined criteria

- FR-024-AC-2 (examined): "Every backend-witness transcript the generator passes to `Witness::parse` is produced by the one adapter rendering function from decoded values, and that function takes no backend-native text as input."
- FR-024-AC-3 (examined): "A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not."
- FR-024-AC-4 (examined): "For each `WitnessPacket` member in turn, a packet missing that member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present with the value none is admitted."
- FR-024-AC-6 (examined): "A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-2 source scan matches only an `ExprCall` whose path is exactly the two segments `Witness`, `parse`. A fully qualified `qsl_replay::Witness::parse(..)` outside `render_witness` passes the test (reproduced with a mutant in `src/replay/frame.rs`). A path use such as `.map(Witness::parse)` also passes. Fix: visit `ExprPath`, not only `ExprCall`, and match any path whose last two segments are `Witness` and `parse`. Alternatively, state these limits in the AC-2 matrix row, as the AC-10 row does for its own scan | tests/it/layout.rs:47-55 |
| FND-002 | low | AC-4 lists the 13 `WitnessPacket` members by hand. If QSL adds a member, the test still passes without exercising it, although the criterion says "each member in turn". Fix: add an exhaustive destructure with no `..` (`let WitnessPacket { obligation_identity: _, ... } = &replay.packet;`), so a new member breaks the build | tests/it/kani_obligations_state_clause_replay.rs:446-458 |
| FND-003 | low | The AC-3 endpoint test with a counting executor drives only `replay_harness_counterexample_through`. The public `replay_counterexample_through` has its own `first_out_of_domain` check (function.rs:1114). Its endpoints and the values next to them are not tested. Only one far-out value is tested, by the FR-016-AC-2 spine test. Fix: run the same table through both entry points | src/replay/function.rs:1230 |

## Verdict

**PASS with findings.** The AC-3, AC-4 and AC-6 tests exercise their criteria, and production mutants fail them. FND-001 is a real hole in the AC-2 scan that is cheap to close. Fix it in this PR before merge. FND-002 and FND-003 are low-severity hardening.

## Dispositions

Disposition pass 1, against head 34704a9a9146bc9f581c34c00aa0e9397882ef4a.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The scan now visits every `ExprPath` whose last two segments are `Witness`, `parse` (tests/it/layout.rs). Reviewer mutants now fail the test: a qualified `qsl_replay::Witness::parse(..)` call, and `.map(qsl_replay::Witness::parse)`, each added in src/replay/frame.rs |
| FND-002 | fixed | An exhaustive `let WitnessPacket { obligation_identity: _, ... family_payload: _ } = &replay.packet;` with no `..` now breaks the build if QSL adds a member |
| FND-003 | fixed | New `skeleton_spine::tc_035_public_function_domain_boundaries_stop_before_replay` runs endpoints and adjacent values through `replay_counterexample_through` with a counting executor. A reviewer mutant that bypasses its `first_out_of_domain` fails the test at `amount_current=-1` |

---
id: SR-17940486709564847047631
title: "IR-631 PR #777 scalar settlement spec dependency review"
type: SpecReview
analysis: dependency
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #777 @ da8e03e5; IR-631; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/replay/matrix/tests.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/spec.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

## Summary

Ticket: IR-631. Spec-only PR #777 (head da8e03e5, one commit on origin/main f38f8024), reviewed in a detached worktree of that head, scoped to git diff origin/main...HEAD (6 files). Checked against origin/main source: src/replay/scalar_builder.rs (ScalarBuildError's 11 variants, settle(&ScalarSentRequest, Option<&OperatorParityReport>) -> Result<qsl_replay::TerminalValue, ScalarBuildError>, plan/complete), src/kani/terminal.rs (ReplaySettlement's six variants, TerminalPairError, run_terminal_value, ir_outcome_terminal_value), src/replay/composite_builder.rs (OriginalReplayContext::new, check_recompiled_context), src/kani/identity.rs, src/core/scalar_authority.rs, tests/it/scalar_replay_binding.rs, scalar_replay_residuals.rs, scalar_playback.rs, scalar_literal_operands.rs, and qsl-replay at the locked rev 434869e (scalar.rs OperatorParityResult and terminal_value, execute/operator_parity.rs, proof_result.rs from_replay_refusal, bounds.rs) plus quire-exact 5db2d5e (numeric.rs, accounting.rs). make spec and quire matrix pass at the PR head; no grammar warnings in the changed files. No cargo and no Kani were run. Dependency: prerequisites by ticket, relationship edges, and open-PR interactions. Linear: IR-648, IR-940, IR-1184 and IR-304 are Done; PRs #481, #702, #741 and #757 are merged. Open PRs: #523 edits FR-024 rows next to the replaced FR-032 row of spec/replay/matrix/tests.md (a textual conflict, no id collision); #776 changes run_terminal_value (WitnessDisagreed), which AC-22's 'same value as the ordinary entry point' will inherit. No open PR adds FR-032 criteria.

## Verdict

PASS WITH LOWS. The new criteria depend only on merged CG and QSL items. Three lows: the remaining gates are named without tickets, TC-047 keeps a stale verifies-FR-029 edge, and AC-19 still names IR-304, which is Done.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-032's status paragraph names the remaining whole-route gates as 'the driver's same-proved-artifact execution and a real falsified Kani run', and FR-032-AC-8 and tests.md row 29 speak of 'the real-Kani lane', none by ticket number. Name the owning tickets, for example the driver's E9/S6b run (IR-610, per the IR-631 description) and the real-Kani falsification lane ticket. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:499 |
| FND-002 | low | This PR removes FR-029-AC-15 from TC-047's criteria in tests.md, and the TC-047 trailer now says FR-029-AC-15 is not checked here. TC-047's frontmatter still declares 'verifies FR-029', so the relationship graph keeps an edge with no criterion behind it. Change it to references, or drop it. | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:5 |
| FND-003 | low | FR-032-AC-19 and tests.md row 31 stay 'PLANNED (IR-304 slice b)'. IR-304 is Done in Linear, and CG origin/main has no guard_inputs decoding, so the criterion is open under a closed ticket. The PR's status refresh did not touch it. Name a live owning ticket. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:408 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-032 status paragraph | examined | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:499 | Status is stated per criterion. AC-1 to AC-6 and AC-15 to AC-18 are **Partial**: the merged native tests (#702, #741, #757, #481; IR-1184) assert the CG-side checks each row names, and the rest of each row stays Planned. AC-7, AC-8, AC-12, AC-14, AC-19 and AC-20 to AC-25 are **Planned**; AC-9 to AC-11 and AC-13 are retired. The IR-648 and IR-940 gates are closed (both Done); the remaining gates for the whole route are the driver's same-proved-artifact execution and a real falsified Kani run. No unsupported-capability refusal exists in `ScalarBuildError` today. The merged function-level decl... |
| FR-032-AC-8 | examined | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:397 | \| FR-032-AC-8 \| PLANNED (IR-631); no test exercises either fixture through the scalar builder (verified by grep: AC-8 has no tag), and real-Kani playback is unavailable to the default lane. The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition fixture with both operands and result in `[-1000,1000]` exercises an admitted operand pair whose exact result is outside the result range. The expected exact result is derived from the actua... |
| FR-032-AC-19 | examined | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:408 | \| FR-032-AC-19 \| PLANNED (IR-304 slice b). For a scalar identity with `guard_inputs`, the expected playback entry count is the operand count plus the guard-input count, and the guard entries follow the operands in `guard_inputs` order: an integer guard as one eight-byte `i64` and a Boolean guard as one byte. A short playback refuses as `kani_witness_arity_mismatch`; a four-byte integer guard entry as `kani_witness_width_mismatch`; a Boolean byte `2` or an integer guard value one outside its narrowed domain as `cg_scalar_witness_guard_out_of_domain`. The decoded operand vector sent to QSL ex... |
| TC-047 frontmatter | examined | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:5 | relationships: - target: ix://agent-ix/quire-contract-codegen/FR-032 type: verifies - target: ix://agent-ix/quire-contract-codegen/FR-029 type: verifies - target: ix://agent-ix/quire-contract-codegen/FR-022 type: references |
| TC-047 trailer | examined | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:221 | All checks are planned; completed-route observations have the same explicit remaining CODE gates as [FR-032](../functional/FR-032-routed-scalar-replay-binding.md). Scalar lowering replay never produces a source-property `Refuted` or `Verified` terminal. The scalar absent-settlement check is FR-032-AC-23; ordinary [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15 is unchanged and still governs `run_terminal_value`. Missing upstream types/causes must be reported as a gate, not implemented as local QSL lookalikes. |
| tests.md TC-047 row | examined | spec/replay/matrix/tests.md:49 | \| TC-047 \| Check public routed scalar lowering replay and staged-content binding \| Integration \| P0 \| FR-032-AC-1, FR-032-AC-2, FR-032-AC-3, FR-032-AC-4, FR-032-AC-5, FR-032-AC-6, FR-032-AC-7, FR-032-AC-8, FR-032-AC-9, FR-032-AC-10, FR-032-AC-11, FR-032-AC-12, FR-032-AC-13, FR-032-AC-14, FR-032-AC-15, FR-032-AC-16, FR-032-AC-17, FR-032-AC-18, FR-032-AC-19, FR-032-AC-20, FR-032-AC-21, FR-032-AC-22, FR-032-AC-23, FR-032-AC-24, FR-032-AC-25 \| 🚧 Partial native evidence (IR-1184): original reader/generator positional bodies, all current integer operators, actual driver observations and QSL repor... |
| tests.md row FR-032 planned | examined | spec/replay/matrix/tests.md:29 | \| FR-032 \| FR-032-AC-7, FR-032-AC-8, FR-032-AC-12, FR-032-AC-14 \| TC-047 \| 🚧 Planned (IR-631): no test is tagged to these criteria; AC-8's bounded-addition fixture and the real-Kani lane have no scalar test. \| |
| tests.md row FR-032-AC-19 | examined | spec/replay/matrix/tests.md:31 | \| FR-032 \| FR-032-AC-19 \| TC-047 \| 🚧 Planned (IR-304 slice b): guard-input playback decoding for guarded routed scalar harnesses awaits code \| |
| FR-029 scalar sentence | examined | spec/kani/functional/FR-029-run-outcome-terminal-record.md:311 | The routed scalar route (FR-032-AC-20 to FR-032-AC-24, Planned under IR-631) likewise uses a distinct typed `ScalarSettlement` and its own pair entry points, which return the same `TerminalPairError`; `ReplaySettlement` and its six readings are unchanged. |
| FR-032-AC-22 | context_only | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:411 | \| FR-032-AC-22 \| PLANNED (IR-631). `scalar_run_terminal_value(&KaniExecutionEvidence, Option<ScalarSettlement>)` and `scalar_ir_outcome_terminal_value(&KaniOutcome, Option<ScalarSettlement>)` pair the settlement with the Kani outcome. A `Falsified` run or a `Counterexample` outcome with a settlement gives the AC-21 value; an outcome of any other kind with a settlement returns `TerminalPairError::UnexpectedSettlement` and no value; an outcome with no settlement gives the same value as the ordinary entry point. `ReplaySettlement` and the ordinary map are unchanged. \| Test \| |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in 22d49b1e; verified at fa59e8a6 |
| FND-002 | fixed | fixed in 22d49b1e; verified at fa59e8a6 |
| FND-003 | fixed | fixed in 22d49b1e; verified at fa59e8a6 |

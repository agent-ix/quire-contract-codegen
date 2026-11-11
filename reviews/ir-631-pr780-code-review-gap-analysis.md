---
id: SR-17940486709564847047645
title: "IR-631 PR #780 scalar settlement gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #780 @ 630e1e39; IR-631; FR-032-AC-7, FR-032-AC-8, FR-032-AC-20..25 against src/ and tests/it/; quire matrix"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

## Summary

Ticket: IR-631. PR agent-ix/quire-contract-codegen#780, branch code/ir-631-scalar-settlement, head 630e1e3941b517477bfd4daa0a588d1d375e248e (one commit), reviewed in a detached worktree /home/peter/dev/worktrees/IR-631-review, scope git diff origin/main...HEAD (12 files). Authoritative spec on main: FR-032-AC-7/8/20..25, TC-047 steps 10-14, FR-029 scalar paragraph, reviews/ir-631-pr777-spec-review-*. Reviewer runs (thermal budget: one focused run plus one mutation control): `cargo test --locked --test it scalar_settlement` with a temporary reviewer probe test: 10/10 PR tests pass and the probe confirmed QSL refuses the bounded-addition source; mutation control (Inconclusive run / TimedOut IR kind moved into the settled arm) with filters scalar_settlement and tc_047_bounded_addition: 12/12 still pass, so the mutant survives. Both edits reverted; worktree clean. No Kani, no make ci. quire matrix tags every flipped AC; quire validate passes on the four edited spec files.

## Verdict

Plan completion: not assessed. NOT MERGEABLE until fixed: two medium status-accuracy findings; the flips are right once the code-review test findings land and the AC-8 wording is corrected.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-8 status overstates the bounded-addition evidence. tc_047_bounded_addition_... decodes a hand-written transcript of the test-chosen pair (600,600); it does not read retained playback. TC-047's FR-032-AC-8 mutation column names exactly this ('assume (600,600) instead of reading actual playback') as a mutation that must fail. The status cell says it 'derives the exact sum 1200 of the decoded (600,600) playback', and the tests.md AC-7/AC-8 row says 'the x + 1 and bounded-addition fixtures (AC-8) are tested without Kani', although the bounded-addition fixture reaches no native observation or QSL call. PARTIAL is the right status; reword both cells to say the bounded-addition test checks the descriptor ranges and the decoder over a synthetic transcript of a chosen pair, and that the retained-playback derivation is not delivered. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:402 |
| FND-002 | medium | AC-22 and AC-24 are flipped to IMPLEMENTED, tests.md marks AC-20 to AC-25 '✅ Covered' and the TC-047 description says scalar_settlement.rs 'implements the settlement steps 10 to 14', while two clauses have no failing test: AC-22's 'an outcome of any other kind with a settlement' (only Verified/Proved/Refused are paired; reviewer mutation control moving Inconclusive/TimedOut into the settled arm survived) and AC-24's Operands cause (never evaluated; TC-047 step 13 requires it). See code-review FND-001 and FND-002 of this PR. The flips become accurate once those tests land; until then the statuses claim more than is asserted. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:416 |

## Coverage

Gap-analysis (manual AC-to-test check plus quire matrix): every flipped criterion has a tagged test; quire matrix reports FR-032-AC-7, AC-8, AC-20..AC-25 as tagged, with binders in tests/it/scalar_settlement.rs and kani_obligations.rs. Per AC: AC-20 asserted (Parity borrows the report's own result; missing report -> Report; changed budget -> SentContext; settle removed). AC-21 asserted over genuine reports for all reachable and facade-only classes plus inspection; Refused-fault by Inspection as specified. AC-22 partially asserted (FND-002). AC-23 asserted with settlement(...).ok() passed directly (never a literal None) for absent and other-run reports to both pair functions. AC-24 asserted for every reachable kind via real errors and the rest via an exhaustive local mirror, except Operands (FND-002). AC-25 asserted (compiled flow with ScalarObligationIdentity only; scan of scalar_builder.rs and lib.rs re-exports). AC-7 PARTIAL is honest: the conversion is asserted over genuine QSL reports, a real falsified Kani run is not. AC-8 PARTIAL is honest in status but its wording overstates (FND-001); its unsatisfiable half is a spec matter (spec-review SR of this PR). No production code without an owning requirement; no stubs.

## Examined units and bindings

```yaml
scope:
- id: FR-032-AC-7
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-631): the conversion is delivered by FR-032-AC-20 to FR-032-AC-24 (which own the mapping table below) and asserted over genuine QSL reports in `tests/it/scalar_settlement.rs` (tagged AC-7); a falsified real-Kani run reaching it stays PLANNED. QSL''s divergence between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive(ScalarAgrees)`; native `Incomplete`/execution fault becomes `GeneratedFault` and `Failed` while exact `Incomplete` becomes `Incomplete(ResourceExhausted)`. `RefusedInput` pr...'
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-631): `scalar_settlement.rs` runs the `x + 1` fixture (input `Int[0,9]`, result `Int[0,10]`) through routed generation, plan, native observation and QSL, and asserts that its operand ranges reach only `[1,10]`, so it has no out-of-result-range value; `kani_obligations.rs` asserts the `[-1000,1000]` bounded-addition descriptor and derives the exact sum `1200` of the decoded `(600,600)` playback outside the result range. Native observation and QSL parity of the bounded-addition fixture stay PLANNED, because QSL''s checker refuses its source (`Unproved(Range)`), so no QSL-emitted pa...'
- id: FR-032-AC-20
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: IMPLEMENTED (IR-631). When the driver passes the genuine `OperatorParityReport` that `replay_operator_parity` returned, with the `ScalarSentRequest` it actually sent, to `CompletedScalarReplay::settlement`, the converter, after the AC-6 checks, returns `ScalarSettlement::Parity` borrowing that report's `OperatorParityResult`. A missing report, another run's report, or a wire or replay budget different from the retained original context returns `ScalarBuildError::Report` or `ScalarBuildError::SentContext` and no `ScalarSettlement`. `ScalarSettlement` is a distinct typed input of the Kani ter...
- id: FR-032-AC-21
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-631). The scalar terminal map reads `OperatorParityResult` through one exhaustive `match` with no wildcard or catch-all arm (`#[deny(clippy::wildcard_enum_match_arm)]`), each arm returning `result.terminal_value()` so that QSL owns every value and a new QSL result class fails the build. QSL''s values are: `Diverged` and `GeneratedFault` give `Failed`; `Agrees` gives `Inconclusive(ScalarAgrees)` carrying that report''s agreement; `RefusedInput` gives `Inconclusive(ReplayRefused)` carrying the refusal''s own code (`invalid_runtime_input`); `Incomplete` gives `Incomplete(ResourceE...'
- id: FR-032-AC-22
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: IMPLEMENTED (IR-631). `scalar_run_terminal_value(&KaniExecutionEvidence, Option<ScalarSettlement>)` and `scalar_ir_outcome_terminal_value(&KaniOutcome, Option<ScalarSettlement>)` pair the settlement with the Kani outcome. A `Falsified` run or a `Counterexample` outcome with a settlement gives the AC-21 value for `Parity` and `Failed` for `CgDefect`; an outcome of any other kind with a settlement returns `TerminalPairError::UnexpectedSettlement` and no value; an outcome with no settlement gives the same value as the ordinary entry point. `ReplaySettlement` and the ordinary map are unchanged.
- id: FR-032-AC-23
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-631). For a falsified run (`KaniRunOutcome::Falsified` evidence and a `Counterexample` IR outcome), omitting the settlement returns `TerminalPairError::MissingSettlement` and no terminal value from both `scalar_run_terminal_value` and `scalar_ir_outcome_terminal_value`. A converter refusal (AC-20) leaves no `ScalarSettlement`: the result of `settlement(...).ok()` for a missing report or another run''s report, passed to either pair function, yields `MissingSettlement`; a refusal cannot be turned into a terminal record.'
- id: FR-032-AC-24
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-631). `ScalarBuildError::setup_settlement` is an exhaustive `match` over every variant and, for `Authority`, over every `ScalarAuthorityError` cause. It returns `None` for a named unavailable capability, which FR-032 says is not relabelled a CG defect and gets no settlement: `Authority` caused by `Unavailable` (a routed `Implies` scalar, which has no operator-parity route: `src/kani/generate/scalar.rs` returns it), `Occurrence` (no unique expression occurrence; multi-occurrence replay awaits an owning generation change) and `Operands`, and `ScalarBuildError::Operation` (a fa...'
- id: FR-032-AC-25
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-631): `src/replay/scalar_builder.rs` names neither `KaniObligationIdentity` nor CG''s `ReplayPackage` (its `raw_package` parameter is `qsl_replay::ReplayPackage`), and `tests/it/scalar_settlement.rs` asserts both halves. A compiled end-to-end call of `generate_routed`, `OriginalScalarContext::plan`, `ScalarReplayPlan::complete`, `CompletedScalarReplay::request`, `CompletedScalarReplay::settlement` and `scalar_run_terminal_value` uses only `ScalarObligationIdentity`, and a source scan finds no `KaniObligationIdentity` or CG `ReplayPackage` token, and no `settle` method, in the...'
- id: FR-032 status paragraph (line 37)
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'The full proof-driven replay pipeline remains Planned under IR-631. CG''s

    original scalar body/limits retention, checked plan completion, native

    observation comparison and the full-claim report check

    (`CompletedScalarReplay::settlement`, which replaced `settle`, FR-032-AC-20),

    the typed settlement and its pairing with the Kani outcome (FR-032-AC-20 to

    AC-25) are implemented. Native tests exercise the actual QSL

    operator route; they do not establish execution of the same proved artifact'
- id: TC-047 Description
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'Planned public-consumer coverage of

    [FR-032](../functional/FR-032-routed-scalar-replay-binding.md), distinguishing lowering parity

    failure from source-property refutation. Partial native evidence exists (IR-1184: #702, #741,

    #757, #481): `scalar_playback.rs`, `scalar_literal_operands.rs`, `scalar_replay_binding.rs` and

    `scalar_replay_residuals.rs` drive the real routed generation, the real QSL operator facade and

    a compiled generated artifact, without a Kani run. `scalar_settlement.rs` implements the

    settlement steps 10 to 14 (IR-631). Steps 1, 2 and 7 to 9 remain Planned, as does any real

    ...'
- id: tests.md FR-032 AC-7/AC-8 row
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-032 | FR-032-AC-7, FR-032-AC-8 | TC-047 | 🚧 Partial (IR-631): the conversion over genuine QSL reports (AC-7) and the `x + 1` and bounded-addition fixtures (AC-8) are tested without Kani. A real falsified Kani run (the driver''s E9 run is IR-610) and native observation of the bounded-addition fixture, whose QSL source is refused by QSL''s checker, remain Planned. |'
- id: tests.md FR-032 AC-20..25 row
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-032 | FR-032-AC-20 through FR-032-AC-25 | TC-047 | ✅ Covered (IR-631): `ScalarSettlement`, `CompletedScalarReplay::settlement` (which replaced `settle`), `ScalarBuildError::setup_settlement`, the scalar pair functions, the missing-settlement checks and the typed-seam scan are asserted in `tests/it/scalar_settlement.rs`; the settlement/terminal pairing is a trusted-driver boundary (FR-032 trust boundary). |'
bindings:
- test_id: scalar_settlement::step_10_settlement_wraps_the_genuine_report_result_and_reads_the_fr032_table
  ac_id: FR-032-AC-20
  trace: correct
- test_id: scalar_settlement::step_10_settlement_wraps_the_genuine_report_result_and_reads_the_fr032_table
  ac_id: FR-032-AC-21
  trace: correct
- test_id: scalar_settlement::step_10_settlement_wraps_the_genuine_report_result_and_reads_the_fr032_table
  ac_id: FR-032-AC-7
  trace: correct
- test_id: scalar_settlement::step_10_refused_input_exact_incomplete_and_refusal_readings_come_from_genuine_reports
  ac_id: FR-032-AC-21
  trace: correct
- test_id: scalar_settlement::step_10_refused_input_exact_incomplete_and_refusal_readings_come_from_genuine_reports
  ac_id: FR-032-AC-7
  trace: correct
- test_id: scalar_settlement::step_10_settlement_refuses_a_changed_input_bytes_budget_and_a_missing_report
  ac_id: FR-032-AC-20
  trace: correct
- test_id: scalar_settlement::step_10_terminal_map_is_an_exhaustive_match_returning_the_qsl_terminal_value
  ac_id: FR-032-AC-21
  trace: correct
- test_id: scalar_settlement::step_11_pairing_requires_a_falsified_run_and_never_refutes
  ac_id: FR-032-AC-22
  trace: correct
- test_id: scalar_settlement::step_12_an_omitted_or_refused_settlement_leaves_no_terminal_value
  ac_id: FR-032-AC-23
  trace: correct
- test_id: scalar_settlement::step_13_each_reachable_setup_error_is_classified_and_pairs_as_specified
  ac_id: FR-032-AC-24
  trace: correct
- test_id: scalar_settlement::step_13_causes_no_public_input_reaches_keep_their_classification
  ac_id: FR-032-AC-24
  trace: correct
- test_id: scalar_settlement::step_14_the_flow_uses_only_the_scalar_identity_and_the_scan_finds_nothing
  ac_id: FR-032-AC-25
  trace: correct
- test_id: scalar_settlement::increment_fixture_has_no_out_of_result_range_value_and_agrees_natively
  ac_id: FR-032-AC-8
  trace: correct
- test_id: kani_obligations::tc_047_bounded_addition_playback_derives_an_out_of_result_range_exact_value
  ac_id: FR-032-AC-8
  trace: correct
```

---
id: SR-17940486709564847047644
title: "IR-631 PR #780 scalar settlement code-review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #780 @ 630e1e39; IR-631; src/kani/terminal.rs, src/replay/scalar_builder.rs, src/lib.rs, tests/it/scalar_settlement.rs, tests/it/kani_obligations.rs, tests/it/scalar_replay_binding.rs, tests/it/scalar_replay_residuals.rs, tests/it/main.rs"
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

NOT MERGEABLE until fixed: production code is correct and matches FR-032-AC-20..AC-25 and the FR-032 per-cause table exactly; three medium test-strength findings (one reproduced by mutation) and four lows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-22 says an outcome of ANY other kind with a settlement returns UnexpectedSettlement, but step_11 only pairs a settlement with KaniRunOutcome::Verified and with KaniOutcomeKind::Proved and Refused. Reviewer mutation control (reproduced): moving KaniRunOutcome::Inconclusive into the Falsified arm of scalar_run_terminal_value and KaniOutcomeKind::TimedOut into the Counterexample arm of scalar_ir_outcome_terminal_value leaves all 11 scalar_settlement tests (plus the AC-8 kani_obligations test) green. CoverUnsatisfied, Inconclusive and Cancelled runs, and the InvalidInput, IncompleteInput, Unavailable, TimedOut, ResourceExhausted, Cancelled and Inconclusive IR kinds are never paired with a settlement. Iterate every non-Falsified KaniRunOutcome variant and every non-Counterexample KaniOutcomeKind (synthetic_evidence and KaniOutcome::non_success already build them) with both Parity and CgDefect and require UnexpectedSettlement; also require the None path equals the ordinary entry point for each. | tests/it/scalar_settlement.rs:319-387 |
| FND-002 | medium | No test ever evaluates setup_settlement on ScalarAuthorityError::Operands. The test-local mirror expected_cg_defect names Operands(_), but the step_13_causes list of unreachable causes omits it, and no reachable fixture produces it, so moving Operands to the CgDefect arm of setup_settlement survives every test. TC-047 step 13 names Operands among the causes the local match must assert, and AC-24 lists it as a None (named unavailable capability). CheckedScalarOperandError has public unit variants (e.g. UnknownNode), so add ("authority operands", ScalarBuildError::Authority(ScalarAuthorityError::Operands(CheckedScalarOperandError::UnknownNode))) to the list. | tests/it/scalar_settlement.rs:734-785 |
| FND-003 | medium | The 'every reachable exact sum lies inside the result range' assertion compares two hard-coded literal intervals ([1,10] inside [0,10]), so it is a tautology: it cannot fail whatever the generated identity or package says. AC-8 requires the witness domain/result-range relation to be asserted, and the FR-032-AC-8 status claims the test 'asserts that its operand ranges reach only [1,10]'. Derive reach from identity.arguments (sum of minima, sum of maxima) and read the result range from the generated harness or admitted package, not from literals. | tests/it/scalar_settlement.rs:869-874 |
| FND-004 | low | The syn inspection counts only top-level Pat::Wild arms. A catch-all binding arm (other => ...) or a `_` inside an or-pattern (OperatorParityResult::Refused(_) / _) is not counted, so the inspection alone does not establish 'no wildcard or catch-all arm'; only clippy under make lint does. Also count Pat::Ident arms without a subpattern and recurse into Pat::Or. The same inspection is not applied to ScalarBuildError::setup_settlement, whose AC-24 also requires an exhaustive match: assert its deny attribute and no wildcard arm too. | tests/it/scalar_settlement.rs:240-249 |
| FND-005 | low | The closing check asserts only that CgDefect on a Counterexample is not Refuted, under a comment saying it 'must differ from the unsettled entry point'. The unsettled entry point returns Err(MissingSettlement), not Refuted, so the comment is wrong and the check is weaker than the AC-22 statement 'Failed for CgDefect'. Assert Ok(TerminalValue::Failed) for both pair functions (or drop it, since assert_classified already does) and fix the comment. | tests/it/scalar_settlement.rs:382-386 |
| FND-006 | low | Only scalar_run_terminal_value's doc states the FR-032 trust boundary (the settlement is not checked to belong to the evidence's obligation; the driver is the trusted pairing authority). scalar_ir_outcome_terminal_value says 'the same shape' but not that it also performs no obligation check, and ScalarSettlement's doc is silent. Nothing claims otherwise, but state the boundary on both pair functions (or once on ScalarSettlement) so the public API carries it. | src/kani/terminal.rs:312-319 |
| FND-007 | low | The AC-8-tagged test ends with a comment-only claim ('The increment case has no such pair: operands in [0, 9] and [1, 1] reach only [1, 10].') that asserts nothing in this test. Remove it or point to the increment test that asserts it (after FND-003 makes that assertion real). | tests/it/kani_obligations.rs:750-751 |

## Coverage

Code-review with the rust-review lane over src/kani/terminal.rs, src/replay/scalar_builder.rs, src/lib.rs and all test changes. Verified against source and spec: (1) setup_settlement implements the FR-032-AC-24 table row by row: CgDefect for Playback, Staging, Identity and Authority(Operation|Ranges|Record); None for Authority(Unavailable|Occurrence|Operands|GeneratedArtifactChanged), Operation, BodyMismatch, Context, Artifacts, Observation, Report, SentContext. Malformed/decode/domain witness failures (Playback) give CgDefect -> Failed. scalar_settled returns result.terminal_value() on every one of six exhaustive arms; qsl-replay 434869e OperatorParityResult::terminal_value and TerminalValue::from_replay_refusal yield only Failed, Inconclusive(ScalarAgrees|ReplayRefused) or Incomplete(ResourceExhausted), so no path yields Refuted, Proved, Tested or Inconclusive(ReplayParity); ScalarSettlement has only Parity and CgDefect. (2) Mutant judgment: swapping a setup_settlement row fails step_13 (mirror match plus explicit asserts and the defects list) except for Operands (FND-002); Diverged -> Refuted fails step_10 (value == report.terminal_value()) and step_11; accepting a settlement on a Verified run fails step_11 but on other non-falsified kinds survives (FND-001, reproduced); a `_` arm fails the syn scan and clippy, a binding catch-all only clippy (FND-004). (4) Trust boundary: no pair function compares obligations, per spec; run doc states it (FND-006 for the IR doc). (5) settle is removed with no alias or shim; grep over src/tests/spec/docs finds no CompletedScalarReplay settle( call (remaining `settle` tokens are unrelated: kani/run/execute.rs::settle, prose). All three migrated callers assert Parity via ptr::eq on report.result(). (6) Trace tags are #[trace("TC-047","FR-032-AC-NN")] attributes; no `/// Trace:` lines; scalar_settlement is a `mod` of tests/it/main.rs; no in-src tests added, so the no-panic scanner sibling rule does not apply; no hashes or pins; Cargo.toml/Cargo.lock untouched (syn already a dev-dependency). (7) New non-test code has no unwrap/expect/panic/index/unsafe/integer conversion; both new matches are exhaustive without wildcard under deny(clippy::wildcard_enum_match_arm, match_wildcard_for_single_variants). kani/terminal.rs imports nothing from replay/ (AD-004); replay/ constructs ScalarSettlement. Lifetimes: settlement<'r> borrows the report, setup_settlement returns 'static. 

## Examined units and bindings

```yaml
scope:
- id: src/kani/terminal.rs::ScalarSettlement
  path: src/kani/terminal.rs
  role: examined
  excerpt: "/// The settlement of a routed scalar counterexample's operator-level replay (FR-032-AC-20).\n///\n/// A distinct input of the scalar pair functions, not a variant of [`ReplaySettlement`]: the\n/// scalar route proves lowering parity, never a source refutation, so `Reproduced` and\n/// `Inconclusive(ReplayParity)` are unrepresentable here. `replay/` builds the value.\n#[derive(Debug)]\npub enum ScalarSettlement<'a> {\n    /// The genuine QSL operator-parity result of the driver's replay.\n    Parity(&'a OperatorParityResult),\n    /// A failure CG raised on a supported scalar family (FR-032-AC-24): ..."
- id: src/kani/terminal.rs::scalar_settled
  path: src/kani/terminal.rs
  role: examined
  excerpt: "/// The value of a scalar settlement: QSL owns every `Parity` value (FR-032-AC-21).\n#[deny(\n    clippy::wildcard_enum_match_arm,\n    clippy::match_wildcard_for_single_variants\n)]\nfn scalar_settled(settlement: ScalarSettlement<'_>) -> TerminalValue {\n    match settlement {\n        ScalarSettlement::Parity(result) => match result {\n            OperatorParityResult::Diverged { .. } => result.terminal_value(),\n            OperatorParityResult::Agrees { .. } => result.terminal_value(),\n            OperatorParityResult::GeneratedFault(_) => result.terminal_value(),\n            OperatorParityResul..."
- id: src/kani/terminal.rs::scalar_run_terminal_value
  path: src/kani/terminal.rs
  role: examined
  excerpt: "/// [`run_terminal_value`] for a routed scalar run (FR-032-AC-22): a `Falsified` run with a\n/// settlement reads as the settlement's value; any other outcome with a settlement is refused,\n/// and an outcome with none gives the ordinary entry point's value.\n///\n/// The settlement is not checked to belong to this evidence's obligation: the driver is the\n/// trusted pairing authority.\n///\n/// # Errors\n///\n/// [`TerminalPairError`] when a falsified run has no settlement, or another outcome has one.\npub fn scalar_run_terminal_value(\n    evidence: &KaniExecutionEvidence,\n    settlement: Option<Sc..."
- id: src/kani/terminal.rs::scalar_ir_outcome_terminal_value
  path: src/kani/terminal.rs
  role: examined
  excerpt: "/// [`ir_outcome_terminal_value`] for a routed scalar outcome (FR-032-AC-22); the same shape as\n/// [`scalar_run_terminal_value`] over a `Counterexample` outcome.\n///\n/// # Errors\n///\n/// [`TerminalPairError`] when a `Counterexample` has no settlement, another kind has one, or\n/// the ordinary entry point refuses.\npub fn scalar_ir_outcome_terminal_value(\n    outcome: &KaniOutcome,\n    settlement: Option<ScalarSettlement<'_>>,\n) -> Result<TerminalValue, TerminalPairError> {\n    match settlement {\n        None => ir_outcome_terminal_value(outcome, None),\n        Some(settlement) => match outc..."
- id: src/replay/scalar_builder.rs::CompletedScalarReplay::settlement
  path: src/replay/scalar_builder.rs
  role: examined
  excerpt: "/// Compare the entire genuine report claim before reading any terminal outcome, then return\n    /// the report's result as the scalar settlement (FR-032-AC-20).\n    /// # Errors\n    /// Another actual-sent claim, including different native evidence or context, refuses.\n    pub fn settlement<'r>(\n        &self,\n        actual_sent: &ScalarSentRequest,\n        report: Option<&'r OperatorParityReport>,\n    ) -> Result<ScalarSettlement<'r>, ScalarBuildError> {\n        if actual_sent.replay_limits != self.replay_limits\n            || !same_wire(&actual_sent.wire, &self.wire)\n        {\n         ..."
- id: src/replay/scalar_builder.rs::ScalarBuildError::setup_settlement
  path: src/replay/scalar_builder.rs
  role: examined
  excerpt: "}\n\nimpl ScalarBuildError {\n    /// The settlement of a falsified run whose replay setup failed (FR-032-AC-24): `CgDefect`\n    /// only for a failure CG raised on a supported scalar family; `None` for a named unavailable\n    /// capability and for a binding mismatch, which must not write a record for another obligation.\n    #[deny(\n        clippy::wildcard_enum_match_arm,\n        clippy::match_wildcard_for_single_variants\n    )]\n    pub fn setup_settlement(&self) -> Option<ScalarSettlement<'static>> {\n        match self {\n            Self::Authority(cause) => match cause {\n                Sc..."
- id: src/lib.rs::kani::terminal re-exports
  path: src/lib.rs
  role: examined
  excerpt: "pub use kani::terminal::{\n    ir_outcome_terminal_value, run_terminal_value, scalar_ir_outcome_terminal_value,\n    scalar_run_terminal_value, ReplaySettlement, ScalarSettlement, TerminalPairError,\n};"
- id: tests/it/scalar_settlement.rs
  path: tests/it/scalar_settlement.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Scalar replay settlement converter and terminal pairing (TC-047 steps 10 to 14, FR-032-AC-8,

    //! FR-032-AC-20 to FR-032-AC-25). Native controls only: no Kani proof or backend falsification.'
- id: tests/it/kani_obligations.rs::tc_047_bounded_addition_playback_derives_an_out_of_result_range_exact_value
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: '/// FR-032-AC-8, bounded-addition half: the descriptor with both operands and the result in

    /// `[-1000,1000]` admits an operand pair whose exact sum is outside the result range, and the

    /// decoded retained playback, not an assumed solver choice, gives the expected exact result.

    /// The native observation and QSL parity of this fixture stay gated: QSL''s checker refuses the

    /// source `function subject(x: Int[-1000, 1000], y: Int[-1000, 1000]): Int[-1000, 1000]

    /// { x + y }` (`Unproved(Range)`), so no QSL-emitted package carries these checked ranges.

    #[trace("TC-047", "FR-032-AC-8")]

    #[tes...'
- id: tests/it/scalar_replay_binding.rs (settle -> settlement migration)
  path: tests/it/scalar_replay_binding.rs
  role: examined
  excerpt: "assert!(matches!(\n        completed.settlement(&sent, Some(&report)).unwrap(),\n        quire_contract_codegen::ScalarSettlement::Parity(result) if std::ptr::eq(result, report.result())\n    ));"
- id: tests/it/scalar_replay_residuals.rs (settle -> settlement migration)
  path: tests/it/scalar_replay_residuals.rs
  role: examined
  excerpt: "assert!(matches!(\n            completed.settlement(&sent, Some(&report)).unwrap(),\n            quire_contract_codegen::ScalarSettlement::Parity(result) if std::ptr::eq(result, report.result())\n        ));"
- id: qsl-replay@434869e scalar.rs::OperatorParityResult::terminal_value
  path: qsl-replay/src/scalar.rs
  role: context_only
  excerpt: Self::Diverged { .. } | Self::GeneratedFault(_) => TerminalValue::Failed, Self::Agrees { agreement, .. } => Inconclusive(ScalarAgrees(agreement.clone())), Self::RefusedInput(refusal) => Inconclusive(ReplayRefused(refusal.code())), Self::Incomplete(_) => Incomplete(ResourceExhausted), Self::Refused(refusal) => TerminalValue::from_replay_refusal(refusal)
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

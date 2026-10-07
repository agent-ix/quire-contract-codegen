---
id: "SR-2230"
title: "CG IR-666 code review: composite parity report converter and full sent-claim binding"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-666-composite-converter (frozen candidate, no PR yet, one commit 'IR-666: bind composite parity reports to sent claims' over main; reviewed revision recorded in the IR-666 Linear marker only, per this repository's no-SHA rule); src/lib.rs, src/replay/composite.rs, src/replay/mod.rs, tests/it/composite_parity_converter.rs, tests/it/main.rs; read against spec/replay/functional/FR-033-composite-parity-replay-binding.md (AC-9, AC-13), spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-28), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (AC-15 to AC-17), spec/kani/matrix/TC-041-ir-outcome-terminal-map.md (steps 14 to 16), spec/replay/matrix/TC-048-composite-parity-replay-binding.md (steps 8a, 10) and the qsl-replay crate at this branch's lock-pinned revision (composite.rs, execute/composite_parity.rs, proof_result.rs)"
---

# SR-2230: CG IR-666 code review

## Summary

Ticket: IR-666. Method: code-review with the rust-review lane folded in. Static review only: the
reviewer ran no cargo build, test, Clippy or Kani. The author reports the five converter tests and
Clippy with `-D warnings` passing. The reviewer did not verify that claim.

The diff adds `composite_parity_terminal_value` and `verified_shadow_terminal_value`, which take
the retained sent `CompositeIdentity` and an optional QSL report. They return
`CompositeReportError::MissingReport` for an absent report and `ClaimMismatch` when
`report.claim() != sent`. Otherwise they return the report's `result()` and QSL's own
`terminal_value()`. The diff also adds five integration tests that obtain real reports from QSL's
public `replay_composite_parity` and `settle_verified_shadow`.

Checked against the qsl-replay source at the lock-pinned revision:

- `CompositeIdentity` derives `PartialEq` over all nine members, the full evidence included.
  `CompositeIdentity::new` is built before `prepare`, so every report carries the full claim,
  a `Refused` report included. A single `!=` therefore compares every member, and no CG code
  keeps a member subset that could drift.
- Terminal ownership. The converter returns `report.terminal_value()` unchanged. It has no CG
  mapping table, no re-derivation and no second terminal vocabulary. QSL's
  `TerminalValue::from_replay_refusal` maps `Fault` and `Admission(Fault)` to `Failed` and every
  other refusal to `ReplayRefused(code)`. CG passes that mapping through, which satisfies the
  FR-029-AC-28 inspection by construction.
- `src/kani/terminal.rs` and `ReplaySettlement` are unchanged, so the ordinary path is unchanged.
- Cargo.toml and Cargo.lock are untouched. The diff adds no dependency, no compatibility layer
  and no copied QSL type.
- The new library code has no `unwrap`, `expect`, indexing or arithmetic. The converter takes no
  limits, and the tests pass `ReplayLimits` as the caller.
- The added files contain no SHA, local path or conflict marker.

Error type: no spec names the refusal type. FR-033 says only "typed CG refusal". The two variants
are the two refusals the spec names (missing report, wrong-claim report). Neither variant carries
data, so `Debug` and `Display` leak nothing.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-030-AC-17 is overclaimed. Two tests carry `Trace: ... FR-030-AC-17`, which moves AC-17 from untagged to tagged in `quire matrix`. Neither test produces an F-7 result. Every report in the file stops at QSL `prepare`, and no test reaches F-7 `Diverged`/`Agrees` or pairs Disagreed with a later admission refusal after a valid `prepare`. AC-17's main statement is that the retained shadow verdict or pair count alone gives F-7 Failed/CgDefect, and agreement gives `Inconclusive(ScalarAgrees)`. Failure scenario: the matrix reports AC-17 as backed, so the F-7 consumer check is never written. The `tc_048_` prefix also binds a TC-041 criterion: TC-048 does not list FR-030-AC-17. Fix: drop FR-030-AC-17 from both Trace lines, or add the F-7 and later-refusal cases under a `tc_041_` test | tests/it/composite_parity_converter.rs:80-82, tests/it/composite_parity_converter.rs:116-118 |
| FND-002 | low | `CompositeParitySettlement` and `VerifiedShadowSettlement` describe a "binding-checked" result, but both fields are `pub` and the struct stores `terminal_value` separately from `result`. Any caller can build one with no binding check, or with a `terminal_value` that disagrees with `result`. Nothing in CG relies on the type as a proof token yet, so nothing is wrong today. Rust idiom for an invariant-carrying output is private fields with `result()` and `terminal_value()` accessors, with the value derived from the result | src/replay/composite.rs:34-49 |
| FND-003 | low | Member-mutation coverage is narrower than FR-033-AC-9 and TC-048 step 10 list. The native observation is mutated only to `Completed(..)` and `Incomplete(cause)`. `ExecutionFault(cause)` and `Refused(code)` and their payloads never appear, and the falsified/verified evidence variant is never swapped. Because QSL derives `PartialEq`, no CG mutant survives this today. The positive binds at :128, :233 and :257 are asserted only with `is_ok()`. Tests at :118, :216 and :244 never assert which QSL result their report carries, and the verified test asserts only `matches!(.., Refused(_))`. A fixture that later reached a different QSL row would pass unnoticed. `Display for CompositeReportError` is untested, so a mutant of its strings survives | tests/it/composite_parity_converter.rs:128, tests/it/composite_parity_converter.rs:168-196, tests/it/composite_parity_converter.rs:288, src/replay/composite.rs:20-29 |

## Verdict

The converter is correct and minimal. It compares the whole `CompositeIdentity` before reading
any result. It refuses a missing report and a mismatched claim with a typed, data-free error.
It carries QSL's result and terminal value through unchanged, and it leaves the ordinary
`ReplaySettlement` path and the dependency graph untouched. The real-report tests use only QSL's
public API, fabricate no report or F-row control, and cover the reachable `prepare` refusal
(FR-029-AC-28) and the binding of a request changed before send. The one material defect is
trace overclaim on FR-030-AC-17 (FND-001). FND-002 and FND-003 are idiom and test-strength
nits.

## Dispositions

Round 1. Reviewed at the branch's second frozen head: fix commit "Bind composite reports and
cover public parity rows", 2 commits and 16 files over the merge base. The revision is recorded
in the IR-666 Linear marker only. This is a static re-check: the reviewer ran no cargo. The
committed `reviews/SR-2230-ir-666-code-review.md` is byte-identical to this artifact's
first-custody text.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | round-1 fix commit "Bind composite reports and cover public parity rows": FR-030-AC-17 is off both `tc_048_` tests and on `tc_041_f7_compares_verdict_and_pair_count_then_preserves_agreement`, which drives real F-7 Diverged (verdict, count) and Agrees reports. The `prepare`-before-Disagreed clause is now uncovered; see FND-004 |
| FND-002 | fixed | round-1 fix commit: both settlements hold one private `result` borrow with `result()` and a `terminal_value()` derived from QSL's own `result.terminal_value()`. They have no `pub` field and no `Clone`, `Default` or `From`, so only the two binding functions can construct them |
| FND-003 | fixed | round-1 fix commit: ExecutionFault and Refused native mutations, cause-only changes for both fault kinds, falsified/verified evidence swaps in both directions, pinned QSL rows on every positive bind, and a Display test |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The fix lost the `prepare`-before-Disagreed control. The previous `tc_048_bound_prepare_refusal_passes_through_qsl_terminal` fixture carried `Refinement::Disagreed`. The rewritten test uses `falsified_evidence()`, whose refinement is `NotExhausted`, so no falsified test pairs a real `prepare` refusal with Disagreed. The verified `prepare` test shows precedence over V-1 only. FR-029-AC-28 ("QSL `prepare` refusal precedes F-1"), FR-030-AC-17 and TC-041 step 16 ("a `prepare` non-fault `Refused` result with Disagreed ... require ... F-1 precedence") each require it. Yet kani/matrix/tests.md:53 and replay/matrix/tests.md:23 mark FR-029-AC-28 "Covered", and the TC-041 Status says step 16's `prepare` refusal is exercised. Failure scenario: a QSL change that let Disagreed win over a `prepare` refusal passes every test. Fix: set `refinement = Refinement::Disagreed` in that test, as before | tests/it/composite_parity_converter.rs:509-543, spec/kani/matrix/tests.md:53, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:100-105 |
| FND-005 | low | F-4 and F-5 do not assert the counter. Both stage checks assert `limit == 0`, `consumed == 0` and `next_charge == 2`, but not `limit_kind`. FR-030-AC-16, FR-033-AC-13 and TC-041 step 15 each require the counter ("with the counter", "Inspect ... each incomplete report stage and counter"). Assert `incomplete.limit_kind` for both stages | tests/it/composite_parity_converter.rs:305-340 |
| FND-006 | low | The `ParityArgument` construction inside `public_composite_fixture` is mis-indented: `identity:` sits left of its enclosing block. The filter-closure line is longer than the 100-column `max_width`, so rustfmt silently gives up on the whole statement. `make fmt-check` passes, but the block stays unformatted. Bind the filtered bounds to a local first so rustfmt can lay the statement out | tests/it/composite_parity_converter.rs:172-182 |

Checked with no finding: every report in the file comes from QSL's public `replay_composite_parity`
or `settle_verified_shadow`. The fixture is source text compiled by QSL's public `compile_package`,
read back through CG's strict `CheckedPackageV2` reader, and given an O-09 identity by QSL's public
`parity_obligation`. No test hand-builds a `CompositeParityReport` or a QSL result. No production
builder, invocation or original-artifact binding was added; the only src change is the
converter. Cargo.toml and Cargo.lock are untouched. The diff has no SHA, local path or conflict
marker. The branch is one commit behind `main` (IR-682, FR-034/TC-049 only). `git merge-tree`
reports a clean merge.

Round 2. Reviewed on the rebased branch `code/ir-666-composite-converter-r2` at its frozen head
(fix commit "Close composite converter review findings"; 3 commits ahead of `main`, 0 behind).
The revision is recorded in the IR-666 Linear marker only. This is a static re-check. No file
under `src/` and neither Cargo file changed since round 1, so the public surface is still the
five items interface-001 declares.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | round-2 fix commit "Close composite converter review findings". The test is now `tc_041_bound_prepare_refusal_precedes_disagreed`, it sets `evidence.refinement = Refinement::Disagreed`, and it requires a QSL `Refused` (`BoundExceeded`) result with `ReplayRefused(code)`. A QSL change that let F-1 win over `prepare` would hit the `else { panic!(..) }` arm, so that mutant dies. It now traces FR-029-AC-28, FR-030-AC-17 and TC-041 |
| FND-005 | fixed | round-2 fix commit: F-4 and F-5 both assert `incomplete.limit_kind.as_str() == "value_occurrences"`. That is quire-exact's `LimitKind::ValueOccurrences` spelling at the lock-pinned revision |
| FND-006 | fixed | round-2 fix commit: the filtered bounds are bound to `parameter_bounds` before `ParityArgument { .. }`. Every line now fits the 100-column width, and the statement is laid out normally |

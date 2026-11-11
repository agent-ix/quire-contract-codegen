---
id: SR-787002
title: "IR-814 PR #787 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #787 at 7ba3df5b; IR-814; FR-033-AC-14..17; spec/replay/matrix/tests.md; tests/it/composite_original_eq.rs; src/replay/composite_builder.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
---

## Summary

Ticket: IR-814. Plan completion: not assessed. `quire matrix --scope .` (quire 0.36.2) on the PR
head: FR-033-AC-14 tagged (composite_original_eq.rs:728), FR-033-AC-15 tagged (:783), FR-033-AC-16
method-without-symbol (Inspection), FR-033-AC-17 untagged (Test). `make spec` exits 0 with no
errors. The DELIVERED flips for AC-14 and AC-15 are backed by tagged tests.

AC-17 stays PLANNED with verification method Test on the claim that no authentic fixture reaches
the inline branch. That claim does not hold up. IR's own production-reader test
(quire-contract-ir tests/it/checked_package_v2_composite_operands.rs:1097-1129, and
`admitted_option_none_exposes_exact_public_value` at :22) admits a `structural.eq`/`structural.ne`
application with an inline `value_kind: "none"` literal of an Option collection type through the
real reader (`read_and_account`). `composite_application_operands` then returns
`UnsupportedOperand { reason: InlineNonInteger }`. The coder tried text, string, boolean, decimal
and integer literals, not `none`. In CG, `OriginalReplayContext::new` (composite_builder.rs:652-708)
checks only source identity, package name, lock sources, dependency selections and function
presence. It does not compare `original` with the raw package bytes. `selected_claim` projects
operands (:273-277) before any recompilation. So a reader-admitted package carrying such an operand
can drive the builder to `CompositeBuildError::Operands(InlineNonInteger)`. A QSL source
`a = none` over an `Option<...>` parameter may also compile to it; that is unverified.

## Verdict

**Gap open: NOT complete for IR-814.** AC-14 and AC-15 are closed. AC-17 is a Test criterion with no
tagged test, and its unreachability premise is contradicted by IR's own admitted fixture. The lead
ruling's preferred branch (a direct test at builder level) appears available, so AC-17 should get
a tagged test, not a downgrade. Only if both the `none`-literal source and the re-read
tampered-wire route are shown to fail should AC-17 move to Inspection (spec edit in SR-787003).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-033-AC-17 (Test) left PLANNED and untagged on an unreachability claim that IR's production-reader `none`-literal composite fixture contradicts; CG's context constructor plus projection-before-recompile make the InlineNonInteger -> Operands branch testable | spec/replay/functional/FR-033-composite-parity-replay-binding.md:375, src/replay/composite_builder.rs:273-277, src/replay/composite_builder.rs:652-708 |

## Coverage

| Criterion | Matrix status | Binder |
| --- | --- | --- |
| FR-033-AC-14 | tagged | tests/it/composite_original_eq.rs:728 |
| FR-033-AC-15 | tagged | tests/it/composite_original_eq.rs:783 |
| FR-033-AC-16 | method-without-symbol | Inspection |
| FR-033-AC-17 | untagged | none |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-033-AC-14 | examined | structural.ne over two composite literals, NotEqual, two positional graph_child arguments with empty Bounds. |
| FR-033-AC-15 | examined | structural.ne self-comparison, two same-identity graph_child arguments each with drawn Node-keyed bounds, neither deduplicated. |
| FR-033-AC-16 | examined | If the owning operand projection refuses an inline integer literal operand (InlineInteger), then the builder returns that refusal through CompositeBuildError::Operands ... verified by Inspection. |
| FR-033-AC-17 | examined | If the owning operand projection refuses an inline non-integer literal operand (InlineNonInteger), then the builder returns that refusal through CompositeBuildError::Operands before any QSL invocation, with no report or terminal value and no new CG error variant. No test of this path exists yet. |

## Dispositions

Round 1, head d094b87a. `quire matrix`: FR-033-AC-17 tagged (composite_original_eq.rs:982); AC-14
:729, AC-15 :790 tagged; AC-16 method-without-symbol. `make spec` exit 0. Test passes at the new
head; mutant M2 (all projection reasons rewritten to `InlineNonInteger`) is killed by its graph-`none`
control. Fixture provenance: `opt_none_with_inline_operand` hand-edits a genuine QSL compile of
`a = none` (QSL itself emits a graph `none`, refused as `NonliteralGraphValue`). The edited wire is
admitted by IR's production reader (`read_checked`), which is the precondition AC-17 states. It
is the same shape IR's own reader tests admit. It is not QSL-producible today; the AC-17 and TC-048
text says so, so this is accepted as defensive-branch evidence, not a forgery of a positive.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Tagged AC-17 test over a reader-admitted inline `none` Option operand, asserting `Operands(UnsupportedOperand { ordinal: 1, reason: InlineNonInteger })`, plus a cause-distinguishing graph-`none` control |

---
id: SR-787001
title: "IR-814 PR #787 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #787 at 7ba3df5b; IR-814; tests/it/composite_original_eq.rs; src/replay/composite_builder.rs (context, unchanged)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
---

## Summary

Ticket: IR-814. The PR adds two integration tests to `tests/it/composite_original_eq.rs`, two
fixture functions (`ne_lits`, `ne_same`) to its shared `SOURCE`, and a `first_child` helper. No
production code changes (`git diff origin/main HEAD -- src Cargo.toml Cargo.lock` is empty). The
rust-review lane is folded into this file.

Executed: one focused job (`cargo test --lib --test it` filtered to the two new tests, the Eq
self-comparison test and the two lib tests, with `QUIRE_CVC5_BIN` set): all 7 passed. One
mutation control: in `selected_claim`, after the length check, `arguments.dedup_by(|a, b| a.identity == b.identity)`.
`original_ne_self_comparison_keeps_two_positional_arguments` FAILED at its `arguments.len() == 2`
assert (left 1, right 2), as did the AC-11 Eq test; the mutant was reverted. The AC-14 test asserts
NotEqual on the sent report, Literal domain, the GraphChild identity equal to each literal's own
node key, empty Bounds, distinct identities and the Tested terminal value; it would catch a
positional-collapse or non-empty-literal-bounds mutant. Trace tags use `#[trace("FR-033-AC-1x", "TC-048")]`;
no `/// Trace:` lines. No unwrap/panic or unsafe surface added outside test code.

## Verdict

**Tests are real, not tautological; two test-strength gaps.** The AC-15 test kills the
deduplication mutant directly. The Equal-request refusal is weakly asserted (AC-14) or absent
(AC-15) relative to the TC-048 procedure, and the AC-15 bounds assertion does not by itself pin
Node-keyed filtering. Neither blocks on its own; the PR's merge blocker is FR-033-AC-17 (SR-787002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-048 requires the AC-1 `OperatorMismatch` refusal for both Ne forms; AC-14's test asserts only `wrong.is_err()` (any error passes, e.g. OperandShape or Identity) and AC-15's test has no Equal-request check | tests/it/composite_original_eq.rs:769-776, tests/it/composite_original_eq.rs:783-825 |
| FND-002 | low | AC-15 test supplies one Node bound and checks only `entries().len() == 1`; it neither adds a Population or foreign-node bound nor asserts the key names the argument, so an unfiltered-bounds mutant survives it (caught only by the AC-18 Eq test) | tests/it/composite_original_eq.rs:789-812 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| original_ne_literal_literal_keeps_two_empty_bounds_children | examined | Ne over two composite literals keeps NotEqual and two empty-bounds graph children. |
| original_ne_self_comparison_keeps_two_positional_arguments | examined | Ne self-comparison keeps two undeduplicated arguments with the same node and drawn bounds. |
| FR-033-AC-14 | examined | When the original application node is a structural.ne whose two operands are both composite literals and the requested operator is NotEqual, the verified request carries NotEqual and a retained checked claim body with two positional graph_child arguments, each the literal's own node with empty Bounds and no parameter position. |
| FR-033-AC-15 | examined | When the original application node is a structural.ne comparing one parameter with itself and the requested operator is NotEqual, the verified request carries NotEqual and a retained checked claim body with two positional graph_child arguments with the same node identity, each retaining the drawn Node-keyed bounds naming that parameter and neither deduplicated. |
| TC-048 | context_only | For the Ne forms (FR-033-AC-14, FR-033-AC-15), require NotEqual and two positional arguments, then apply the AC-20 report binding and the AC-1 OperatorMismatch refusal when Equal is requested. |
| src/replay/composite_builder.rs selected_claim | context_only | Projection, operator check, Literal empty bounds / Parameter Node-filtered bounds, two-argument check. |

---
id: SR-11666
title: "IR-803 PR #778 typed composite operator: code review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #778 @ a70f29af vs origin/main; IR-803; src/replay/composite_builder.rs, tests/it/composite_original_eq.rs (diff only); context src/replay/composite_artifact.rs, src/routed/contract_package_tests.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
---

## Summary

Ticket: IR-803. Code review, with the rust-review lane, of PR #778, head a70f29af574a4a214e15dcf529f963383e3612d5. It is one commit on origin/main: 2 files, +37/-45, no dependency, Cargo.toml or Cargo.lock change. `selected_claim` now takes the operator from the public `CheckedCompositeOperands.operator` field (quire-contract-model at c8dca8c6, from IR PR #410). It maps `Eq` to `Equal` and `Ne` to `NotEqual` and compares the result with the requested operator to decide `OperatorMismatch`. The PR deletes `node_equality_operator`, the `application_identity` import and the unit-test module. It adds the integration test `a_non_equality_operation_refuses_as_an_ineligible_operator`.

Reviewer runs, in worktree /home/peter/dev/worktrees/IR-803-review, with no Kani and no `make ci`, in one focused cargo job (`cargo test --test it composite_original_eq`, CARGO_TARGET_DIR in the worktree, target/ deleted afterwards):

- **Baseline:** 25 passed, 0 failed, no warnings.
- **Mutant (swap Eq and Ne in the new match):** 20 of 25 failed, including `original_operator_mismatch_refuses_with_typed_operators`, `original_ne_parameter_operands_keep_order_and_operator`, `original_ne_literal_operand_keeps_empty_bounds` and `repeated_requests_keep_each_nodes_original_operation`. The control can fail. The mutant was reverted and the tree is clean.

## Verdict

APPROVE, with one low finding. What is right:

- **No identity re-read.** composite_builder.rs has no `application_identity`, `.term(`, `quire.op.*` literal, `serde_json` or `/operation` pointer read left. The only "raw" text left is `raw_package`, the retained replay emission used for the byte-equality check, which is not an operation lookup. This meets the ticket's "no raw JSON re-read left in CG composite_builder.rs".
- **Mapping.** `CheckedCompositeOperator` (quire-contract-model composite_operands.rs:36) is a closed enum with two variants, `Eq` and `Ne`, and is not `#[non_exhaustive]`. The match has no wildcard arm, so if IR adds a variant, CG fails to compile instead of mapping it silently. IR's `from_identity` maps `structural.eq` to `Eq` and `structural.ne` to `Ne`, so the CG mapping agrees with it.
- **Refusal order.** On main, the operand projection already ran before `node_equality_operator`. IR refuses every catalogued non-eq/ne application as `IneligibleOperator`, and an absent or uncatalogued identity as `UnknownOperator`. So the deleted helper's refusal arm was unreachable from `selected_claim`, and the PR changes no observable result. The new test pins this for `quire.op.integer.lt` with both requested operators.
- **Deleted unit test.** Its two assertions were:
  - quantity.convert gives `UnsupportedNodeOperation`. The helper no longer exists. Through the builder, a non-eq/ne application is covered by the new test, though not a conversion specifically (see gap-analysis SR-11667 FND-002).
  - structural.eq gives `Equal`. This is covered by `original_operator_mismatch_refuses_with_typed_operators` and the Eq constructor tests, and the mutant run confirms it.
- **`UnsupportedNodeOperation` is still used.** Its producers are composite_artifact.rs:77 and :79 (no generated origin entry, or more than one, matches the node, operator and operand types), so it is not dead code.
- **Trace tags.** The new test uses `#[trace("FR-014-AC-2", "FR-033-AC-1", "TC-024", "TC-048")]`. No `/// Trace:` lines were added. FR-033-AC-1/TC-048 fits. The FR-014-AC-2 binding conflicts with that AC's current text (gap-analysis SR-11667 FND-001).
- **The skipped tidy at src/routed/contract_package_tests.rs:1869.** This is a test assertion in the routed module, outside composite_builder.rs. It reads `/operation/identity` from the serialized lowered closure as an independent oracle for the lowered body. It is not a production re-read and not in the file the acceptance names, so skipping it does not breach the acceptance. Not a finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `CompositeBuildError::UnsupportedNodeOperation` is documented as "The selected node has no supported structural equality operation". After this PR its only producers are composite_artifact.rs:77/79, which raise it when no generated origin entry matches (or more than one does). Nothing in src/ or tests/ asserts the variant any more: the deleted unit test was the only test that referenced it. Fix: reword the doc to the artifact-binding meaning (no unique generated origin entry for the node, operator and operand types), and optionally add a test for that path. | src/replay/composite_builder.rs:86-87 |

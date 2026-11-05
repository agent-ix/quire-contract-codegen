---
id: SR-2026110283873
title: "CG PR 748 inline Integer operand code review"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen; IR-1346; src/oracle/equality/mod.rs; tests/it/inline_integer_equality.rs; tests/it/main.rs; tests/inline_integer_equality_support/agreement_cases.rs"
---

# SR-2026110283873: CG PR 748 code review

## Summary

Ticket IR-1346; frozen PR #748 head 93649a9ba4d62124497adf6f3bfee4bdf63e5626. Examined FR-018-AC-47 and TC-029's inline Integer control against the production operand type read, authentic QSL source admission, generated native value comparison, and mistyped descriptor refusal. FR-018-AC-15's reference and conversion rule was examined for preservation. All four changed files were reviewed. CI workflows did not change.

## Verdict

**CONDITIONAL** — the production branch reads the admitted literal type id and preserves the reference/conversion path. One low severity doc comment still describes every operand as a reference, contradicting the newly supported inline literal case. Focused test, Clippy and format were reported passing by the coder; the code lead owns the final scoped gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Operand type reader comment still says every binary operand has its own referenced node, despite the new direct literal branch | src/oracle/equality/mod.rs:1487 |

## Coverage

- FR-018-AC-47 (examined): admitted inline Integer literal `type_id` drives generation; generated oracle and direct runtime values agree; a wrong descriptor is refused.
- TC-029 inline Integer literal controls (examined): authentic source, unchanged IR admission, generated native test and mismatch control.
- FR-018-AC-15 (examined): reference operands and nested convert source-type traversal remain unchanged in the diff.
- `tests/it/main.rs` (examined): the new test is registered in the existing integration binary.
- `tests/inline_integer_equality_support/agreement_cases.rs` (examined): generated crate executes the oracle against direct `TypeEnvironment::check_equality` on three value pairs.
- `src/oracle/equality/mod.rs` (examined): the new literal branch returns the IR retained `CheckedTerm::Literal.type_id`, never deriving a type from token or descriptor.
- No changes to Cargo manifests, lockfile, CI workflows, vendored artifacts, or public API.

## Analysis

At lines 1487-1488 the comment says each operand is its own node and the equality arguments hold a reference to it. A direct `CheckedTerm::Literal` is an argument term with its own admitted `type_id`; it is not a reference to an operand node. The comment describes the former precondition as universal even though the adjacent new branch specifically accepts the contrary form. Update the comment to name both reference and direct literal cases. This is documentation drift, with no observed wrong runtime result.

The native test is not a mock of the generator: it compiles authored QSL source, reads the emitted package through Contract IR, lowers it, generates the oracle, compiles the generated artifact and executes its value path. The generated oracle's Boolean result is compared both with direct runtime equality and an independent expected truth table. The distinct Boolean `semantic_type` supplies the wrong descriptor; the refusal asserts the original Integer `type_id` as `found`. The existing generated-crate test harness checks that its one nested test runs and passes.

No aggregate gate was run in this review, as the lane requires scoped CG gates only and the code lead will run the final scoped gate after findings are fixed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The comment now explicitly identifies an admitted inline scalar literal and its checked type node id alongside referenced operands. |

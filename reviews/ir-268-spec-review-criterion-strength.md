---
id: SR-11793
title: "IR-268 criterion-strength review of AD-004, FR-014 and TC-024 amendments"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-contract-codegen@85c0e9747ad5ca0dad5411087c93895a9ee99050 (PR 396, diff vs origin/main); spec/assurance/AD-004-cg-crate-layout.md; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

## Summary

Ticket: IR-268. Criterion strength of the amended FR-014-AC-2 and FR-014-AC-18 and the TC-024 step that verifies them. Done as a manual semantic judgment, because Jev was not invoked in this pass. AC-18's refusal clause can fail: the step-6 derivation check detects a wrong reason or a built descriptor. AC-2's 'no function or harness' clause has no TC-024 step that could fail it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-2's verification is Test (TC-024), and it claims that no descriptor, function or harness is produced for a quire.op.quantity.convert node. TC-024 step 6 checks only that derivation refuses with the named capability and 'no descriptor'. No TC-024 step generates over a convert node and checks that no function is produced, and the Expected Results never mention the convert refusal. Harness absence is evidenced only by a TC-033 routed test (tc_033_a_quantity_conversion_is_unsupported_with_its_named_capability_and_no_harness), which AC-2's Verification column does not cite. Failure: a regression that emits a convert oracle function on the caller-declared generate path passes every TC-024 step. Fix: add a TC-024 generate-side check and an Expected Results line, and cite TC-033 for the harness clause or drop that clause from FR-014. | FR-014:423; TC-024:41-51; TC-024:131-143; tests/it/routed_generation.rs:2140-2180 |

## Scope examined

- `FR-014-AC-2` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, examined)
- `FR-014-AC-18` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, examined)
- `FR-014 derivable set (Deriving the descriptor)` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, examined)
- `FR-014 Outputs ClaimDerivationRefusal codes` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, examined)
- `TC-024 step 6` (spec/oracle/matrix/TC-024-exact-scalar-oracles.md, examined)
- `AD-004 target tree generate/` (spec/assurance/AD-004-cg-crate-layout.md, examined)
- `AD-004 L-2 generate/ order` (spec/assurance/AD-004-cg-crate-layout.md, examined)
- `AD-004 family lowerers` (spec/assurance/AD-004-cg-crate-layout.md, examined)
- `AD-004 L-3` (spec/assurance/AD-004-cg-crate-layout.md, context_only)
- `AD-004 interim exceptions (render)` (spec/assurance/AD-004-cg-crate-layout.md, context_only)
- `spec/oracle/matrix/tests.md FR-014-AC-2 row` (spec/oracle/matrix/tests.md, context_only)

## Verdict

Approved intent checked and met: the FR-014 refusal names QuantityConversionUnsupported and QUANTITY_CONVERSION_CAPABILITY and cites IR-818. No quantity.convert descriptor, QuantityTarget or conversion support claim remains anywhere under spec/ (grep). FR-033:178 'Quantity conversion preserves its exact scalar and unit' concerns replay value canonicalization, not the oracle, and is not a contradiction. In AD-004, collection is ranked with scalar before negotiate, its local imports are outcome and record only, and negotiate's list gains collection, all matching tests/it/layout.rs. The new text narrates no history. The findings above are the remaining gaps.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | AC-2 says 'The QSpec TC-187 rows that convert a quantity are refusal rows under this criterion' and cites tc_024_tc187_conversion_rows_are_refused_with_the_named_capability. That test loops eleven row labels (U01 ... U18) over the same three corpus conversion nodes, and uses the label only in the assertion message (tests/it/exact_scalar_generation.rs:3514-3545). No node is built from any TC-187 row's operands or target. Failure: if a listed row did not convert a quantity, or a converting row were missing from the list, the test would still pass. The clause is satisfied by labels, not by row content. Fix: build each row's conversion node from its TC-187 operands, or reword the clause to say the rows are excluded from AC-6 because the identity is refused. | FR-014:433; FR-014-AC-6; TC-024:54-55; tests/it/exact_scalar_generation.rs:3514-3545 |

## Dispositions

Round 1, checked against fix commit 7e16de1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified in 7e16de1 |

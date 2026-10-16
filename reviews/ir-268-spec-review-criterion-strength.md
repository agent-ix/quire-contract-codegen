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
| FND-002 | fixed | verified in 45f72ee (round 2) |
| FND-003 | fixed | verified in 1cce284 (round 3) |
| FND-004 | fixed | verified in 9c5f291 (round 4) |
| FND-005 | fixed | verified in 9c5f291 (round 4) |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AC-6 now names exactly the 14 TC-187 conversion vectors it excludes. I checked that list against quire-specification TC-187 and it is right; U16 evaluates U07. So every other TC-187 vector is now, by name, inside the agreement claim, and tests.md:13 marks AC-6 Covered. tc_024_tc187_quantity_oracles_agree (tests/exact_scalar_support/agreement_cases.rs:483-602) executes the generated grid over m, cm, in, rev and u2, plus U03, U05, U13, U19, U21, U23, U24 and U28. No agreement row executes U02, U09, U12 (2 m * 3 s; the grid has no s), U14, U20 or U27. U06 and U11 are admission rows with no oracle at all. Failure: AC-6 and its Covered status claim agreement for vectors that no test runs. U09 is newly in scope after this round, so the gap is now explicit rather than implied. Fix: list the executed TC-187 vectors, or mark the unexecuted and admission-only rows outside AC-6 or Partial in tests.md. The gap predates this PR for the unconverted rows. | FR-014:440; spec/oracle/matrix/tests.md:13; tests/exact_scalar_support/agreement_cases.rs:483-602; quire-specification spec/test-cases/TC-187-quantity-and-unit-conversion.md:39-68 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | AC-6 now names the TC-187 vectors it covers (U03, U13, U19, U21, U23, U24, U28 and the generated grid). It says they yield results 'equal to direct runtime execution and to the QSL value authority'. Every TC-187 case in tc_024_tc187_quantity_oracles_agree goes through agree2! (tests/exact_scalar_support/agreement_cases.rs:487-605). agree2! compares the direct runtime with the generated oracle only (tests/exact_scalar_support/agreement.rs:42-44) and never calls quire_spec_language::value. The tests.md AC-6 row says only 'agree with direct runtime execution', so the matrix row and the AC disagree. Failure: a QSL value-authority divergence on a listed TC-187 vector passes every test while AC-6 claims three-way agreement. Fix: say the TC-187 vectors are compared runtime-to-oracle only, or add the QSL leg. This predates the PR, but the vector list makes it explicit now. | FR-014:447; spec/oracle/matrix/tests.md:14; tests/exact_scalar_support/agreement_cases.rs:487-605; tests/exact_scalar_support/agreement.rs:42-44 |
| FND-005 | low | AC-2 now says four further readers each read a closed set of operation identities and refuse a conversion before rendering: StateComparison::from_operation (src/kani/generate/frame.rs:115), resolve_predicate (src/kani/generate/collection.rs:532), the replay composite builder's UnsupportedNodeOperation (src/replay/composite_builder.rs:272) and ScalarOperation::of (src/kani/generate/scalar.rs:52). AC-2's Verification is Test, and it cites no test for any of the four. No test in tests/ feeds a quantity.convert identity to them. Failure: adding a catch-all arm, or a convert arm, to one of these readers passes every cited test, so this clause of AC-2 cannot fail. Fix: cite a test per reader, or mark the clause Inspection. | FR-014:443; src/kani/generate/frame.rs:115; src/kani/generate/collection.rs:532; src/replay/composite_builder.rs:272; src/kani/generate/scalar.rs:52 |

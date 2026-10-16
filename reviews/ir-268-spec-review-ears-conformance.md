---
id: SR-11791
title: "IR-268 ears-conformance review of AD-004, FR-014 and TC-024 amendments"
type: SpecReview
analysis: ears-conformance
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

Ticket: IR-268. EARS check of the new and amended FR-014 statements and ACs. The amended ACs read as testable declaratives. The new refusal rule is stated as descriptive prose under 'Deriving the descriptor from the node', not as a shall-statement. The other derivation refusals are written the same way, which is an existing pattern.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The refusal is stated as 'is refused as QuantityConversionUnsupported', with no actor and no trigger. EARS form would be: 'If a requested node's operation.identity is quire.op.quantity.convert, then derive_exact_scalar_items shall refuse it as NoDerivableClaim carrying QuantityConversionUnsupported, naming the identity and QUANTITY_CONVERSION_CAPABILITY.' Failure: the statement does not say which entry point owns the refusal (derivation, or caller-declared generation too). That is the gap FND-001 of the failure-domain review turns on. | FR-014:213-218 |

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

## Dispositions

Round 1, checked against fix commit 7e16de1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified in 7e16de1 |

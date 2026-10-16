---
id: SR-11790
title: "IR-268 base review of AD-004, FR-014 and TC-024 amendments"
type: SpecReview
analysis: base
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

Ticket: IR-268. Base checklist over the IR-268 spec diff (AD-004, FR-014, TC-024) at 85c0e97. IDs, links and the hash/pin antipattern are clean; no new hash, digest or pin use is introduced. The FR-014 refusal and the AD-004 collection rank match the approved intent. One low finding: FR-014-AC-2 now joins a generation criterion and a refusal criterion that AC-18 already states, and the matrix row for AC-2 was not updated.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-014-AC-2 now carries two criteria: the positive generation list, and the quantity.convert refusal that FR-014-AC-18 also states. The matrix row spec/oracle/matrix/tests.md:14 still says only 'Covered for catalogued checked-package expression families' and says nothing about the refusal clause. Failure: a test that covers only the generation half, or only the refusal half, reads as covering the whole AC, and the matrix row cannot record which half is evidenced. Fix: keep AC-2 positive, leave the refusal to AC-18 (or give it its own AC), and update the tests.md row. | FR-014-AC-2 (FR-014:423); FR-014-AC-18 (FR-014:439); spec/oracle/matrix/tests.md:14 |

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

---
id: SR-11792
title: "IR-268 integrity review of AD-004, FR-014 and TC-024 amendments"
type: SpecReview
analysis: integrity
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

Ticket: IR-268. Integrity of the AD-004 collection-family amendment and the FR-014/TC-024 refusal amendment. The L-2 rank and import lists match tests/it/layout.rs: generate_rank puts scalar and collection at 3, collection's exact list is outcome and record, and negotiate's list includes collection. A grep of spec/ finds no remaining claim of quantity.convert support, no QuantityTarget, and no derived convert descriptor. Two medium gaps remain in AD-004, where two passages still describe the families without collection. One low gap is in TC-024.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The One Kani generator section still says 'Four family lowerers (scalar, precondition, contract, frame) each take lowered IR claims and return HarnessSpecs'. collection is not named, although the approved intent says the family-lowerer list names it. The tree row at AD-004:215 likewise keeps collection out of the 'family lowerers; each returns a HarnessSpec' line. Failure: AD-004 contradicts itself. L-2 ranks collection as a family, but the generator contract excludes it, so nothing in the AD requires collection to return a HarnessSpec, and a later step can leave it as a private renderer. | AD-004:413-416; AD-004:215-221 |
| FND-002 | medium | collection.rs renders its own Kani template: it emits #[kani::proof], kani::any, kani::assume and kani::cover! (src/kani/generate/collection.rs:279-293). The interim-exceptions table lists the own-template families as scalar.rs, precondition.rs, contract.rs, frame.rs and the corpus, removed at 4b, 4c, 4d and 4g. The L-3 interim list (AD-004:633-639) and migration steps 4b-4g name no step that ports collection to HarnessSpec and render.rs. Failure: when L-3's literal scan lands after 4f and 4g, collection.rs fails it with no scheduled removal, or the scan gains an exception the AD never recorded. | AD-004:1112; AD-004:633-639; AD-004:775-818; src/kani/generate/collection.rs:279-293 |
| FND-003 | low | Step 6 says to derive every quantity.convert node 'at each rounding mode', but neither TC-024 nor FR-014 defines that set. The fixture's three conversion nodes all carry mode exact (tests/exact_scalar_support/package.rs:1447-1449, 2693) and differ only in their operand bound. Failure: the step cannot be executed as written. A reviewer cannot tell whether a single exact-mode node discharges it, and the variation the fixture actually covers (shared, unbounded and bounded operand) is not what the step asks for. Fix: say 'every quantity.convert node, whatever its operand bounds and mode'. | TC-024:46-48; tests/exact_scalar_support/package.rs:1447-1449 |

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

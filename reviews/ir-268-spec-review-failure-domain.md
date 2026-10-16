---
id: SR-11795
title: "IR-268 failure-domain review of AD-004, FR-014 and TC-024 amendments"
type: SpecReview
analysis: failure-domain
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

Ticket: IR-268. Failure domains of the quantity-conversion refusal across FR-014's two entry points, derive_exact_scalar_items and generate_exact_scalar_oracles with a caller-declared descriptor, and the routed Kani path. The refusal is unconditional on derivation and precedes mode and operand checks. The spec does not say what the caller-declared generate path returns for a convert node.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-2 says, with no condition, that a quire.op.quantity.convert node 'is unsupported with the named capability QUANTITY_CONVERSION_CAPABILITY'. The capability is raised only on the derivation path (src/oracle/scalar/mod.rs:4196). generate_exact_scalar_oracles never checks the identity. A caller-declared descriptor over a convert node is refused there by the AC-3 arity or type mismatch, or, for a descriptor that matched shape, would be generated as caller_declared under the 'comparisons disagree' Behavior rule. Failure: AC-2 as written fails on the generate path, and two implementers disagree on whether generation must also refuse the identity. Fix: scope the named-capability clause to derivation and routed generation, or add a generate-side Behavior rule that refuses the identity whatever the descriptor. | FR-014:423; FR-014:253-280 (Behavior, caller_declared rule); FR-014-AC-3; src/oracle/scalar/mod.rs:4196 |

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
| FND-002 | fixed | verified in 1cce284 (round 3) |
| FND-003 | fixed | verified in 9c5f291 (round 4) |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The S1 exemption names the wrong earlier refusal for a reachable input. AC-2 and Behavior (FR-014:278-281) say the function-application generator 'accepts only a binary expression, so it refuses a conversion node earlier, as FormMismatch naming the form conversion'. That is not what happens: (1) classify_body_shape (src/oracle/function/mod.rs:2049-2059) checks the operator before the form, so a Scalar body with Negate, Divide or any operator other than Add, Subtract or Multiply is refused as UnsupportedOperator, which FR-021-AC-21 itself requires for Negate. (2) The Call body accepts form call, so 'accepts only a binary expression' is false for the function generator. Both tests use only Scalar{Add} and CompositeEquality{Equal}. Failure: a test written to AC-2 for a Scalar{Negate} declaration over a conversion node expects FormMismatch and gets UnsupportedOperator. FR-014 and FR-021-AC-21 then state different refusals for the same input, against S1's 'exempt only if a provably earlier typed refusal applies, stated with that refusal cited'. Fix: state that the exempt generators refuse a conversion node with a typed refusal before any artifact, namely UnsupportedOperator for a Scalar operator outside Add, Subtract and Multiply and otherwise FormMismatch naming conversion, and add an UnsupportedOperator case to the test. 'Never receive the operation' also does not cover FR-018-AC-15, where a convert node is read through as an equality operand. There the cited refusal is Unsupported quantity (tests.md:59), not FormMismatch, so cite that path or narrow the claim to a requested root. | FR-014:278-281; FR-014:436; FR-021-AC-21 (FR-021:266); FR-018-AC-15; src/oracle/function/mod.rs:2049-2059, 2952-3007; tests/it/exact_scalar_generation.rs:3560-3620 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | FR-014 Behavior (FR-014:278-291) now says a conversion reached from a declared function body is refused with the capability 'whatever the declared body kind'. The code does this check before classify_body_shape (src/oracle/function/mod.rs: the closure scan precedes the call at :2341). FR-021-AC-21 still reads, with no condition, 'A scalar Negate body is refused as UnsupportedOperator', and FR-021 does not mention that FR-014's quantity-conversion refusal takes precedence. Failure: for a Negate-body declaration over a conversion node, FR-021-AC-21 read alone predicts UnsupportedOperator, while FR-014 and the code give QuantityConversionUnsupported. The conflict FND-002 found is reduced to a missing precedence note. Fix: qualify FR-021-AC-21 with 'over a node that reaches no quantity conversion', or have it cite FR-014-AC-2's precedence. | FR-021:266; FR-014:278-291; src/oracle/function/mod.rs:2341 |

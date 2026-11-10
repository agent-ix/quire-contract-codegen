---
id: SR-17940486709564847047606
title: "IR-1497 PR #773 FR-050 EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #773 @ b58d99bc; IR-1497; spec/kani/functional/FR-050-quantity-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/matrix/TC-064-quantity-scalar-harnesses.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/kani/matrix/tests.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-064
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
---

## Summary

Ticket: IR-1497 (related IR-1439). Spec-only PR #773 (head b58d99bc, one commit on 67739cf5): FR-050 draws exact quantity magnitudes from the carried_quantity leaf of a convert<Quantity<u>> operand and composes the conversion into the operation proof; new FR-050-AC-11..14, FR-025-AC-18 and carried-leaf ABI row, TC-064 steps 10-12, TC-036 step 18, tests.md, spec.md row 96, TC-030 row. The decision itself (language unchanged, bounded_quantity is a dense interval, carried_quantity gives the finite domain) is not re-litigated; this review checks that the spec implements it correctly. Checked against: quire-semantic-value origin/main src/quantity.rs (convert_quantity, convert, QuantityTarget::Exact, Conversion::identified_quantity, evaluate_quantity_unit, compare_quantity); CG src/oracle/scalar/render.rs and lower.rs (QuantityConversion body calls quantity_owner = quire_semantic_value::quantity; check_operand); CG src/kani/generate/negotiate.rs and outcome.rs (RequiresBound, MissingBound, NoFiniteEncoding); Contract IR origin/main 7add9a5 operations.rs (check_bounded_dimensions, check_quantity_conversion, check_operands carried-operand refusal, SameDimension not enforced); Contract IR FR-038-AC-73/248/471/485/486; QSpec FR-142-AC-27..31 and FR-154 'Unit bounds'; CG FR-014-AC-21..25, 48..52, 59..68; FR-046-AC-2, FR-051-AC-2/AC-5; FR-028; Linear IR-1443 (comment of 2026-11-10).

## Verdict

PASS. The edited FR statement keeps the event-driven 'When ... CG SHALL generate' form with the new operand precondition in the trigger; the new Behavior statements ('When evaluating the composed operation, the harness SHALL ...'; 'CG SHALL draw only that leaf parameter ... and SHALL NOT draw the converted magnitude') are well-formed event-driven/ubiquitous clauses with one actor each. No EARS defects introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050-P2 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:44 | Two prerequisites are named and neither is delivered. (P1) K-EXACT-quantity (ticket IR-1443) is the quire-exact carrier giving a symbolic carried-quantity leaf built from its carrier's physical bindings and a native expression over it. (P2, ticket IR-1497) The finite magnitude domain of an operand comes from a carried leaf, not from the operand itself, because no other bounded form is finite: Contract IR FR-038-AC-73, FR-038-AC-471 and FR-014-AC-60 make every bare exact `unit` or `compound_unit` parameter `requires_bound` before CG; the `bounded_quantity` form (Contract IR FR-038-AC-485 and AC |
| FR-050-behavior-compose | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:103 | A quantity operand is composable when it is a reader-admitted application of `quire.op.quantity.convert` whose mode is `exact`, whose result type is an exact `unit` or `compound_unit`, and whose source is a `value`/`parameter` typed at a `scalar_type`/`carried_quantity`. When the operation is representable, CG SHALL draw only that leaf parameter symbolically, from its carrier's inclusive domain through the carrier, and SHALL NOT draw the converted magnitude: the converted magnitude is computed by the owner's conversion over the drawn leaf. When evaluating the composed operation, the harness SH |
| FR-050-inputs | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:81 | - For each operand, the carried-quantity leaf parameter (P2) of the |

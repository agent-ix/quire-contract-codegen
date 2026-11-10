---
id: SR-17940486709564847047605
title: "IR-1497 PR #773 FR-050 integrity analysis"
type: SpecReview
analysis: integrity
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

PASS WITH LOW FINDINGS. Structure is intact: FR-050-AC-11..14 and FR-025-AC-18 are in their AC tables with unique sequential ids; TC-036 step and expected result 18 are paired; tests.md lists FR-025-AC-18 on the TC-036 row and FR-050-AC-1 through AC-14 on the FR-050 row; no stale 'ten' counts remain for FR-050; quire validate passes on every changed file. Two low cross-document items.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-051 still says 'FR-050 excludes conversions, so no row here or there proves them'. FR-050 now composes an exact convert as an operand, though it still does not prove one on its own. The FR-025 Rational row ('planned FR-046 predicate slice only') and Decimal row ('planned FR-051 scalar operations only') keep their 'only' scope, while the new carried row and FR-050-AC-13 reuse both encodings. None of this is wrong in effect, but readers of FR-051 and FR-025 get a stale scope. Update FR-051's sentence to 'FR-050 composes an exact convert as an operand and proves no conversion on its own', and widen or cross-reference the two rows' scope notes. | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:61 |
| FND-002 | low | 'including the harness's memory ceiling (the project's 16 GiB Kani budget)': FR-028 makes the memory ceiling a per-request value recorded in each harness. 16 GiB is the measured profile used by FR-015-AC-126 and TC-027, not a fixed FR-028 value. Write 'the FR-028 memory ceiling of the request (16 GiB in the current proof profile)' so the requirement does not hard-code a number that FR-028 leaves to the request. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:63 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050-status | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:190 | PLANNED (IR-1439). This specification supplies no implementation or generated-proof credit. All fourteen criteria remain planned and untagged until P1 and P2 are delivered with the IR-269 quantity harness. Oracle agreement under FR-014 is not Kani credit. |
| TC-064-status | examined | spec/kani/matrix/TC-064-quantity-scalar-harnesses.md:92 | PLANNED: all fourteen FR-050 criteria are unimplemented and untagged. No missing |
| tests.md-FR-050-row | examined | spec/kani/matrix/tests.md:17 | / FR-050 / FR-050-AC-1 through FR-050-AC-14 / TC-064 / 🚧 PLANNED (IR-1439, IR-1497): no quantity scalar Kani harness; needs K-EXACT-quantity (IR-1443) building a carried leaf from its carrier bindings; magnitudes are drawn from the carried leaf of a `convert` operand, and whether the composed proof fits the memory ceiling is measured under IR-1443. / |
| tests.md-TC-036-row | examined | spec/kani/matrix/tests.md:133 | / TC-036 / Verify the generated harness subject ABI / Integration / P0 / FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8, FR-025-AC-9, FR-025-AC-14, FR-025-AC-15, FR-025-AC-16, FR-025-AC-17, FR-025-AC-18 / 🚧 Planned / |
| spec.md-FR-050-row | examined | spec/spec.md:96 | / Quantity scalar operation harnesses / [FR-050](kani/functional/FR-050-quantity-scalar-harnesses.md) planned bounded quantity operation proof over the carried leaf of a `convert` operand (conditional on IR-1443; decision IR-1497) / [TC-064](kani/matrix/TC-064-quantity-scalar-harnesses.md) planned generated-proof and mutant controls / |
| TC-036-step-18 | examined | spec/kani/matrix/TC-036-generated-subject-abi.md:94 | 18. For the planned [FR-050](../functional/FR-050-quantity-scalar-harnesses.md) quantity row, generate a quantity operation over a `convert` of a carried leaf for each of an `Int[..]`, a `Decimal[..]` and a `Rational[..]` carrier and read its persisted identity: assert the carrier's ordered `i64` bindings (one; coefficient then scale; numerator then denominator), grouped to the leaf parameter with its carrier domain, and no binding for the converted operand. Decode retained playback with the existing I64 checks and caps; assert a valid tuple submits one owner carried quantity per leaf, and a m |
| TC-036-expected-18 | examined | spec/kani/matrix/TC-036-generated-subject-abi.md:155 | 18. One carried-quantity leaf has its carrier's ordered `i64` bindings grouped with its domain and fixed source unit; a valid tuple replays as one owner carried quantity per leaf, each malformed or out-of-domain case refuses with no replay, and an endpoint beyond `i64` is `unsupported` with no harness (FR-025-AC-18). |
| FR-025-AC-18 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:193 | PLANNED (IR-1497, IR-1443). Every supported FR-050 carried-quantity leaf has the ordered `i64` physical bindings of its carrier (one for `Int[..]`, coefficient then scale for `Decimal[..]`, numerator then denominator for `Rational[..]`) grouped to its original parameter, carrier domain and fixed source unit; no binding is created for the converted operand. Playback decodes with the existing I64 width/comment checks and entry and byte caps, validates each component against its carrier domain (and the Decimal and Rational rows' own constraints), then submits one owner carried quantity per leaf p |
| FR-051-conversion-note | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:61 | Decimal equality and inequality (`quire.op.decimal.eq`, `.ne`) are outside this requirement: FR-014 refuses them as `OperationNotDerivable`, so no oracle exists to prove. Quantity carried-Decimal conversions are a quantity operation: FR-014-AC-62 through AC-65 own their oracle, and FR-014-AC-51 and TC-030's Quantity scalar row own their Kani settlement. FR-050 excludes conversions, so no row here or there proves them. |
| FR-050-fallback | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:63 | Whether the composed proof (leaf construction, conversion and operation in one harness) fits the proof ceilings of FR-028, including the harness's memory ceiling (the project's 16 GiB Kani budget), is not known and is measured later under IR-1443. If it does not fit, this requirement does not narrow the domain to make it fit: the finite-grid bound for exact quantity parameters in the QSpec FR-154 "Unit bounds" is then the fallback and is to be specified under its own ticket, which does not exist yet. |
| TC-030-quantity-row | examined | spec/routed/matrix/TC-030-capability-settlement.md:401 | / Quantity scalar `expression` claim, IR-confirmed generated claim, including identity `quire.op.quantity.convert`, read through source-authenticated V1 handoff / Planned (IR-269) / Quantity scalar operation forms; the conversion's settlement is FR-014-AC-51; non-conversion operations are planned under FR-050 (conditional on IR-1443; operands are conversions of carried-quantity leaves, IR-1497), which supersedes this row for them only when delivered. / `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). / |

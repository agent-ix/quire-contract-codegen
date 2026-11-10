---
id: SR-17940486709564847047607
title: "IR-1497 PR #773 FR-050 dependency analysis"
type: SpecReview
analysis: dependency
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

PASS. depends_on FR-046 and FR-051 is correct: AC-13 and FR-025-AC-18 reuse the Rational (canonical numerator/denominator, FR-046-AC-2) and Decimal (coefficient/scale with lifted-bound fit, FR-051-AC-2/AC-5) encodings; FR-051 only references FR-050 and FR-046 has no edge to it, so no cycle. P1 (IR-1443) is stated as needing a carried-quantity constructor from the carrier's physical bindings, matching the 2026-11-10 IR-1443 comment; the IR-1443 ticket ACs themselves do not yet list that constructor (comment only), which is outside this PR. The unfiled A1 fallback is described as not yet ticketed. The native reference depends on QSV (quire-semantic-value) convert_quantity, which FR-014 already depends on. Wording of the fallback is covered by the base review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050-relationships | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:26 | - target: ix://agent-ix/quire-contract-codegen/FR-051 |
| FR-050-deps | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:173 | K-EXACT-quantity (IR-1443) is prerequisite P1: it must build an owner carried quantity from the leaf carrier's physical bindings (AC-13). P2 is IR-1497, which decided this carried-leaf composition; it needs no Contract IR or QSpec change, as the language and FR-038 already admit the conversion of a carried parameter. IR-1439 is the row's specification ticket and IR-263 the MVP row it serves. - The Rational and Decimal leaf encodings are those of [FR-046](./FR-046-rational-function-predicate-harnesses.md) and [FR-051](./FR-051-decimal-scalar-harnesses.md), reused through FR-025-AC-18. - TC-030' |
| FR-050-fallback | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:63 | Whether the composed proof (leaf construction, conversion and operation in one harness) fits the proof ceilings of FR-028, including the harness's memory ceiling (the project's 16 GiB Kani budget), is not known and is measured later under IR-1443. If it does not fit, this requirement does not narrow the domain to make it fit: the finite-grid bound for exact quantity parameters in the QSpec FR-154 "Unit bounds" is then the fallback and is to be specified under its own ticket, which does not exist yet. |
| FR-050-AC-13 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:165 | A leaf's physical bindings are those of its carrier's FR-025 row (FR-025-AC-18): an `Int[..]` carrier has one `i64` inside `[lo, hi]`; a `Decimal[..]` carrier has the coefficient then scale pair of the Decimal row, whose lifted bound `max(\/lo\/,\/hi\/) x 10^(scale_max - scale_min)` must fit `i64`; a `Rational[..]` carrier has the numerator then denominator pair of the Rational row, the numerator inside `[n1, n2]`, the denominator inside `[d1, d2]` and the pair canonical. An endpoint of any component that does not fit `i64` yields `unsupported` retaining the endpoint (AC-5), checked per compon |
| FR-025-AC-18 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:193 | PLANNED (IR-1497, IR-1443). Every supported FR-050 carried-quantity leaf has the ordered `i64` physical bindings of its carrier (one for `Int[..]`, coefficient then scale for `Decimal[..]`, numerator then denominator for `Rational[..]`) grouped to its original parameter, carrier domain and fixed source unit; no binding is created for the converted operand. Playback decodes with the existing I64 width/comment checks and entry and byte caps, validates each component against its carrier domain (and the Decimal and Rational rows' own constraints), then submits one owner carried quantity per leaf p |

---
id: SR-2026110283899
title: "IR-1439 PR #765 FR-050 mutant-control strength analysis"
type: SpecReview
analysis: criterion-strength
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #765 @ 51d9a3f9; IR-1439; spec/kani/functional/FR-050-quantity-scalar-harnesses.md, spec/kani/matrix/TC-064-quantity-scalar-harnesses.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-064
    type: reviews
---

## Summary

Ticket: IR-1439. Checked whether each FR-050-AC-8 mutant control can actually fail as the criterion says. Jev (typesafe.ai System One) is not installed on this host, so calibrated semantic judgment was not available; this is a manual reading of the criterion text against Kani semantics and the FR-046 precedent.

## Verdict

CHANGES REQUIRED. Wrong operator, wrong result unit, dropped or reordered charge and stopped-reported-completed are real falsifying mutants. Two listed mutants can never fail an assertion, and one is equivalent for symmetric comparisons.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-8 requires every mutant to be 'rejected by a failed assertion with actual solver playback rather than a passed proof', including 'a clipped magnitude endpoint' and 'an outcome-based assume'. Both only remove behaviours from the proof, so the proof still passes and there is no failing assertion to play back: the criterion cannot be met for them. FR-046 detects these differently: clipping fails correspondence validation (FR-046-AC-2) and an outcome-based assume is rejected by inspection (FR-046-AC-4). Move both to AC-2/AC-4 (TC-064 step 7 likewise). | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:100 |
| FND-002 | medium | 'Swapped operand order in ... a comparison' is an equivalent mutant for equality and inequality, which are symmetric, so 'each rejected' cannot hold for them; restrict it to ordering comparisons. Likewise the wrong-operator and swapped `sub`/`div` mutants fail only if the control domains contain distinguishing values (a nonzero second operand, unequal operands); AC-8 should require such domains. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:100 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050-AC-2 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:94 | Symbolic magnitudes are drawn from the exact inclusive domain of each operand through K-EXACT-quantity, each declared magnitude has exactly one accepted representation, and a clipped endpoint, duplicate encoding or extra value fails the correspondence check. Units are fixed and resolved through `UnitTable::resolve` or `UnitScope::resolve`; no symbolic unit, CG-created unit id or unrelated metadata pairing is generated. |
| FR-050-AC-4 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:96 | Dimension mismatch (`IllTyped`), identity refusal (`Identity`), arithmetic on an affine unit, division by a zero quantity and an exponent outside its bound are each asserted as the agreed typed outcome with unchanged charges where the owner refuses before charge. None is an assumption, none is relabelled a different cause and a stopped outcome is never proved as completed. |
| FR-050-AC-8 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:100 | Mutant controls are each rejected by a failed assertion with actual solver playback rather than a passed proof: a wrong operator (`add` for `sub`), swapped operand order in `sub`, `div` or a comparison, a result unit differing from the owner's identified unit, an altered `pow` exponent, a dropped charge or changed charge order, a stopped outcome reported completed, a clipped magnitude endpoint and an outcome-based assume. |
| TC-064 | examined | spec/kani/matrix/TC-064-quantity-scalar-harnesses.md:47 | 7. Run the mutants of AC-8 one at a time (wrong operator, swapped operands, wrong result unit, altered exponent, dropped or reordered charge, stopped reported completed, clipped endpoint, outcome-based assume). Each must fail with actual playback and not pass (AC-8). |
| FR-046-AC-2 | context_only | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:152 | Every accepted symbolic argument has canonical constructor/accessor roundtrip equality and lies in its original inclusive component domains. Its two physical I64 bindings retain numerator/denominator roles, original parameter identity and bounds in the FR-025 order. Every semantic value in those domains has exactly one accepted representation. Zero is `0/1`; negative, fractional and boundary values remain reachable. For the original domain there are 111 canonical values per argument and 12,321 argument tuples; admitting all 171 raw component pairs, restricting denominators to one, or clipping  |
| FR-046-AC-4 | context_only | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:154 | Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed false value is not labelled a confirmed semantic false counterexample. |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in f255c11d |
| FND-002 | fixed | fixed in f255c11d |

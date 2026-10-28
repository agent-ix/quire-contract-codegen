---
id: SR-615004
title: IR-1091 PR 615 unique collection capacity refusal — evidence
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; IR-1091; PR 615 head 18ebb99edf790cd9546ad2fa8d3329a2bf758fbe vs main 92ebb4e2943a47cc172a4237c82077c8b7da9859; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: subset
---

## Summary

Ticket: IR-1091. Evidence review of FR-015-AC-96's verification method and
the new TC-025 procedure at head 18ebb99e. `quoin advise --json` (quoin
0.29.1) reports FR-015-AC-96 as authored `Test` with no mismatch.

Its recommendations are:

- `property-based-testing` (property shape `universal`). This one is
  relevant to the capacity boundary.
- `dast`, `fuzzing` and `negative-abuse-testing` (keyword
  characteristics). These are judged not applicable to an offline code
  generator's refusal rule; that is an LLM judgement, not a rule verdict.

The TC-025 paragraph is strong on the main obligations:

- Boolean and IntRange capacity
- max == capacity keeps both endpoints and the distinct-value and order
  assumptions
- max == capacity + 1 refuses with the typed reason, no harness and no proof
  credit, and is not an invalid result
- min > capacity refuses the same way
- a comparable non-unique field stays eligible for its full maximum
- the existing ordered-set control is explicitly disclaimed as evidence

## Verdict

PASS WITH FINDINGS. The planned evidence backs the feasible and refused
branches and protects against vacuity. FND-001 (medium) lets a single control
stand in for three distinct precedence cases. FND-002 (low) leaves the
capacity arithmetic example-checked only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-025's precedence control is a disjunction: "an endpoint that cannot fit the harness ABI or a resource ceiling". One case satisfies it, and it does not require that case to also have max > capacity. As written, a representability-only control would pass while the unwind and memory ceiling orderings stay unchecked. It should require each of ABI representability, unwind and memory, each with a unique max above capacity, and assert the FR-015-AC-101 reason. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:81-83 |
| FND-002 | low | The capacity boundary is checked only by examples (Boolean and one IntRange, at capacity and at capacity + 1). quoin advise recommends property-based testing for this universal-shaped AC. No case covers a single-point IntRange (capacity 1), a negative range, or a full-width `i64` range where capacity overflows 64-bit arithmetic (see SR-615003 FND-003). A property over (lower, upper, min, max) with refusal iff max > upper - lower + 1 would back the formula. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:71-80 |

## Coverage

Examined the FR-015-AC-96 verification cell (`Test`, method only, no TC id)
and the TC-025 lines 71-86. Context: the advise output for FR-015-AC-96, and
the existing fixtures `items` (SmallInt [0,3], max 3) and `flags`
(Boolean, max 2) in `tests/it/kani_obligations_symbolic_domains.rs:69-176`,
which give ready-made feasible and equality cases.

## Dispositions

Round 1, exact fix head 00c799c99c82626ab55109d7e6750baaba5f4d73 (fix delta 18ebb99e..00c799c9; merge-base with main 92ebb4e2). Checked against the fix diff and the current spec/code context; `quire validate --scope . "spec/**/*.md"` exits 0. The PR remains a draft for the owner decision; AC-96 is not complete.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-025 now requires separate representability, unwind and memory controls, each paired with a unique max above capacity, plus unsupported-form and empty-domain cases, each with its exact typed reason and no harness. |
| FND-002 | fixed | TC-025 now requires a property with a widened-integer oracle covering a single value, negative endpoints, below/equal/above capacity and the full-width interval, matching the quoin advise property-based recommendation. |

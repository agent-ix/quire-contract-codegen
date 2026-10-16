---
id: SR-11771
title: IR-364 slice 2 integrity review of the V2 strategy chain criteria
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@f30f17b1a086211cd0ab87f2445eb5565fac74b5 (PR 400, diff vs origin/main 007417d); spec/strategy/**, spec/assurance/AD-001, spec/assurance/AD-004
review_set: subset
---

## Summary

Ticket: IR-364 (slice 2). This review covers numbering, traceability and redundancy. I checked:

- whether the AC ids are stable;
- whether the tests.md rows and the TC "Traces To" columns agree with the new ACs;
- whether TC-017 steps 6 and 7 and the TC-018 step 7 expected results cite the right ACs;
- whether text from the V1 wording is left over.

## Verdict

**PASS with two low findings.**

- No AC id is renumbered or removed.
- In tests.md, AC-7 and AC-9 go to TC-017 and AC-8 goes to Inspection. TC-017 traces AC-7 and AC-9
  but not AC-8, which is correct. FR-009-AC-7 goes to TC-018.
- The `PLANNED.` prefix matches the convention CG #397 uses for FR-014.
- `quire validate` reports nothing on the changed files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-008-AC-7 repeats an obligation that FR-008-AC-3 already carries. AC-3 says "Each of integer.eq/ne/lt/le/gt/ge has an executable selected-claim comparison oracle whose result agrees with the ordered relation and generated tags". AC-7 restates that agreement with an exhaustive domain sweep. Two ACs now carry the six-operator oracle agreement, and only AC-7 says how it is measured. Remove the oracle sentence from AC-3, or have AC-3 defer to AC-7. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:117,121 |
| FND-002 | low | The rewritten FR-012 Description still ends "This refines FR-002-AC-4 for bound populations". The rest of the paragraph now says "population of a selected V2 claim". Change "bound populations" to the V2 term so no V1 wording is left in the rewritten paragraph. | spec/strategy/functional/FR-012-constraint-preserving-shrinking.md:21 |

## Examined units

```yaml
scope:
- id: FR-008-AC-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "Each of integer.eq/ne/lt/le/gt/ge has an executable selected-claim comparison oracle whose result agrees with the ordered relation and generated tags."
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. For each of integer.eq, ne, lt, le, gt and ge, the selected claim's V2 comparison oracle returns, for every valuation of a 1- to 4-member domain ..., the result of an independently evaluated ordered relation."
- id: FR-012
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: "When proptest shrinks a failing case drawn from a `Satisfying`, `Violating`, or `Broad` population of a selected V2 claim (FR-008), every candidate it visits shall stay inside the shared domain ... This refines FR-002 FR-002-AC-4 for bound populations"
- id: TM-002
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: "| FR-008 | FR-008-AC-7, FR-008-AC-9 | TC-017 | 🚧 V2 planned | ... | FR-008 | FR-008-AC-8 | Inspection | 🚧 V2 planned |"
- id: TC-017
  path: spec/strategy/matrix/TC-017-bound-domain-admission.md
  role: examined
  excerpt: "6. For each of integer.eq, ne, lt, le, gt and ge, build a selected claim over a 1- to 4-member domain ... 7. In a CG integration test, obtain the comparison from IR's public accessor through its public API only ..."
- id: TC-018
  path: spec/strategy/matrix/TC-018-constructive-correlated-populations.md
  role: examined
  excerpt: "7. For each noncommutative operator, generate a read-first comparison and its literal-first mirror, and compare the generatable sets of each population with an independent enumeration."
bindings: []
```

## Dispositions

Round 1 reviewed d8f2f8a6af1c791f5b0b48e5bb32eafae4474b6a.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |

## Disposition round 1 verdict

**PASS.**

- **FND-001.** FR-008-AC-7 now names FR-008-AC-3 as the per-operator agreement it refines. AC-7 is
  the exhaustive 1- to 4-member sweep. AC-3 covers the tag and relation agreement and the
  sibling-refusal behaviour.
- **FND-002.** FR-012 now says "the populations of a selected V2 claim".
- **New AC-10.** FR-008-AC-10 is traced to Inspection in tests.md, and TC-017 step 8 and its expected
  result cite it.

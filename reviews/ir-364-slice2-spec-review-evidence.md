---
id: SR-11775
title: IR-364 slice 2 evidence review of the V2 strategy chain criteria
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen@f30f17b1a086211cd0ab87f2445eb5565fac74b5 (PR 400, diff vs origin/main 007417d); spec/strategy/functional/FR-008, FR-009, spec/strategy/matrix/TC-017, TC-018, tests.md
review_set: subset
---

## Summary

Ticket: IR-364 (slice 2). This review checks the verification method of each new criterion:
FR-008-AC-7 (Test), AC-8 (Inspection), AC-9 (Test) and FR-009-AC-7 (Test), together with their TC
procedures. It used `quire matrix`: AC-7, AC-9 and FR-009-AC-7 are untagged, and AC-8 is
method-without-symbol, which is expected for planned criteria.

## Verdict

**CONDITIONAL: one medium finding.**

- FR-008-AC-7 and FR-009-AC-7 are exhaustive, independent-oracle Tests over small domains. They are
  well matched to their method.
- Inspection is acceptable for FR-008-AC-8. Its subject is where the code comes from, not how it
  behaves; AC-7 already tests the eq and ne behaviour. A test could only show that the oracle module
  does not import the V1 oracle. The AD-004 layout test, which fails when a V1 type is named outside
  its allowed place, already covers the V1 half.
- AC-8's wording has its own high finding, SR-11770 FND-001. Rewriting AC-8 by input and ownership
  would keep Inspection as the right method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-008-AC-9 is verified by Test but ends with "with no `CheckedSemanticNodeV2.body` read". `body` is a `pub` field on IR main (`checked_package/v2/mod.rs:179`), so an integration test that reads it compiles and runs, and no assertion in the test can observe its absence. That clause can only be checked by Inspection, and FR-008-CON-1 already checks it that way. TC-017 step 7's "Assert no `CheckedSemanticNodeV2.body` read" cannot be implemented as a test assertion. Drop the clause from AC-9 and leave it with CON-1, or move it to an Inspection criterion of its own. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:123, spec/strategy/matrix/TC-017-bound-domain-admission.md:48-51 |

## Examined units

```yaml
scope:
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. For each of integer.eq, ne, lt, le, gt and ge, the selected claim's V2 comparison oracle returns, for every valuation of a 1- to 4-member domain ..., the result of an independently evaluated ordered relation."
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. The V2 comparison oracle for integer.eq and for integer.ne is produced by the V2 comparison-oracle path and not by `generate_exact_scalar_oracles` or the V1 bound oracle, which have no integer equality or inequality descriptor."
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. A CG integration test calls IR's public clause-context accessor through its public API only, and the generator's census, ordered relation and diagnostics equal the accessor's operator, ordinals, provenance, observation, bounds and refusal fields for each closed operator, provenance and observation variant, with no `CheckedSemanticNodeV2.body` read."
- id: FR-008-CON-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: "The generator SHALL read V2 operand identity, provenance, observation, operator and bounds only through IR's public typed accessor. A private decoder or copied schema is forbidden."
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "PLANNED. For each noncommutative operator, a comparison written with the literal in the first position produces the same satisfying and violating sets as its mirrored read-first form, enumerated independently."
bindings: []
```

## Dispositions

Round 1 reviewed d8f2f8a6af1c791f5b0b48e5bb32eafae4474b6a.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |

## Disposition round 1 verdict

**PASS.**

- **FND-001.** FR-008-AC-9 drops the "no body read" clause. A new criterion, FR-008-AC-10, carries
  it: "Source inspection shows the generator reads no `CheckedSemanticNodeV2.body` field and decodes
  no body JSON". It is verified by Inspection, matching CON-1. TC-017 step 7 no longer asserts it,
  and step 8 inspects for it.
- **Method.** Inspection remains the right method for the restated FR-008-AC-8.

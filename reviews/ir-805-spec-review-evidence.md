---
id: "SR-12103"
title: "IR-805 spec-review/evidence review of reference-graph reachability Kani claims"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@edbe6aabdd756a864482c2b73b1c0c113b101894; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-12103: IR-805 spec-review/evidence review

## Summary

Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.
Method: spec-review/evidence (can each control be produced with the declared Test method and
TC-025's QSL-emission-only rule).

## Scope

- TC-025 graph-family paragraph and "Open upstream inputs" :70-105 (examined).
- FR-015-AC-106 (examined), FR-015-AC-107 (examined), FR-015-AC-98 (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-025 says "the field-edge multiplicity is available now through `model_object_fields`", implying the field-edge Verified control of AC-106 is backable once pinned. It is not: both forms need a finite population, and no QSL-emitted state-clause reaches has one (SR-12101 FND-001) nor does IR map a claim to one. The limitation statement should say every healthy/mutant Kani control (field-edge and relationship-end) stays unbacked, and only refusal controls are testable now. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:99 |
| FND-002 | low | AC-107 and TC-025 step (4) require a "function-body claim" control, but QSL refuses `reaches` in a function body (FR-365-AC-3) and TC-025 forbids copied wire or hand-built builders, so no such package can be produced. State the actual input (a request naming a non-`state_clause` node, e.g. a function body node, as the clause id). | spec/kani/functional/FR-015-bounded-kani-obligations.md:775 |

## Verdict

Revise before accept. TC-025's fixture shape is otherwise emittable at QSL 46aa650be: an `Order`
object type with Option/Reference/sequence-of-Reference fields, relationships `next` (`{0,1}`),
`kids` (`{0,3}` unique), `litter` (`{0,3}` non-unique) and a `{2,3}` end, and invariant/pre/post
`reaches(self, target, <name>)` clauses, all match reachability.rs and FR-038-AC-252; the
`Population<Order>[N]` binding is the part that cannot bound a clause's reaches.

## New findings (disposition pass 7)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | TC-025 step (4) makes each `relationship_end` refusal (`UnknownNode`, `NotReachesApplication`, `UnresolvedEnd`) a control that is "testable now", but TC-025 allows only QSL-emitted, IR-admitted packages with no hand-built wire. Under that rule none of the three is reachable. CG resolves the clause path to an existing node first, and AC-107 refuses a non-reaches path before the accessor is called. IR FR-038 (:3275-3280) says `UnresolvedEnd` is reachable only through crate-internal post-admission mutation. Either name the seam that injects these refusals (for example a test double for the accessor) or record the mapping as Inspection. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:86 |

## Dispositions

Reviewed at e1873b156ea042a0b76fedf2a804becf17ac89e0 (round 1; fix commits d088e9c, e1873b1).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d088e9c..e1873b1: TC-025 now lists only refusal steps as testable now and moves every healthy and mutant Kani control, both forms, to unbacked pending QSL-819. |
| FND-002 | fixed | d088e9c..e1873b1: function-body item removed from AC-107 and TC-025 step 5; TC-025 records it as an Inspection that QSL refuses it. |

Round 7 note at 94718fd04fe90be72842cc6e840e9dd94afb2dd1: FND-001 and FND-002 stay `fixed`; new FND-003 is above.

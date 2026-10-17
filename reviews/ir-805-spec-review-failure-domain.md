---
id: "SR-12101"
title: "IR-805 spec-review/failure-domain review of reference-graph reachability Kani claims"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@edbe6aabdd756a864482c2b73b1c0c113b101894; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-12101: IR-805 spec-review/failure-domain review

## Summary

Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.
Method: spec-review/failure-domain. Ground truth: QSL 46aa650be (FR-104, FR-365,
qsl-semantics/src/check/lowering.rs:3669-3727, qsl-package/src/emit/tests/reachability.rs) and
IR 64f884e7 (FR-053, selected_population.rs, model_fields.rs, model_members.rs, FR-038-AC-252..257).

## Scope

- FR-015-AC-98 (examined), FR-015-AC-100 graph clause (examined), FR-015-AC-101 graph clause (examined), FR-015-AC-105 (examined), FR-015-AC-106 (examined), FR-015-AC-107 (examined).
- FR-015 description :109-148 and Inputs :204-210 (examined).
- TC-025 graph-family paragraph :70-105 (examined).

Verified clean: the reaches node shape (`semantic_form` reachability, `operator` reaches,
`quire.op.model.reaches` with member `{kind: relationship_end, declaration: <relationship model
node>, name: <role>}`, no laws/leaves/mode, two `reference` arguments, Boolean result) and the
`quire.op.model.reaches_field` field-member form match lowering.rs:3669-3727 and TC-1047; claim
identity = `state_clause` node id + argument path matches FR-365 "What a clause claim carries";
positive-length semantics match FR-365-AC-7; `{0,3}` unique/non-unique and `{2,3}` ends are
admitted by IR FR-038-AC-252; field-edge multiplicity via `model_object_fields` /
`CheckedMemberType` exists; IR has no public relationship-end multiplicity accessor
(`RelationshipEnd` is `pub(super)` in model_members.rs:220) and no claim-to-population mapping
(FR-053 takes a consumer-named selector; QSL requirement records are not on the V2 wire).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-98's harness path needs a finite `[0,N]` population for a state-clause reaches, but QSL never gives one: FR-104 "Requirements" says a domain-package population "never has a maximum ... always an unbounded Population(None) domain", and FR-365 says a reaches walks "the one population with no maximum that covers E". `Population<T>[N]` is a function-parameter type and QSL refuses reaches in a function body. So every QSL-emittable graph claim at 46aa650be has an unbounded population (select_population Declaration -> Unbounded); AC-105/AC-106 Verified controls and TC-025's `Population<Order>[N]` + clause fixture cannot be realized even after IR adds the two accessors. The spec presents the blocker as IR-only; it also needs a QSL change binding a clause's walked population to a finite binding, and should say current claims settle `requires_bound`/`unsupported` only. | spec/kani/functional/FR-015-bounded-kani-obligations.md:766, spec/kani/functional/FR-015-bounded-kani-obligations.md:140, spec/kani/matrix/TC-025-bounded-kani-obligations.md:70 |
| FND-002 | medium | AC-98/AC-105/AC-100 give start and target types separate populations and AC-105 draws "a subtype operand 1 ... from its own population", but reaches is homogeneous: QSL FR-365 walks one population covering E by conformance, FR-104 keys subtypes to the same DomainKey, and IR FR-038-AC-252 requires both operands to conform to the receiver endpoint type. With two populations, "every target is an object of the population" is ambiguous and a target outside the walked population is unreachable by construction. | spec/kani/functional/FR-015-bounded-kani-obligations.md:773 |
| FND-003 | medium | AC-98 assigns dispositions to `Unbounded`, `UnknownClosureOpenExtent`, unavailable, stale, foreign and non-population selections, but IR FR-053's `CheckedSelectedPopulationOutcome` also has `MissingOccurrence`, `UnknownClosureUnresolvedObjectType` and `UnknownClosureUnresolvedSubtypeClosure`; those three have no stated disposition, so implementers may map them to either `requires_bound` or `unsupported`. | spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |

## Verdict

Revise before accept. On the two IR gaps: both confirmed (no public relationship-end multiplicity
accessor; no reaches-claim-to-population mapping, FR-053 needs a consumer-named binding and
`Type` occurrence). Settling them `unsupported` is correct and testable as refusal behaviour, but
the success path is additionally blocked upstream in QSL semantics (FND-001), which the spec does
not state.

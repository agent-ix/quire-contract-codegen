---
id: SR-016
title: "Failure-domain review of numeric and state Kani lowering"
type: SpecReview
analysis: failure-domain
scope: "FR-003, interface-001 kani_slice, TC-003 and TC-014"
review_set: subset
---

## Summary

The failure-domain review examined customer subject and proof-extension boundaries, dependency
identity, framing purity, state topology, unsupported semantics, and resource failure. The amended
specification now distinguishes generated proof claims from external subject failures and refuses
every unrepresentable semantic shape without a partial bundle.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The first draft named a framing region without defining what state it framed, which could be read as a proof claim over unmodeled globals, heap or aliases. FR-003 and interface-001 now limit framing to copied primitive arguments and returned primitive values. | FR-003; interface-001 |
| FND-002 | medium | Customer subject paths and assumed/stubbed proof paths are extension points, but generation-time refusal and external compile/Kani failure were not separated. The interface now validates path syntax and the typed ABI while reserving signature, link and body failures to external observations that cannot become proof success. | interface-001; TC-014 |
| FND-003 | medium | Kani-specific failure conditions lacked a stable code allocation. The interface now assigns binding, propagated clause, backend, identity, dependency, unwind, attestation, syntax, serialization and resource failures without a fallback code or partial output. | FR-003-AC-3; interface-001; TC-003 |

## Failure controls

The full checked `DependencyIdentity` is the uniqueness and ordering key. Identical references unify;
conflicting logical declarations refuse. Proof assumptions and stubs retain separate source sites,
while IR model bounds remain typed binding facts and never masquerade as completed proof edges.

Only direct primitive values enter the adapter. Indirect dependencies, dereference, reachability,
object/graph reads, definedness obligations, numeric arithmetic/negation, and post-state in a
precondition refuse before publication. Consequently there is no callback traversal, cyclic graph,
alias topology, or hidden mutation path inside the generated semantic model. Existing IR and source
size limits bound the remaining finite dependency and generated-source populations.

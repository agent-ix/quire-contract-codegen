---
id: FR-036
title: "Export a typed source-obligation census and producer facts"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: traces_to
  - target: ix://agent-ix/quire-contract-codegen/AD-005
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-045
    type: references
  - target: ix://agent-ix/quire-contract-ir/AD-008
    type: references
---
# FR-036: Export a typed source-obligation census and producer facts

## Description

When a source-obligation census is requested, Codegen shall export every candidate
from the selected authoritative semantic-obligation inventory before capability
negotiation, generation or execution, retaining its authored criterion and
production-symbol mappings or an explicit typed missing-mapping reason.

## Inputs

- One selected admitted checked package and the complete owner-exported semantic
  obligation inventory for that package, including each obligation's occurrence
  identity and declared domain.
- The selected authoritative criterion and production-module/function inventories,
  and their reviewed typed correspondence to the semantic obligations.
- Separately, actual capability decisions, pre-execution refusals and execution
  evidence from their owning producers; these are not census inputs.

The authored correspondence is the upstream prerequisite
[IR-759](https://linear.app/agent-ix/issue/IR-759). Its concrete type
name and placement are pending that owner's reviewed contract; they are not current
IR or QSL APIs. [AD-005](../../assurance/AD-005-source-obligation-producer.md)
identifies the usable current inputs and the missing correspondence.

## Outputs

A `SourceObligationCensus` containing the selected inventory identities, every
candidate, the original criterion/module/function entities including those with no
mapped obligation, and explicit many-to-many mappings or missing reasons.
Separate `SourceCapabilityFact`, `SourcePreExecutionRefusal` and
`SourceExecutionFact` records describe attempts without changing the census.
[interface-001](../../core/functional/interface-001-codegen-api.md) declares the
planned public operation and the producer-fact vocabulary.

## Behavior

The census boundary shall enumerate obligations from the supplied complete owner
inventory rather than a requested generation subset, generated harnesses, test tags
or successful results. The census boundary shall preserve the source occurrence,
selected source identity and declared domain independently of attempt identity.
The census boundary shall retain all criterion and production entities from the
selected inventories, including entities with no mapped obligation.

When the reviewed correspondence supplies several links, the census boundary shall
retain every criterion-to-obligation and obligation-to-module/function link.
When a mapping is absent, the census boundary shall retain the candidate with a
`MissingMapping` reason distinguishing absent criterion correspondence, absent
production correspondence and an explicitly authored absence of a production function.
If an input relation names a foreign selected inventory or a nonexistent endpoint,
then the census boundary shall return a typed `SourceCensusRefusal` with the offending
relation and no complete census. If the authoritative inventory or required typed
input is absent or incomplete, then the census boundary shall return a typed refusal
rather than publishing a truncated census or synthesizing a source fact.

The capability exporter shall retain a decision's producer, stage, candidate,
capability and actual disposition/reason without inventing an artifact or run.
The refusal exporter shall retain the original producer, stage and typed cause when
generation or run admission refuses before backend execution.
The execution exporter shall retain the actual method, attempt identity, source/profile identity, proof
subject, declared domain, effective bounds, assumptions, terminal outcome and reason.
When an optional producer observation is absent, the exporter shall record explicit
absence rather than a measured zero, unsupported decision or inferred success.
When a producer state is unfamiliar, the exporter shall retain its received identity
and typed unclassified state rather than converting it into a familiar terminal state.

The execution exporter shall retain a load-bearing check's explicit claim binding
separately from the backend SUCCESS-check count. The execution exporter shall retain
production reachability evidence that identifies the asserted production behavior
separately from incidental call-graph presence. When those observations are unavailable,
the execution exporter shall record them as missing rather than infer them from a
check location, test tag, LLVM observation or positive check count.
The execution exporter shall retain production versus shadow subject, narrower
effective bounds and separately executed refinement observations, including the
refinement's sampled scope and assumptions. When refinement has not executed, the
execution exporter shall record its absence rather than infer refinement from a
planned obligation or successful shadow proof.

The producer boundary shall preserve each actual result record, including duplicate
attempt records and records with mismatched source/profile/subject/domain associations,
for the IR accounting boundary to assess. Codegen exports facts; IR FR-045 owns
eligibility, evidence joins, qualified credit and requirement/module/function/obligation
projections. Codegen emits no eligibility decision, qualified-proof flag, numerator,
denominator or blended percentage in this seam.

The public boundary shall expose the census and separate fact records as borrowable
Rust types through the acyclic consumer seam in AD-005. The consumer shall read their
identities, relations and reasons without parsing source text, transcript prose,
test names or a second schema. When selected inventories are unchanged, the census
boundary shall produce the same ordered census. The census boundary shall determine
membership from authoritative inputs without a manually maintained file, tool,
version or pin catalog.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-036-AC-1 | A genuine typed fixture with two criteria, three semantic obligations and two production functions in one module exports all three obligations, all selected entities, explicit many-to-many links and distinct missing-criterion/missing-production/authored-no-function reasons; an entity with zero mapped obligations remains present. Removing one authoritative obligation and its relations changes the candidate census without editing an auxiliary catalog. | Test |
| FR-036-AC-2 | For the unchanged source fixture, omitting an execution result, reporting unsupported negotiation or a pre-run refusal, retrying a run and changing method/profile/subject leave every candidate key, declared domain and source mapping unchanged; enumeration succeeds before any producer is invoked. | Test |
| FR-036-AC-3 | Capability facts preserve unsupported capability and reason without artifact/run/outcome; pre-run refusal facts preserve producer/stage/cause separately from actual execution outcomes. An absent decision or observation and an unfamiliar producer state remain distinct, and missing, counterexample, invalid/incomplete, unavailable, timeout, exhaustion, cancellation and inconclusive evidence states survive export. | Test |
| FR-036-AC-4 | A matching executed production fact retains its exact source/profile/subject/domain/bounds and explicitly bound load-bearing check plus production reachability observation. Zero-check, check-location-only, tag-only, LLVM-only and incidental-reachability controls retain their actual weaker or missing facts and never acquire the positive observation by export. | Test |
| FR-036-AC-5 | Shadow-only evidence, a narrower effective bound and separately executed sampled native refinement retain distinct subject/scope/assumption observations. Missing refinement stays missing; adding a refinement observation does not change the census or create an unqualified-production-proof flag. | Test |
| FR-036-AC-6 | A downstream consumer above CG and IR borrows the public census/fact types over the reviewed neutral correspondence contract without source/transcript/name parsing or a copied schema. Duplicate and source/profile/subject/domain-mismatched result controls survive as separate inspectable records without changing census rows; repeated identical inputs produce identical ordering. | Test |
| FR-036-AC-7 | Missing/incomplete authoritative inputs, a relation targeting a foreign selection and a dangling relation endpoint produce distinct typed census refusals with their offending input/relation and no complete census. A valid absent mapping remains a candidate gap instead of that global refusal. | Test |

## Dependencies

The upstream IR-759 reviewed neutral typed correspondence is required before the
complete export and its consumer conformance can be implemented. IR-740 implements
this planned producer contract after that prerequisite. IR-737 implements the
accounting policy defined by `ix://agent-ix/quire-contract-ir/FR-045` and
`ix://agent-ix/quire-contract-ir/AD-008`; it does not give CG permission to mint
missing upstream facts.

[FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md),
[FR-017](../../kani/functional/FR-017-kani-execution-evidence.md) and
[FR-019](../../routed/functional/FR-019-capability-settlement.md) own current
negotiation/generation, execution and capability facts. [FR-004](FR-004-vacuity-evidence.md)
owns LLVM vacuity observations; such an observation is not a proof verdict.
[TC-051](../matrix/TC-051-source-obligation-census.md) plans the falsifying controls.
All acceptance criteria are Planned; complete typed producer/consumer tests are
Gated on IR-759. This spec-only change claims no Cargo or Kani execution.

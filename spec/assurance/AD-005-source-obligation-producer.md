---
id: AD-005
title: "Source-obligation census producer and acyclic consumer seam"
type: ArchitectureDescription
status: proposed
owner: Contract Codegen lane
system: selected source candidate census and separate capability, refusal and execution facts
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-036
    type: realizes
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-045
    type: references
  - target: ix://agent-ix/quire-contract-ir/AD-008
    type: references
---
# Source-obligation census producer and acyclic consumer seam

## System Boundary

This AD defines the planned [FR-036](../evidence/functional/FR-036-source-obligation-census.md)
CG producer seam. The source owners supply selected inventories and authored mappings;
CG exports their complete census and actual producer facts. IR FR-045/AD-008 own
eligibility, joins, proof credit and the four accounting projections. The driver or
another adapter above both owners connects the borrowed exports to IR accounting.
This seam adds no solver, production-source parser or assurance verdict.

## Views

The views identify existing source authorities, the missing authored relation,
population and producer-fact flow, and the acyclic accounting consumer.

### Current owner inputs

The following API capabilities are measured, not evidence that the complete census
already exists. References name authoritative items rather than maintain a revision
or file ledger.

| Authority | Existing typed input | Information available and boundary |
| --- | --- | --- |
| IR model | `quire_contract_model::CheckedPackageV2::{package_id,lock,graph,node_kinds,source_map}` in `checked_package/v2/mod.rs` | Admitted selected semantic graph, source occurrences and original selections. Authored semantic Function nodes are not production Rust functions. |
| QSL checked package | `qsl_package::CheckedPackage::graph()` and `qsl_semantics::check::CheckedGraph::{source,semantic_graph,occurrences,requirements}` | Immutable source and checked claim records keyed by `OccurrenceKey`; `RequirementRecord` exposes capability/extent, result bound and path conditions before backend negotiation. These records are not authored FR acceptance criteria. |
| QSL native composed checker | `quire_spec_language::checking::composed::DeclarationTypes::obligations()` and its `Obligation`/`Site` | Source-owned pending proof/runtime conditions, preserving declaration/unit/expression location; distinct from the checked claim-record population and not discharged proofs. |
| QSL native bindings | `checking::{ClauseBinding,CheckBindings}` | Caller-authored requirement/clause correspondence; no acceptance-criterion or production-symbol relation. |
| Production structural extraction | `quire_code_rs::{SourceFile,parse_file,extract}`, `ParsedFile::facts`, `CodeFact` | Existing Module/Function facts with qualified name, parent, path, span and diagnostics from selected source bytes. Production selection and obligation mapping require explicit authority; mention/call edges do not authenticate those mappings or load-bearing reachability. |

A census selects one reviewed source population and its complete owner inventory;
it does not union unrelated checker populations or declare every semantic graph node
a proof obligation. A QSL request subset or a CG claim map generated only for selected
items is not that complete inventory. The selection records the inventory owner and
population scope so absence of a family cannot be hidden behind an empty success.

### Missing authored correspondence

[IR-759](https://linear.app/agent-ix/issue/IR-759) owns the missing neutral typed input linking selected criterion identities to
semantic obligation occurrences and selected production module/function identities.
It must retain the inventories themselves, many-to-many links, explicit missing
mappings and foreign/dangling relation refusals. The existing IR checked package and
QSL records do not provide that relation. Existing production extraction supplies
symbols, not the authored semantic relation. CG must not fabricate it from a name,
tag, check location or call graph.

Its concrete type name, placement and adapter contract remain open pending IR-759's
reviewed upstream specification. This AD does not define a replacement upstream
schema. IR-759 is an implementation prerequisite of IR-740, which supplies this
producer; IR-737 consumes the resulting facts through the IR-owned accounting port.

### Population and fact flow

The planned `source_obligation_census` operation accepts only selected authoritative
package, semantic-obligation, criterion, production-symbol and correspondence inputs.
No negotiation, artifact or result parameter influences population membership.
It returns separate complete obligation/entity inventories, complete-key candidate
rows and explicit entity/production mapping-gap records. Re-reading
`ix://agent-ix/quire-contract-ir/FR-045` Behavior and
`ix://agent-ix/quire-contract-ir/AD-008` Claim and population establishes the key:
owning criterion × semantic obligation × selected source identity × declared domain.
Every distinct authored complete tuple is one candidate; several criteria linked to
one obligation remain separate candidates. Different source identities or declared
domains also remain separate. This is enumeration of authored tuples, not an inferred
Cartesian product, and module/function projection links do not add a duplicate tuple.

An obligation lacking criterion/source correspondence stays in the complete obligation
inventory with a typed gap, without a sentinel or invented criterion-keyed candidate.
A criterion or source symbol with no mapped obligation likewise remains its own
inventory/gap record. An existing complete candidate key survives absent production
correspondence or authored no-function with its typed production gap. Missing/incomplete
inventories and foreign or dangling relations refuse the complete export rather than
silently omit rows.

The separate producer records have these responsibilities:

| Proposed CG type | Facts retained | Qualification owner |
| --- | --- | --- |
| `SourceCapabilityFact` | Complete candidate association, producer/stage, requested capability, actual disposition and typed reason; no invented run/artifact for unsupported | IR |
| `SourcePreExecutionRefusal` | Complete candidate/attempt association, producer/stage and original typed refusal cause; absent executed run/outcome stays absent; an existing generated artifact may be retained without asserting a run | IR |
| `SourceExecutionFact` | Complete candidate association and attempt identity; actual typed artifact and run identities retained separately with explicit missing-identity reasons; method, actual source/profile, subject, declared/effective domain and bounds, assumptions, terminal state and reason; check binding, production reachability and subject/refinement observations with explicit absence | IR |

Candidate membership is independent of method, subject, profile or attempt. Reported
attempt associations are retained as reported even when they mismatch the selected
census. AD-008 Evidence and projections explicitly joins artifact and run identity;
neither is reconstructed from an attempt identity, harness path or subject. Wrong,
missing and mismatched artifact/run identities remain inspectable for IR joining.
Duplicate results stay separate records, so IR can reject credit without losing the
source candidate. There is no CG qualified-proof field or accounting total.

A load-bearing observation names the candidate and the check/property about its
asserted production behavior. A production reachability observation names the exact
production symbol and evidence that the asserted behavior was in the proved path.
An incidental callee, SUCCESS total, generated source location, test tag or LLVM probe
alone supplies neither observation. Production/shadow subject, narrowed effective
scope and sampled executed refinement remain separate facts with assumptions; a
planned refinement supplies no executed observation.

### Acyclic consumer

CG already depends on `quire-contract-ir` for Kani APIs and on the cycle-free
`quire-contract-model` for semantic types. IR root cannot add a CG dependency to read
this export. The neutral IR-759 contract must retain the model's prohibition on QSL,
CG and IR-root dependencies. Concrete QSL/CG adapters belong above those authorities.

CG's public result is borrowable in a consumer that already depends on CG and IR.
That consumer implements the IR-owned accounting input port using the reviewed neutral
correspondence contract and CG records. IR owns the port and policy; the adapter passes
actual facts without inventing missing evidence. The upstream owner determines exact
port/type placement. Compile conformance is required before this seam is implemented;
these planned names do not claim a currently available upstream API.

## Decisions

- Enumerate before producer work, from complete selected source inventories. This
  requires an explicit correspondence input but makes retries and omissions unable
  to alter population membership.
- Reuse existing IR/QSL semantic and production-extraction authorities. Source
  parsing and schema copies in CG would add another authority and are excluded.
- Separate capability, pre-run refusal and execution records. This preserves an
  unsupported decision with no invented execution and identifies the refusal stage.
- Publish facts and explicit absence. IR determines eligibility and credit; CG cannot
  promote a weaker observation while translating producer records.
- Use an adapter above CG/IR over an IR-owned port. A direct IR-to-CG dependency
  would close the existing dependency cycle and is excluded.

## Risks

- **IR-759 correspondence unavailable.** IR owns the upstream contract. Review its
  selected inventories, identity and missing/foreign/dangling semantics before
  implementing IR-740; complete tests remain Gated until that review/delivery.
- **Incomplete family population.** The semantic inventory owner must state its
  selected scope and completeness. CG returns an incomplete-input refusal when the
  authority cannot supply it; no candidate inference from generated artifacts.
- **Unavailable production observations.** CG owns producer instrumentation under
  IR-740. Until a run supplies claim-bound checks, production reachability or executed
  refinement, record absence; IR determines the resulting qualification gaps.
- **Acyclic seam placement unresolved.** IR-759 review owns the exact neutral types
  and port. The implementation must demonstrate borrowed downstream consumption
  without a new reverse dependency before FR-036-AC-6 can pass.

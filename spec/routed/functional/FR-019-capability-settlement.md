---
id: FR-019
title: "Settle every capability claim at one negotiation point"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/TC-271
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-288
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-029
    type: references
---
# FR-019: Settle every capability claim at one negotiation point

## Description

When a backend provider envelope requests claims of this generator, the
generator shall settle each requested item at exactly one point, a `negotiate_*`
arm over a closed backend kind, and shall settle no capability anywhere else.

AD-016 places the single negotiation point here so that language admission
stays language-only and the IR stays target-neutral. That boundary holds only
when adding a backend kind is a compile error everywhere the new kind must be
handled; an open set of negotiate functions gives the same behavior at run time
and none of the enforcement, because a new kind simply has no settlement and
nothing says so.

## Inputs

- The in-process CG view of the FR-331 envelope: its contract version, its
  capability vocabulary, the registered backend descriptors, and the request
  items. Each CG-owned `BackendDescriptor` carries a typed `ProviderOrigin`
  (`Linked` or `Process`)
  beside its identity and advertised (kind, mode) pairs.
- Per request item: its one required FR-290 capability kind, its extent and
  that extent's `bounded` or `unbounded` classification, the backend the caller
  names when it names one, and its `candidates`.
- `candidates` is read, never computed. The `quire-spec-language` registry
  computes it under FR-290's candidate-set rule; this generator is the consumer
  on the far side of that seam.

## Outputs

- One disposition per requested item: `supported`, `requires-bound`,
  `unsupported` (warned) or `invalid-request`.
- A typed cause for every disposition that is not `supported`, drawn from the
  FR-290 vocabulary and naming the offending backend or candidates where that
  vocabulary requires it.
- No artifact for an item settled `unsupported`, `requires-bound` or
  `invalid-request`.

## Behavior

- The generator shall dispatch settlement through one `negotiate_*` arm per
  variant of a closed backend kind, with no catch-all arm. A backend kind with
  no arm shall fail to compile.
- The generator shall refuse an envelope whose `capability_vocabulary` is not
  exactly `quire.capability-kind/v1`, or is absent, as
  `invalid_capability`/`unsupported-version`, and shall read none of its kinds.
- The generator shall evaluate each item's rules in FR-290's stated order, and
  the first rule that matches shall settle the item: an absent or unknown kind;
  an absent extent classification; then the candidate table, top row first.
- If an item names an unregistered backend, or names a registered backend that
  has no negotiation arm, then the generator shall settle it `invalid-request`
  with `invalid_capability`/`unknown-backend`, naming the backend.
- If an item's candidates include one absent from the registered descriptors, one that does
  not advertise the item's kind, or a set disagreeing with the named backend,
  then the generator shall settle it `invalid-request` with
  `invalid_capability`/`inconsistent-candidates`, naming each offending
  candidate.
- If an item's candidate set is empty, then the generator shall settle it
  `unsupported` with a warning naming the item's kind, and the named backend
  when the request names one, with cause
  `unsupported_projection`/`unsupported-requested-capability`.
- If an item has more than one candidate and the request names no backend, then
  the generator shall settle it `invalid-request` with
  `invalid_capability`/`ambiguous-backend`, naming every candidate in candidate
  order. The generator shall not select among several candidates, and
  registration order, display names and ambient state shall never change a
  candidate set, its order or a disposition.
- If an item has exactly one candidate, then that candidate's arm shall settle
  it under the advertised-mode rules.
- If an item's extent is unbounded and its one candidate advertises `bounded`
  only, then the generator shall settle it `requires-bound` when a finite bound
  is available.
- If an item's extent is unbounded, its one candidate advertises `bounded`
  only, and no finite bound is available, then the generator shall settle it
  `unsupported`, warned, with
  `unsupported_projection`/`unbounded-extent`.
- The generator shall settle an unbounded extent `supported` only on a
  candidate advertising `unbounded` for the item's kind.
- The generator shall never narrow an extent to reach a disposition.
- If an item carries no extent classification, then the generator shall settle
  it `invalid-request` with `invalid_capability`/`absent-extent`.
- The generator shall settle `requires-bound` only as a negotiation
  disposition.
- The generator shall never surface `requires-bound` as a verification result.

### The process-provider kind (QSL ADR-029 PV-4)

- The generator shall give the closed backend kind one variant for process providers, beside `Kani`.
- The generator shall list that variant in `BackendKind::ALL`.
- The generator shall settle an item whose candidate is a process-provider backend in that variant's
  `negotiate_*` arm.
- The generator shall settle such an item in no other place.
- The process-provider arm shall settle an item from the candidate's descriptor in the envelope manifest
  and the item's extent classification alone.
- The process-provider arm shall never settle an unbounded extent `supported` against a descriptor that
  advertises `bounded` only for the item's kind.
- The process-provider arm shall never narrow an extent.
- The process-provider arm shall never call, start, open or otherwise reach the plugin that
  descriptor stands for.
- The process-provider arm shall return a `Disposition` and nothing else.
- The generator shall never return a terminal value, a verification result or an artifact from settling.

### Provider origin at the QSL to CG boundary (IR-633)

The driver shall make one projection from each descriptor in `Registry::descriptors()` to this
crate's `BackendDescriptor` for the FR-331 envelope. It copies the registry descriptor's `id`, advertised
(kind, mode) pairs and `origin()` into CG's corresponding typed members without changing their
meaning. At registration, the driver's manifest-to-QSL-descriptor conversion supplies
`ProviderOrigin::Linked` for a compile-time provider or `ProviderOrigin::Process` for a plugin
`hello` to QSL `BackendDescriptor::new` or `admit` (QSL ADR-029 PV-1). The QSL registry holds that
typed value and checks conflicting registrations; neither the manifest nor the plugin supplies
the origin. CG shall preserve that value and shall not derive or override it from the backend
identity, executable or provider bytes, or a second identity-to-origin side map. A
built-in Kani descriptor supplied by the driver has origin `Linked`; its existing Kani
classification remains the built-in path. A process descriptor's origin is the sole fact that
permits the planned `Process(id)` classification, including when its identity text resembles a
built-in identity. QSL's registry conflict rule removes registrations with one identity and
different origins before this projection (ADR-029 PV-1).

This is a Rust input boundary, not a new FR-331 wire member. QSL layer R owns the closed,
two-valued origin vocabulary and descriptor semantics (QSL FR-288; ADR-029 PV-1); its registry
holds the authoritative value. CG's enum is a local typed projection of those meanings and
defines no independent origin category. QSL ADR-013 T-7 requires separate native representations
across this boundary, so the driver maps QSL `ProviderOrigin` to CG `ProviderOrigin` in one exhaustive match
with no wildcard arm and tests both values. A new QSL variant fails the driver build until that
projection is updated. CG takes no direct `qsl-route` dependency (FR-022 "Where the input type
lives"). This slice does not add `BackendKind::Process`, change its serialization, or settle a
process item; IR-629 owns those
behavioral changes and the open process-provider questions below.

Measured present fact, not a requirement of this change: `BackendKind::from_identity` (`src/routed/capability.rs`)
returns `Some(Kani)` for `kani` and `None` for every other identity, and CG's own calls to it
(`unroutable_named_backend` and `negotiate_single_candidate` in `capability.rs`, and the
`BackendKindDisagrees` check in `generate.rs`) rely on that. This specification changes none of it.
Every change to `from_identity`, and the mapping from a process-origin descriptor to the
process-provider variant, remains IR-629 work. The typed origin input above resolves which fact
that mapping reads, without implementing the mapping here.

Adding the variant breaks the public `BackendKind` enum for every exhaustive match outside this crate. The
quire-driver lane's pre-negotiation conversion is the paired adaptation (QSL ADR-029 Amendments, ADR-012
§7.2); this repository edits no other repository.

IR-629 follow-on decisions (the process-provider criteria remain planned; IR-633 implements none
of their arms):

1. QSL ADR-029 PV-4 now says the process arm reads advertised domains and bounds from the
   manifest as well as (kind, mode) pairs, with existing FR-290 causes until QSpec adds specific
   mismatch causes. IR-629 owns those additional descriptor inputs and dispositions (AC-12).
2. QSL ADR-029 PV-1 and PV-4 make the descriptor's typed origin the classification input;
   `Process(BackendId)` carries the plugin identity as data and `from_identity` remains the
   built-in lookup. IR-629 owns that variant and its exhaustive dispatch (AC-11).
3. QSL ADR-029 PV-4 now assigns CG an empty process generation output and assigns execution,
   replay and terminal settlement to the driver. IR-629 owns the corresponding exhaustive arms;
   this slice adds none.
4. PV-4 names `Process(BackendId)` but does not state a serialized label for the data-bearing
   variant. IR-633 defines no serialization.
5. PV-4 now states the bounded and unbounded mode decisions from the process manifest. IR-629
   owns the concrete arm and tests (AC-12); the Kani arm is unchanged here.
6. QSL ADR-029 PV-1 withdraws linked and process descriptors that conflict on one `BackendId`,
   regardless of registration order. Neither reaches CG as a candidate; CG does not resolve that
   conflict from identity text.

Every criterion below that needs the new variant, FR-019-AC-11 to AC-13, remains IR-629 work and
requires its exhaustive arms to compile. FR-019-AC-14 is a property of the return type.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-019-AC-1 | Settlement dispatches one arm per variant of the closed backend kind, and every variant reaches an arm that settles rather than falling through. | Test (TC-030) |
| FR-019-AC-2 | Every requested item receives exactly one of `supported`, `requires-bound`, `unsupported` and `invalid-request`, settled from its `candidates` and extent classification, and `candidates` is read from the request rather than computed here. | Test (TC-030) |
| FR-019-AC-3 | An unbounded extent against a `bounded`-only advertisement settles `requires-bound` when a finite bound is available, rather than `unsupported`; with no finite bound available it settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`; and it never settles `supported`. | Test (TC-030) |
| FR-019-AC-4 | An item with more than one candidate and no named backend settles `invalid-request` with `invalid_capability`/`ambiguous-backend`, naming every candidate in candidate order, identically under every registration order. | Test (TC-030) |
| FR-019-AC-10 | Only an item settled `supported` routes, and a registry repeating a backend identity routes nothing rather than choosing between its entries. | Test (TC-030) |
| FR-019-AC-7 | An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`, or is absent, is refused as `invalid_capability`/`unsupported-version` with none of its kinds read. | Test (TC-030) |
| FR-019-AC-8 | An item with an absent extent classification settles `invalid-request` with `invalid_capability`/`absent-extent`, and an absent or unknown kind settles before the candidate table is consulted. | Test (TC-030) |
| FR-019-AC-9 | A backend kind added without a negotiation arm does not compile: the dispatch is an exhaustive `match` over the closed kind with no catch-all arm. | Analysis |
| FR-019-AC-11 | PLANNED (IR-629). `BackendKind` has the process-provider variant beside `Kani`, `BackendKind::ALL` lists both with `index` returning each position, and an item whose one candidate is a process-origin backend reaches that variant's arm and settles there. A mutant that leaves the variant out of `ALL` fails this. Classification reads the descriptor's typed origin as this requirement states. | Test (TC-046) |
| FR-019-AC-12 | PLANNED (IR-629). The process-provider arm's disposition is a function of the descriptor's advertised capability, mode, domain and bound and the item's extent classification: descriptors with equal advertisements and different identity text, manifest position or ambient state settle identically apart from the backend each names, and an unbounded extent against `bounded`-only never settles `supported`. Identity text cannot choose a disposition beyond naming the backend. The concrete mode, domain and bound rows follow QSL ADR-029 PV-4 and FR-290. | Test (TC-046) |
| FR-019-AC-13 | Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent executable settles identically to one with an ordinary identity, apart from the backend each names, and starts nothing, and a descriptor whose identity names an executable that records its own start leaves no record. A mutant arm that starts or resolves the identity as a process either changes the first disposition or leaves the record, and fails this. | Test (TC-046) |
| FR-019-AC-14 | The process-provider arm's return type is `Disposition`, which has no terminal-value, verification-result or artifact member, so settling returns none of them. A change that returns one does not compile against that type. | Analysis |
| FR-019-AC-15 | PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin` projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288, ADR-029 PV-1); layer R owns that vocabulary. The driver projects each descriptor from `Registry::descriptors()` once into CG's descriptor, copying `id` and advertised pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and `Process` to `Process`, with no wildcard or origin inference from identity, manifest or provider bytes or a side map. Two registry descriptors identical except for origin produce CG descriptors identical except for origin; a linked Kani descriptor remains `Linked`. The CG dependency graph adds no direct `qsl-route` edge and the FR-331 wire gains no origin field. | Test (TC-030) |

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and the
  `quire-spec-language` registry that computes `candidates`.
- **Downstream**: [TC-030](../matrix/TC-030-capability-settlement.md),
  [TC-046](../matrix/TC-046-process-provider-settlement.md); quire-driver's pre-negotiation
  conversion, the paired adaptation to the new variant, to be added to quire-driver IR-609 (which does
  not mention it today) or to a new driver ticket by the driver lane at merge;
  [FR-022](./FR-022-routed-generation.md), the generation arm of the same seam.

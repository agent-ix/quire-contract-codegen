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
  - target: ix://agent-ix/quire-spec-language/FR-335
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-036
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-029
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
  beside its identity, advertised (kind, mode) pairs, and the FR-331 manifest's
  advertised `domains` and `bounds`. The manifest's `bounds` are run-limit defaults, not
  admission limits; the process arm does not compare their numbers with item
  proof bounds (QSpec FR-331). The driver copies `domains` and `bounds` from the same FR-331
  manifest descriptor as the advertised pairs; QSL's route descriptor carries
  advertised pairs but no `domains` or `bounds`. QSL-654 owns the matching
  PV-4 wording update.
- Per request item: its one required FR-290 capability kind, its extent and
  that extent's `bounded` or `unbounded` classification, the backend the caller
  names when it names one, its typed IR tag/form, and its `candidates`. The
  typed IR tag/form is the form lowered from the admitted checked item; it is
  an in-process negotiation input, not a new FR-331 wire member. For a bounded extent the arm
  reads `extent.bounds[].kind`, the domain kind of each substituted `ProofBound`
  (QSpec FR-331; QSL-654 owns the producer); for an unbounded extent it reads
  `extent.domains[].kind` and `finite_bound_available`.
- `candidates` is read, never computed. The `quire-spec-language` registry
  computes it under FR-290's candidate-set rule; this generator is the consumer
  on the far side of that seam.

## Outputs

- One disposition per requested item: `supported`, `requires-bound`,
  `unsupported` (warned) or `invalid-request`.
- A typed cause for every disposition that is not `supported`, drawn from the
  FR-290 vocabulary and naming the offending backend or candidates where that
  vocabulary requires it.
- A Kani arm that does not discharge an admitted IR form uses
  `unsupported_projection`/`unsupported-requested-capability` and names the
  typed form and Kani candidate.
- No artifact for an item settled `unsupported`, `requires-bound` or
  `invalid-request`.

For an unsupported IR profile entry, IR FR-029 owns the absence of an artifact,
`KaniOutcome` and `TerminalValue`; CG settlement emits no such result.

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
- After the existing request and candidate checks select exactly one Kani
  candidate, the Kani arm shall inspect the item's typed IR tag/form before
  returning `supported` or allowing generation. A form outside Kani's
  capability shall settle `unsupported`, warned, with
  `unsupported_projection`/`unsupported-requested-capability`, naming the
  typed form and Kani candidate; it shall emit no artifact and reach no
  generator. A form Kani admits shall continue through the existing extent
  and advertised-mode rules. The form decision shall use the typed IR tag and
  form, never display or debug text.
- CG shall expose `BackendKind::from_descriptor(&BackendDescriptor) ->
  Option<BackendKind>` as its public descriptor-to-kind conversion. It shall
  return `Some(Kani)` for a linked descriptor whose identity is the built-in
  `kani`, `None` for an unknown linked identity, and `Some(Process(id))` for a
  process-origin descriptor with its exact identity even when that text is
  `kani`. It shall read origin and identity, never infer process origin from
  identity text.
- When a request names a registered process descriptor, CG shall classify that
  named candidate with `from_descriptor` and reach its `Process(id)` arm.
- CG shall not classify a named process descriptor with the built-in
  `from_identity` lookup.
- If an item's extent is unbounded and its one Kani candidate advertises `bounded`
  only, then the Kani arm shall settle it `requires-bound` when a finite bound
  is available.
- If an item's extent is unbounded, its one Kani candidate advertises `bounded`
  only, and no finite bound is available, then the Kani arm shall settle it
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
- The generator shall list the finite built-in kinds only in `BackendKind::ALL`. A process
  identity is data in `Process(BackendId)`, so the generator shall enumerate
  process candidates from the request's descriptors, not from `ALL`.
- CG shall represent the `BackendId` inside `Process(id)` with its own backend
  identity string, the same value held by `Candidate.identity`; it shall not
  import QSL's Rust `BackendId` type. `BackendKind::index` shall return `0` for
  Kani and `1` for any Process identity as a category sort key, not as an
  enumeration of process identities.
- The generator shall settle an item whose candidate is a process-provider backend in that variant's
  `negotiate_*` arm.
- The generator shall settle such an item in no other place.
- The process-provider arm shall settle an item from the candidate's descriptor
  in the envelope manifest and the requested item's capability kind and full `extent`.
- The process-provider arm shall compare the item's capability kind and extent
  classification with the descriptor's advertised (kind, mode) pairs.
- For a bounded extent, the process-provider arm shall compare every explicit
  `extent.bounds[].kind` with manifest `domains`, including when the descriptor
  advertises `unbounded` only for the item's kind. If such an admitted
  unbounded-only descriptor omits `domains`, the arm shall treat it as covering
  no bound kind for this check. An empty `bounds` passes this check (QSpec
  FR-290-AC-13; FR-331-AC-22).
- When a bounded extent has an unadvertised `bounds[].kind`, the process arm
  shall settle it `unsupported`, warned, with
  `unsupported_projection`/`unsupported-requested-capability`, naming the
  offending domain kind and candidate (QSpec FR-290-AC-13).
- When a bounded extent passes that check, the process arm shall settle it
  `supported` if the descriptor advertises `bounded` for its kind; otherwise
  it shall settle it `unsupported`, warned, with
  `unsupported_projection`/`unsupported-requested-capability`.
- When an unbounded extent's descriptor advertises `unbounded` for its kind,
  the process arm shall settle it `supported` without a domain-kind check.
- When an unbounded extent's descriptor advertises only `bounded` for its
  kind, the process arm shall compare only the boundable
  `extent.domains[].kind` values (`collection`, `population`, `integer`,
  `recursive`) with manifest `domains`, before the advertised-mode decision.
  It shall skip the non-boundable kinds (`quantity`, `loop`, `infinite-trace`)
  and leave them to the advertised-mode table (QSL ADR-014 §4; QSpec
  FR-290-AC-13).
- When that comparison finds an unadvertised boundable kind, the process arm
  shall settle the unbounded item `unsupported`, warned, with
  `unsupported_projection`/`unsupported-requested-capability`, naming the
  offending domain kind and candidate, regardless of
  `finite_bound_available`.
- When every compared kind is advertised and the item's
  `finite_bound_available` is true, the process arm shall settle an unbounded
  item on a `bounded`-only descriptor `requires-bound`.
- When every compared kind is advertised and the item's
  `finite_bound_available` is false, the process arm shall settle an unbounded
  item on a `bounded`-only descriptor `unsupported`, warned, with
  `unsupported_projection`/`unbounded-extent`.
- QSL supplies admitted descriptors (QSpec FR-290 "Advertised mode";
  FR-331-AC-22). When the process arm receives an admitted bounded-capable
  descriptor, it shall use its manifest `domains` and supply no
  missing-`domains` fallback. QSL-654 owns the admit-side `invalid-domains`
  enforcement.
- The process-provider arm shall compare domain-kind membership only. It shall
  neither derive a kind from `DomainKey` nor compare a proof-bound numeric value
  with a manifest run-limit default. It shall read the requested item's `extent`.
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
lives"). The IR-633 origin slice did not add `BackendKind::Process`, change its
serialization or settle a process item; IR-629 owns those behavioral changes.

Measured present fact: `BackendKind::from_identity` (`src/routed/capability.rs`) returns
`Some(Kani)` for `kani` and `None` for every other identity. IR-629 keeps it as
the built-in lookup; the behavior above owns the origin-aware conversion.

Adding the variant breaks the public `BackendKind` enum for every exhaustive match outside this crate. The
quire-driver lane's pre-negotiation conversion is the paired adaptation (QSL ADR-029 Amendments, ADR-012
§7.2); this repository edits no other repository.

IR-629 follow-on decisions (the process-provider criteria remain planned; IR-633 implements none
of their arms):

1. QSL ADR-029 PV-4 requires the process arm to read advertised domains and
   (kind, mode) pairs from the manifest. QSpec FR-290-AC-13 owns the domain-kind
   check; QSL-654 owns admit-side enforcement. QSpec FR-331 defines manifest `bounds` as
   run-limit defaults, so they are not admission inputs. QSpec's required
   `ProofBound.kind` is merged; QSL-654 owns its producer and the PV-4 wording.
   The arm reads that kind instead of deriving one from a `DomainKey` (AC-12,
   AC-19 and AC-23).
2. QSL ADR-029 PV-1 and PV-4 make the descriptor's typed origin the classification input;
   `Process(BackendId)` carries the plugin identity as data and `from_identity` remains the
   built-in lookup. IR-629 owns that variant and its exhaustive dispatch (AC-11).
3. QSL ADR-029 PV-4 now assigns CG an empty process generation output and assigns execution,
   replay and terminal settlement to the driver. IR-629 owns the corresponding exhaustive arms;
   this slice adds none.
4. PV-4 names `Process(BackendId)` but does not state a serialized label for the data-bearing
   variant. IR-633 defines no serialization. `BackendKind::ALL` cannot enumerate
   dynamic process identities; CG enumerates those from descriptors.
5. PV-4 states the bounded and unbounded mode decisions from the process manifest. IR-629
   owns the concrete arm and tests (AC-12, AC-16 and AC-17); the Kani arm is unchanged here.
6. QSL ADR-029 PV-1 withdraws linked and process descriptors that conflict on one `BackendId`,
   regardless of registration order. Neither reaches CG as a candidate; CG does not resolve that
   conflict from identity text.

Every criterion below that needs the new variant, FR-019-AC-11 to AC-13 and AC-16 to AC-23, remains IR-629 work and
requires its exhaustive arms to compile. FR-019-AC-14 is a property of the return type.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-019-AC-1 | Settlement dispatches one arm per variant of the closed backend kind, and every variant reaches an arm that settles rather than falling through. | Test (TC-030) |
| FR-019-AC-2 | Every requested item receives exactly one of `supported`, `requires-bound`, `unsupported` and `invalid-request`, settled from its `candidates` and extent classification, and `candidates` is read from the request rather than computed here. | Test (TC-030) |
| FR-019-AC-3 | On the Kani arm, an unbounded extent against a `bounded`-only advertisement settles `requires-bound` when a finite bound is available, rather than `unsupported`; with no finite bound available it settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`; and it never settles `supported`. | Test (TC-030) |
| FR-019-AC-4 | An item with more than one candidate and no named backend settles `invalid-request` with `invalid_capability`/`ambiguous-backend`, naming every candidate in candidate order, identically under every registration order. | Test (TC-030) |
| FR-019-AC-10 | Only an item settled `supported` routes, and a registry repeating a backend identity routes nothing rather than choosing between its entries. | Test (TC-030) |
| FR-019-AC-7 | An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`, or is absent, is refused as `invalid_capability`/`unsupported-version` with none of its kinds read. | Test (TC-030) |
| FR-019-AC-8 | An item with an absent extent classification settles `invalid-request` with `invalid_capability`/`absent-extent`, and an absent or unknown kind settles before the candidate table is consulted. | Test (TC-030) |
| FR-019-AC-9 | A backend kind added without a negotiation arm does not compile: the dispatch is an exhaustive `match` over the closed kind with no catch-all arm. | Analysis |
| FR-019-AC-11 | PLANNED (IR-629). `BackendKind` has `Process(id)` beside `Kani`; `ALL` lists finite built-in kinds only, while CG enumerates process candidates from descriptors. CG's descriptor-to-kind conversion returns `Process(id)` exactly when a descriptor has process origin, including identity text `kani`. A named, registered process backend reaches that arm and receives one disposition; no process item disappears for being absent from `ALL`. | Test (TC-046) |
| FR-019-AC-12 | PLANNED (IR-629; non-empty bounded rows depend on QSL-654's producer and admit-side enforcement). A bounded process item with an uncovered `extent.bounds[].kind` settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming the offending domain kind and candidate (QSpec FR-290-AC-13). An item that passes the domain check, including one with empty `bounds`, settles `supported` exactly when its descriptor advertises `bounded` for its kind; otherwise it settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`. | Test (TC-046) |
| FR-019-AC-16 | PLANNED (IR-629). An unbounded process item settles `supported` when its descriptor advertises `unbounded` for its kind, regardless of whether any `extent.domains[].kind` belongs to manifest `domains`. | Test (TC-046) |
| FR-019-AC-17 | PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item settles `requires-bound` exactly when `finite_bound_available` is true and every boundable `extent.domains[].kind` is in manifest `domains` (QSpec FR-290-AC-13). | Test (TC-046) |
| FR-019-AC-18 | PLANNED (IR-629). Given the same process advertisements and item extent, changing only backend identity, manifest position, manifest run-limit defaults or ambient state leaves the disposition and cause unchanged apart from the backend named in the output. | Test (TC-046) |
| FR-019-AC-19 | PLANNED (IR-629; bounded row depends on QSL-654's producer). The process arm reads each explicit `extent.bounds[].kind` and never derives a domain kind from `DomainKey`; two otherwise equal bounded items with the same `DomainKey` and different explicit kinds can settle differently under one manifest `domains` set. | Test (TC-046) |
| FR-019-AC-20 | PLANNED (IR-629). CG's process descriptor retains the FR-331 manifest's advertised (kind, mode) pairs, `domains` and `bounds` beside the IR-633 `id` and `origin` projection. | Test (TC-046) |
| FR-019-AC-21 | PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item with any unadvertised boundable `extent.domains[].kind` settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming the offending domain kind and candidate, regardless of `finite_bound_available`; non-boundable kinds are not compared (QSpec FR-290-AC-13). | Test (TC-046) |
| FR-019-AC-22 | PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item with every boundable `extent.domains[].kind` advertised and the item's `finite_bound_available=false` settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`. CG reads that flag from the item and does not derive it from the domain kinds (QSpec FR-290-AC-13). | Test (TC-046) |
| FR-019-AC-23 | PLANNED (IR-629; bounded row depends on QSL-654). Changing only a bounded item's proof-bound numeric maximum leaves its process-provider `supported` or `unsupported` disposition and cause unchanged. | Test (TC-046) |
| FR-019-AC-13 | Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent executable settles identically to one with an ordinary identity, apart from the backend each names, and starts nothing, and a descriptor whose identity names an executable that records its own start leaves no record. A mutant arm that starts or resolves the identity as a process either changes the first disposition or leaves the record, and fails this. | Test (TC-046) |
| FR-019-AC-14 | The process-provider arm's return type is `Disposition`, which has no terminal-value, verification-result or artifact member, so settling returns none of them. A change that returns one does not compile against that type. | Analysis |
| FR-019-AC-15 | PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin` projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288, ADR-029 PV-1); layer R owns that vocabulary. The driver projects each descriptor from `Registry::descriptors()` once into CG's descriptor, copying `id` and advertised pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and `Process` to `Process`, with no wildcard or origin inference from identity, manifest or provider bytes or a side map. Two registry descriptors identical except for origin produce CG descriptors identical except for origin; a linked Kani descriptor remains `Linked`. The CG dependency graph adds no direct `qsl-route` edge and the FR-331 wire gains no origin field. | Test (TC-030) |
| FR-019-AC-24 | PLANNED (IR-338). Given QSL FR-335's `value-validity` claim rooted at an unbounded `s: Set<Int[0,9999]>`, an empty candidate registry settles `unsupported` with a warning naming `value-validity` and emits no artifact or Kani outcome; one linked Kani candidate advertising (`value-validity`, `bounded`) with `finite_bound_available=true` settles the unbounded item `requires-bound` without substituting a bound or emitting an artifact; a separately requested bounded `ProofBound::Cardinality{maximum: 8}` item with that same sole candidate settles `supported` under its own request index, while the unbounded item remains `requires-bound`; and an unboundable added root with `finite_bound_available=false` against that candidate settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`. The bounded disposition alone claims no Kani execution outcome. | Test |

| FR-019-AC-25 | At the single Kani candidate arm, an admitted bounded typed IR form continues through the advertised-mode rules; a typed IR form outside Kani's capability settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming that form and Kani, before any generation, and never by inspecting display/debug text. This form refusal remains distinct from empty candidates (`unsupported-requested-capability` naming the missing capability) and ambiguous candidates (`invalid_capability`/`ambiguous-backend`). For an admitted unbounded form on bounded-only Kani, `finite_bound_available=true` settles `requires-bound` with no substituted bound or artifact; when the caller makes a new bounded request, its exact supplied bound/domain identity is preserved and the bounded disposition is `supported`. | Test (TC-030) |

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), the
  `quire-spec-language` registry that computes `candidates`, and QSL-654's
  QSpec FR-290-AC-13 domain-kind rule and FR-331/schema addition of required
  `ProofBound.kind` (merged); QSL-654 owns the producer, admit-side
  `invalid-domains` enforcement, and ADR-029 PV-4 reword of the domain-check
  scope and bounded admission rule.
- **Downstream**: [TC-030](../matrix/TC-030-capability-settlement.md),
  [TC-046](../matrix/TC-046-process-provider-settlement.md); quire-driver
  IR-609's pre-negotiation conversion and adaptation to the new variant;
  [FR-022](./FR-022-routed-generation.md), the generation arm of the same seam.
- **Boundary**: QSL FR-335/TC-845 owns the source-to-claim fixture and request writer;
  quire-integration owns the composed CG and driver run. IR FR-036 owns the absent-capability
  warning and IR FR-029 owns unsupported entries without artifact or Kani outcome.

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

- The FR-331 envelope: its contract version, its capability vocabulary, the
  registered backend descriptors, and the request items.
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

- The generator shall give the closed backend kind one variant for process providers, beside `Kani`:
  `BackendKind::Process(BackendId)`, which carries the plugin's backend identity as data (QSL-637 Q2).
  The identity is a value inside the variant, never a new kind.
- `BackendKind::from_identity` shall keep mapping each built-in static identity to its own kind
  (QSL-637 Q2).
- `BackendKind::from_identity` shall map every other identity to `None`, and shall never return the
  process-provider variant (QSL-637 Q2).
- Each `BackendDescriptor` shall carry a typed provider origin with exactly two values, `Linked` and
  `Process`, which this crate reads and never derives (QSL-637 amendment, option (b); see "How CG learns
  that a descriptor is a process provider" below).
- The generator shall classify a descriptor to a `BackendKind` from its origin and its identity only: origin
  `Process` gives `Process(identity)`, and origin `Linked` gives `from_identity(identity)`, whose `None` is
  `unknown-backend`.
- The generator shall never classify a descriptor as `Process` from an unknown identity, from the text of
  an identity, or from anything but its origin.
- The generator shall use that one classification in `negotiate_single_candidate` and in
  `unroutable_named_backend` (which finds the named backend's descriptor in the manifest), and nowhere
  else.
- Neither function shall call `from_identity` on a descriptor's identity other than through that
  classification.
- The generator shall settle an item whose single candidate classifies to `Process(id)` in that variant's
  `negotiate_*` arm.
- The generator shall settle such an item in no other place.
- The process-provider arm shall settle an item from the candidate's descriptor in the envelope manifest
  and the item's extent classification alone. The descriptor carries the manifest's advertised (kind,
  mode) pairs, domains and bounds (QSL-637 Q1).
- If the item's domain is not one the descriptor advertises for the item's kind, then the
  process-provider arm shall settle the item `unsupported`, warned, with the cause QSpec FR-290 gives for
  an unadvertised domain (QSL-637 Q1; open question 7 for the inputs).
- If the descriptor's advertised bound for the item's kind does not cover the item's bound, then the
  process-provider arm shall settle the item `unsupported`, warned, with the cause QSpec FR-290 gives for
  an uncovered bound (QSL-637 Q1; open question 7 for the inputs).
- Where the item's extent is `bounded`, the process-provider arm shall settle the item `supported` only
  when the descriptor advertises `bounded` for the item's kind and its advertised bound covers the
  item's bound (QSL-637 Q5).
- Where the item's extent is `unbounded`, the process-provider arm shall settle the item `supported`
  only when the descriptor advertises `unbounded` for the item's kind (QSL-637 Q5).
- If an unbounded extent meets a descriptor advertising `bounded` only for the kind, then the
  process-provider arm shall settle the item `requires-bound` when a finite bound is available and
  otherwise `unsupported`, warned, `unsupported_projection`/`unbounded-extent` (FR-290's rows; QSL-637 Q5).
- If a bounded extent meets a descriptor advertising `unbounded` only for the kind, then the
  process-provider arm shall settle the item `unsupported`, warned,
  `unsupported_projection`/`unsupported-requested-capability` (this requirement's reading of "declines
  under FR-290", QSL-637 Q5).
- The process-provider arm shall have no default disposition (QSL-637 Q5).
- The process-provider arm shall never settle an unbounded extent `supported` against a descriptor that
  advertises `bounded` only for the item's kind.
- The process-provider arm shall never narrow an extent.
- The process-provider arm shall never call, start, open or otherwise reach the plugin that
  descriptor stands for.
- The process-provider arm shall return a `Disposition` and nothing else.
- The generator shall never return a terminal value, a verification result or an artifact from settling.
- Every other `match` over `BackendKind` in this crate (generation, adapter, execution evidence and
  terminal-record map, ADR-002 Q4) shall have an arm for the process-provider variant that is a typed
  pass-through or an empty output (QSL-637 Q3).
- No such arm shall panic, call `unreachable!`, `todo!` or `unimplemented!`, or fall into a wildcard arm
  (QSL-637 Q3).

Measured present fact, not a requirement of this change: `BackendKind::from_identity` (`src/routed/capability.rs`)
returns `Some(Kani)` for `kani` and `None` for every other identity. `BackendDescriptor` holds only
`identity` and `advertised`; `RequestItem` holds `kind`, `extent`, `named_backend` and `candidates`;
`negotiate_single_candidate` settles a candidate whose identity `from_identity` cannot map
`invalid-request`/`unknown-backend`, and `unroutable_named_backend` flags a named backend the same way, so
nothing in the present input says which descriptor is a process provider. The requirement above is the
change that gives it that input; `from_identity` stays as measured and is no longer what those two
functions ask about a descriptor.
`BackendKind` is today a `Copy` enum of unit variants with a `const` `ALL`, `index` and `identity`; a
variant that carries a `BackendId` changes that surface (the code change's concern). `ALL` remains the list
of the built-in kinds, those that have a static identity; the closed-kind guarantee is the exhaustive
`match` of FR-019-AC-9, which does not depend on `ALL`. The three things that read `ALL` or `index` today
(the grouping loop of `generate_routed`, the order of `RoutedGeneration.rejected`, and the sort key) are
stated for the variant in FR-022's process-provider section.

### How CG learns that a descriptor is a process provider

Nothing in the descriptor says so today (measured, above), `BackendKind::from_identity` cannot (a plugin's
identity is arbitrary), and CG must not guess. QSL-637's amendment of 2026-10-05, a ticket comment by the QSL
planner (cited as data, and not yet in QSL `origin/main`, where `qsl_route`'s descriptor has no such
member), rules option (b): `qsl_route::BackendDescriptor` gains a typed `ProviderOrigin` with the values
`Linked` and `Process`; QSL's registry builder sets it at the one PV-1 conversion (`Linked` for a
compile-time `Provider`, `Process` for a plugin `hello`); QSL owns the type and the driver carries the value
into the descriptor it hands this crate unchanged; two registrations of one identity with different origins
differ in a member and so conflict under the registry's existing duplicate-backend rule, both withdrawn. It
rejected a driver-supplied map beside the descriptors (a second source of truth that can disagree with the
registry) and a manifest member (a plugin must not be able to claim to be linked).

This requirement states this crate's side of that contract and places no obligation on QSL or the driver:
the origin arrives on the descriptor; this crate maps origin `Process` to `Process(identity)` in its own
settlement; a `Linked` descriptor with an unknown identity is still `unknown-backend`; and this crate infers
nothing, and the origin is the one classification path. `Process("kani")` can reach neither negotiation nor generation under the registry's rules above
(both `kani` registrations are withdrawn on conflict); FR-022 refuses it if it ever arrives, as a
fail-closed check and not as `kani` conflict logic.

Adding the variant breaks the public `BackendKind` enum for every exhaustive match outside this crate, and
the origin member changes the public `BackendDescriptor`. The quire-driver lane's construction of the
descriptor, carrying QSL's origin into it, is the paired adaptation (QSL ADR-029 Amendments, ADR-012
§7.2); this repository edits no other repository.

### What QSL decided (QSL-637), and the contract this crate relies on

QSL-637 is the QSL planner's ruling of 2026-10-05 on open questions 1 to 6 of the IR-629 specification.
Its text is a ticket, so it is cited as data. The amendment of QSL ADR-029 PV-4 that it assigns to
itself was not on QSL `origin/main` when this was written: PV-4 there still reads as in the IR-629
specification, which agrees with each ruling below and does not state them. The rulings are measured
against what is merged: QSL ADR-029 PV-1 to PV-4 and PL-4, PL-5 and PL-7; QSpec FR-290 "Advertised mode"
and "Candidate set and negotiation"; QSL FR-075-AC-2.

1. Domains and bounds (answered). The arm checks the advertised (kind, mode) pairs, domains and bounds
   against the item's extent classification. CG's descriptor for the variant carries the manifest's
   domains and bounds, from the FR-331 manifest the `BackendDescriptor` came from. An item whose domain or
   bound the manifest does not advertise declines under FR-290, which this requirement settles
   `unsupported`, warned (the one non-`supported` disposition for a capability the registry does not
   give; FR-290-AC-4). QSpec FR-290 has no cause for it at its present text (measured): QSL-637
   assigns the cause to QSpec FR-290, and the criterion that names it stays planned until that cause
   exists (FR-019-AC-12).
2. The variant (answered, with its amendment). `BackendKind::Process(BackendId)`. `from_identity` is
   unchanged for the built-ins. The ticket's first text had the driver's pre-negotiation conversion build
   `Process(id)`; its later amendment (above, "How CG learns...") moves the fact to a typed origin on the
   descriptor, set by QSL's registry builder at the PV-1 conversion (QSL ADR-029 PV-1, PV-4, PL-5), and CG's
   settlement maps origin `Process` to `Process(id)`. This repository relies on that and places no
   obligation on QSL or the driver. Not answered: the variant's serialized label. `BackendKind`
   serializes kebab-case today; a variant carrying a value has a form QSL-637 does not state. The
   label stays open, and no criterion states it.
3. The variant's other arms (answered). Only negotiation is this crate's for a process provider.
   Generation yields `KindOutput::Process` with no artifact, because the plugin receives the v2 package
   bytes and not generated code (QSL ADR-029 PL-4, PL-7). Adapter, execution and terminal map are the
   driver's plugin host and its typed FR-331 reader: the host settles per PL-7, `refuted` only through
   S6a replay, and a `proved` result keeps the basis label `trusted`. This crate's arms for the variant
   are typed pass-throughs or empty outputs, never a panic or `unreachable!` (FR-019-AC-16,
   FR-022-AC-17).
5. Bounded and unbounded (answered). The disposition comes from the manifest alone. A bounded item
   routes when the manifest advertises the bounded mode with a covering bound. An unbounded item routes
   only when the manifest advertises the unbounded mode. Otherwise it declines under FR-290. There is no
   default (FR-019-AC-15). The mapping of "declines" to FR-290's rows in the Behavior list above is this
   requirement's reading, not a ruling of QSL-637.
6. A plugin that declares `kani` (answered; a QSL contract, not an obligation here). Neither
   wins. `qsl_route::Registry` is keyed by `BackendId`. A plugin manifest declaring `kani` that differs
   from the built-in manifest conflicts, and so does one whose origin differs from the built-in's (the
   origin is a member of the descriptor: QSL-637's amendment): both are withdrawn and `kani` stays
   unregistered (FR-290-AC-9
   and AC-10, QSpec "Candidate set and negotiation", QSL FR-075-AC-2 and AC-4, ADR-029 PV-2 item 2). An
   item that names `kani` then receives the unknown-backend mark and settles `invalid-request`,
   `invalid_capability`/`unknown-backend` (FR-290-AC-10, FR-075-AC-4); an item that names no backend has an
   empty candidate set and settles `unsupported` (FR-290-AC-4). QSL-637's own words are "decline". A
   registration equal to the built-in one in every member, origin included, is idempotent. It is decided at registration, independent of order,
   and, per FR-075-AC-4, a later registration of a conflicted identity is refused on arrival. Measured in
   QSpec FR-290 and QSL FR-075 as stated. This crate relies on it and keeps no `kani` conflict logic of its
   own: a `Process("kani")` item cannot arrive, and FR-022-AC-17 refuses one as a fail-closed check if it
   does.

Still open, for the QSL owner:

- Open question 4 (the serialized label of the variant), above.
- The cause QSpec FR-290 gives to an unadvertised domain or an uncovered bound (Open question 1's
  remainder), until QSpec adds it.
- The origin is not yet in QSL: QSL-637's amendment is a ticket comment, and `qsl_route`'s descriptor and
  QSL ADR-029 PV-1 and PV-4 do not carry it at the time of writing. GATE: FR-019-AC-11, AC-12, AC-13,
  AC-15 and AC-17 stay planned, and this crate's code for them waits, until the QSL change that adds
  `ProviderOrigin` to `qsl_route::BackendDescriptor` (with its PV-1 and PV-4 and FR-288 text) has merged
  and this crate's dependency on QSL has moved to a revision that contains it. The merge revision is not
  yet known.
- Open question 7, for the QSL and QSpec owners: what carries the item's domain and bound, and the
  descriptor's advertised domains and bounds, to this crate, and what "covers" means. Measured: QSpec FR-331
  puts `domains` and `bounds` in the manifest descriptor and "domains and limits" and the #222 extent
  classification on the request, but not per item; this crate's `RequestItem` holds an
  `ExtentClassification` of a mode and a `finite_bound_available` flag only, and `BackendDescriptor` holds
  no domain or bound; and neither QSL-637 nor QSpec FR-290 defines a covering predicate. Options: (a) QSL
  computes, per (item, candidate), whether the item's domain is advertised and its bound covered, and this
  crate reads two flags beside `finite_bound_available`, as it reads that one and computes nothing; (b) QSL
  supplies the item's domain and bound values and the descriptor's advertised ones in #222's representation,
  and this crate compares them with a predicate #222 or FR-290 states (for example interval containment
  for a numeric bound). This specification does not choose. Until it is answered, the domain and bound
  clauses of FR-019-AC-12 and the covering clause of FR-019-AC-15 name no input and stay planned, and no
  criterion states what "covers" is.

Every criterion below that needs the new variant, FR-019-AC-11 to AC-13, AC-15 and AC-16, also needs the
variant's arms in every exhaustive match to compile (FR-019-AC-16). FR-019-AC-14 is a property of the
return type.

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
| FR-019-AC-11 | PLANNED (IR-629; waits on the origin in QSL). `BackendKind::Process(id)` exists beside `Kani` and carries `id`: an item whose one candidate is a descriptor with origin `Process` and identity `id` reaches that variant's arm and settles there, naming `id` as its backend, and two such items with different ids name different backends. `from_identity("kani")` is `Some(Kani)` and `from_identity` of any other identity, a plugin's included, is `None`. A mutant `from_identity` that returns `Process` for an unknown identity fails the `None` check; a variant that drops `id`, or an arm that names a fixed identity, fails the two-ids check. | Test (TC-046) |
| FR-019-AC-17 | PLANNED (IR-629; waits on the origin in QSL). The one classification of a descriptor reads its origin and nothing else: a descriptor with origin `Process` and an identity no built-in kind has settles in the process-provider arm, as a single candidate and as the named backend; a descriptor with origin `Linked` and the same identity settles `invalid-request`, `invalid_capability`/`unknown-backend`, naming the identity, as a single candidate and as the named backend; a `Linked` descriptor whose identity reads like a plugin (a path, an executable name) is still `unknown-backend`; and a `Linked` descriptor with identity `kani` settles in the Kani arm. A mutant that infers `Process` from an unknown identity fails the `Linked` rows; one that ignores the origin fails the first two rows; one that classifies the single candidate by origin but the named backend by `from_identity` fails the named-backend rows. | Test (TC-046) |
| FR-019-AC-12 | PLANNED (IR-629; waits on the origin in QSL, and its domain and bound clauses on open question 7 and on the QSpec FR-290 cause). The process-provider arm's disposition is a function of the descriptor's advertised (kind, mode) pairs, domains and bounds and the item's extent classification, domain and bound alone: two descriptors with equal pairs, domains and bounds and different identity text, manifest position or ambient state settle identically apart from the backend each names, which is the descriptor's own identity (Outputs). An item whose domain is not advertised for its kind, and an item whose bound the advertised bound does not cover (as open question 7 defines the inputs and the predicate), each settle `unsupported`, warned, with the FR-290 cause for it, and the same item against a descriptor that advertises them does not. A mutant arm whose disposition or cause depends on the identity text beyond echoing it as the named backend fails the first; one that ignores the domains, or one that ignores the bounds, settles the unadvertised case `supported` and fails the second. | Test (TC-046) |
| FR-019-AC-13 | PLANNED (IR-629). Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent executable settles identically to one with an ordinary identity, apart from the backend each names, and starts nothing, and a descriptor whose identity names an executable that records its own start leaves no record. A mutant arm that starts or resolves the identity as a process either changes the first disposition or leaves the record, and fails this. | Test (TC-046) |
| FR-019-AC-14 | PLANNED (IR-629). The process-provider arm's return type is `Disposition`, which has no terminal-value, verification-result or artifact member, so settling returns none of them. A change that returns one does not compile against that type. | Analysis |
| FR-019-AC-15 | PLANNED (IR-629; waits on the origin in QSL, and the covering-bound clause on open question 7). The process-provider arm settles bounded and unbounded items from the descriptor alone, with no default: a bounded item against a descriptor advertising `bounded` for its kind with a covering bound (open question 7) settles `supported`; a bounded item against `unbounded` only settles `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability`; an unbounded item against a descriptor advertising `unbounded` settles `supported`; an unbounded item against `bounded` only settles `requires-bound` when a finite bound is available and otherwise `unsupported`, warned, `unsupported_projection`/`unbounded-extent`, and never `supported`. A mutant arm whose unmatched rows default to `supported` fails the second and fourth rows; one that settles a bounded item against `unbounded` only `supported` fails the second; one that narrows the unbounded extent to reach `supported` fails the fourth. | Test (TC-046) |
| FR-019-AC-16 | PLANNED (IR-629). Every `match` over `BackendKind` in this crate has an arm for the process-provider variant that is a typed pass-through or an empty output: a source scan of the crate finds in no such arm a wildcard, `panic!`, `unreachable!`, `todo!` or `unimplemented!`, and a call of each public entry that matches on the kind (settlement, routed generation) with a `Process(id)` item returns without panicking. A mutant arm that calls `unreachable!` or `panic!` fails the call; one that falls into a wildcard fails the scan and, as FR-019-AC-9 states, the exhaustive-match compile check. | Test (TC-046) |

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and the
  `quire-spec-language` registry that computes `candidates`; QSL-637 (the rulings above) and the QSL
  ADR-029 PV-4 amendment it assigns; QSpec FR-290, which owes the unadvertised domain and bound cause.
- **Downstream**: [TC-030](../matrix/TC-030-capability-settlement.md),
  [TC-046](../matrix/TC-046-process-provider-settlement.md); quire-driver's construction of the
  descriptor with QSL's origin, the paired adaptation to the new variant, to be added to quire-driver
  IR-609 (which does not mention it today) or to a new driver ticket by the driver lane at merge;
  [FR-022](./FR-022-routed-generation.md), the generation arm of the same seam, which states the
  process-provider generation arm (FR-022-AC-17).

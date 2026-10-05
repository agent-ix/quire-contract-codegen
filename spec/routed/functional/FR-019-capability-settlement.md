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
- `BackendKind::from_identity` shall keep mapping each built-in static identity to its own kind and
  every other identity to `None`; it shall never return the process-provider variant (QSL-637 Q2).
- The generator shall settle an item whose candidate converts to `Process(id)` in that variant's
  `negotiate_*` arm.
- The generator shall settle such an item in no other place.
- The process-provider arm shall settle an item from the candidate's descriptor in the envelope manifest
  and the item's extent classification alone. The descriptor carries the manifest's advertised (kind,
  mode) pairs, domains and bounds (QSL-637 Q1).
- If the item's domain, or the item's bound, is not one the descriptor advertises for the item's kind,
  then the process-provider arm shall settle the item `unsupported`, warned, with the cause QSpec FR-290
  gives for an unadvertised domain or bound (QSL-637 Q1).
- Where the item's extent is `bounded`, the process-provider arm shall settle the item `supported` only
  when the descriptor advertises `bounded` for the item's kind with a bound that covers the item's
  bound; where the item's extent is `unbounded`, only when the descriptor advertises `unbounded` for the
  item's kind (QSL-637 Q5).
- If neither condition holds, then the process-provider arm shall settle the item by FR-290's rows,
  never by a default: an unbounded extent against `bounded` only settles `requires-bound` when a finite
  bound is available and otherwise `unsupported`, warned, `unsupported_projection`/`unbounded-extent`;
  a bounded extent against `unbounded` only settles `unsupported`, warned,
  `unsupported_projection`/`unsupported-requested-capability` (QSL-637 Q5).
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
  pass-through or an empty output, and no such arm shall panic, call `unreachable!`, `todo!` or
  `unimplemented!`, or fall into a wildcard arm (QSL-637 Q3).

Measured present fact, not a requirement of this change: `BackendKind::from_identity` (`src/routed/capability.rs`)
returns `Some(Kani)` for `kani` and `None` for every other identity, and CG's own calls to it
(`unroutable_named_backend` and `negotiate_single_candidate` in `capability.rs`, and the
`BackendKindDisagrees` check in `generate.rs`) rely on that. The requirement above keeps that mapping.
`BackendKind` is today a `Copy` enum of unit variants with a `const` `ALL`, `index` and `identity`; a
variant that carries a `BackendId` changes that surface (the code change's concern, with FR-022's
`BackendKindDisagrees` rule, which compares a routed kind with `from_identity`: see FR-022's
process-provider section). `ALL` remains the list of the built-in kinds, those that have a static
identity; the closed-kind guarantee is the exhaustive `match` of FR-019-AC-9, which does not depend on
`ALL`.

Adding the variant breaks the public `BackendKind` enum for every exhaustive match outside this crate. The
quire-driver lane's pre-negotiation conversion is the paired adaptation (QSL ADR-029 Amendments, ADR-012
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
2. The variant (answered). `BackendKind::Process(BackendId)`. `from_identity` is unchanged for the
   built-ins. The driver's pre-negotiation conversion builds `Process(id)` for every descriptor that came
   from a plugin `hello` (QSL ADR-029 PV-1, PV-4, PL-5). This repository relies on that conversion and
   places no obligation on the driver. Not answered: the variant's serialized label. `BackendKind`
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
6. A plugin that declares `kani` (answered; a QSL and driver contract, not an obligation here). Neither
   wins. `qsl_route::Registry` is keyed by `BackendId`. A plugin manifest declaring `kani` that differs
   from the built-in manifest conflicts: both are withdrawn, `kani` stays unregistered, and its items
   have no candidate and settle `unsupported` (FR-290-AC-9 and AC-10, QSpec "Candidate set and
   negotiation", QSL FR-075-AC-2, ADR-029 PV-2 item 2). A manifest equal to the built-in one is
   idempotent. It is decided at registration, independent of order. Measured in QSpec FR-290 and QSL
   FR-075 as stated. This crate relies on it: `from_identity("kani")` is `Some(Kani)`, so a routed
   `Process("kani")` is never valid (FR-022-AC-17), and this crate keeps no `kani` conflict logic of its
   own.

Still open, for the QSL owner:

- Open question 4 (the serialized label of the variant), above.
- The cause QSpec FR-290 gives to an unadvertised domain or bound (Open question 1's remainder), until
  QSpec adds it.

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
| FR-019-AC-11 | PLANNED (IR-629). `BackendKind::Process(id)` exists beside `Kani` and carries `id`: an item whose one candidate converts to `Process(id)` reaches that variant's arm and settles there, naming `id` as its backend, and two items with different ids name different backends. `from_identity("kani")` is `Some(Kani)` and `from_identity` of any other identity, a plugin's included, is `None`. A mutant `from_identity` that returns `Process` for an unknown identity fails the `None` check; a variant that drops `id`, or an arm that names a fixed identity, fails the two-ids check; an arm that treats `Process("kani")` as `Kani` settles it as the Kani arm would and fails the comparison against the process arm's own disposition. | Test (TC-046) |
| FR-019-AC-12 | PLANNED (IR-629; the unadvertised-domain and bound cause also waits on QSpec FR-290). The process-provider arm's disposition is a function of the descriptor's advertised (kind, mode) pairs, domains and bounds and the extent classification alone: two descriptors with equal pairs, domains and bounds and different identity text, manifest position or ambient state settle identically apart from the backend each names, which is the descriptor's own identity (Outputs). An item whose domain is not advertised for its kind, and an item whose bound the advertised bound does not cover, each settle `unsupported`, warned, with the FR-290 cause for it, and the same item against a descriptor that advertises them does not. A mutant arm whose disposition or cause depends on the identity text beyond echoing it as the named backend fails the first; one that ignores the domains, or one that ignores the bounds, settles the unadvertised case `supported` and fails the second. | Test (TC-046) |
| FR-019-AC-13 | PLANNED (IR-629). Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent executable settles identically to one with an ordinary identity, apart from the backend each names, and starts nothing, and a descriptor whose identity names an executable that records its own start leaves no record. A mutant arm that starts or resolves the identity as a process either changes the first disposition or leaves the record, and fails this. | Test (TC-046) |
| FR-019-AC-14 | PLANNED (IR-629). The process-provider arm's return type is `Disposition`, which has no terminal-value, verification-result or artifact member, so settling returns none of them. A change that returns one does not compile against that type. | Analysis |
| FR-019-AC-15 | PLANNED (IR-629). The process-provider arm settles bounded and unbounded items from the descriptor alone, with no default: a bounded item against a descriptor advertising `bounded` for its kind with a covering bound settles `supported`; a bounded item against `unbounded` only settles `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability`; an unbounded item against a descriptor advertising `unbounded` settles `supported`; an unbounded item against `bounded` only settles `requires-bound` when a finite bound is available and otherwise `unsupported`, warned, `unsupported_projection`/`unbounded-extent`, and never `supported`. A mutant arm whose unmatched rows default to `supported` fails the second and fourth rows; one that settles a bounded item against `unbounded` only `supported` fails the second; one that narrows the unbounded extent to reach `supported` fails the fourth. | Test (TC-046) |
| FR-019-AC-16 | PLANNED (IR-629). Every `match` over `BackendKind` in this crate has an arm for the process-provider variant that is a typed pass-through or an empty output: a source scan of the crate finds in no such arm a wildcard, `panic!`, `unreachable!`, `todo!` or `unimplemented!`, and a call of each public entry that matches on the kind (settlement, routed generation) with a `Process(id)` item returns without panicking. A mutant arm that calls `unreachable!` or `panic!` fails the call; one that falls into a wildcard fails the scan and, as FR-019-AC-9 states, the exhaustive-match compile check. | Test (TC-046) |

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and the
  `quire-spec-language` registry that computes `candidates`; QSL-637 (the rulings above) and the QSL
  ADR-029 PV-4 amendment it assigns; QSpec FR-290, which owes the unadvertised domain and bound cause.
- **Downstream**: [TC-030](../matrix/TC-030-capability-settlement.md),
  [TC-046](../matrix/TC-046-process-provider-settlement.md); quire-driver's pre-negotiation
  conversion, the paired adaptation to the new variant, to be added to quire-driver IR-609 (which does
  not mention it today) or to a new driver ticket by the driver lane at merge;
  [FR-022](./FR-022-routed-generation.md), the generation arm of the same seam, which states the
  process-provider generation arm (FR-022-AC-17).

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

Measured present fact, not a requirement of this change: `BackendKind::from_identity` (`src/routed/capability.rs`)
returns `Some(Kani)` for `kani` and `None` for every other identity, and CG's own calls to it
(`unroutable_named_backend` and `negotiate_single_candidate` in `capability.rs`, and the
`BackendKindDisagrees` check in `generate.rs`) rely on that. This specification changes none of it.
Every change to `from_identity`, and the mapping from a plugin `BackendId` to the process-provider
variant, is left to open questions 2 and 6.

Adding the variant breaks the public `BackendKind` enum for every exhaustive match outside this crate. The
quire-driver lane's pre-negotiation conversion is the paired adaptation (QSL ADR-029 Amendments, ADR-012
§7.2); this repository edits no other repository.

Open questions for the QSL owner (PV-4 does not settle them, and the criteria that depend on each stay planned):

1. PV-4 says the arm settles "against the item's extent classification, under QSpec FR-290's rules" from
   "the advertised (kind, mode) pairs, domains and bounds". `BackendDescriptor` carries only (kind, mode)
   pairs, and no FR-290 cause names a domain or bound mismatch. Does the manifest carry domains and
   bounds to CG, and if so which disposition and cause does a mismatch settle? Until answered the arm
   reads (kind, mode) pairs only (FR-019-AC-12).
2. PV-4 says "The driver's pre-negotiation conversion maps every plugin `BackendId` to that variant".
   `BackendKind::from_identity` resolves an identity string through each variant's one static
   `identity()`, which a plugin's arbitrary `BackendId` cannot equal. Does the variant carry the
   `BackendId`, does the envelope carry the kind beside each descriptor, or does `from_identity` change?
   Until answered no criterion states how a candidate reaches the arm (FR-019-AC-11).
3. PV-4 states "its arm settles" and says nothing of the variant's generation arm (FR-022), adapter
   (FR-026), execution evidence or terminal-record map (FR-029). ADR-002 Q4 makes each match a compile
   error until it has an arm, so each needs a stated arm. What do they do for a process provider?
4. PV-4 does not name the variant or its serialized label. The label is public (`BackendKind`
   serializes kebab-case).
5. PV-4 says the arm settles "under QSpec FR-290's rules", and FR-290's advertised-mode table
   answers the rows (`bounded` against an advertisement that includes `bounded`, `bounded` against
   `unbounded` only, `unbounded` against one that includes `unbounded`) with "the negotiation arm's own
   disposition for the IR form". PV-4 does not state that disposition for the process-provider arm, so
   this requirement states none of those rows for it. What does the arm settle for each? The Kani
   arm's disposition for them is not assumed to carry over.
6. PV-4 says the driver "maps every plugin `BackendId` to that variant", and a plugin may declare the
   `BackendId` `kani`, which `BackendKind::from_identity` resolves to `Kani`. Which wins for such a
   plugin, and where is that enforced, in the driver's conversion or in CG? No criterion states it.

Every criterion below that needs the new variant, FR-019-AC-11 to AC-13, also waits on open question 3 to
compile, since every exhaustive match needs an arm for it. FR-019-AC-14 is a property of the return type.

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
| FR-019-AC-11 | `BackendKind` has the process-provider variant beside `Kani`, `BackendKind::ALL` lists both with `index` returning each position, and an item whose one candidate is a process-provider backend reaches that variant's arm and settles there. A mutant that leaves the variant out of `ALL` fails this. How a candidate reaches the arm waits on open question 2. | Test (TC-046) |
| FR-019-AC-12 | The process-provider arm's disposition is a function of the descriptor's advertised (kind, mode) pairs and the extent classification alone: two descriptors with equal pairs and different identity text, manifest position or ambient state settle identically apart from the backend each names, which is the descriptor's own identity (Outputs), and an unbounded extent against `bounded`-only never settles `supported`. A mutant arm whose disposition or cause depends on the identity text beyond echoing it as the named backend, or that settles that unbounded extent `supported`, fails this. The rows PV-4 leaves to the arm wait on open question 5, and domains and bounds on open question 1. | Test (TC-046) |
| FR-019-AC-13 | Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent executable settles identically to one with an ordinary identity, apart from the backend each names, and starts nothing, and a descriptor whose identity names an executable that records its own start leaves no record. A mutant arm that starts or resolves the identity as a process either changes the first disposition or leaves the record, and fails this. | Test (TC-046) |
| FR-019-AC-14 | The process-provider arm's return type is `Disposition`, which has no terminal-value, verification-result or artifact member, so settling returns none of them. A change that returns one does not compile against that type. | Analysis |

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and the
  `quire-spec-language` registry that computes `candidates`.
- **Downstream**: [TC-030](../matrix/TC-030-capability-settlement.md),
  [TC-046](../matrix/TC-046-process-provider-settlement.md); quire-driver's pre-negotiation
  conversion, the paired adaptation to the new variant, to be added to quire-driver IR-609 (which does
  not mention it today) or to a new driver ticket by the driver lane at merge;
  [FR-022](./FR-022-routed-generation.md), the generation arm of the same seam.

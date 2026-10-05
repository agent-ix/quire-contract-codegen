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

- The generator shall give the closed backend kind one variant for process providers, beside `Kani`,
  and shall list it in `BackendKind::ALL`.
- The generator shall settle an item whose candidate is a process-provider backend in that variant's
  `negotiate_*` arm, and in no other place.
- The process-provider arm shall settle an item from the candidate's descriptor in the envelope manifest
  and the item's extent classification alone.
- The process-provider arm shall apply the advertised-mode rules above to that descriptor's advertised
  (kind, mode) pairs, with the same dispositions and causes as every other arm.
- The process-provider arm shall never call, start, open or otherwise reach the plugin that
  descriptor stands for.
- The process-provider arm shall return a negotiation disposition only, and shall never return a
  terminal value, a verification result or an artifact.
- The generator shall leave the mapping from a plugin `BackendId` to the process-provider variant to
  the driver's pre-negotiation conversion, and shall do no plugin discovery of its own.

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
| FR-019-AC-11 | `BackendKind` has the process-provider variant beside `Kani`, `BackendKind::ALL` lists both with `index` returning each position, and an item whose one candidate is a process-provider backend reaches that variant's arm and settles there. Planned: how a candidate reaches the arm waits on open question 2. | Test (TC-046) |
| FR-019-AC-12 | The process-provider arm settles from the descriptor's advertised (kind, mode) pairs and the extent classification alone: a bounded extent settles `supported`; an unbounded extent settles `supported` against an advertised `unbounded`, `requires-bound` against `bounded`-only with a finite bound available, and `unsupported` with `unsupported_projection`/`unbounded-extent` against `bounded`-only with none; it never narrows an extent. A mutant that settles an unbounded extent `supported` against `bounded`-only, or that settles `requires-bound` without a finite bound, fails this. Domains and bounds wait on open question 1. | Test (TC-046) |
| FR-019-AC-13 | Settling a process-provider item calls no plugin: a descriptor whose identity names an executable that records its own start leaves no record, and the arm takes no argument from which a plugin can be reached. A mutant arm that starts the descriptor's identity as a process leaves the record and fails this. | Test (TC-046) |
| FR-019-AC-14 | A process-provider item settled `supported` is the disposition `supported` naming the backend and nothing more: no terminal value, no verification result and no artifact comes from settlement, and no non-`supported` process-provider settlement routes. A mutant arm that returns any other disposition for a `bounded` extent that the manifest advertises, or that routes an `unsupported` item, fails this. | Test (TC-046) |

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and the
  `quire-spec-language` registry that computes `candidates`.
- **Downstream**: [TC-030](../matrix/TC-030-capability-settlement.md),
  [TC-046](../matrix/TC-046-process-provider-settlement.md); quire-driver's pre-negotiation
  conversion (IR-609), the paired adaptation to the new variant, filed by the driver lane at merge;
  [FR-022](./FR-022-routed-generation.md), the generation arm of the same seam.

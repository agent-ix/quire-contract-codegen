---
id: TC-046
title: "Verify process-provider settlement and empty generation"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: verifies
---
# TC-046: Verify process-provider settlement and empty generation

## Description

Verify that `Process(BackendId)` settles from the FR-331 manifest's advertised
(kind, mode) pairs and `domains`, against the requested item's capability kind
and full `extent`.
Manifest `bounds` are run-limit defaults and do not set admission maxima.
Bounded `ProofBound.kind` is required by QSpec FR-331; QSL-654 owns the
producer. QSpec FR-290-AC-13 owns the domain-kind check and per-registration
`invalid-domains` refusal. The test does not infer a kind from `DomainKey`.
Verify the empty generation arm and absence of plugin calls (QSL ADR-029
PV-4).

## Test Procedure

1. Inspect `BackendKind::ALL` and CG's descriptor-to-kind conversion. Settle
   one process-origin candidate with an arbitrary identity, then one whose
   identity text is `kani`. With two providers advertising the same kind, name
   the process backend and settle it again. Verify that every case reaches
   `Process(id)` even though `ALL` lists built-in kinds only. Construct routed
   items through `RoutedGenerationItem::from_descriptor`: compare a linked
   `kani` descriptor and a process descriptor also named `kani`; try a linked
   descriptor with an identity that has no built-in kind, and mismatched backend
   and descriptor identities. Inspect whether callers can set `backend` or
   `kind` directly after construction.
2. Settle the following single-candidate rows. Keep the requested capability
   kind fixed and vary only the listed manifest advertisement or item extent.

   | Item extent | Manifest advertisement | Expected disposition and cause |
   | --- | --- | --- |
   | bounded, `bounds` empty | `bounded` for kind | `supported` |
   | bounded, `bounds` empty | both modes for kind | `supported` |
   | bounded, `bounds` empty | `unbounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, `bounds[].kind` all in `domains` | `bounded` for kind | `supported` |
   | bounded, `bounds[].kind` all in `domains` | both modes for kind | `supported` |
   | bounded, one `bounds[].kind` absent from `domains` | `bounded` for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, one `bounds[].kind` absent from `domains` | both modes for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, covered domain | `unbounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, one `bounds[].kind` absent from `domains` | `unbounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | unbounded, `finite_bound_available=false` | `unbounded` for kind, with unrelated or omitted `domains` | `supported` |
   | unbounded, `finite_bound_available=true` | both modes for kind, with one `domains[].kind` unadvertised | `supported` |
   | unbounded, `finite_bound_available=true`, `integer` and `collection` domains | `bounded` only for kind; `domains` contains both | `requires-bound` |
   | unbounded, `finite_bound_available=false`, `integer` plus `quantity`, `loop`, or `infinite-trace` domain in separate cases | `bounded` only for kind; `domains` contains `integer` | `unsupported`, warned, `unsupported_projection`/`unbounded-extent`; non-boundable kind is not compared |
   | unbounded, `finite_bound_available=true`, `population` domain | `bounded` only for kind; `domains` lacks `population` | `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability` naming `population` |
   | unbounded, `finite_bound_available=false`, `population` and `quantity` domains | `bounded` only for kind; `domains` lacks `population` | `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability` naming `population`; domain check precedes mode row |

3. Repeat representative rows with the same advertisements and item extent but
   different backend identity text, manifest position, manifest `bounds` run
   defaults and ambient state. Check that CG's process descriptor retains
   the manifest's advertised pairs, `domains` and `bounds`. Vary the numeric maximum of a bounded item's
   proof bound while keeping its explicit `kind`; leave the domain coverage
   decision unchanged. Give the same `DomainKey` two distinct explicit domain
   kinds and verify that the explicit kind controls coverage.
   Supply only admitted descriptors: QSpec FR-290-AC-13 refuses a registration
   advertising `bounded` with absent `domains`, or any registration with empty,
   non-boundable or repeated `domains`, as per-registration
   `invalid_capability`/`invalid-domains`. CG does not receive that descriptor
   and does not substitute a `domains` fallback.
4. Use an identity that names a non-existent executable, then one that names
   an executable recording its own start. Settle both against equal
   advertisements and inspect for process starts. Route a supported
   `Process(id)` item to generation with no process context; inspect its output
   under a permutation of Kani and two process identities. In a CG internal
   test, construct or mutate an item whose private routed `Process(id)` kind
   disagrees with its private backend; an external caller cannot produce that
   mismatch through the public constructor.

## Expected Results

1. `ALL` contains finite built-in Kani; CG's conversion and the named-backend
   path reach `Process(id)` and receive one disposition (FR-019-AC-11).
   `from_descriptor` constructs Kani for linked `kani` and Process for process
   `kani`; a linked identity with no built-in kind refuses as
   `UnknownLinkedBackend`, and a backend/descriptor mismatch refuses as
   `DescriptorBackendMismatch` naming both. External callers cannot set
   `backend` or `kind` directly (FR-022-AC-20 to AC-22).
2. Every row has its stated disposition and cause. An unbounded item never
   settles `supported` on `bounded`-only advertisement; `quantity`, `loop`, and
   `infinite-trace` continue to the advertised-mode table (FR-019-AC-12,
   FR-019-AC-16, FR-019-AC-17, FR-019-AC-21, FR-019-AC-22).
3. Identity, position, run defaults and ambient state do not alter a
   disposition or cause apart from the backend named. Numeric maximum and
   `DomainKey` do not determine domain coverage; explicit extent kinds do
   (FR-019-AC-18, FR-019-AC-19, FR-019-AC-20, FR-019-AC-23).
4. No plugin starts or is resolved during settlement; the process item has
   one empty `KindOutput::Process` and no CG artifact or terminal record.
   Results are invariant under input permutation; an internal identity
   mismatch refuses the call with `BackendKindDisagrees` and
   `converted=Some(Process(item.backend.identity))` (FR-019-AC-13,
   FR-022-AC-17, FR-022-AC-18, FR-022-AC-19).

FR-019-AC-14 is verified by analysis of the arm's `Disposition` return type, not by a step here.

## Seeded mutants

| Mutant | Caught by |
|---|---|
| Enumerate process identities through static `ALL` and drop one | step 1 |
| Check mode but ignore an uncovered bounded domain | step 2 |
| Compare a non-boundable unbounded domain with manifest `domains` | step 2 |
| Let `finite_bound_available=false` mask an unadvertised `population` | step 2 |
| Derive a domain kind from `DomainKey` instead of reading the explicit kind | step 3 |
| Compare numeric bound maximum with manifest run defaults | step 3 |
| Settle unbounded on `bounded` only as `supported` | step 2 |
| Start the plugin while settling | step 4 |
| Drop a routed process item or require a process generation context | step 4 |
| Accept a routed `Process(id)` whose identity differs from its backend | step 4 |
| Let a linked descriptor named `kani` construct a Process item | step 1 |
| Accept a backend/descriptor identity mismatch in the public constructor | step 1 |

## Status

Planned (IR-629). QSpec FR-331 now requires `ProofBound.kind` in the wire schema;
QSL-654 still owns the producer and ADR-029 PV-4 wording. The process variant
and its arms are not implemented in this spec-only change.

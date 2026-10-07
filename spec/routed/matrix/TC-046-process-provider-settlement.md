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
Bounded `ProofBound.kind` is supplied by QSL-654; the test does not infer a kind
from `DomainKey`. Verify the empty generation arm and absence of plugin calls
(QSL ADR-029 PV-4; QSpec FR-290 and FR-331).

## Test Procedure

1. Inspect `BackendKind::ALL` and CG's descriptor-to-kind conversion. Settle
   one process-origin candidate with an arbitrary identity, then one whose
   identity text is `kani`. With two providers advertising the same kind, name
   the process backend and settle it again. Verify that every case reaches
   `Process(id)` even though `ALL` lists built-in kinds only.
2. Settle the following single-candidate rows. Keep the requested capability
   kind fixed and vary only the listed manifest advertisement or item extent.

   | Item extent | Manifest advertisement | Expected disposition and cause |
   | --- | --- | --- |
   | bounded, `bounds` empty | `bounded` for kind | `supported` |
   | bounded, `bounds` empty | `unbounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, `bounds[].kind` all in `domains` | `bounded` for kind | `supported` |
   | bounded, `bounds[].kind` all in `domains` | both modes for kind | `supported` |
   | bounded, one `bounds[].kind` absent from `domains` | `bounded` for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, covered domain | `unbounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | unbounded | `unbounded` for kind, with unrelated or omitted `domains` | `supported` |
   | unbounded | both modes for kind, with one `domains[].kind` unadvertised | `supported` |
   | unbounded, `finite_bound_available=true`, all `domains[].kind` advertised | `bounded` only for kind | `requires-bound` |
   | unbounded, `finite_bound_available=false`, all `domains[].kind` advertised | `bounded` only for kind | `unsupported`, `unsupported_projection`/`unbounded-extent` |
   | unbounded, `finite_bound_available=true`, one `domains[].kind` absent | `bounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | unbounded, `finite_bound_available=false`, one `domains[].kind` absent | `bounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |

3. Repeat representative rows with the same advertisements and item extent but
   different backend identity text, manifest position, manifest `bounds` run
   defaults and ambient state. Vary the numeric maximum of a bounded item's
   proof bound while keeping its explicit `kind`; leave the domain coverage
   decision unchanged. Give the same `DomainKey` two distinct explicit domain
   kinds and verify that the explicit kind controls coverage.
4. Use an identity that names a non-existent executable, then one that names
   an executable recording its own start. Settle both against equal
   advertisements and inspect for process starts. Route a supported
   `Process(id)` item to generation with no process context; inspect its output
   under a permutation of Kani and two process identities. Mutate the routed
   `Process(id)` identity so it disagrees with the routed backend.

## Expected Results

1. `ALL` contains finite built-in Kani; CG's conversion and the named-backend
   path reach `Process(id)` and receive one disposition (FR-019-AC-11).
2. Every row has its stated disposition and cause. An unbounded item never
   settles `supported` on `bounded`-only advertisement (FR-019-AC-12,
   FR-019-AC-16, FR-019-AC-17, FR-019-AC-21, FR-019-AC-22).
3. Identity, position, run defaults and ambient state do not alter a
   disposition or cause apart from the backend named. Numeric maximum and
   `DomainKey` do not determine domain coverage; explicit extent kinds do
   (FR-019-AC-18, FR-019-AC-19).
4. No plugin starts or is resolved during settlement; the process item has
   one empty `KindOutput::Process` and no CG artifact or terminal record.
   Results are invariant under input permutation; an identity mismatch
   refuses the call with `BackendKindDisagrees` (FR-019-AC-13,
   FR-022-AC-17, FR-022-AC-18, FR-022-AC-19).

FR-019-AC-14 is verified by analysis of the arm's `Disposition` return type, not by a step here.

## Seeded mutants

| Mutant | Caught by |
|---|---|
| Enumerate process identities through static `ALL` and drop one | step 1 |
| Check mode but ignore an uncovered bounded domain | step 2 |
| Derive a domain kind from `DomainKey` instead of reading the explicit kind | step 3 |
| Compare numeric bound maximum with manifest run defaults | step 3 |
| Settle unbounded on `bounded` only as `supported` | step 2 |
| Start the plugin while settling | step 4 |
| Drop a routed process item or require a process generation context | step 4 |
| Accept a routed `Process(id)` whose identity differs from its backend | step 4 |

## Status

Planned (IR-629). Non-empty bounded-domain rows depend on QSL-654 adding required
`ProofBound.kind` to the FR-331 wire schema and producer. The process variant
and its arms are not implemented in this spec-only change.

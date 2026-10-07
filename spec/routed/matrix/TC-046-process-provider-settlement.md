---
id: TC-046
title: "Verify process-provider settlement and empty generation"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-026
    type: verifies
---
# TC-046: Verify process-provider settlement and empty generation

## Description

Verify that `Process(BackendId)` settles from the FR-331 manifest's advertised
(kind, mode) pairs and `domains`, against the requested item's `extent` alone.
Manifest `bounds` are run-limit defaults and do not set admission maxima.
Bounded `ProofBound.kind` is supplied by QSL-654; the test does not infer a kind
from `DomainKey`. Verify the empty generation arm and absence of plugin calls
(QSL ADR-029 PV-4; QSpec FR-290 and FR-331).

## Test Procedure

1. Inspect `BackendKind::ALL` and settle one process-origin candidate with an
   arbitrary identity, then one whose identity text is `kani`. Verify that
   each reaches `Process(id)` even though `ALL` lists built-in kinds only.
2. Settle the following single-candidate rows. Keep the requested capability
   kind fixed and vary only the listed manifest advertisement or item extent.

   | Item extent | Manifest advertisement | Expected disposition and cause |
   | --- | --- | --- |
   | bounded, `bounds` empty | `bounded` for kind | `supported` |
   | bounded, `bounds[].kind` all in `domains` | `bounded` for kind | `supported` |
   | bounded, one `bounds[].kind` absent from `domains` | `bounded` for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | bounded, covered domain | `unbounded` only for kind | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
   | unbounded | `unbounded` for kind, with unrelated or omitted `domains` | `supported` |
   | unbounded, `finite_bound_available=true`, all `domains[].kind` advertised | `bounded` only for kind | `requires-bound` |
   | unbounded, `finite_bound_available=false`, all `domains[].kind` advertised | `bounded` only for kind | `unsupported`, `unsupported_projection`/`unbounded-extent` |
   | unbounded, `finite_bound_available=true`, one `domains[].kind` absent | `bounded` only for kind | `unsupported`, `unsupported_projection`/`unbounded-extent` |

3. Repeat representative rows with the same advertisements and item extent but
   different backend identity text, manifest position, manifest `bounds` run
   defaults and ambient state. Vary the numeric maximum of a bounded item's
   proof bound while keeping its explicit `kind`; leave the domain coverage
   decision unchanged. Give a temporal item conflicting
   `temporal.subject.proof_bounds` and confirm that only its request `extent`
   controls settlement.
4. Use an identity that names a non-existent executable, then one that names
   an executable recording its own start. Settle both against equal
   advertisements and inspect for process starts. Route a supported
   `Process(id)` item to generation with no process context; inspect its output
   under a permutation of Kani and two process identities. Inspect the group
   traversal for Kani first, then process identities by their canonical bytes.

## Expected Results

1. `ALL` contains finite built-in Kani; both process identities reach
   `Process(id)` and receive one disposition (FR-019-AC-11).
2. Every row has its stated disposition and cause. An unbounded item never
   settles `supported` on `bounded`-only advertisement (FR-019-AC-12).
3. Identity, position, run defaults, ambient state, proof-bound numeric maximum
   and separate temporal-subject bounds do not alter a coverage decision.
   Only explicit item extent domain kinds determine domain coverage
   (FR-019-AC-12).
4. No plugin starts or is resolved during settlement; the process item has
   one empty `KindOutput::Process` and no CG artifact or terminal record.
   Results are invariant under input permutation; grouping traverses Kani
   before process identities in bytewise canonical-identity order
   (FR-019-AC-13, FR-022-AC-17, FR-026-AC-5).

FR-019-AC-14 is verified by analysis of the arm's `Disposition` return type, not by a step here.
QSL ADR-029 PV-4 does not prescribe the cross-kind order in step 4; it is CG's
deterministic local traversal rule in FR-022, not an FR-331 wire member.

## Seeded mutants

| Mutant | Caught by |
|---|---|
| Enumerate process identities through static `ALL` and drop one | step 1 |
| Check mode but ignore an uncovered bounded domain | step 2 |
| Read `temporal.subject.proof_bounds` instead of item `extent` | step 3 |
| Compare numeric bound maximum with manifest run defaults | step 3 |
| Settle unbounded on `bounded` only as `supported` | step 2 |
| Start the plugin while settling | step 4 |
| Drop a routed process item or require a process generation context | step 4 |

## Status

Planned (IR-629). Bounded-domain rows depend on QSL-654 adding required
`ProofBound.kind` to the FR-331 wire schema and producer. The process variant
and its arms are not implemented in this spec-only change.

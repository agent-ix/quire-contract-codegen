---
id: TC-046
title: "Verify the process-provider backend kind settles from its manifest alone"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
---
# TC-046: Verify the process-provider backend kind settles from its manifest alone

## Description

Verify that `BackendKind` has the process-provider variant, that its `negotiate_*` arm settles from the
descriptor's advertised (kind, mode) pairs and the extent classification alone, that it never reaches
the plugin, and that it returns a negotiation disposition only (QSL ADR-029 PV-4).

## Test Procedure

1. Read `BackendKind::ALL` and each member's `index`.
2. Settle one item per advertised-mode row against a process-provider descriptor: a bounded extent;
   an unbounded extent against `unbounded`; an unbounded extent against `bounded`-only with a finite
   bound available; the same with none.
3. Settle an item whose descriptor identity names an executable that records its own start, and check
   for the record.
4. Settle a `supported` item and an `unsupported` item and read what each returns and whether each routes.

## Expected Results

1. Both variants are listed and each index is its position (FR-019-AC-11).
2. `supported`; `supported`; `requires-bound`; `unsupported` with
   `unsupported_projection`/`unbounded-extent` (FR-019-AC-12).
3. No record exists (FR-019-AC-13).
4. The `supported` item is the disposition `supported` naming its backend, with no terminal value, result or
   artifact, and the `unsupported` item routes nothing (FR-019-AC-14).

## Seeded mutants

| Mutant | Caught by |
|---|---|
| The variant is left out of `ALL` | step 1 |
| Unbounded extent settles `supported` against `bounded`-only | step 2, third row |
| `requires-bound` settles with no finite bound available | step 2, fourth row |
| The arm starts the descriptor's identity as a process | step 3 |
| The arm routes an `unsupported` settlement | step 4 |

## Status

Planned. The variant does not exist at this revision; open questions 1 and 2 in FR-019 bound steps 1 and 2.

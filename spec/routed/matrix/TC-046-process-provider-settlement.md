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
descriptor's advertised (kind, mode) pairs and the extent classification alone, and that it never reaches
the plugin (QSL ADR-029 PV-4).

## Test Procedure

1. Read `BackendKind::ALL` and each member's `index`, and settle an item whose one candidate is a
   process-provider backend.
2. Settle one item against two process-provider descriptors with equal advertised pairs and different
   identity text, manifest position and ambient state, and compare the dispositions apart from the
   backend each names. Settle an unbounded extent against a `bounded`-only descriptor.
3. Settle an item against a descriptor whose identity names a non-existent executable and compare it, apart
   from the backend named, with one whose identity is ordinary. Settle another against a descriptor
   whose identity names an executable that records its own start, and look for the record.

## Expected Results

1. Both variants are listed, each index is its position, and the item settles in the process-provider arm
   (FR-019-AC-11).
2. The two dispositions are identical apart from the named backend, which is each descriptor's own
   identity, and the unbounded extent never settles `supported` (FR-019-AC-12).
3. The dispositions are identical apart from the named backend, nothing is started, and no record exists
   (FR-019-AC-13).

FR-019-AC-14 is verified by analysis of the arm's `Disposition` return type, not by a step here.

## Seeded mutants

| Mutant | Caught by |
|---|---|
| The variant is left out of `ALL` | step 1 |
| The disposition or cause depends on the identity text beyond echoing it as the named backend | step 2 |
| Unbounded extent settles `supported` against `bounded`-only | step 2 |
| The arm starts or resolves the descriptor's identity as a process | step 3 |

## Status

Planned. The variant does not exist at this revision, and every step that needs it also waits on FR-019
open question 3 to compile. Per criterion: FR-019-AC-11 waits on open question 2, FR-019-AC-12 on open
questions 5 and 1, and FR-019-AC-13 on none beyond question 3.

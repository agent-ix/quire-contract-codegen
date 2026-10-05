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
the plugin, and that identity resolution is unchanged for built-in and unnamed identities (QSL ADR-029 PV-4).

## Test Procedure

1. Read `BackendKind::ALL` and each member's `index`, and settle an item whose one candidate is a
   process-provider backend.
2. Settle one item against two process-provider descriptors with equal advertised pairs and different
   identity text, manifest position and ambient state, and compare the dispositions. Settle an unbounded
   extent against a `bounded`-only descriptor.
3. Settle an item against a descriptor whose identity names a non-existent executable and compare it with
   one whose identity is ordinary. Settle another against a descriptor whose identity names an executable
   that records its own start, and look for the record.
4. Call `BackendKind::from_identity` with `kani` and with an identity no variant names, and settle an
   item naming the latter.

## Expected Results

1. Both variants are listed, each index is its position, and the item settles in the process-provider arm
   (FR-019-AC-11).
2. The two dispositions are identical, and the unbounded extent never settles `supported`
   (FR-019-AC-12).
3. The dispositions are identical, nothing is started, and no record exists (FR-019-AC-13).
4. `Kani` and `None`, and the item settles `invalid-request`/`unknown-backend` (FR-019-AC-15).

FR-019-AC-14 is verified by analysis of the arm's `Disposition` return type, not by a step here.

## Seeded mutants

| Mutant | Caught by |
|---|---|
| The variant is left out of `ALL` | step 1 |
| The arm reads the identity text | step 2 |
| Unbounded extent settles `supported` against `bounded`-only | step 2 |
| The arm starts or resolves the descriptor's identity as a process | step 3 |
| `from_identity` returns the process-provider variant for an unnamed identity | step 4 |

## Status

Planned. The variant does not exist at this revision. Open questions 1, 2, 3, 5 and 6 in FR-019 bound
steps 1 to 4: FR-019-AC-11 to AC-13 and AC-15 wait on questions 1, 2 and 3 as FR-019 states each.

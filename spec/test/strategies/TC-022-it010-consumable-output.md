---
id: TC-022
title: "Verify strategy output is consumable without a local wire schema"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-013
    type: verifies
---
# TC-022: Verify strategy output is consumable without a local wire schema

## Description

Verify that a downstream crate can compile and read generated bound strategies using only the
generated Rust, `proptest`, and `quire-contract-runtime`, that the header names the source clause,
and that no local serialized format exists.

## Test Procedure

1. Generate the `VersionUnchanged` strategy bundle. Write it into a consumer fixture crate whose
   manifest depends only on `proptest` and `quire-contract-runtime`, and build that crate under denied
   warnings.
2. In the fixture, draw cases and read each value through the generated declaration-name and
   observation-name constants; read the out-of-domain array the same way.
3. Inspect the `BoundGenerationError::NameCollision` mapping to `UnsupportedClause`. Bound
   generation gives every oracle symbol a positional counter, so no concrete fixture produces a
   collision.
4. List the bundle's files, and diff `schemas/` against the base revision.
5. Generate two different `ClauseRef`s from one package, and compare the headers and the artifact
   paths.

## Expected Results

- The fixture builds and reads the `versionNumber` field's declaration, by its SL field-alias
  `SymbolName`, at `"pre"` and at `"post"` by name and observation.
- The total `NameCollision` mapping returns `UnsupportedClause` carrying the colliding full
  `ClauseRef` and `invalid-input` terminal state if the preflight branch is reached.
- The bundle holds only generated Rust; no serialized case, census, or summary file exists;
  `schemas/` is unchanged.
- Each header carries its full `ClauseRef`, and the two clauses produce different headers and
  artifact paths.

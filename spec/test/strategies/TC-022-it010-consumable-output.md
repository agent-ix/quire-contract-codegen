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
generated Rust, `proptest`, and `quire-contract-runtime`, that the attestation is Quoin's packaged
shape bound to the source package, and that no local serialized format exists.

## Test Procedure

1. Generate the `VersionUnchanged` strategy bundle. Write it into a consumer fixture crate whose
   manifest depends only on `proptest` and `quire-contract-runtime`, and build that crate under denied
   warnings.
2. In the fixture, draw cases and read each value through the generated declaration-name and
   observation-name constants; read the out-of-domain array the same way.
3. Inspect the `BoundGenerationError::NameCollision` mapping to `UnsupportedClause`. A concrete
   fixture is intentionally unavailable: bound generation includes a full SHA-256 identity suffix
   in every oracle symbol, so constructing one would require manufacturing a SHA-256 collision.
4. List the bundle's files, and diff `schemas/` against the base revision.
5. Validate the strategy attestation body against the bytes `quoin change-assurance schema` publishes,
   with format assertion on, and seal it through the real CLI.
6. Regenerate with a different `BoundPackage` digest, then with a different `ClauseRef`, and compare
   the header, the `--requirement`, `--clause`, and `--input-digest` argv, and the artifact
   identities.

## Expected Results

- The fixture builds and reads the `versionNumber` field's declaration, by its SL field-alias
  `SymbolName`, at `"pre"` and at `"post"` by name and observation.
- The total `NameCollision` mapping returns `UnsupportedClause` carrying the colliding full
  `ClauseRef` and `invalid-input` terminal state if the collision-resistant preflight branch is
  reached.
- The bundle holds only generated Rust and its attestation body; no serialized case, census, or
  summary file exists; `schemas/` is unchanged.
- The attestation validates and seals.
- The header carries the package digest and full `ClauseRef`; the argv carries
  `--requirement <requirement>@<revision>`, `--clause <clause id>`, and
  `--input-digest <BoundPackage digest>`; each change produces a distinct artifact identity.

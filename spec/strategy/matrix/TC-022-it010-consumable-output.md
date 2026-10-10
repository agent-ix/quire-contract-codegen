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

Verify that a downstream crate can include and compile the generated bound-strategy Rust source
with only `proptest` and `quire-contract-runtime` as manifest dependencies. Verify that its header
names the source clause and no local serialized format exists.

## Test Procedure

1. Generate the `VersionUnchanged` strategy bundle from a fresh QSL-produced, IR-admitted V2
   package with direct `self.versionNumber = pre(self.versionNumber)`. Include the generated Rust
   source in a consumer fixture crate whose manifest depends only on `proptest` and
   `quire-contract-runtime`, and build that crate under denied warnings.
2. In the fixture, draw cases and read each value through the generated typed V2 identity, selected-path and observation constants; read the out-of-domain array the same way.
3. List the bundle's files, and diff `schemas/` against the base revision.
4. Generate two distinct authentic checked claim occurrences from one package, and compare the headers and the artifact
   paths.

## Expected Results

- The fixture builds and reads the `versionNumber` field's declaration, by its typed StateField identity and selected path, at `"pre"` and at `"post"` by name and observation.
- The bundle holds only generated Rust; no serialized case, census, or summary file exists;
  `schemas/` is unchanged.
- Each header carries its selected checked clause id and authentic claim occurrence, and the two clauses produce different headers and
  artifact paths.

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

Verify that a downstream crate shaped like SL IT-010 can compile and read generated numeric strategies
using only the generated Rust, `proptest`, and `quire-contract-runtime`. Also verify that the
attestation is Quoin's packaged shape and that no local serialized format exists.

## Test Procedure

1. Generate the `VersionUnchanged` strategy bundle. Write it into a consumer fixture crate whose
   manifest depends only on `proptest` and `quire-contract-runtime`, and build that crate.
2. In the fixture, draw cases and read each value through the generated (declaration name,
   observation) accessors.
3. List the bundle's files, and diff `schemas/` against the base revision.
4. Validate the strategy attestation body against the bytes `quoin change-assurance schema` publishes,
   with format assertion on, and seal it through the real CLI.
5. Regenerate with a different `BoundPackage` digest, then with a different `ClauseRef`, and compare
   the artifact identities.

## Expected Results

- The fixture builds and reads `versionNumber` at `Pre` and at `Post` by name and observation.
- The bundle holds only generated Rust and its attestation body; no serialized case, census, or
  summary file exists; `schemas/` is unchanged.
- The attestation validates and seals.
- The header and argv carry the package digest and full `ClauseRef`, and each change produces a
  distinct artifact identity.

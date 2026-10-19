---
id: TC-001
title: "Verify deterministic derivation and generated headers"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: verifies
---
# TC-001: Verify deterministic derivation and generated headers

## Description

Verify repeated generation is byte-identical and every generated Rust family retains its
input-carried identity, generator name and licensing.

## Test Procedure

Generate each corpus package repeatedly while permuting irrelevant input ordering and supported
platform execution; compare artifact names, bytes, and source maps.
For every generated `.rs` artifact of each family in NFR-002 Scope, inspect its header against its
generation input: check the generator name, every subject identity the file renders under the Scope
rules, and every requirement, revision and clause identifier that the input carries. Include
oracle-package `src/lib.rs` files, equality extra modules, and function `src/schedule_*.rs` part
files; do not accept a single passing file as coverage for a family. Include inputs without the
optional identifiers; do not infer them from output names. Check the SPDX line on every artifact.

## Expected Results

Bundles are byte-identical; every generated `.rs` artifact, including package `src/lib.rs` and
module part files, has a header naming `quire-contract-codegen`, every input-carried subject it
renders and each requirement, revision and clause identifier present in its input. Every such
artifact has the `MIT OR Apache-2.0` SPDX line (NFR-002-AC-2). No environmental path or
nondeterministic order enters the output.

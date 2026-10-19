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
For each generated Rust family in NFR-002 Scope, inspect the header against its generation input:
check the generator name and subject identity (obligation or source node when present, otherwise
strategy or population), and check every requirement, revision and clause identifier that the input
carries. Include inputs without those optional identifiers; do not infer them from output names.

## Expected Results

Bundles are byte-identical; every generated Rust header names `quire-contract-codegen` and its
input-carried subject identity, names each requirement, revision and clause identifier present in
its input, and carries the SPDX identity. No environmental path or nondeterministic order enters
the output.

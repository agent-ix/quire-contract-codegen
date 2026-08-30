---
id: TC-001
title: "Verify deterministic derivation"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-001
    type: verifies
---
# TC-001: Verify deterministic derivation

## Description

Verify repeated generation is byte-identical and every output retains complete identity and licensing.

## Test Procedure

Generate each pinned corpus package repeatedly while permuting irrelevant input ordering and supported
platform execution; compare artifact names, bytes, manifests, source maps, and bundle digests.

## Expected Results

Bundles are byte-identical, identities and SPDX headers are complete, and no environmental path or
nondeterministic order enters the output.

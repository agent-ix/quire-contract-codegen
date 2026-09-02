---
id: TC-005
title: "Verify Kani proof dependency closure"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: verifies
---
# TC-005: Verify Kani proof dependency closure

## Description

Verify generated Kani source, framing, bindings, and dependency graphs preserve assumptions and block
complete-proof claims when required dependencies are not successful.

## Test Procedure

Generate fixtures with complete, missing, failed, and assumed dependency edges under the pinned Kani
adapter; execute bounded proofs and inspect graph/evidence classifications.

## Expected Results

Only complete successful dependency closure can support a complete proof; every assumption, version,
option, and non-success state remains visible.

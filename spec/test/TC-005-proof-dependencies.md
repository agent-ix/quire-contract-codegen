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

Verify generated Kani source, framing, typed bindings, and dependency graphs preserve assumptions and
block complete-proof claims when required dependencies are not successful. Keep model-domain bounds
distinct from assumed proof edges.

## Test Procedure

Generate fixtures with complete, missing, failed, and assumed dependency edges under the Kani
adapter; execute bounded proofs and inspect graph/evidence classifications. Include Boolean and
bounded-integer bindings, reorder the caller dependency census, and require one stable source marker
per proof assumption/stub. Inspect model-domain binding records independently and confirm they do not
alter dependency readiness.

## Expected Results

Only complete successful dependency closure can support a complete proof; every assumption,
option, typed domain bound, and non-success state remains visible. Generation always reports
`proofExecutionState: not_run`; a ready graph is not a completed Kani proof.

## Status

Retired with [FR-003](../functional/FR-003-kani-lowering.md). Its tests stay until the code retires
the V1 path. The proof-dependency readiness FR-003 carried is FR-015-AC-22, verified by
[TC-025](./complete-v1/TC-025-bounded-kani-obligations.md).

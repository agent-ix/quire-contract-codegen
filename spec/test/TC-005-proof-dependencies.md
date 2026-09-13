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

Generate fixtures with complete, missing, failed, and assumed dependency edges under the pinned Kani
adapter; execute bounded proofs and inspect graph/evidence classifications. Include Boolean and
bounded-integer bindings, reorder the caller dependency census, and require one stable source marker
per proof assumption/stub. Inspect model-domain binding records independently and confirm they do not
alter dependency readiness.

## Expected Results

Only complete successful dependency closure can support a complete proof; every assumption, version,
option, typed domain bound, and non-success state remains visible. Generation always reports
`proofExecutionState: not_run`; a ready graph is not a completed Kani proof.

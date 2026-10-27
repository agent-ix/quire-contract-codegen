---
id: TC-005
title: "Verify Kani proof dependency closure"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
---
# TC-005: Verify Kani proof dependency closure

## Description

Verify the generation-time declared proof-dependency census on supported V2
clause obligations. A census changes the sealed harness identity and persisted
record, but generation never claims that the proof ran. The former V1 bundle's
`ProofDependencyGraph`, assumed and stubbed edges, and its backend procedure are
historical; they supply no V2 evidence after AD-004 step 4f. Other surviving
proof-family and installed-backend controls remain in TC-025.

## Test Procedure

Use an admitted QSL V2 precondition through the public Kani obligation request.
Declare empty and duplicate identities, a state or path inconsistent with its
kind, and a non-`Required` kind. Require typed invalid-request records and no
harness. Declare two passed required dependencies, then change one to missing
and failed. Reverse the declaration order. For every supported case, parse the
persisted identity record and compare its full sorted dependency list, derived
readiness and proof execution state against independent expected values.

## Expected Results

Only all-passed required dependencies yield `ready`; missing or failed yields
`incomplete`. Reordering leaves the full harness and persisted record unchanged.
The record reports `proofExecutionState: not_run` for every valid census. No
generation-time readiness value is an installed-Kani proof verdict.

---
id: TC-007
title: "Verify cross-backend semantic parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: verifies
---
# TC-007: Verify cross-backend semantic parity

## Description

Verify executable, proptest, Kani, and coverage lowerings agree on the shared bounded corpus and every
permitted differential fixture has an attributed disposition.

## Test Procedure

Execute the same canonical cases through each backend and compare normalized clause outcomes,
diagnostics, dependencies, and proof attestations with golden and permitted attributed fixtures.

## Expected Results

Shared semantics agree exactly; each difference is retained as a regression fixture or documented
profile/backend distinction rather than suppressed.

---
id: TC-026
title: "Verify witness decoding and native replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# TC-026: Verify witness decoding and native replay

## Description

Verify that retained counterexamples are decoded, domain-checked and replayed
natively before any failure is reported.

## Test Procedure

Replay a reproducing witness, a malformed witness, an out-of-domain witness, a
witness whose native outcome disagrees with the harness, and a witness whose
native replay is unavailable.

## Expected Results

Only the reproducing witness is reported as a failure; the others yield
malformed, out-of-domain, mismatch and unavailable results respectively.

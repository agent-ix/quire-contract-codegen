---
id: TC-025
title: "Verify separate bounded Kani obligations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-025: Verify separate bounded Kani obligations

## Description

Verify that complete-V1 contracts produce one pinned, bounded harness per
obligation kind and refuse non-finite obligations.

## Test Procedure

Generate harnesses for a contract with a precondition, postcondition,
invariant and frame condition over bounded scalar domains, and for a contract
with an unbounded domain. Inspect harness identities, bounds, backend pins and
assumptions, and run the pinned Kani backend on the bounded harnesses.

## Expected Results

Four distinct harnesses carry their bounds and backend pin; no assumption
excludes an undefined, refused or incomplete outcome; the unbounded obligation
is refused with no harness.

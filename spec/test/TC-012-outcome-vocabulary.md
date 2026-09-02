---
id: TC-012
title: "Verify twelve verification outcomes stay distinguishable"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: verifies
---
# TC-012: Verify twelve verification outcomes stay distinguishable

## Description

Verify that pass, fail, unavailable, unsupported, inconclusive, not-computed, malformed, partial,
stale, suspect, vacuous, and tampered are each demonstrated by a case that produced it and matched,
that every negative is paired with a positive control that was observed to be accepted, and that the
generator's own Interface-001 terminal states survive alongside them.

## Test Procedure

Collect the states demonstrated by the assurance chain and by the compatibility census, and require
the union to cover all twelve. Require each declared negative scenario to name a control that ran.
Separately, require the generation corpus to have reached more than one non-success terminal state,
so `unsupported` and `invalid-input` are shown not to have collapsed into one another.

## Expected Results

All twelve outcomes are demonstrated by matching cases; a scenario that demonstrates no outcome
carries a null rather than borrowing a label; every negative has a control; and the corpus census row
reports its declared terminal states at or above its floor.

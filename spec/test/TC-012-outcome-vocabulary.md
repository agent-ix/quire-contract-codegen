---
id: TC-012
title: "Verify the demonstrable verification outcomes stay distinguishable"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: verifies
---
# TC-012: Verify the demonstrable verification outcomes stay distinguishable

## Description

Verify that pass, fail, unavailable, inconclusive, not-computed, partial, stale, suspect, vacuous,
and tampered are each demonstrated by a case that produced it and matched, that `unsupported` and
`malformed` are not silently re-acquired from a refusal or from another state under a second name,
that every negative is paired with a positive control that was observed to be accepted, and that the
generator's own Interface-001 terminal states survive alongside them.

## Test Procedure

Collect the states demonstrated by the assurance chain and require them to cover the ten. Require
`unsupported` and `malformed` to be absent from that set: the compatibility census over retained
evidence was their only demonstration here and it is deleted. Neither may be replaced by an adapter
refusal, because a refusal to produce a state is not a demonstration of one; and `malformed` may not
be replaced by a producer stream declaring that outcome, because the adapter maps it onto the same
`fail` and the resulting receipt is indistinguishable from the `fail` case. Require each declared
negative scenario to name a control that ran, and that the control could fail independently of the
scenario. Separately, require the generation corpus to have reached more than one non-success
terminal state, so `unsupported` and `invalid-input` are shown not to have collapsed into one another
as Interface-001 terminal states.

## Expected Results

All ten required outcomes are demonstrated by matching cases; `unsupported` and `malformed` are
reported by nothing; a scenario that demonstrates no outcome carries a null rather than borrowing a
label; every negative has a control; and the corpus census row reports its declared terminal states
at or above its floor.

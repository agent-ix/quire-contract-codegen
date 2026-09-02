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

Verify that pass, fail, unavailable, inconclusive, not-computed, malformed, partial, stale, suspect,
vacuous, and tampered are each demonstrated by a case that produced it and matched, that
`unsupported` is not silently re-acquired from a refusal, that every negative is paired with a
positive control that was observed to be accepted, and that the generator's own Interface-001
terminal states survive alongside them.

## Test Procedure

Collect the states demonstrated by the assurance chain and require them to cover the eleven.
`malformed` is among them and is demonstrated by a producer stream that really declares that outcome,
derived from the real corpus run. Require `unsupported` to be absent from the set: the compatibility
census over retained evidence was its only demonstration here and it is deleted, `unsupported` is not
in the adapter's producer vocabulary, and the chain's adapter refusals must not be relabelled to
replace it, because a refusal to produce a state is not a demonstration of one. Require each declared
negative scenario to name a control that ran. Separately, require the generation corpus to have
reached more than one non-success terminal state, so `unsupported` and `invalid-input` are shown not
to have collapsed into one another as Interface-001 terminal states.

## Expected Results

All eleven required outcomes are demonstrated by matching cases; `unsupported` is reported by
nothing; a scenario that demonstrates no outcome carries a null rather than borrowing a label; every
negative has a control; and the corpus census row reports its declared terminal states at or above
its floor.

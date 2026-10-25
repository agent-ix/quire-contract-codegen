---
id: TC-057
title: "Check typed checked-invariant causes from generated oracles"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: verifies
---
# TC-057: Check typed checked-invariant causes from generated oracles

## Description

Planned owner control for FR-042-AC-1's IR-709 generated producer migration.

## Test Procedure

1. Exercise every generated Boolean, equality, function and pre-check placeholder constructor for the seven causes named in FR-042 Behavior. Inspect the emitted source for each named path and count no cause from an unreachable branch or a fabricated failure.
2. Compare each typed cause and nested `IllTypedCause` against the input result; check the generated charge and failure boundary.
3. Confirm no former unit constructor or invented default remains. The QSL replay consumer is verified at its owner.

## Expected Results

Every one of the seven named CG producer paths retains its typed cause and original nested payload without changing charge or failure behavior; no fabricated failure is counted as evidence.

## Status

PLANNED under IR-709.

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

Planned owner control for FR-042-AC-1 after the IR-497 typed-payload cutover and IR-709 producer migration.

## Test Procedure

1. Exercise every generated Boolean, equality, function and pre-check placeholder checked-invariant constructor listed in the second cause table of quire-exact FR-369.
2. Compare each typed cause and nested `IllTypedCause` against the input result; check the generated charge and failure boundary.
3. Confirm no former unit constructor or invented default remains. The QSL replay consumer is verified at its owner.

## Expected Results

Every CG constructor retains the table cause and original nested payload without changing charge or failure behavior.

## Status

PLANNED under IR-709 after the IR-497 cutover.

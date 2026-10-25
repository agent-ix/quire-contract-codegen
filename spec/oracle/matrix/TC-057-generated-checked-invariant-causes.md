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

1. Exercise every generated Boolean, equality, function and pre-check placeholder checked-invariant constructor listed in the second cause table of quire-exact FR-369. Inspect the emitted source for each of the nine named paths; do not count an unreachable branch or a fabricated failure as evidence.
2. Once the planned paths exist, use a real generated operation whose `Outcome::Completed(value)` violates its declared result type to check `GeneratedUnexpectedOutcome`; use an emitted function building its environment from real admitted nominal declarations whose construction rejects to check `GeneratedEnvironmentRejected`. Verify the actual executed owner path and its typed rejection in each case, with no synthesized outcome or environment failure.
3. Compare each typed cause and nested `IllTypedCause` against the input result; check the generated charge and failure boundary.
4. Confirm no former unit constructor or invented default remains. The QSL replay consumer is verified at its owner.

## Expected Results

All nine genuine CG producer paths retain their table causes and original nested payloads without changing charge or failure behavior; no fabricated failure is counted as evidence.

## Status

PLANNED under IR-709.

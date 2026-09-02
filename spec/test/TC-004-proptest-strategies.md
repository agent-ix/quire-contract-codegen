---
id: TC-004
title: "Verify shaped proptest strategies"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: verifies
---
# TC-004: Verify shaped proptest strategies

## Description

Verify generated harnesses preserve tri-state outcomes and strategy construction/shrinking respects
supported constraints and boundaries.

## Test Procedure

Generate campaigns for range, membership, correlated, state-pinned, boundary, and residual-filter
fixtures; execute seeded cases and shrinking while retaining accepted/rejected/failed/discarded counts.

## Expected Results

Supported constraints are shaped directly, inside/outside boundaries execute, residual rejection is
explicit, and no rejected case becomes a pass.

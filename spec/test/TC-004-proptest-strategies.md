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

Generate campaigns for range, membership, enum, correlated, state-pinned, boundary, and residual
fixtures. Execute seeded cases and shrinking through the generated campaign runner. Exercise an
accepted-case floor, an explicit-discard ceiling, zero configured cases, all-rejected cases,
expectation mismatches, empty residual exclusions, and correlated populations whose representable
boundary census would otherwise be vacuous.

## Expected Results

Supported constraints are shaped directly, every emitted boundary census contains accepted and
rejected cases, residual rejection is explicit, and no rejected case becomes a pass. Integer and
enum values carry executable expected-domain checks. Boolean campaign constructors bind disposition
to their exact values; the generated runner owns invocation, explicit discard accounting, and final
accepted/rejected/failed/discarded validation. It rejects zero/below-floor campaigns and campaigns
above the requested discard ceiling.

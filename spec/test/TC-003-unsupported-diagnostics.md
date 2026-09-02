---
id: TC-003
title: "Reject unsupported or invalid constructs explicitly"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: verifies
---
# TC-003: Reject unsupported or invalid constructs explicitly

## Description

Verify malformed, orphaned, partial, and unsupported constructs cannot yield complete artifacts.

## Test Procedure

Run every negative conformance fixture through every applicable backend and inspect diagnostics,
manifest completeness state, exit status, and staged output directory.

## Expected Results

Each fixture produces its expected stable diagnostic and no falsely complete backend artifact.

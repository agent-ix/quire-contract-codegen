---
id: TC-037
title: "Verify the backend adapter trait and its closed-enum dispatch"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-026
    type: verifies
---
# TC-037: Verify the backend adapter trait and its closed-enum dispatch

## Description

Verify that the Kani backend is one implementation of the adapter trait and that the adapter owns its
execution evidence type.

## Test Procedure

1. Call a function generic over the adapter trait with the Kani adapter, and reach its generation arm, execution, transcript parser and witness renderer through it.
2. Read the Kani adapter's associated execution evidence type.

## Expected Results

1. Each part is reached through the trait (FR-026-AC-1).
2. It is `KaniExecutionEvidence` (FR-026-AC-4).

## Status

Planned. No adapter trait exists at this revision.

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

Verify that the Kani backend is one implementation of the adapter trait, that every dispatch reaches
an adapter only through an exhaustive match on `BackendKind`, that every Kani-specific item lives in
the Kani adapter module, and that the adapter owns its execution evidence type.

## Test Procedure

1. Call a function generic over the adapter trait with the Kani adapter, and reach its generation arm, execution, transcript parser and witness renderer through it.
2. Parse every non-test source under `src/` with `syn`. Find each place settlement, routed
   generation, execution and the terminal-record map reach an adapter, and every use of the adapter
   trait as a trait object.
3. In the same parse, find where the Kani printed-output
   wording, playback typing and witness rendering are defined.
4. Read the Kani adapter's associated execution evidence type.

## Expected Results

1. Each part is reached through the trait (FR-026-AC-1).
2. Each dispatch sits inside a `match` on `BackendKind` with no wildcard arm, and no trait object of
   the adapter trait exists (FR-026-AC-2).
3. Each is defined only inside the Kani adapter module (FR-026-AC-3).
4. It is `KaniExecutionEvidence` (FR-026-AC-4).

## Status

Planned. No adapter trait exists at this revision.

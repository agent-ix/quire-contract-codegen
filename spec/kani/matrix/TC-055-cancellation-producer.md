---
id: TC-055
title: "Verify live Kani cancellation provenance through execution, IR and QSL"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-040
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-055: Verify live Kani cancellation provenance through execution, IR and QSL

## Description

Plan a discriminating production-path check for [FR-040](../functional/FR-040-cancellation-producer.md).
The event reader must supply the observed source; a prebuilt IR outcome tests only the terminal
map. The existing [TC-041](./TC-041-ir-outcome-terminal-map.md) continues to own the separate
[FR-030](../functional/FR-030-ir-outcome-terminal-map.md) AC-7 source inspection.

## Test Procedure

1. Create a fresh `quire_exact::Cancel` for each public `KaniExecutionRequest`. Cancel one before
   execution, and for another launch a backend with a positive running barrier, then call
   `cancel(Requested)` and `cancel(Deadline)` on caller-held clones. Repeat the live launch with
   the two calls reversed. Observe the actual bounded launcher stop and its owned teardown; inspect
   execution evidence, the IR outcome constructed by CG from that evidence and the QSL terminal
   reached through the production call path. Drive a no-event ceiling control and a cancellation
   sent only after completion. Tag the running controls to FR-040-AC-1 through AC-3 and AC-6.
2. Use positive barriers to stage completion, memory overage, memory observation failure, output
   over-limit, unread capture and cleanup nonconfirmation against cancellation in each relevant
   order. Inspect the returned result/refusal and owner state. Include a cancellation that stops
   the launcher before a later elapsed deadline. Tag to FR-040-AC-4. Tests may inject failures at
   the existing monitor/capture seams, but must invoke the real public execution entry and its
   launcher observation path rather than substituting the event reader.
3. Run a batch with two harnesses that share one handle and a third harness with a distinct handle
   in another group. Cancel the first group's handle after both members start, reverse
   `Requested`/`Deadline` order in a second run, and let the other group finish. Inspect the
   returned `KaniGroupRun` entries, their actual process grouping and cleanup. Also submit
   otherwise groupable requests with distinct handles and verify separate launcher processes.
   Tag to FR-040-AC-5.
4. Compile the direct `quire-exact::CancelCause` use with
   `qsl-replay::IncompleteCause::Cancelled`, and inspect the typed IR-to-exact exhaustive match and
   QSL imports. As map-unit controls, supply both source-bearing IR kinds and both typed
   `KaniRunOutcome::Cancelled` sources directly and assert the exact QSL cause; pair the latter
   with a replay settlement and assert `UnexpectedSettlement`. Keep these unit controls separate
   from step 1's producer evidence. Tag to FR-029-AC-15, FR-029-AC-29,
   FR-040-AC-6 and AC-7.

## Expected Results

The pre-spawn request starts no backend. Each running request stops its owned backend and retains
the first recorded cause in the launch conclusion, public `KaniRunOutcome`, CG-constructed IR
`Cancelled { source }` and QSL `IncompleteCause::Cancelled { source }`. The reversed order reverses
the retained source. The no-event ceiling remains timed out; a late cancellation never relabels
completion or another established stop. Cleanup and evidence refusals retain their established
priority. A cancelled multi-member group returns one typed `BatchCancelled` refusal with its
source and member count, no member evidence, while the independent group keeps its result. The
source-bearing map is exhaustive, uses the one exact cause type and has no metadata inference.

## Status

Planned for IR-701 implementation. No test or real-Kani execution result is asserted by this
specification artifact.

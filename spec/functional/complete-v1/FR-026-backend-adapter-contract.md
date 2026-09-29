---
id: FR-026
title: "Reach each backend through one adapter trait and the closed backend enum"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
---
# FR-026: Reach each backend through one adapter trait and the closed backend enum

## Description

The code generator shall hold everything specific to one proof backend in that backend's adapter,
one implementation of a single adapter trait, and shall reach an adapter only through an exhaustive
match on the closed `BackendKind` enum
([ADR-002](../../decisions/ADR-002-backend-adapter-boundary.md) Q0, Q1 and Q4). ADR-002 Q4 states how a
second backend registers.

## Inputs

- The closed `BackendKind` enum ([FR-019](./FR-019-capability-settlement.md)).
- One adapter per `BackendKind` variant. The Kani adapter is the one adapter today.

## Outputs

- An adapter trait whose associated items are the four parts of an adapter: the generation arm,
  execution, the transcript parser and the witness renderer.
- One implementation of that trait per `BackendKind` variant.

## Behavior

- The generator shall define one adapter trait whose associated items are the generation arm
  ([FR-022](./FR-022-routed-generation.md)), execution
  ([FR-017](./FR-017-kani-execution-evidence.md)), the transcript parser and
  the witness renderer ([FR-024](./FR-024-counterexample-envelope-intake.md)).
- The generator shall implement that trait once for each `BackendKind` variant.
- The generator shall reach an adapter from settlement, routed generation, execution and the
  terminal-record map ([FR-029](./FR-029-run-outcome-terminal-record.md)) only through an
  exhaustive match on `BackendKind` with no catch-all arm.
- The generator shall register no adapter at run time.
- The generator shall define every backend-specific item, namely the printed-output wording, the playback typing and the witness rendering, inside that backend's
  adapter module.
- Each adapter shall own its own execution evidence type.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-026-AC-1 | The Kani adapter implements the adapter trait, and a function generic over that trait reaches the Kani adapter's generation arm, execution, transcript parser and witness renderer through it. | Test (TC-037) |
| FR-026-AC-4 | The execution evidence type is an associated type of the adapter trait, and the Kani adapter's is `KaniExecutionEvidence`. | Test (TC-037) |

## Dependencies

- **Upstream**: [FR-019](./FR-019-capability-settlement.md), whose closed `BackendKind` is the
  registry; [FR-022](./FR-022-routed-generation.md); QSL ADR-013 T-7, under which QSL converts the
  FR-331 provider envelope.
- **Downstream**: [TC-037](../../test/complete-v1/TC-037-backend-adapter-contract.md),
  [FR-029](./FR-029-run-outcome-terminal-record.md).

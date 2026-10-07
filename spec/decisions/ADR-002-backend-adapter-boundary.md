---
id: ADR-002
title: "Backend adapter boundary"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-026
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: relates_to
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: depends_on
---
# ADR-002: Backend adapter boundary

## Status

Accepted.

## Context

Kani is the one backend registered under the FR-331 contract. `BackendKind` has one variant, `Kani`
(`src/capability.rs`). Settlement (FR-019) and routed generation (FR-022) dispatch over it
exhaustively.

QSL owns `Witness`, `ReplaySource`, the counterexample envelope, the FR-331 terminal record and
`ObligationIdentity` in `qsl-replay`. Contract IR holds no copy of them, and the Kani transcript
parser stays in CG as part of its backend adapter.
[AD-001](../assurance/AD-001-codegen-architecture.md) states that ownership. CG's run outcome is
`KaniRunOutcome` (FR-017).

## Decision

### Q0 and Q1: the linked Kani adapter, reached through the closed enum

The linked Kani backend has one CG adapter trait whose associated items are its four parts:

1. **Generation arm.** The per-kind arm FR-022 dispatches to.
2. **Execution.** Launch the installed backend and classify the run (FR-017).
3. **Transcript parser.** The one module that reads the backend's native output, its
   counterexample included, into a typed transcript.
4. **Witness renderer.** The one function that renders QSL's backend-witness transcript from the
   decoded values, admitted by `qsl_replay::Witness::parse` (FR-024).

The closed `BackendKind` enum dispatches CG settlement and generation exhaustively. The Kani
adapter is reached through its kind's arm; no CG adapter is registered at run time.
[FR-026](../routed/functional/FR-026-backend-adapter-contract.md) states this.

### Q2: the adapter runs the installed Kani

The adapter runs the Kani installation it is given.

### Q3: one total match from `KaniRunOutcome` to QSL's terminal record

The CG adapter maps every `KaniRunOutcome`, with its inconclusive reason, to
`qsl_replay::TerminalValue` in one match that has no catch-all arm. Each outcome that one of QSL's
existing terminal values states maps to that value, and QSL adds no terminal value for CG. A
vacuous proof and a cover-unsatisfied run both map to `Proved { success_checks: 0 }`, which QSL
reads as category `inconclusive` with the vacuity cause `KaniVacuousProof`, the record QSpec
FR-331-AC-8 requires. Every run outcome maps to exactly one value, so every run item has exactly
one terminal record (QSpec FR-331). An
item settled `unsupported` at negotiation has no run and no terminal value; its warning names its
capability kind from `quire.capability-kind/v1` (QSpec FR-290).
[FR-029](../kani/functional/FR-029-run-outcome-terminal-record.md) states this.

### Q4: another linked backend registers through the closed enum

Another linked backend registers in these steps:

1. Its descriptor appears in the FR-331 provider envelope, which QSpec authors and QSL converts
   (QSL ADR-013 T-7).
2. A new `BackendKind` variant is added. The settlement, routed-generation, execution and
   terminal-record matches then fail to compile until each has an arm.
3. The new adapter implements the adapter trait, with real transcript captures of its own under
   `tests/fixtures/`.
4. The adapter owns its own execution evidence type.
5. FR-019, FR-022 and the test matrix gain the kind's rows.

### Amendment (IR-629): the process-provider variant

QSL ADR-029 PV-4 gives `BackendKind` one `Process(BackendId)` variant for process providers.
Its `negotiate_*` arm settles from the provider's manifest and the item's capability kind and full extent;
it never calls the plugin ([FR-019](../routed/functional/FR-019-capability-settlement.md)).
The process generation arm returns `KindOutput::Process` with no CG artifact. CG has no adapter,
execution or terminal-record arm for a process provider: the driver runs the plugin host's process
adapter and reads its FR-331 result. Q4's linked-backend adapter steps therefore do not apply to
`Process(BackendId)`.

## Consequences

- FR-026 and FR-029 state the linked Kani adapter trait and terminal-record map.
- FR-019 and FR-022 state the process-provider settlement and empty generation arms.

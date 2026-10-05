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

### Q0 and Q1: one adapter trait per backend kind, reached through the closed enum

A backend adapter owns everything specific to one `BackendKind`. It is one Rust trait, implemented
once per kind, whose associated items are the adapter's four parts:

1. **Generation arm.** The per-kind arm FR-022 dispatches to.
2. **Execution.** Launch the installed backend and classify the run (FR-017).
3. **Transcript parser.** The one module that reads the backend's native output, its
   counterexample included, into a typed transcript.
4. **Witness renderer.** The one function that renders QSL's backend-witness transcript from the
   decoded values, admitted by `qsl_replay::Witness::parse` (FR-024).

The closed `BackendKind` enum is the registry. An adapter is reached only through an exhaustive
match on it, and no adapter is registered at run time.
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

### Q4: a second backend registers through the closed enum

A second backend registers in these steps:

1. Its descriptor appears in the FR-331 provider envelope, which QSpec authors and QSL converts
   (QSL ADR-013 T-7).
2. A new `BackendKind` variant is added. The settlement, routed-generation, execution and
   terminal-record matches then fail to compile until each has an arm.
3. The new adapter implements the adapter trait, with real transcript captures of its own under
   `tests/fixtures/`.
4. The adapter owns its own execution evidence type.
5. FR-019, FR-022 and the test matrix gain the kind's rows. For the process-provider variant FR-019,
   FR-022 and the matrix have them (IR-629).

### Amendment (IR-629): the process-provider variant

QSL ADR-029 PV-4 gives `BackendKind` one variant for process providers. Its `negotiate_*` arm settles
from the provider's manifest alone and never calls the plugin
([FR-019](../routed/functional/FR-019-capability-settlement.md)). Q4's step 2 applies to it as to any
variant. The QSL planner answered the questions the first version of this amendment left open
(QSL-637, answered 2026-10-05; its PV-4 amendment was not on QSL `origin/main` when this was written):

- The variant is `BackendKind::Process(BackendId)`. The plugin's identity is data inside the variant, and
  `from_identity` keeps mapping the built-in identities only. A descriptor carries a typed origin
  (`Linked` or `Process`, set by QSL's registry builder at the one conversion from a plugin `hello`;
  QSL-637's amendment of 2026-10-05, not yet in QSL's spec), and this crate's settlement maps origin
  `Process` to `Process(id)` and infers it from nothing else.
- Only negotiation is this crate's. Its arm checks the advertised (kind, mode) pairs, domains and bounds.
  Generation gives `KindOutput::Process` with no artifact, because the plugin receives the v2 package
  bytes. The adapter, execution and terminal record are the driver's plugin host and its typed FR-331
  reader (QSL ADR-029 PL-7), so every arm of this crate over the variant is a typed pass-through or an
  empty output, never a panic. This repeats Q4's rule that each match has an arm, with the arm stated.
- A plugin that declares `kani` conflicts with the built-in registration in QSL's registry, which
  withdraws both. That is decided at registration in QSL, independent of order, and this crate keeps no
  logic for it.

FR-019 and FR-022 state the criteria. Still open: the variant's serialized label, the QSpec FR-290
cause for an unadvertised domain or an uncovered bound, what carries the item's and the descriptor's
domains and bounds and what "covers" means (FR-019 open question 7), and the origin landing in QSL.

## Consequences

- FR-026 and FR-029 state the adapter trait and the terminal-record map.
- FR-019 and FR-022 gain rows when a backend kind is added: FR-019 and FR-022 have the process-provider
  rows.

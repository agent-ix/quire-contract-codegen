---
id: FR-030
title: "Map every Contract IR Kani outcome to exactly one QSL terminal value, preserving refusal causes"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-031
    type: references
---
# FR-030: Map every Contract IR Kani outcome to exactly one QSL terminal value, preserving refusal causes

## Description

The generator shall map every Contract IR `KaniOutcome`, paired with the settlement of its replay, to
exactly one `qsl_replay::TerminalValue`. The map is total over the pair (outcome, replay
settlement), not over the outcome alone, and it preserves the outcome's refusal cause: the outcome
kinds that QSL's terminal value collapses into one variant stay distinguishable through that
variant's typed cause. This is QSL ADR-013 C-09 as merged, whose two inputs are the IR `KaniOutcome`
and, for a `Counterexample`, the result of its replay. The replay settlement, its six readings and
the rules that classify them are those of [FR-029](./FR-029-run-outcome-terminal-record.md), which
this requirement refers to and does not restate; they apply here to a `Counterexample` exactly as
they apply there to `falsified`.

Contract IR retired its own outcome-to-terminal map (Contract IR FR-031-AC-5, Linear IR-358) because the
terminal value belongs to QSL and Contract IR must not depend on QSL. This repository owns the map.

Two maps exist and their domains do not overlap. [FR-029](./FR-029-run-outcome-terminal-record.md)
maps this repository's own `KaniRunOutcome`, which describes a run this repository executed. This
requirement maps a `KaniOutcome` that Contract IR's Kani boundary produced and handed in. One run
has one outcome of one of those two types and so one terminal value. Precedence: when this
repository executed the run, FR-029 governs and FR-030 is not applied to any outcome derived from
that run; FR-030 applies only to an outcome that arrived from Contract IR and was not produced by
a run of this repository. Items settled at negotiation (`unsupported`, `requires-bound`,
`invalid-request`; [FR-019](../../routed/functional/FR-019-capability-settlement.md)) have no `KaniOutcome` and no
terminal value, so neither map applies to them.

## Inputs

- One `quire_contract_ir::kani::KaniOutcome`: its closed `KaniOutcomeKind` and its stable cause
  `code`. The IR outcome carries kind, `code`, `source_id` and `context` only; it has no
  SUCCESS-check count.
- For a `Proved` outcome, the SUCCESS-check count taken from the Kani transcript this generator
  parsed, passed to the map as an explicit input, as [FR-029](./FR-029-run-outcome-terminal-record.md)
  takes it.
- For a `Counterexample` outcome, the replay settlement of its counterexample, as
  [FR-029](./FR-029-run-outcome-terminal-record.md) states it. No other kind takes a settlement.

## Outputs

- One `qsl_replay::TerminalValue`.
- A typed refusal, `TerminalPairError`, for a pair the map's input does not express: a
  `Counterexample` with no replay settlement (`MissingSettlement`), or any other kind with one
  (`UnexpectedSettlement`).

## Behavior

- The generator shall map outcomes in exactly one function, public so the driver calls it, whose
  `match` over the pair (`KaniOutcomeKind`, replay settlement) has no wildcard arm, so a kind added
  to Contract IR fails to compile here.
- The generator shall map each outcome kind as the table states:

  | Contract IR kind | Result |
  |---|---|
  | `Proved`, with `n` SUCCESS checks from the transcript, `n` at least one | `Proved { success_checks: n }` |
  | `Proved`, with zero SUCCESS checks from the transcript | `Proved { success_checks: 0 }` |
  | `Counterexample`, with a reproduced replay | `Refuted` |
  | `Counterexample`, with a replay disagreement | `Inconclusive(InconclusiveCause::ReplayParity)` |
  | `Counterexample`, with a non-fault replay refusal | `Inconclusive(InconclusiveCause::ReplayRefused)` carrying the refusal's QSL catalog code |
  | `Counterexample`, with a setup refusal on data (a non-fault `CallSiteRefusal` or `DependencyLockError::Input`) | `Inconclusive(InconclusiveCause::ReplayRefused)` carrying the refusal's QSL catalog code |
  | `Counterexample`, with a fault or a CG defect (the errors FR-029-AC-11 lists) | `Failed` |
  | `Refused` | `Declined { cause: ProofRefusalCause::Refused, code: DeclineCode::Std001(c) }`, `c` the outcome's `Std001Code` |
  | `InvalidInput` | `Declined { cause: ProofRefusalCause::InvalidInput, code: DeclineCode::Std001(c) }`, `c` the outcome's `Std001Code` |
  | `IncompleteInput` | `Declined { cause: ProofRefusalCause::IncompleteInput, code: DeclineCode::Std001(c) }`, `c` the outcome's `Std001Code` |
  | `Unavailable` with cause `kani_solver_absent` | `Unsupported(UnavailabilityCause::SolverAbsent)` |
  | `Unavailable` with cause `kani_backend_absent` | `Unsupported(UnavailabilityCause::BackendAbsent)` |
  | `Unavailable`, any other cause | `Unsupported(UnavailabilityCause::BackendAbsent)` |
  | `TimedOut` | `Incomplete(IncompleteCause::TimedOut)` |
  | `ResourceExhausted` | `Incomplete(IncompleteCause::ResourceExhausted)` |
  | `Cancelled` | `Incomplete(IncompleteCause::Cancelled)` |
  | `Inconclusive` with cause `kani_vacuous_proof` | `Proved { success_checks: 0 }` |
  | `Inconclusive`, any other cause | `Failed` |

- The generator shall produce `Declined` only from `Refused`, `InvalidInput` and `IncompleteInput`,
  which refuse the obligation's own input before Kani runs, so nothing was proved.
- The generator shall not map a setup refusal that arises after Kani refuted to `Declined`: a
  non-fault `CallSiteRefusal` and `DependencyLockError::Input` are `Inconclusive(ReplayRefused)`
  with their QSL code, as [FR-029](./FR-029-run-outcome-terminal-record.md) states, because QSL
  reserves `Declined` for a refusal before any backend run. A repeated dependency identity is
  refused by QSL's `DependencyInput::new` and arrives as `DependencyLockError::Input` with
  `invalid_package`, so it is `ReplayRefused` too (FR-029-AC-14).
  `ReplayPackageError::InvalidFunction` and `FrameReplayError::Name` carry no QSL code and map to
  `Failed`.
- The generator shall refuse a `Counterexample` given no replay settlement with
  `TerminalPairError::MissingSettlement`, and any other kind given a settlement with
  `TerminalPairError::UnexpectedSettlement`, and shall return no terminal value for either. The
  refusal is typed rather than a value, as in [FR-029](./FR-029-run-outcome-terminal-record.md)
  (FR-029-AC-15).
- The generator shall map a `Counterexample` to `Refuted` only with a reproduced replay.
- The generator shall classify a fault and the CG-raised failures FR-029-AC-11 lists as
  [FR-029](./FR-029-run-outcome-terminal-record.md) states: by walking the whole error, and with
  `ReplayRefused` carrying only QSL's closed catalog of codes.
- The generator shall expose no `proof_category` function
  ([AD-003](../../assurance/AD-003-evidence-chain.md) E-9).
- The generator shall map no outcome to `Tested`.
- The generator shall use QSL's terminal-value type, defining none of its own.
- The generator shall not read the outcome's `source_id` or `context` to choose the result. It reads
  the kind, and the `code` to choose the result only for `Unavailable` and `Inconclusive`, as the
  table states. For `Refused`, `InvalidInput` and `IncompleteInput` it carries the `code` unchanged
  into `Declined` as `DeclineCode::Std001`, and chooses nothing by it.
- The generator shall carry the `Declined` code as `DeclineCode::Std001` and never as a QSL catalog
  `Code`. It records no issuing registry and does not refuse a code because
  `Std001Code::is_registered` is false: a code this repository itself mints
  (`kani_corpus_identity_collision`) is carried like any other.
- The map preserves the refusal kind and, for `Declined`, the code. For every other kind the
  outcome's stable `code` is not carried into the result, because `TerminalRecord` holds an item
  and a value only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-030-AC-1 | Every pair (`KaniOutcomeKind`, replay settlement) that the map's input can express maps to exactly one `TerminalValue`. | Test (TC-041) |
| FR-030-AC-2 | `Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively, so no refusal cause is lost, and each carries the outcome's `Std001Code` unchanged as `DeclineCode::Std001`, including a code STD-001 does not register. | Test (TC-041) |
| FR-030-AC-3 | `TimedOut`, `ResourceExhausted` and `Cancelled` map to `Incomplete` with `IncompleteCause::TimedOut`, `ResourceExhausted` and `Cancelled` respectively. | Test (TC-041) |
| FR-030-AC-4 | `Proved` with a transcript count of three SUCCESS checks maps to `Proved { success_checks: 3 }`, `Proved` with a count of zero maps to `Proved { success_checks: 0 }`, and `Counterexample` with a reproduced replay maps to `Refuted`. | Test (TC-041) |
| FR-030-AC-5 | `Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { success_checks: 0 }`, and `Inconclusive` with any other cause maps to `Failed`. | Test (TC-041) |
| FR-030-AC-6 | No outcome maps to `Tested`. | Test (TC-041) |
| FR-030-AC-7 | The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement) with no wildcard arm. | Inspection (TC-041) |
| FR-030-AC-8 | `Unavailable` with cause `kani_solver_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`. | Test (TC-041) |
| FR-030-AC-9 | `Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`, and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()` of that refusal. | Test (TC-041) |
| FR-030-AC-10 | `Counterexample` with a fault, walked through every wrapper FR-029-AC-10 lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`. | Test (TC-041) |
| FR-030-AC-11 | Across every replay settlement other than reproduced, `Counterexample` maps to a value other than `Refuted`. | Test (TC-041) |
| FR-030-AC-12 | `Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal's QSL catalog code, never to `Declined`. | Test (TC-041) |
| FR-030-AC-13 | `Counterexample` with a `DependencyLockError::Input` that carries QSL's `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect is a repeated library identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`. | Test (TC-041) |
| FR-030-AC-14 | A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`; neither returns a terminal value. | Test (TC-041) |

## Dependencies

- **Upstream**: Contract IR's `KaniOutcome` (its FR-030, FR-031); QSL's `qsl-replay`, which defines
  `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011 T-13;
  and FR-121; QSpec FR-331; [FR-029](./FR-029-run-outcome-terminal-record.md). The `Inconclusive`
  terminal value and its `ReplayParity` and `ReplayRefused` causes, and `Declined`'s `DeclineCode`
  with its `Std001` arm, are merged in QSL `main` (QSL #634); `qsl-replay` re-exports `Std001Code`
  and `std001_code!` from its root, so this repository builds the `Declined` arm through
  `qsl-replay` alone.
- **Downstream**: [TC-041](../matrix/TC-041-ir-outcome-terminal-map.md).

## Status

Implemented except FR-030-AC-10 (Linear IR-465, IR-358). `ir_outcome_terminal_value` in
`kani/terminal.rs` is the one `match` over the pair (`KaniOutcomeKind`, replay settlement), with no
wildcard arm, built on `qsl-replay` at QSL `main` (the commit is informational; the lock names it): `TerminalValue` has `Inconclusive` with
`ReplayParity` and `ReplayRefused(Code)`, and `Declined { cause, code: DeclineCode }`, whose
`DeclineCode` is `Qsl(Code)` or `Std001(Std001Code)`. `Refused`, `InvalidInput` and
`IncompleteInput` map to `Declined` with the outcome's own `Std001Code` as `DeclineCode::Std001`;
IR's cause (for example `kani_identity_invalid`) is never spelled as a QSL catalog code. The
replay-settlement half of the map is the one built for FR-029 (`ReplaySettlement` and its
conversions), reused unchanged. QSL ruled, relayed on IR-465 (a QSL ruling recorded by the
planner), that vacuity stays `Proved { success_checks: 0 }` and that a setup refusal after a
refutation is `ReplayRefused`, as [FR-029](./FR-029-run-outcome-terminal-record.md)'s Status
states.

FR-030-AC-10 stays planned and unbacked, for the reason FR-029-AC-10 does: its fault half names QSL's
`InternalFault` wrappers, whose tests are not yet written. `qsl-replay` now re-exports
`InternalFault` and it is constructible (QSL `main` bcca433, QSL #635), so a follow-up code change
can back it. The CG-raised failures (the other half) and the
`ReplaySettlement::Fault` reading are asserted by a test traced to TC-041 only, and carry no
FR-030-AC-10 tag, so the criterion does not read as backed. It follows FR-029-AC-10.

When a lock has several defects, QSL's `DependencyInput::new` reports the first by its own order
(libraries in supply order; for each, an empty identity, then a repeated identity, then a shared
source owner), which differs from the order CG's removed pre-check imposed. FR-030-AC-13 is
therefore limited to a lock whose only defect is the repeated identity.

The `Declined` code. `Declined { cause, code }` carries a `DeclineCode`, either a QSL catalog `Code`
or a STD-001 registry code (QSL #634, merged). The map's input, Contract IR's `KaniOutcome` at the
revision CG locks (IR `dec8ade`), is a kind plus a `quire_contract_model::Std001Code` (for example
`kani_identity_invalid` or `kani_population_incomplete`), which is not a QSL catalog `Code`. QSL
takes IR's type, since it already depends on `quire-contract-model`: IR exports `Std001Code` from
`quire-contract-model` (IR FR-044) and `KaniOutcome.code` is that type, not `String` (IR-605,
merged). `Std001Code` guarantees the STD-001 code form only, not the issuing registry, so a code CG
itself mints (`kani_corpus_identity_collision`) is a valid `Std001Code` that STD-001 does not list.
QSL's `DeclineCode::Std001` names the registry and records no issuer, and QSL does not refuse an
unregistered code; this map therefore carries the code unchanged and does not refuse one either. IR's
incomplete population is not the replay's missing byte-provision input; the codes are distinct.
FR-030-AC-2 asserts the cause and the code.

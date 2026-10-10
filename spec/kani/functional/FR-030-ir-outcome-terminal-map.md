---
id: FR-030
title: "Map Kani and SMT proof outcomes to QSL terminal values with explicit proof certification"
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
  - target: ix://agent-ix/quire-spec-language/FR-127
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-314
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-031
    type: references
---
# FR-030: Map Kani and SMT proof outcomes to QSL terminal values with explicit proof certification

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

Two Kani maps exist and their domains do not overlap. [FR-029](./FR-029-run-outcome-terminal-record.md)
maps this repository's own `KaniRunOutcome`, which describes a run this repository executed. This
requirement maps a `KaniOutcome` that Contract IR's Kani boundary produced and handed in. One Kani
run has one outcome of one of those two types and so one terminal value. Precedence: when this
repository executed the run, FR-029 governs and FR-030 is not applied to any outcome derived from
that run; FR-030 applies only to an outcome that arrived from Contract IR and was not produced by
a run of this repository. Items settled at negotiation (`unsupported`, `requires-bound`,
`invalid-request`; [FR-019](../../routed/functional/FR-019-capability-settlement.md)) have no `KaniOutcome` and no
terminal value, so neither map applies to them. The SMT map below has a distinct input domain: a
registered SMT backend result, not a `KaniOutcome` or `KaniRunOutcome`. It preserves the qualified
Kani path and cannot relabel a Kani result as an SMT proof.

## Inputs

- One `quire_contract_ir::kani::KaniOutcome`: its closed `KaniOutcomeKind` and its stable cause
  `code`. The IR outcome carries kind, `code`, `source_id` and `context` only; it has no
  SUCCESS-check count.
- For a `Proved` outcome, the SUCCESS-check count taken from the Kani transcript this generator
  parsed, passed to the map as an explicit input, as [FR-029](./FR-029-run-outcome-terminal-record.md)
  takes it.
- For a `Counterexample` outcome, the replay settlement of its counterexample, as
  [FR-029](./FR-029-run-outcome-terminal-record.md) states it. No other kind takes a settlement.
- IR-666 direct report converter; IR-635 original-artifact invocation planned: for an IR `Counterexample` that represents a falsified composite
  `bounded_shadow` parity claim, the distinct typed QSL `CompositeParityReport` and its full
  `CompositeIdentity` binding. It is not the ordinary source-predicate `ReplaySettlement` above;
  [FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md) owns its construction.
- For an accepted SMT item, the registered backend's bounded-complete or inductive proof basis
  and depth, its solver verdict, optional Alethe certificate, QSL FR-314 checker result, and
  independent Z3 cross-check result. A counterexample also supplies its FR-197 replay result.
  The checker receives the original admitted item and FR-315 canonical queries; CG does not
  declare a second certificate checker.

## Outputs

- One `qsl_replay::TerminalValue`.
- A typed refusal, `TerminalPairError`, for a pair the map's input does not express: a
  `Counterexample` with no replay settlement (`MissingSettlement`), or any other kind with one
  (`UnexpectedSettlement`).
- IR-666 direct report converter; IR-635 original-artifact invocation planned: the terminal value and report result of the same-claim composite
  parity settlement, or its typed claim-binding refusal with no value.
- For an SMT item, one FR-331 terminal result with its proof basis and exactly one certification
  when `proved`, or an explicit non-proof result and cause. The independent cross-check stays
  alongside the result.

## Behavior

- The generator shall map outcomes in exactly one function, public so the driver calls it, whose
  `match` over the pair (`KaniOutcomeKind`, replay settlement) has no wildcard arm, so a kind added
  to Contract IR fails to compile here.
- The generator shall map each outcome kind as the table states:

  | Contract IR kind | Result | Proof basis / certification |
  |---|---|---|
  | `Proved`, with `n` SUCCESS checks from the transcript, `n` at least one | `Proved { basis: ProofBasis::Checks { success_checks: n }, certification: Certification::Certified }` | `Checks { success_checks: n }` / `Certified` |
  | `Proved`, with zero SUCCESS checks from the transcript | `Proved { basis: ProofBasis::Checks { success_checks: 0 }, certification: Certification::Certified }`; QSL reports inconclusive/vacuous | `Checks { success_checks: 0 }` / `Certified` |
  | `Counterexample`, with a reproduced replay | `Refuted` | none / none |
  | `Counterexample`, with a replay disagreement | `Inconclusive(InconclusiveCause::ReplayParity)` | none / none |
  | `Counterexample`, with a non-fault replay refusal | `Inconclusive(InconclusiveCause::ReplayRefused)` carrying the refusal's QSL catalog code | none / none |
  | `Counterexample`, with a setup refusal on data (a non-fault `CallSiteRefusal` or `DependencyLockError::Input`) | `Inconclusive(InconclusiveCause::ReplayRefused)` carrying the refusal's QSL catalog code | none / none |
  | `Counterexample`, with a fault or a CG defect (the errors FR-029-AC-11 lists) | `Failed` | none / none |
  | `Refused` | `Declined { cause: ProofRefusalCause::Refused, code: DeclineCode::Std001(c) }`, `c` the outcome's `Std001Code` | none / none |
  | `InvalidInput` | `Declined { cause: ProofRefusalCause::InvalidInput, code: DeclineCode::Std001(c) }`, `c` the outcome's `Std001Code` | none / none |
  | `IncompleteInput` | `Declined { cause: ProofRefusalCause::IncompleteInput, code: DeclineCode::Std001(c) }`, `c` the outcome's `Std001Code` | none / none |
  | `Unavailable` with cause `kani_solver_absent` | `Unsupported(UnavailabilityCause::SolverAbsent)` | none / none |
  | `Unavailable` with cause `kani_backend_absent` | `Unsupported(UnavailabilityCause::BackendAbsent)` | none / none |
  | `Unavailable`, any other cause | `Unsupported(UnavailabilityCause::BackendAbsent)` | none / none |
  | `TimedOut` | `Incomplete(IncompleteCause::TimedOut)` | none / none |
  | `ResourceExhausted` | `Incomplete(IncompleteCause::ResourceExhausted)` | none / none |
  | `Cancelled` | `Incomplete(IncompleteCause::Cancelled)` | none / none |
  | `Inconclusive` with cause `kani_vacuous_proof` | `Proved { basis: ProofBasis::Checks { success_checks: 0 }, certification: Certification::Certified }`; QSL reports inconclusive/vacuous | `Checks { success_checks: 0 }` / `Certified` |
  | `Inconclusive`, any other cause | `Failed` | none / none |

The Kani rows are a qualified-core prove path (QSL FR-127); every `Proved` payload they
produce carries `ProofBasis::Checks` and `Certification::Certified`, including a zero-check
payload whose *category* is inconclusive. A non-`Proved` terminal value carries neither proof
basis nor certification. The map SHALL NOT infer either field from a result label alone.

For an accepted item routed to the registered SMT backend, the generator SHALL map the backend
outcome and QSL's checker/replay settlement to exactly one terminal result as follows. The
SMT `Proved` rows carry the backend's actual `BoundedComplete { depth }` or
`Inductive { depth }` basis without changing depth. The same rows apply to both bases.

| SMT input and settlement | Terminal result | Proof basis / certification |
|---|---|---|
| Solver proves; QSL FR-314 verifies every query and checked Alethe step | `Proved { basis, certification: Certified }` | actual `BoundedComplete { depth }` or `Inductive { depth }` / `Certified` |
| Solver proves; no certificate accompanies the proof | `Proved { basis, certification: Uncertified }` | actual `BoundedComplete { depth }` or `Inductive { depth }` / `Uncertified` |
| Solver proves; FR-314 returns `Unverifiable` for an unchecked rule, with no failed check | `Proved { basis, certification: Uncertified }`, retaining the first unchecked step | actual `BoundedComplete { depth }` or `Inductive { depth }` / `Uncertified` |
| Solver proves; FR-314 rejects a query mismatch, shape mismatch, invalid checked step or proof without empty-clause refutation | `Inconclusive(CertificateRejected { rule, at })`, retaining QSL's exact rule and locus; no proof basis or certification | none / none |
| FR-314 rejects the certificate and cvc5 and configured Z3 disagree on the same query | `Inconclusive(CertificateRejected { rule, at })` is the one terminal cause; retain both solver verdicts and the parity failure alongside it | none / none |
| cvc5 and configured Z3 disagree on the same query, with no rejected certificate | `Inconclusive`, retaining both verdicts and their parity failure; no proof basis or certification | none / none |
| Solver refutes; FR-197 replay reproduces the counterexample | `Refuted`; no proof basis or certification | none / none |
| Solver refutes; replay disagrees or refuses without fault | `Inconclusive` with the typed replay cause; no proof basis or certification | none / none |
| Solver refutes; replay or CG faults | `Failed`; no proof basis or certification | none / none |
| Solver returns inconclusive or stops before a verdict | The corresponding QSL typed `Inconclusive`, `Incomplete` or `Failed` cause, never `Proved` | none / none |

The SMT map SHALL use QSL's terminal type and FR-127/FR-314 settlement. It SHALL NOT
construct `Certification::Trusted`: QSpec FR-354 reserves `trusted` for a third-party plugin.
A missing certificate or unchecked rule is an explicit `Uncertified` proof, while a certificate
shown wrong is `CertificateRejected` and cannot settle `Proved`. When certificate rejection and
cvc5/Z3 disagreement coincide, QSL FR-127/FR-314 and QSpec FR-354 require
`CertificateRejected { rule, at }` as the one terminal cause; the FR-349 parity failure, including
both verdicts, remains independent evidence beside that terminal. A Z3 disagreement without
certificate rejection takes the FR-349 inconclusive row; an agreeing cross-check does not upgrade certification, and an absent
Z3 cross-check carries FR-349's warning without changing cvc5's result. An SMT item rejected
during capability negotiation under FR-019 or QSpec FR-349 receives its negotiation disposition
and no proof terminal from this map. Every settled SMT terminal is accounted once; no SMT row
is `Tested`, and no SMT result is passed through the Kani outcome map.

For an IR `Counterexample` representing a falsified composite `bounded_shadow` parity claim, the
generator shall consume QSL FR-358's `CompositeParityReport` through the distinct typed binding of
[FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md). After the common claim
checks, it shall preserve the report's first-applicable F-1 to F-7 result and terminal value:

| QSL row | Report result | CG terminal reading |
|---|---|---|
| F-1 | `Disagreed` | `Failed`, CG-owned `CgDefect`; wins over native, admission and all limits |
| F-2 | `GeneratedFault { native }` for native `Incomplete` or `ExecutionFault` | `Failed`, retaining the actual native outcome and `NativeCause`; wins over an invalid operand and later limits |
| F-3 | `RefusedInput(OperandRefusal)` | `Inconclusive(ReplayRefused(code))`, retaining the operand index and QSL code; wins over `CeilingReached` |
| F-4 | `Incomplete { stage: Admission(incomplete) }` | `Incomplete(ResourceExhausted)`, retaining the request accounting counter, configured limit and count reached; QSL owns skipped exact evaluation |
| F-5 | `Incomplete { stage: ExactEvaluation(incomplete) }` | `Incomplete(ResourceExhausted)`, retaining QSL's exact counter, configured limit and count reached; wins over `CeilingReached` |
| F-6 | `Incomplete { stage: RefinementCeiling }` | `Incomplete(ResourceExhausted)`, retaining the actual refinement ceiling evidence |
| F-7 | `Diverged` or `Agrees` | `Failed`/CG-owned `CgDefect` for a verdict or pair-count divergence; `Inconclusive(ScalarAgrees)` with `CompositeEquality` claim and `Equality` outcome for agreement |

The generator shall keep the three `Incomplete` stages distinct in the report although their
terminal values agree. CG prechecks, missing reports and failed full-identity binding return typed
refusals without a report terminal value, as FR-033 owns. QSL's `prepare` refusal instead returns
a `CompositeParityResult::Refused` report before F-1. QSL may also return `Refused` from operand
admission after F-1/F-2 or from exact comparison after admission; Disagreed wins over those later
refusals. After full identity binding, a non-fault
`ReplayRefusal` yields `Inconclusive(ReplayRefused(code))` with QSL's code;
`ReplayRefusal::Fault` or `ReplayRefusal::Admission(AdmissionFailure::Fault)` yields `Failed`.
The generator shall read that report's terminal value without inventing a CG code. No F row is `Refuted`; native
`Completed` and `Refused` are evidence rather than a settlement oracle. Backend timeout or memory
exhaustion remains the ordinary `TimedOut` or `ResourceExhausted` row above and never enters the
composite facade. QSL #645 owns these rows. IR-666 tests their public reports through CG's binding converter; IR-635 still owns original-artifact construction and invocation.

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
| FR-030-AC-4 | `Proved` with three SUCCESS checks maps to `Proved { basis: Checks { success_checks: 3 }, certification: Certified }`, zero checks maps to `Proved { basis: Checks { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous), and `Counterexample` with reproduced replay maps to `Refuted`. | Test (TC-041) |
| FR-030-AC-5 | `Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { basis: Checks { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous), and `Inconclusive` with any other cause maps to `Failed`. | Test (TC-041) |
| FR-030-AC-6 | No outcome maps to `Tested`. | Test (TC-041) |
| FR-030-AC-7 | The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement) with no wildcard arm. | Inspection (TC-041) |
| FR-030-AC-8 | `Unavailable` with cause `kani_solver_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`. | Test (TC-041) |
| FR-030-AC-9 | `Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`, and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()` of that refusal. | Test (TC-041) |
| FR-030-AC-10 | `Counterexample` with a fault, walked through every wrapper FR-029-AC-10 lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`. | Test (TC-041) |
| FR-030-AC-11 | Across every replay settlement other than reproduced, `Counterexample` maps to a value other than `Refuted`. | Test (TC-041) |
| FR-030-AC-12 | `Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal's QSL catalog code, never to `Declined`. | Test (TC-041) |
| FR-030-AC-13 | `Counterexample` with a `DependencyLockError::Input` that carries QSL's `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect is a repeated library identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`. | Test (TC-041) |
| FR-030-AC-14 | A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`; neither returns a terminal value. | Test (TC-041) |
| FR-030-AC-15 | PARTIAL (IR-666 direct public report controls; IR-635 must reach the same rows from a CG-built request over the original proving context). For an identity-valid IR `Counterexample` composite claim, F-1 Disagreed yields Failed/CgDefect even with a native fault or invalid operand; without Disagreed, native Incomplete and ExecutionFault each yield a binding-checked GeneratedFault/Failed report with the actual NativeCause even when an operand would refuse, admission or exact limits would stop, or refinement is CeilingReached. CG asserts the typed result and terminal value; QSL FR-358 owns the internal F-2 no-admission/no-exact-evaluation rule. | Test |
| FR-030-AC-16 | PARTIAL (IR-666 direct public report controls; IR-635 must reach the same rows from a CG-built request over the original proving context). With valid identity, no Disagreed and Completed native, an out-of-domain operand yields F-3 RefusedInput/ReplayRefused with its index and QSL code even with CeilingReached; a request accounting limit reached during admission yields F-4 Incomplete(ResourceExhausted)/Admission with the counter; an exact limit reached after admission yields F-5 Incomplete(ResourceExhausted)/ExactEvaluation even with CeilingReached; with sufficient limits, CeilingReached yields F-6 Incomplete(ResourceExhausted)/RefinementCeiling. CG asserts the distinct report stages and their terminal values; QSL FR-358 owns the internal F-4 no-exact-evaluation rule. No case becomes Tested or Refuted. | Test |
| FR-030-AC-17 | PARTIAL (IR-666 F-7 and binding controls; IR-635 original-artifact path planned). With valid identity and all earlier rows absent, changing only the retained shadow verdict or pair count yields F-7 Diverged/Failed/CgDefect; agreement yields Inconclusive(ScalarAgrees) with the CompositeEquality claim and Equality outcome. A binding-valid QSL `prepare` non-fault `Refused` report precedes Disagreed and yields Inconclusive(ReplayRefused) with QSL's code. `Refused` from later admission or exact comparison follows Disagreed; if QSL returns `Fault` or `Admission(Fault)`, its bound report yields Failed as FR-029-AC-28 states. CG prechecks, an absent report or a mismatched full `CompositeIdentity` yield no terminal value under FR-033-AC-9. The ordinary source-predicate MissingSettlement/UnexpectedSettlement rule does not reject a valid typed composite report. | Test |
| FR-030-AC-18 | Every Kani `Proved` row carries `ProofBasis::Checks` with the transcript count and `Certification::Certified`, including the zero-check row; zero checks remain category inconclusive/vacuous. No Kani non-`Proved` row carries basis or certification. | Test (TC-041) |
| FR-030-AC-19 | For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT proof whose certificate FR-314 verifies maps to `Proved` with the same basis/depth and `Certified`; no certificate or an `Unverifiable` unchecked rule maps to `Proved` with the same basis/depth and `Uncertified`, retaining the unverifiable locus when present. No SMT proof maps to `Trusted`. | Test (TC-041) |
| FR-030-AC-20 | For either SMT basis, a foreign query, wrong certificate shape, invalid checked proof step or missing final empty clause rejected by FR-314 maps to `Inconclusive(CertificateRejected { rule, at })` retaining the exact rejection; none produces `Proved`, basis or certification. When cvc5 and Z3 also disagree, `CertificateRejected` remains the one terminal cause while the parity failure and both verdicts are retained alongside it. | Test (TC-041) |
| FR-030-AC-21 | SMT/cross-check disagreement without certificate rejection keeps both verdicts and settles inconclusive; agreement does not change certification; missing Z3 retains its warning and cvc5 settlement. An SMT counterexample becomes `Refuted` only after FR-197 replay reproduces it; replay disagreement, non-fault refusal and fault map to their distinct non-proof results. | Test (TC-041) |
| FR-030-AC-22 | Every expressible accepted SMT backend/checker/replay settlement produces one terminal result, never `Tested`; no non-`Proved` result carries basis or certification. Negotiation refusals produce no terminal from this map, and neither SMT nor Kani map consumes the other backend's outcome. | Test (TC-041) |

## Dependencies

- **Upstream**: Contract IR's `KaniOutcome` (its FR-030, FR-031); QSL's `qsl-replay`, which defines
  `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011 T-13;
  FR-127, FR-314 and FR-121; QSpec FR-331, FR-349 and FR-354;
  [FR-029](./FR-029-run-outcome-terminal-record.md). The `Inconclusive`
  terminal value and its `ReplayParity` and `ReplayRefused` causes, and `Declined`'s `DeclineCode`
  with its `Std001` arm, are merged in QSL `main` (QSL #634); `qsl-replay` re-exports `Std001Code`
  and `std001_code!` from its root, so this repository builds the `Declined` arm through
  `qsl-replay` alone.
- **Downstream**: [TC-041](../matrix/TC-041-ir-outcome-terminal-map.md).
- **SMT producer**: QSpec FR-349's registered cvc5 backend produces the SMT verdict and
  `BoundedComplete` or `Inductive` proof. QSL FR-127 owns settlement and runs FR-314's
  certificate checker before a terminal is returned; CG's SMT map supplies the backend result
  and preserves that settlement. QSpec FR-354 owns the consumer certification meanings.
- **Composite consumer**: QSL FR-358 (F-1 to F-7, merged in #645),
  [FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md) and
  [FR-029](./FR-029-run-outcome-terminal-record.md) own the same-claim parity binding and
  source-run map; [TC-041](../matrix/TC-041-ir-outcome-terminal-map.md) verifies this IR input.

## Status

The existing Kani map is implemented (Linear IR-465, IR-358). Its proof payload still has
the count-only shape; the `ProofBasis`/`Certification` amendment and the separate SMT map
are planned and require QSL FR-127/FR-314 and QSpec FR-349 to become available in code.
`ir_outcome_terminal_value` in
`kani/terminal.rs` is the one `match` over the pair (`KaniOutcomeKind`, replay settlement), with no
wildcard arm, built on `qsl-replay`: `TerminalValue` has `Inconclusive` with
`ReplayParity` and `ReplayRefused(Code)`, and `Declined { cause, code: DeclineCode }`, whose
`DeclineCode` is `Qsl(Code)` or `Std001(Std001Code)`. `Refused`, `InvalidInput` and
`IncompleteInput` map to `Declined` with the outcome's own `Std001Code` as `DeclineCode::Std001`;
IR's cause (for example `kani_identity_invalid`) is never spelled as a QSL catalog code. The
replay-settlement half of the map is the one built for FR-029 (`ReplaySettlement` and its
conversions), reused unchanged. QSL ruled, relayed on IR-465 (a QSL ruling recorded by the
planner), that vacuity stays `Proved { success_checks: 0 }` and that a setup refusal after a
refutation is `ReplayRefused`, as [FR-029](./FR-029-run-outcome-terminal-record.md)'s Status
states.

FR-030-AC-10 is backed: `tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed` builds each
fault wrapper FR-029-AC-10 lists from QSL's constructible `InternalFault` (re-exported by
`qsl-replay`, QSL #635) and each CG-raised failure FR-029-AC-11 lists, and
asserts `Failed` for a `Counterexample`. It follows FR-029-AC-10.

When a lock has several defects, QSL's `DependencyInput::new` reports the first by its own order
(libraries in supply order; for each, an empty identity, then a repeated identity, then a shared
source owner), which differs from the order CG's removed pre-check imposed. FR-030-AC-13 is
therefore limited to a lock whose only defect is the repeated identity.

The `Declined` code. `Declined { cause, code }` carries a `DeclineCode`, either a QSL catalog `Code`
or a STD-001 registry code (QSL #634, merged). The map's input, Contract IR's `KaniOutcome` at the
revision CG consumes, is a kind plus a `quire_contract_model::Std001Code` (for example
`kani_identity_invalid` or `kani_population_incomplete`), which is not a QSL catalog `Code`. QSL
takes IR's type, since it already depends on `quire-contract-model`: IR exports `Std001Code` from
`quire-contract-model` (IR FR-044) and `KaniOutcome.code` is that type, not `String` (IR-605,
merged). `Std001Code` guarantees the STD-001 code form only, not the issuing registry, so a code CG
itself mints (`kani_corpus_identity_collision`) is a valid `Std001Code` that STD-001 does not list.
QSL's `DeclineCode::Std001` names the registry and records no issuer, and QSL does not refuse an
unregistered code; this map therefore carries the code unchanged and does not refuse one either. IR's
incomplete population is not the replay's missing byte-provision input; the codes are distinct.
FR-030-AC-2 asserts the cause and the code.

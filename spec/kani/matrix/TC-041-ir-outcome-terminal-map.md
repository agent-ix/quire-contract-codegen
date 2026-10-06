---
id: TC-041
title: "Verify the total map from a Contract IR Kani outcome to QSL's terminal value"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-041: Verify the total map from a Contract IR Kani outcome to QSL's terminal value

## Description

Verify that each pair of Contract IR `KaniOutcome` and replay settlement maps to the QSL terminal
value FR-030's table states, that refusal kinds survive the map, that a `Counterexample` is
`Refuted` only with a reproduced replay, that the `Unavailable` cause code selects the unavailability
cause, and that the map is one match with no wildcard arm.

## Test Procedure

1. Map an outcome of every `KaniOutcomeKind` and collect the values.
2. Map `Refused`, `InvalidInput` and `IncompleteInput`, each with an IR `Std001Code`, one of them a
   code STD-001 does not register.
3. Map `TimedOut`, `ResourceExhausted` and `Cancelled`.
4. Map `Proved` with a transcript count of three SUCCESS checks and with a count of zero, and map
   `Counterexample` with a reproduced replay.
5. Map `Inconclusive` with cause `kani_vacuous_proof`, and with another cause.
6. Map `Unavailable` with cause `kani_solver_absent`, with `kani_backend_absent`, and with another
   cause.
7. Inspect the map's source for a wildcard arm (inspection step, FR-030-AC-7).
8. Map `Counterexample` with a replay disagreement, and with a non-fault `ReplayRefusal`.
9. Map `Counterexample` with a fault in each position FR-029-AC-10 lists, and with each CG-raised
   failure FR-029-AC-11 lists.
10. Map `Counterexample` under every replay settlement other than reproduced.
11. Map `Counterexample` with a non-fault `CallSiteRefusal` and with a `DependencyLockError::Input`,
    each bare and wrapped.
12. Map `Counterexample` with the refusal QSL's `DependencyInput::new` returns for a lock whose
    only defect is one library identity selected twice.
13. Map `Counterexample` with no replay settlement, and each other `KaniOutcomeKind` with one.
14. For an IR `Counterexample` carrying a valid falsified composite parity claim, consume a real
    QSL `CompositeParityReport` from the distinct typed route. Combine Disagreed with native
    ExecutionFault and an out-of-domain operand. Without Disagreed, use native Incomplete and
    ExecutionFault, each with an out-of-domain operand, too-small admission and exact limits, and
    CeilingReached. Inspect the retained native outcome and NativeCause.
15. With valid identity, Completed native and no Disagreed, exercise an out-of-domain operand with
    CeilingReached; a request accounting limit reached during admission with CeilingReached; a QSL
    exact limit reached after admission with CeilingReached; and CeilingReached with sufficient
    limits. Inspect the operand index/code and each incomplete report stage and counter.
16. With all earlier rows absent, change the retained shadow verdict and pair count independently,
    then make both agree. Through the public QSL facade, send a valid wire request with a replay
    input-byte limit below its encoded size to obtain a `prepare` non-fault `Refused` result with
    Disagreed; pass that actual report to CG's public converter and require full claim binding and
    F-1 precedence (FR-029-AC-28). With a valid `prepare`, pair
    Disagreed with an operand that would fail admission and verify F-1 wins. Inspect the public QSL
    terminal mapping and CG pass-through for `Fault` and `Admission(Fault)` reports without
    fabricating a report or claiming a public input can trigger an invariant fault. Also
    exercise a CG precheck refusal, a missing report and a report whose `CompositeIdentity`
    changes one observation member. Inspect whether a report and terminal record exist, and the
    actual result, QSL code and category when they do.

## Expected Results

1. Exactly one value per expressible pair, and none is `Tested` (FR-030-AC-1, FR-030-AC-6).
2. `Declined` with three distinct causes, each carrying the outcome's code unchanged as
   `DeclineCode::Std001`, the unregistered code included (FR-030-AC-2).
3. `Incomplete` with three distinct causes (FR-030-AC-3).
4. `Proved { success_checks: 3 }`, `Proved { success_checks: 0 }` and `Refuted` (FR-030-AC-4).
5. `Proved { success_checks: 0 }` and `Failed` (FR-030-AC-5).
6. `Unsupported(SolverAbsent)`, `Unsupported(BackendAbsent)` and `Unsupported(BackendAbsent)`
   (FR-030-AC-8).
7. The `match` over the pair has no wildcard arm (FR-030-AC-7).
8. `Inconclusive(ReplayParity)`, and `Inconclusive(ReplayRefused)` carrying the refusal's catalog
   code (FR-030-AC-9).
9. Each is `Failed` (FR-030-AC-10).
10. No value is `Refuted` (FR-030-AC-11).
11. The call-site and lock-input refusals are `Inconclusive(ReplayRefused)` carrying their QSL
    catalog code and none is `Declined` (FR-030-AC-12).
12. `Inconclusive(ReplayRefused)` carrying `invalid_package` (FR-030-AC-13).
13. `TerminalPairError::MissingSettlement` for the counterexample and `UnexpectedSettlement` for each
    other kind, with no terminal value (FR-030-AC-14).
14. Disagreed is F-1 Failed/CgDefect. Each native stop is F-2 GeneratedFault/Failed with its own
    NativeCause despite every later competing condition. CG asserts result and terminal evidence;
    QSL FR-358 owns skipped internal admission and exact work (FR-030-AC-15).
15. The invalid operand is F-3 RefusedInput/ReplayRefused with its index and QSL code. Admission
    accounting is F-4 Incomplete(ResourceExhausted)/Admission with its request counter. QSL FR-358
    owns skipped internal exact work. Exact exhaustion is F-5 Incomplete(ResourceExhausted)/ExactEvaluation, and only
    after exact evaluation completes does CeilingReached become F-6
    Incomplete(ResourceExhausted)/RefinementCeiling. The three report stages remain distinct and no
    case is Tested or Refuted (FR-030-AC-16).
16. Verdict and count divergence each give F-7 Failed/CgDefect; agreement gives
    Inconclusive(ScalarAgrees) with CompositeEquality/Equality. A binding-valid QSL `prepare`
    non-fault `Refused` report wins over Disagreed and yields Inconclusive(ReplayRefused) with its
    QSL code; Disagreed wins over later operand refusal. Inspection shows that an actual bound
    `Fault` or `Admission(Fault)` report would yield Failed, never ReplayRefused. A CG
    precheck refusal, missing report or wrong-claim report yields no terminal value, including when
    one observation member changes; the valid composite report is not `UnexpectedSettlement`
    (FR-029-AC-28; FR-030-AC-17).

## Status

Steps 1 to 13 are implemented in `tests/it/terminal_map.rs`; steps 14 to 16 are planned for the
IR-666 code consumer after QSL #645 and IR-665's gate repair. Step 7's inspection is a `syn`
test over `kani/terminal.rs`. Step 9 maps a `Counterexample` with `ReplaySettlement::Fault`, with
each fault wrapper FR-029-AC-10 lists (built from QSL's constructible `InternalFault`, QSL
bcca433), and with each CG-raised failure FR-029-AC-11 lists, in one test traced to FR-030-AC-10.

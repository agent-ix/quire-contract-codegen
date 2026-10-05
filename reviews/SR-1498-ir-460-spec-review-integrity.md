---
id: SR-1498
title: "Integrity review of quire-contract-codegen PR 276 (IR-460 state-clause replay spec)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@d22cc5d849db3edd7bba68c3638bb9c4995ac723; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md (PR #276 diff vs origin/main 7345463; QSL qsl-replay locked at c8f0c28)"
review_set: subset
---

## Summary

Ticket: IR-460. Checked the new FR-024 Behavior bullets and FR-024-AC-11..17 for completeness and consistency against the rest of FR-024, FR-015-AC-26/27/39, FR-016, and QSL's replay_state_clause at c8f0c28 (qsl-replay/src/execute/state_clause.rs, witness/state_clause.rs, result.rs). Verified true: the payload has members clause, observation and witness only; Postcondition requires ObservationForm::Invocation; an evaluated false settles violation, true settles inconclusive with DisagreementCause::Verdicts{proved: violation, replayed: success}; documents are read from the byte provision by sha256-jcs digest. Four consistency or completeness defects remain in FR-024 as edited.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024's existing out-of-domain and reconstruct-refusal Behavior bullets still say "shall not call `replay` or `replay_frame`", while AC-16 requires that replay_state_clause is not called either. Neither bullet was extended. Also the domain check is stated against "the declared domain of the parameter it binds", but a postcondition's drawn inputs are state fields bound through `self` (FR-015-AC-27/39 integer_range), so which domain AC-16 checks a state field against is not stated. | FR-024-AC-16, FR-024 Behavior lines 77-85 |
| FND-002 | medium | Inputs and Outputs are not updated for the state-clause path. It consumes the playback's pre-state field values and a native run's post-state values, and it produces CG-built invocation and snapshot documents that must enter the request's byte provision. Yet Inputs lists the byte provision as a caller-supplied proving-run request member, and Behavior says the generator "shall invent no request member". As written, the new path contradicts that rule or has no stated source for its documents. | FR-024-AC-13, FR-024 Inputs, FR-024 Behavior line 86 |
| FND-003 | medium | Only the invocation document's pre and post snapshots are specified. QSL FR-106 also admits its model header, context, operation, self, parameters, result, created and deleted members (AdmittedObservations carries parameters, result, created and deleted), and the test twin hard-codes result {boolean:true} and parameters {}. Where the code PR takes those values from, and whether it may fill them with constants, is unspecified. This is the "invent no member" question the snapshot bullets answer for fields only. | FR-024-AC-13, FR-024 Behavior lines 102-105 |
| FND-004 | medium | Neither the envelope's ReplaySource arm nor the payload's `witness` (Option<SeparatingWitnessRecord>) is specified. QSL settles ReproducedWithEvaluatedWitness only on the Witness arm (the Input arm settles ReproducedWithoutWitness), and only when the payload's witness record equals the one QSL re-derives (None/None agrees; a one-sided record settles inconclusive with the Witness cause). FR-024 sends a non-backend counterexample to the Input arm, so AC-15's synthetic-playback case can reach its stated settlement only through arm and witness choices the spec does not make. | FR-024-AC-15, FR-024-AC-17, FR-024 Behavior lines 87-88 |

## Dispositions

Round 1, reviewed at 02a5f67616fed74c0c014d546e2938e555791968.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: the out-of-domain and reconstruct-refusal bullets now name replay_state_clause, and a state field's domain is the integer range its framed object's body member declares (FR-015-AC-27). |
| FND-002 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: Inputs give the byte provision as the caller's domain package documents only, list the CG-built documents and their sources, and state the documents as the one addition to the byte provision. |
| FND-003 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: every invocation member is specified (format, identity label, model header, context, operation, self, pre/post refs, parameters, result, created, deleted). The claim that the four empty or null members are derived is wrong for some operations; see FND-006. |
| FND-004 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: Witness arm, payload witness none. Verified at QSL c8f0c28: a record is derived only when the decision path ends at a forall/exists with a stop report (witness/derivation.rs). FR-015-AC-26's single comparison is closed-scope with no record, so None/None agrees and the violating run reproduces. A respecting run settles Verdicts (proved != replayed), and Witness is the cause only for a violating run of a decisive clause. The spec's statements match. |

Round 2, reviewed at 6df9b318b612af8b7544f34e8c2d9e87ce65590c.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 6df9b318b612af8b7544f34e8c2d9e87ce65590c: the FR-029 CG-defect definition now reads "exactly the errors FR-029-AC-11 and FR-029-AC-16 list", naming the seven StateClauseReplayError variants, and the setup-refusal text lists StateClauseReplayError as a wrapper. Widening AC-13 created the inflation recorded as FND-007. |
| FND-006 | fixed | 6df9b318b612af8b7544f34e8c2d9e87ce65590c: StateClauseReplayError::UnsupportedOperationShape is returned before any document is built and before replay_state_clause is called (FR-024-AC-19, TC-035 step 17), and the Scope paragraph narrows the supported shape. Mapping it to Failed is the right reading. After a falsified run FR-029's table offers only Refuted, Inconclusive(ReplayParity), Inconclusive(ReplayRefused) carrying a QSL-supplied code (FR-029-AC-11 forbids a CG-invented code) and Failed. Declined and Unsupported belong to the pre-run negotiation (FR-030: "Declined for a refusal before any backend run"; Unsupported means solver or backend absent). A falsified harness CG cannot replay is a CG gap, so Failed holds. Advisory, outside IR-460: FR-015-AC-29 could refuse the same shape at harness generation so the state is unreachable. |

Round 3, reviewed at 19196250716f86adf9af536a1cfce0399cd10424.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-007 | fixed | 19196250716f86adf9af536a1cfce0399cd10424: FR-029-AC-13 is reverted to its two wrappers (ReplayPackageError and FrameReplayError) with no planned note. TC-040 step 15 now cites only FR-029-AC-16, which carries the StateClauseReplayError wrapping. The Covered row 'FR-029-AC-11 through FR-029-AC-15' (spec/kani/matrix/tests.md:36) holds no planned clause, so AC-13's backing by tests/it/terminal_map.rs:427 is truthful again. Strict coverage: 12/15 for FR-029; 301/432 backed and 66 unbacked overall, unchanged from round 2. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | FR-029's definitions were not extended. "CG defect: exactly the errors FR-029-AC-11 lists" still excludes the six StateClauseReplayError variants that FR-029-AC-16 maps to Failed. The setup-refusal definition (and FR-029-AC-13) covers CallSiteRefusal and DependencyLockError only "bare or wrapped in ReplayPackageError or FrameReplayError". FR-029-AC-16 therefore contradicts the definitions it relies on. | FR-029:42-56, FR-029-AC-11, FR-029-AC-13, FR-029-AC-16 |
| FND-006 | medium | The claim that "parameters empty, result null ... are a consequence of the harness and not a placeholder" is wrong. QSL admission checks those members against the domain package's operation declaration, not the subject ABI (document.rs admit_parameters, admit_result: a declared parameter or result missing is refused as missing-member). FR-015-AC-26/29 do not refuse an operation that declares extra parameters or a result: the clause's parameters are self, then the result and every operation parameter (src/kani/generate/frame.rs:510), and AC-29 refuses only reads through another parameter. For such an operation every replay is refused at admission and reads as Inconclusive(ReplayRefused), which reports a CG unsupported shape as a QSL data refusal. The current test twin's deposit declares returns Boolean. Refuse the shape up front with a typed CG error, or narrow the supported shape. | FR-024-AC-13, FR-024 Behavior (invocation document), FR-015-AC-29 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | Coverage inflation. FR-029-AC-13 was widened with a StateClauseReplayError clause that the AC itself marks "planned (IR-460) and not yet backed", yet spec/kani/matrix/tests.md still lists FR-029-AC-13 in the "FR-029-AC-11 through FR-029-AC-15 ... Covered (IR-465)" row. Strict coverage keeps it backed through tests/it/terminal_map.rs:427 (FR-029 at 12/15), so an untested clause counts as covered. FR-029-AC-16 already says CallSite and Dependencies read as CallSiteRefusal and DependencyLockError do, and TC-040 step 15 tests it. Revert the AC-13 widening, or move AC-13 out of the Covered range into a Planned row. | FR-029-AC-13, spec/kani/matrix/tests.md:36, FR-029-AC-16 |

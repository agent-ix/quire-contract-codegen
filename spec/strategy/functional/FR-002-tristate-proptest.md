---
id: FR-002
title: "Generate tri-state harnesses and shaped strategies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-002: Generate tri-state harnesses and shaped strategies

## Description

When executable pre/post contracts are lowered, the generator shall emit entry-point harnesses and
proptest strategies that preserve pass, failed postcondition, and rejected precondition outcomes.

## Inputs

- Executable clauses, execution points, state bindings, ranges, memberships, and supported relations.
- Planned (IR-815): for the V2 harness, an admitted `CheckedPackageV2` with one selected
  precondition and one selected postcondition, each selected by its checked clause id and authentic
  `claim` `CheckedOccurrence`. Planned (IR-1013, FR-058): Contract IR's typed accessor, which does
  not yet exist, returns each selected tuple's accepted QSL `ClauseBinding.execution_point` as its
  owned `quire_contract_model::ExecutionPoint` (variant and `AnchorName`). QSL owns `ClauseBinding`.
  One checked clause node may have multiple authentic claims; precondition claims sharing one body
  and operation anchor share one `Pre` point, and postcondition claims sharing one body and operation
  anchor share one `Post` point (QSpec FR-341). The two harness selections may have different points.
  For FR-002-AC-12, a repeated V2 selection names the same checked clause id and authentic claim
  occurrence; the clause identity checks of FR-002-AC-10 retain their original scope.
  The caller supplies no separate point. Each requirement identity is the QSL driver's bound
  FR-331 item's authentic `claim` occurrence in this same admitted package; its full `package_id`
  supplies the revision. For a process provider, CG compares FR-331 `request.package_id` with the
  separately admitted package. For a linked provider, the driver exports, emits, and admits one
  package before CG receives that admitted package and selected claim. QSL export refusals stay
  with the driver; CG checks the occurrence and referenced V2 model bindings defensively.

## Outputs

- Tri-state harness source and broad, boundary, state-pinned, or no-event strategies.
- Planned (IR-364): the generated V2 harness exposes a CG-owned
  `V2HarnessCaseResult`. Its `Verdict` branch carries a completed case's runtime verdict.
  Its `NonCompleted` branch carries the precondition or postcondition selection's checked clause
  id and authentic `claim` occurrence, the original typed stop from
  `Result<rt::Outcome<bool>, ClauseStop>`, and the typed
  `ExecutionPoint` accepted for that exact selection. The stop distinguishes
  `Outcome::Undefined`, `Outcome::Refused`, `Outcome::Incomplete`,
  `ClauseStop::Oracle(OracleStop)`, and `ClauseStop::InvalidConstant` without
  flattening their causes to a string. `NonCompleted` is not a runtime verdict;
  the point's runtime string is derived from its typed value using FR-011's rendering.
- Planned (IR-364): the generated V2 campaign has a distinct
  `V2HarnessCampaignResult::NonCompleted` terminal variant carrying that case result and a
  `V2HarnessCampaignSummary`. The summary retains the runtime report's prior accepted,
  rejected, failed, and discarded counts, plus CG-owned `non_completed` and
  `attempted: Option<u64>` counts.
  For the first stopped case with a report below its count limit, `non_completed` is one and
  `attempted` is `Some(runtime report counts().total() + 1)`; the stopped evaluation is an
  attempt even though no runtime verdict is recorded. A report already at its count limit
  retains that limit signal, and the V2 summary sets `attempted` to `None`
  while still recording one non-completed case. Completed cases retain `non_completed` zero
  and their existing attempted count. Failed remains a subset of accepted.
- Planned (IR-815): generation refusals `HarnessErrorCode::MissingClaimOccurrence`,
  `HarnessErrorCode::MissingExecutionPoint`, and `HarnessErrorCode::ExecutionPointMismatch`, with
  the affected selection named and no artifact. The harness also has
  `HarnessErrorCode::OwnerIdentityMismatch`, `HarnessErrorCode::UnboundModelElement`, and
  `HarnessErrorCode::InvalidIdentityRendering`; each produces no harness artifact. FR-011's
  corresponding numeric-runner variants belong to `StrategyErrorCode`.

## Behavior

- A harness shall snapshot pre-state before invoking the subject.
- A harness shall check post-state after invoking the subject.
- Ranges, memberships, and supported correlated relations shall shape admissible strategies.
- Residual constraints shall use explicit rejection and retain the rejected-invocation numerator and
  total-invocation denominator needed to report their rejection rate.
- Shrinking shall preserve shaped constraints or report residual rejection.
- Generated harnesses, rather than caller glue, shall own the ordering of pre-state snapshot,
  precondition evaluation, subject invocation, and post-state evaluation.
- Unsupported constraint forms shall produce a structured diagnostic.
- Strategy generation shall not silently fall back to filtering.
- The generated campaign runner shall own the proptest execution loop, explicit framework-discard
  accounting, and terminal conclusion. It shall enforce caller-supplied minimum accepted, minimum
  rejected, and maximum explicit-discard invocation counts. A zero rejected floor declares a total
  precondition; a positive rejected floor requires observed rejection coverage.
- An aborted search shall return an exhausted result with its framework reason and observed
  accounting, including any violated policy threshold as a nested policy result. A floor missed during
  an incomplete search shall not replace the exhaustion result. Completed searches shall enforce
  the same coverage floors.
- Campaign policy shall apply to the complete supplied report, including preexisting invocations
  and explicit discards; a completed search shall not pass when that report exceeds its discard ceiling.
- Every generated integer strategy case shall bind its exact values to an executable accepted or
  rejected domain check against runtime tri-state verdicts. Enum strategy populations contain only
  declared admissible members and shall execute their admission expectation without synthesizing an
  out-of-membership customer enum value.
- Generated harness campaign constructors shall bind exact Boolean inputs to accepted, rejected, or
  discarded disposition.
- Harness and strategy generation shall return deterministic source, or a structured non-generated
  terminal state with no partial bundle.
- Planned (IR-815): when a request selects one precondition and one postcondition of an admitted
  `CheckedPackageV2`, the generator shall embed in the harness the FR-014 V2 clause oracle of each
  selected clause body (FR-014-AC-38).
- Planned (IR-815): the V2 harness shall keep the snapshot, precondition, invocation and
  postcondition ordering of this requirement.
- Planned (IR-364): if an embedded V2 clause oracle returns `Ok(Outcome::Undefined)`,
  `Ok(Outcome::Refused)`, or `Ok(Outcome::Incomplete)`, then the V2 harness shall return
  `V2HarnessCaseResult::NonCompleted` with that original typed stop and its selected
  precondition or postcondition identity and accepted typed `ExecutionPoint`.
- Planned (IR-364): if an embedded V2 clause oracle returns
  `Err(ClauseStop::Oracle(OracleStop))` or `Err(ClauseStop::InvalidConstant)`, then the V2 harness shall
  return `V2HarnessCaseResult::NonCompleted` with that original typed stop and its selected
  precondition or postcondition identity and accepted typed `ExecutionPoint`.
- Planned (IR-364): if the V2 harness returns a non-completed case result, then the generated
  campaign runner shall return `V2HarnessCampaignResult::NonCompleted` containing that case
  result and the V2 summary with this case included once in `non_completed` and, when exact
  counting is available, once in `attempted`.
- Planned (IR-364): if the supplied runtime report's snapshot is `at_limit` when a V2 oracle
  stops, then the V2 campaign runner shall set its terminal summary's exact `attempted` to
  `None` rather than derive a count from saturated runtime counters.
- Planned (IR-364): for a non-completed case, the V2 harness shall not construct a runtime
  verdict or record one in the runtime report.
- Planned (IR-364): for a non-completed case, the V2 campaign runner shall not return a successful
  case or campaign conclusion, a precondition rejection, postcondition failure, explicit discard,
  or framework exhaustion for that stop.
- Planned (IR-364): for a non-completed case, the V2 campaign runner shall retain the supplied
  runtime report's prior accepted, rejected, failed, and discarded counters unchanged.
- Planned (IR-815): if the embedded precondition oracle returns an outcome that does not complete,
  then the V2 harness shall neither invoke the subject nor evaluate the postcondition oracle.
- Planned (IR-815): the V2 harness generator shall read clause kind, identity and body only through
  the one typed node-access layer `core/ir/` of AD-004.
- Planned (IR-815): the V2 harness generator shall not name or read a `BoundPackage`, a `ClauseRef`,
  a `quire_contract_model::TypedExpression`, or the `oracle::boolean_v1` module.
- Planned (IR-815): if a selected clause identity is unknown, is not of the expected kind, or
  names the same clause as both selections, then the V2 harness generator shall return a typed,
  located diagnostic and no artifact.
- Planned (IR-815): if a selected clause body is outside the FR-014-AC-38 shape, then the V2
  harness generator shall return a structured unsupported diagnostic and no artifact, with no
  fallback to filtering.
- When a V2 precondition and postcondition are selected, the V2 harness generator shall call the planned Contract IR FR-058 accessor for the one admitted package and each selected checked clause id and `claim` occurrence tuple (planned, IR-815).
- Planned (IR-815): the V2 harness generator shall take each execution point only from the
  accepted QSL binding point returned for that exact selection.
- Planned (IR-815): CG shall use each QSL driver-bound FR-331 `claim` occurrence only after the
  FR-011 process owner comparison when a process frame exists, and the admitted V2 relation check
  for either route. Linked generation uses its one E5-admitted package. CG shall not call
  `export_bindings`, accept a bare unbound item, or substitute a sibling package's occurrence.
- Planned (IR-815): if a process FR-331 `request.package_id` differs from the admitted V2 package's
  full id, then the harness generator shall return `HarnessErrorCode::OwnerIdentityMismatch`
  before publishing an artifact, constructing a verdict, or mutating a report.
- Planned (IR-815): if a selected claim references a model element absent from the admitted V2
  relation, then the harness generator shall return `HarnessErrorCode::UnboundModelElement` naming
  that element and publish no artifact.
- Planned (IR-815): if a V2 harness identity text comparison boundary receives malformed or
  noncanonical `qocc2_` or `qpkg2_` text, then the harness generator shall return
  `HarnessErrorCode::InvalidIdentityRendering` before publication or report mutation and shall not
  normalize the text.
- Planned (IR-815): the V2 harness generator shall validate each selected clause id's existence,
  expected kind, and distinctness under FR-002-AC-10 before checking its claim occurrence under
  FR-002-AC-20; an unknown clause id or wrong kind retains AC-10's typed, located diagnostic.
- Planned (IR-815): if either supplied occurrence is not an authentic `claim` of its selected
  checked clause in that package, then the harness generator shall return
  `HarnessErrorCode::MissingClaimOccurrence`, name that selection, and produce no artifact.
- Planned (IR-815): the harness generator shall not substitute a sibling occurrence of the same
  checked clause node for either selected occurrence.
- Planned (IR-815): if either selected package, clause, and authentic claim occurrence has no
  accepted binding point, then the harness
  generator shall return `HarnessErrorCode::MissingExecutionPoint`, name that selection, and
  produce no artifact.
- Planned (IR-815): if a rendered harness or verdict point differs from the point returned for its
  selected claim occurrence, then the harness generator shall return
  `HarnessErrorCode::ExecutionPointMismatch`, name that selection, and produce no artifact before
  publication.
- Planned (IR-815): the harness generator shall not derive a point from clause kind.
- Planned (IR-815): the harness generator shall render each bound typed point into a runtime string
  as `<variant>:<utf8-byte-length>:<AnchorName>`, with the same injective variant and UTF-8 byte
  length rule as FR-011.
- Planned (IR-815): the generated harness shall assign the selected precondition's rendered point
  to a `RejectedPrecondition` verdict.
- Planned (IR-815): the generated harness shall assign the selected postcondition's rendered point
  to a `FailedPostcondition` or `Passed` verdict after completed postcondition evaluation.
- Planned (IR-815): the generated harness shall retain the selected precondition's rendered point
  when its oracle does not complete, alongside that oracle's cause and without counting it as passed.
- Planned (IR-815): the generated harness shall retain the selected postcondition's rendered point
  when its oracle does not complete, alongside that oracle's cause and without counting it as passed.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-002-AC-1 | Rejected inputs remain distinct from passing cases in generated reports. | Test (TC-004) |
| FR-002-AC-2 | Boundary strategies exercise immediately inside and outside supported constraints. | Test (TC-004) |
| FR-002-AC-3 | Supported correlated constraints do not rely on improbable filtering. | Test (TC-004) |
| FR-002-AC-4 | Shrinking preserves generated constraints or records a residual rejection. | Test (TC-004) |
| FR-002-AC-5 | The generated campaign runner owns execution, explicit-discard accounting, and conclusion, returns a typed terminal result, and rejects campaigns below either configured coverage floor or above the explicit-discard ceiling; exhausted searches retain their reason and any missed floor without becoming a completed floor failure. | Test (TC-004) |
| FR-002-AC-6 | Integer and generated Boolean harness campaign cases bind exact values to executable accepted/rejected expectations; enum cases execute an all-admitted membership expectation. | Test (TC-004) |
| FR-002-AC-7 | PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's FR-014 V2 clause oracle. | Test (TC-004) |
| FR-002-AC-8 | PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition oracle, subject and postcondition oracle record that the snapshot precedes precondition evaluation, which precedes subject invocation, which precedes postcondition evaluation, for every case. | Test (TC-004) |
| FR-002-AC-9 | PLANNED (IR-815). Source inspection shows the V2 harness generator opens a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names no `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression` or `oracle::boolean_v1` item, each of which today's V1 harness names or uses. | Inspection |
| FR-002-AC-10 | PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or the same for both selections yields a typed, located diagnostic naming the selection and no artifact. | Test (TC-004) |
| FR-002-AC-11 | PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape yields a structured unsupported diagnostic and no artifact, and the generator emits no filtering strategy for it. | Test (TC-004) |
| FR-002-AC-12 | PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected checked clause ids, authentic `claim` occurrences, and campaign policy is byte-identical. The repeated selections include the same FR-011 identity inputs; the generated header, runtime identities, source map, symbols, and remaining bytes all agree across repetitions. | Test (TC-004) |
| FR-002-AC-13 | PLANNED (IR-815). A seeded V2 case on which both oracles complete true runs the subject once and is a pass. | Test (TC-004) |
| FR-002-AC-14 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes false is a rejected precondition and the subject is not invoked. | Test (TC-004) |
| FR-002-AC-15 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes true and the postcondition oracle completes false is a failed postcondition. | Test (TC-004) |
| FR-002-AC-16 | PLANNED (IR-364). For each selected precondition and postcondition oracle, a seeded `Ok(Outcome::Undefined)`, `Ok(Outcome::Refused)`, `Ok(Outcome::Incomplete)`, `Err(ClauseStop::Oracle(OracleStop))`, and `Err(ClauseStop::InvalidConstant)` returns CG-owned `V2HarnessCaseResult::NonCompleted` retaining the exact typed stop cause, selected claim identity, and selected accepted typed `ExecutionPoint`. The generated campaign returns `V2HarnessCampaignResult::NonCompleted` carrying that case and a summary whose `attempted` includes this stopped case once and whose `non_completed` is one. No runtime verdict is constructed or recorded for the stopped case; prior runtime accepted, rejected, failed, and discarded counters are unchanged. The case is not reported as a pass, rejection, failure, explicit discard, or framework exhaustion. | Test (TC-004) |
| FR-002-AC-17 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that does not complete does not invoke the subject. | Test (TC-004) |
| FR-002-AC-18 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that does not complete does not evaluate the postcondition oracle. | Test (TC-004) |
| FR-002-AC-19 | PLANNED (IR-815). Each V2 selection preserves the accepted QSL `ClauseBinding.execution_point` for its exact admitted package, checked clause id, and authentic `claim` occurrence through the injective runtime rendering; precondition and postcondition points may differ. Two requests selecting different authentic claims on the same precondition node each retain that node's accepted `Pre` operation point while selecting the correct occurrence. A rejection carries the precondition point; a failed or passed verdict after completed postcondition evaluation carries the postcondition point; a non-completed case carries the point of the oracle that did not complete and retains its cause without counting as a pass. | Test (TC-004) |
| FR-002-AC-20 | PLANNED (IR-815). After FR-002-AC-10's clause id, kind, and distinctness checks, a wrong-role, wrong-ordinal, or other-node `claim` occurrence for either selected V2 clause yields `HarnessErrorCode::MissingClaimOccurrence` and no artifact even if a sibling occurrence has a point; an authentic selected tuple with no accepted point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact even if another authentic occurrence of that node has a point; a render mutant that changes only that selection's generated harness or verdict point yields `HarnessErrorCode::ExecutionPointMismatch` before publication and no artifact. Each diagnostic names the affected precondition or postcondition selection, and no point is inferred from clause kind. | Test (TC-004) |
| FR-002-AC-21 | RETIRED, replaced by FR-002-AC-25; the former digest-shaped identity rendering is obsolete and earns no implementation credit. | Inspection |
| FR-002-AC-22 | PLANNED (IR-815). For either selection, an out-of-driver missing or non-claim occurrence returns `HarnessErrorCode::MissingClaimOccurrence`, a referenced model element absent from the admitted V2 relation returns `HarnessErrorCode::UnboundModelElement` naming it, and a process FR-331 owner differing from the admitted package returns `HarnessErrorCode::OwnerIdentityMismatch`. Each produces no artifact. A linked request uses one driver-exported/E4-emitted/E5-admitted package and a selected claim checked against that package; a foreign claim absent from it returns `HarnessErrorCode::MissingClaimOccurrence`. QSL `export_bindings`, process package-byte/id, and linked E4/E5 handoff refusals retain their own upstream causes. | Test (TC-004) |
| FR-002-AC-23 | RETIRED, replaced by FR-002-AC-26; the former digest-shaped text control is obsolete and earns no implementation credit. | Inspection |
| FR-002-AC-24 | PLANNED (IR-364). With a supplied runtime report whose snapshot is `at_limit` before a seeded non-completed V2 oracle case, `V2HarnessCampaignResult::NonCompleted` retains that case and a summary with `non_completed == 1` and exact `attempted == None`; it does not infer an exact prior count from the saturated report. The report's existing counters and `at_limit` signal remain unchanged, and no runtime verdict is constructed or recorded for the stopped case. | Test (TC-004) |
| FR-002-AC-25 | PLANNED (IR-815; replaces AC-21). The V2 harness header, `RequirementId`, `RevisionId`, source map and symbols carry FR-011's canonical structural `qocc2_`/`qpkg2_` renderings of each driver's selected claim's complete node, role and ordinal and the same admitted package's name and complete lock. Changing any one structural identity component changes its rendered text. An unchanged node and lock identity never excuses a changed admitted body from the existing body comparison. | Test (TC-004) |
| FR-002-AC-26 | PLANNED (IR-815; replaces AC-23). At a V2 harness identity text comparison boundary, malformed, truncated, case-folded or otherwise noncanonical structural `qocc2_`/`qpkg2_` text, including a wrong nested path, name, lock member or byte count, returns `HarnessErrorCode::InvalidIdentityRendering` before publication and without normalizing or mutating a report; exact typed and canonical-text comparisons agree on the accepted selection. | Test (TC-004) |

## Open Questions

- OQ-1 (resolved, identity): FR-011 OQ-2 resolves each requirement to its selected QSL
  driver-bound FR-331 claim occurrence in its one admitted Contract IR package; process FR-331
  owner comparison and linked E4/E5 admission establish that package. Its
  the admitted package name plus complete lock supplies the revision. The generated header,
  runtime identities, source map, and symbols carry the FR-011 canonical renderings. FR-002-AC-10
  still governs unknown, wrong-kind, and duplicate checked clause ids before FR-002-AC-20 checks
  occurrence authenticity; neither criterion is narrowed by the identity source.
- OQ-2 (resolved, non-completed oracle outcome): `V2HarnessCaseResult::NonCompleted` retains the
  original typed `Outcome` stop or outer `ClauseStop` error, selected claim identity, and accepted
  typed `ExecutionPoint`. The CG-owned `V2HarnessCampaignResult::NonCompleted` retains this case
  and counts its evaluation as one attempted, non-completed case without a runtime verdict.
  Completed oracle cases continue to use runtime verdicts. No new runtime verdict kind or runtime
  accounting category is required for this V2 harness boundary.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md);
  QSL-1082 (accepted binding point on the V2 carrier; planned prerequisite);
  IR-1013 / Contract IR FR-058 (typed accessor for the accepted QSL binding point keyed by admitted
  package, checked clause id, and authentic claim occurrence; planned,
  blocks the V2 execution-point slice).
- **Downstream**: bound numeric/state-scalar refinement in
  [FR-008](./FR-008-bound-domain-strategy-admission.md) through
  [FR-013](./FR-013-it010-consumable-output.md).

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
- Planned (IR-815): for the V2 harness, an admitted `CheckedPackageV2` with the clause ids of one
  selected precondition and one selected postcondition, read through Contract IR's public typed
  API. Each selected V2 node exposes its accepted QSL `ClauseBinding`, whose mandatory
  `execution_point` is the QSL-owned `quire_contract_model::ExecutionPoint` (variant and
  `AnchorName`); CG reads each through typed access. The two selected bindings may carry different
  points. The caller supplies no separate point. The requirement and revision identity source
  remains open (Open Questions).

## Outputs

- Tri-state harness source and broad, boundary, state-pinned, or no-event strategies.
- Planned (IR-815): generation refusals `HarnessErrorCode::MissingExecutionPoint` and
  `HarnessErrorCode::ExecutionPointMismatch`, with the affected selection named and no artifact.

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
- Planned (IR-815): if an embedded V2 clause oracle returns an outcome that does not complete, then
  the V2 harness shall report the case as a non-completed case that retains the oracle's own cause,
  and shall not count it as a pass.
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
- When a V2 precondition and postcondition are selected, the V2 harness generator shall take each execution point only from that selected node's accepted QSL `ClauseBinding.execution_point` (planned, IR-815).
- Planned (IR-815): if either selected node lacks an accepted binding point, then the harness
  generator shall return `HarnessErrorCode::MissingExecutionPoint`, name that selection, and
  produce no artifact.
- Planned (IR-815): if a rendered harness or verdict point differs from its selected accepted
  binding point, then the harness generator shall return `HarnessErrorCode::ExecutionPointMismatch`,
  name that selection, and produce no artifact before publication.
- Planned (IR-815): the harness generator shall not derive a point from clause kind.
- Planned (IR-815): the harness generator shall render each bound typed point into a runtime string
  as `<variant>:<utf8-byte-length>:<AnchorName>`, with the same injective variant and UTF-8 byte
  length rule as FR-011.
- Planned (IR-815): a `RejectedPrecondition` verdict shall use the selected precondition's rendered
  point; a `FailedPostcondition` or `Passed` verdict after completed postcondition evaluation shall
  use the selected postcondition's rendered point.
- Planned (IR-815): a non-completed precondition outcome shall retain the selected precondition's
  rendered point, and a non-completed postcondition outcome shall retain the selected
  postcondition's rendered point alongside the oracle's cause, without classifying it as passed.

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
| FR-002-AC-12 | PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities and campaign policy is byte-identical. The requirement and revision identity that the generated header, `RequirementId` and symbols carry is not an input here and waits on OQ-1. | Test (TC-004) |
| FR-002-AC-13 | PLANNED (IR-815). A seeded V2 case on which both oracles complete true runs the subject once and is a pass. | Test (TC-004) |
| FR-002-AC-14 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes false is a rejected precondition and the subject is not invoked. | Test (TC-004) |
| FR-002-AC-15 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes true and the postcondition oracle completes false is a failed postcondition. | Test (TC-004) |
| FR-002-AC-16 | PLANNED (IR-815). A seeded V2 case on which the precondition or postcondition oracle returns an outcome that does not complete (stopped or undefined) is reported as a non-completed case retaining the oracle's cause, is not counted as a pass. | Test (TC-004) |
| FR-002-AC-17 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that does not complete does not invoke the subject. | Test (TC-004) |
| FR-002-AC-18 | PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that does not complete does not evaluate the postcondition oracle. | Test (TC-004) |
| FR-002-AC-19 | PLANNED (IR-815). Each V2 selection preserves its accepted QSL `ClauseBinding.execution_point` through the injective runtime rendering; precondition and postcondition points may differ. A rejection carries the precondition point; a failed or passed verdict after completed postcondition evaluation carries the postcondition point; a non-completed case carries the point of the oracle that did not complete and retains its cause without counting as a pass. | Test (TC-004) |
| FR-002-AC-20 | PLANNED (IR-815). For either selected V2 clause, an absent accepted binding point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact; a render mutant that changes only that selection's generated harness or verdict point yields `HarnessErrorCode::ExecutionPointMismatch` before publication and no artifact. Each diagnostic names the affected precondition or postcondition selection, and no point is inferred from clause kind. | Test (TC-004) |

## Open Questions

- OQ-1 (identity): the V2 source of the harness's requirement and revision identity is the open
  question recorded as FR-011 OQ-2, and the V2 entry shape follows the open interface-001 direction.
  The generated harness writes the requirement and revision into its header comment, its
  `RequirementId` and its symbol names, so
  the final bytes of a V2 harness, FR-002-AC-12 across different identities, and the NFR-002-AC-1
  header requirement wait on FR-011 OQ-2; FR-002-AC-7 to AC-20 are stated independently of that
  identity and none invents a source.
- OQ-2 (non-completed verdict): which runtime verdict kind carries a non-completed oracle outcome
  (FR-002-AC-16) is not decided here; the criterion fixes only that the case is not a pass and
  keeps its cause.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md).
- **Downstream**: bound numeric/state-scalar refinement in
  [FR-008](./FR-008-bound-domain-strategy-admission.md) through
  [FR-013](./FR-013-it010-consumable-output.md).

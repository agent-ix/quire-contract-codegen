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
  API. The source of the execution point and of the requirement and revision identity for this
  input is open (Open Questions); no V2 criterion below depends on it.

## Outputs

- Tri-state harness source and broad, boundary, state-pinned, or no-event strategies.

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
  selected clause body (FR-014-AC-38) and shall keep the snapshot, precondition, invocation and
  postcondition ordering of this requirement.
- Planned (IR-815): the V2 harness generator shall read clause kind, identity and body only through
  Contract IR's public typed API for the admitted package, and shall take no `BoundPackage` or
  `ClauseRef` input.
- Planned (IR-815): if a selected clause identity is unknown, is not of the expected kind, or
  names the same clause as both selections, then the V2 harness generator shall return a typed,
  located diagnostic and no artifact.
- Planned (IR-815): if a selected clause body is outside the FR-014-AC-38 shape, then the V2
  harness generator shall return a structured unsupported diagnostic and no artifact, with no
  fallback to filtering.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-002-AC-1 | Rejected inputs remain distinct from passing cases in generated reports. | Test (TC-004) |
| FR-002-AC-2 | Boundary strategies exercise immediately inside and outside supported constraints. | Test (TC-004) |
| FR-002-AC-3 | Supported correlated constraints do not rely on improbable filtering. | Analysis |
| FR-002-AC-4 | Shrinking preserves generated constraints or records a residual rejection. | Test (TC-004) |
| FR-002-AC-5 | The generated campaign runner owns execution, explicit-discard accounting, and conclusion, returns a typed terminal result, and rejects campaigns below either configured coverage floor or above the explicit-discard ceiling; exhausted searches retain their reason and any missed floor without becoming a completed floor failure. | Test (TC-004) |
| FR-002-AC-6 | Integer and generated Boolean harness campaign cases bind exact values to executable accepted/rejected expectations; enum cases execute an all-admitted membership expectation. | Test (TC-004) |
| FR-002-AC-7 | PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's FR-014 V2 clause oracle, and a seeded case on which the precondition is false is rejected with the subject not invoked, one on which the postcondition is false is a failed postcondition, and any other is a pass. | Test (TC-004) |
| FR-002-AC-8 | PLANNED (IR-815). In the V2 harness the pre-state snapshot precedes precondition evaluation, which precedes subject invocation, which precedes postcondition evaluation, for every case, as an instrumented subject observes. | Test (TC-004) |
| FR-002-AC-9 | PLANNED (IR-815). Source inspection shows the V2 harness generator reads clause meaning only through Contract IR's public typed API and names no `BoundPackage`, `ClauseRef` or private body decoder. | Inspection |
| FR-002-AC-10 | PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or the same for both selections yields a typed, located diagnostic naming the selection and no artifact. | Test (TC-004) |
| FR-002-AC-11 | PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape yields a structured unsupported diagnostic and no artifact, and the generator emits no filtering strategy for it. | Test (TC-004) |
| FR-002-AC-12 | PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities and campaign policy is byte-identical. | Test (TC-004) |

## Open Questions

- OQ-1 (execution point and identity): the V2 source of the harness's execution point and of the
  requirement and revision identity is the open question recorded as FR-011 OQ-1 and OQ-2, and the
  V2 entry shape follows the open interface-001 direction. No FR-002 V2 criterion depends on the
  answer, and none invents a source.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md).
- **Downstream**: bound numeric/state-scalar refinement in
  [FR-008](./FR-008-bound-domain-strategy-admission.md) through
  [FR-013](./FR-013-it010-consumable-output.md).

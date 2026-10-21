---
id: SR-14943
title: IR-621 CG direct-outcome specification — failure-domain
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen; IR-621; spec/assurance/AD-003-evidence-chain.md,
  spec/core/functional/interface-001-codegen-api.md, spec/core/non-functional/NFR-005-no-generation-panics.md,
  spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
review_set: subset
---

## Summary

Ticket: IR-621. Fresh rebased five-file specification delta reviewed independently against current main; eight historical artifacts retained as historical evidence. PASS. Fresh context retains profile/dispatch refusal, malformed graph input, graph expansion limit, collection exhaustion and corpus identity/dependency/serialization adverse controls. Exact kind/cause, source/profile/bound context, absence of Boolean claim, no partial artifact and no premature identity claim remain required. No new graph algorithm, callback or evaluator behavior is introduced. Renumbering does not merge these direct-result refusals with the distinct IR942 per-step identity/setup refusals.

## Verdict

PASS — PASS. Fresh context retains profile/dispatch refusal, malformed graph input, graph expansion limit, collection exhaustion and corpus identity/dependency/serialization adverse controls. Exact kind/cause, source/profile/bound context, absence of Boolean claim, no partial artifact and no premature identity claim remain required. No new graph algorithm, callback or evaluator behavior is introduced. Renumbering does not merge these direct-result refusals with the distinct IR942 per-step identity/setup refusals.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

PASS. Fresh context retains profile/dispatch refusal, malformed graph input, graph expansion limit, collection exhaustion and corpus identity/dependency/serialization adverse controls. Exact kind/cause, source/profile/bound context, absence of Boolean claim, no partial artifact and no premature identity claim remain required. No new graph algorithm, callback or evaluator behavior is introduced. Renumbering does not merge these direct-result refusals with the distinct IR942 per-step identity/setup refusals.

All five changed files examined. No applicable installed AssuranceProfile or required review selection exists. Dependency analysis is inapplicable (no relationship edges changed); risk-complexity adds no separate lens for this bounded API-surface migration. Criterion-strength/Jev and optional whole-repository semantic review were not selected.

```yaml
scope:
- id: AD-003
  path: spec/assurance/AD-003-evidence-chain.md
  role: examined
  excerpt: "`Std001Code::is_registered` is false for them. Under the planned IR-621\
    \ CG adoption,\n  the retained family/corpus paths use IR's public infallible\
    \ `KaniOutcome::refused`,\n  `invalid_input` and `resource_exhausted` constructors\
    \ for their respective observed kinds.\n  They pass the original typed cause,\
    \ source identity and selected profile/bound context;\n  CG remains responsible\
    \ for selecting the actual refusal and does not infer a run result.\n  The three\
    \ public operations in `interface-001` return `Result<Success, KaniOutcome>`\n\
    \  directly, without `FamilyLoweringError`, `BoundedCorpusError`, an "
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "    output: Result<CollectionLowering, KaniOutcome>\n    output: Result<GraphLowering,\
    \ KaniOutcome>\n    output: Result<BoundedCorpusCase, KaniOutcome>\n    error_surface:\
    \ planned IR-621 retirement of public FamilyLoweringError and BoundedCorpusError,\
    \ their OutcomeConstruction variants, and replacement aliases or wrappers; terminal\
    \ cancellation pairing is unchanged"
- id: NFR-005
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '| `render_artifacts` (`src/kani/generate/corpus/bounded_kani_corpus.rs`),
    `deterministic_json(&proof_graph_value).expect(..)` | serialization failure of
    plain data | Under planned IR-621 CG adoption, `render_artifacts` returns a `Result`,
    and a failure is the case''s direct `KaniOutcome::refused` with code `kani_corpus_serialization_failed`
    (built with `std001_code!`), original source identity and selected profile/bound
    context, returned as `Err(KaniOutcome)` by `generate_bounded_kani_corpus_case`,
    with no artifact emitted. This infallible construction route has no outcome-construction
    error br'
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-621). External public-consumer calls to `prepare_bounded_collection_query`,
    `prepare_finite_graph_reaches` and `generate_bounded_kani_corpus_case` have the
    direct result signatures specified by `interface-001`, with `KaniOutcome` as their
    error type and no public `FamilyLoweringError`, `BoundedCorpusError`, `OutcomeConstruction`
    branch, or replacement alias or wrapper.
- id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-621). After infallible-constructor adoption, each retained
    family/corpus adverse control returns the original exact non-success kind, typed
    cause, source identity and selected profile/bound context, with no Boolean claim,
    partial artifact or premature identity claim; successful outputs and failure ordering
    remain unchanged.
- id: FR-015
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'While the finite-input family/corpus APIs remain live, their non-success
    paths

    shall use the matching infallible IR `KaniOutcome::refused`, `invalid_input` or

    `resource_exhausted` constructor and return the direct `KaniOutcome` error side

    specified by `interface-001`. This adoption retires only the impossible public

    outcome-construction error surface. The existing profile census, family/corpus

    success, refusal ordering and identity contracts remain in force. Canonical

    proof-graph serialization failure remains `Refused` with the exact

    `kani_corpus_serialization_failed` cause under NFR-005. Term'
- id: TC-023
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'from an external consumer test, explicitly

    assign each public collection, graph and corpus operation''s result to

    `Result<CollectionLowering, KaniOutcome>`, `Result<GraphLowering, KaniOutcome>`

    and `Result<BoundedCorpusCase, KaniOutcome>` respectively. Exercise each retained

    family''s actual profile/dispatch refusal, graph malformed-input and expansion-bound

    refusals, collection bound exhaustion, and the corpus identity/dependency/serialization

    failure controls. Assert the original exact kind and typed code, source identity,

    selected profile/bound context, `boolean_claim() == None`, and absence '
- id: FR-015-AC-120
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After
    the selected owner and occurrence validate under AC-122, CG forms the per-step
    record content identity from an RFC 8785 object whose only members are `subject_kind:
    "protocol-refinement-step"` and `record: StepSimulationRecord` encoded by QSV.
    Two separately valid selected refinements with the same obligation and hypotheses
    but different record `refinement` values, each with its own matching occurrence,
    yield different content identities; a record with a mismatched owner has no identity
    to compare. Within one admitted ref'
- id: FR-015-AC-121
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned
    `StepSimulationRecord` as a typed subject, carries its content identity and the
    owning refinement's O-07 `generated` occurrence key separately, and keeps any
    CG proof or obligation kind in the harness record rather than the record content
    preimage. CG publishes a persisted harness-record digest over every `KaniObligationIdentity`
    member except `source_span`; changing any included member changes this digest,
    while changing only `source_span` does not. This digest is distinct from the per-step
    record content identity. The r
- id: FR-015-AC-122
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88
    through AC-90 with an `Initial` record and `Step { row: i }` records for its admitted
    FR-375 `steps` order yields generated occurrences on the owner node at ordinals
    0 and `i + 1`, respectively. An ineligible refinement under FR-322-AC-91 (hidden
    field, history field, or `any` step target) carries only its `declaration` occurrence
    and yields no per-step record, harness, or proof credit; the IR reader refuses
    a generated occurrence on it as `invalid_package`/`invalid-value` before CG setup.
    Two step rows tied on nod'
```

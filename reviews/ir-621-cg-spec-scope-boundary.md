---
id: SR-14596
title: IR-621 CG direct-outcome specification — scope-boundary
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-contract-codegen; IR-621; spec/assurance/AD-003-evidence-chain.md,
  spec/core/functional/interface-001-codegen-api.md, spec/core/non-functional/NFR-005-no-generation-panics.md,
  spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
review_set: subset
---

## Summary

Ticket: IR-621. Strict five-file specification delta reviewed independently; stacked parent changes are excluded. PASS. CG owns selection of the observed cause, family/corpus result surface, preservation of source/context and no partial output. IR owns infallible per-kind construction and its separate constructor/Boolean-kind/terminal/proof-count validation; this spec consumes that API without accepting IR implementation. Public external-consumer tests guarantee the CG seam; full unsupported negotiation, terminal cancellation pairing and AC-111 retirement remain independently owned and excluded.

## Verdict

PASS — PASS. CG owns selection of the observed cause, family/corpus result surface, preservation of source/context and no partial output. IR owns infallible per-kind construction and its separate constructor/Boolean-kind/terminal/proof-count validation; this spec consumes that API without accepting IR implementation. Public external-consumer tests guarantee the CG seam; full unsupported negotiation, terminal cancellation pairing and AC-111 retirement remain independently owned and excluded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

PASS. CG owns selection of the observed cause, family/corpus result surface, preservation of source/context and no partial output. IR owns infallible per-kind construction and its separate constructor/Boolean-kind/terminal/proof-count validation; this spec consumes that API without accepting IR implementation. Public external-consumer tests guarantee the CG seam; full unsupported negotiation, terminal cancellation pairing and AC-111 retirement remain independently owned and excluded.

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
- id: FR-015-AC-120
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-621). External public-consumer calls to `prepare_bounded_collection_query`,
    `prepare_finite_graph_reaches` and `generate_bounded_kani_corpus_case` have the
    direct result signatures specified by `interface-001`, with `KaniOutcome` as their
    error type and no public `FamilyLoweringError`, `BoundedCorpusError`, `OutcomeConstruction`
    branch, or replacement alias or wrapper.
- id: FR-015-AC-121
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
```

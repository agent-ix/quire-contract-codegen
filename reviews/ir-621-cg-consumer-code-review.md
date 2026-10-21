---
id: SR-15230
title: IR-621 CG PR494 direct outcomes code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-621; spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/corpus/bounded_kani_corpus.rs,
  src/kani/generate/lower/bounded_collections.rs, src/kani/generate/lower/finite_reference_graphs.rs,
  src/kani/generate/lower/mod.rs, src/lib.rs, tests/it/bounded_kani_corpus.rs
review_set: subset
---

## Summary

Ticket: IR-621. Frozen head 541560c36bf437b6ad542856087e14127a546ba0; base bc4aea2bdc4635cd6fb76e5a7cf9adb9ac528301. Independent reviewer; no source edits.

PASS for the exact introduced change. Reviewed all eight changed files and the Rust lane: public direct Result signatures, root exports, every replaced constructor and propagation site, retained/new tests, CI diff (unchanged), numeric/resource/panic surface, seam boundaries, duplication, and success/identity transaction flow. No introduced defect, vendoring, wrapper/alias, new unsafe/panic, test-only production bypass, suppression or gate weakening was found. Original profile classifier outcomes propagate directly; every CG-selected Refused/InvalidInput/ResourceExhausted path preserves kind, typed cause, source and exact selected revision. Failure checks and identity claim ordering remain unchanged. The removed manufactured Boolean-kind wrapper test tested a deleted impossible variant; real refusal controls replace it, while producer broad-kind tests remain IR-owned. Native external tests exercise direct public types and real adverse requests. The serializer control exercises the real private render_artifacts boundary; it is not evidence of an observed public typed serialization failure. Valid public requests encounter the earlier 1MiB identity preimage bound before a 16MiB proof-graph serialization failure; validated graph endpoints make the render_graph_qsl missing-endpoint Option branch unreachable. These limitations are stated honestly by TC-023 and do not add behavior to this constructor migration. Same-request dispatch failure then successful retry and independent identical case equality establish identity rollback and unchanged bytes for that request. Full success implementation/renderers/identities are otherwise unchanged. Existing IR-615 classifier/legacy rejection controls and terminal cancellation code are unchanged.

## Verdict

PASS — no introduced findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

PASS for the exact introduced change. Reviewed all eight changed files and the Rust lane: public direct Result signatures, root exports, every replaced constructor and propagation site, retained/new tests, CI diff (unchanged), numeric/resource/panic surface, seam boundaries, duplication, and success/identity transaction flow. No introduced defect, vendoring, wrapper/alias, new unsafe/panic, test-only production bypass, suppression or gate weakening was found. Original profile classifier outcomes propagate directly; every CG-selected Refused/InvalidInput/ResourceExhausted path preserves kind, typed cause, source and exact selected revision. Failure checks and identity claim ordering remain unchanged. The removed manufactured Boolean-kind wrapper test tested a deleted impossible variant; real refusal controls replace it, while producer broad-kind tests remain IR-owned. Native external tests exercise direct public types and real adverse requests. The serializer control exercises the real private render_artifacts boundary; it is not evidence of an observed public typed serialization failure. Valid public requests encounter the earlier 1MiB identity preimage bound before a 16MiB proof-graph serialization failure; validated graph endpoints make the render_graph_qsl missing-endpoint Option branch unreachable. These limitations are stated honestly by TC-023 and do not add behavior to this constructor migration. Same-request dispatch failure then successful retry and independent identical case equality establish identity rollback and unchanged bytes for that request. Full success implementation/renderers/identities are otherwise unchanged. Existing IR-615 classifier/legacy rejection controls and terminal cancellation code are unchanged.

Reviewer executed no Cargo, rustc, metadata, formatter, native test, backend, Kani, goto, aggregate or heavy command. Read actual retained command JSON and raw logs: family 10 passed, corpus 26 passed, public 7 passed/8 ignored, doctests 16 passed; fmt-check and locked all-target clippy passed. First external test run failed because a new expected string omitted the existing cover diagnostic; fix changed the expected literal, leaving the production cover unchanged. git diff 4bf03452..541560c -- src tests is empty, so focused passing source/test bytes survive the rebase. Manifest and lock are unchanged in the reviewed diff. Receipts state every phase had 16GiB memory/swap0, maximum peak4212391936 bytes and zero memory events; release.json records all eleven scopes gone and TEN empty. This is retained author evidence, not an independent test rerun or final aggregate. Root retains the final aggregate and fresh-base merge gate. Eight Kani tests remain ignored; no new backend qualification is claimed.

No applicable AssuranceProfile exists. Rust review is folded into code-review. Base spec review applies to status only; further specification lenses and Jev are not selected. All changed source/test files and relevant owning context are examined. Historical reports are not current evidence.

```yaml
scope:
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: External public-consumer calls to `prepare_bounded_collection_query`, `prepare_finite_graph_reaches`
    and `generate_bounded_kani_corpus_case` have the direct result signatures specified
    by `interface-001`, with `KaniOutcome` as their error type and no public `FamilyLoweringError`,
    `BoundedCorpusError`, `OutcomeConstruction` branch, or replacement alias or wrapper.
- id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: After infallible-constructor adoption, each retained family/corpus adverse
    control returns the original exact non-success kind, typed cause, source identity
    and selected profile/bound context, with no Boolean claim, partial artifact or
    premature identity claim; successful outputs and failure ordering remain unchanged.
- id: AD-003
  path: spec/assurance/AD-003-evidence-chain.md
  role: context_only
  excerpt: "STD-001 does not list those, and\n  `Std001Code::is_registered` is false\
    \ for them. Under the planned IR-621 CG adoption,\n  the retained family/corpus\
    \ paths use IR's public infallible `KaniOutcome::refused`,\n  `invalid_input`\
    \ and `resource_exhausted` constructors for their respective observed kinds.\n\
    \  They pass the original typed cause, source identity and selected profile/bound\
    \ context;\n  CG remains responsible for selecting the actual refusal and does\
    \ not infer a run result.\n  The three public operations in `interface-001` return\
    \ `Result<Succes"
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: context_only
  excerpt: "th no reverse Contract IR dependency; a malformed request is a typed refusal,\
    \ not a disposition\n  # IR-621 direct error signatures below are planned; success\
    \ and refusal semantics stay live.\n  - name: prepare_bounded_collection_query\n\
    \    status: live finite-input auxiliary under FR-015-AC-88 and AC-89; deleted\
    \ by the collection slice (FR-015-AC-110)\n    inputs: [KaniProfile, DispatchIndex,\
    \ ValidatedFiniteInput, CollectionQuery]\n    output: Result<CollectionLowering,\
    \ KaniOutcome>\n    semantics: CG-owned bounded collection-query lowering; bound\
    \ e"
- id: NFR-005
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: context_only
  excerpt: ministic_json(&proof_graph_value).expect(..)` | serialization failure of
    plain data | Under planned IR-621 CG adoption, `render_artifacts` returns a `Result`,
    and a failure is the case's direct `KaniOutcome::refused` with code `kani_corpus_serialization_failed`
    (built with `std001_code!`), original source identity and selected profile/bound
    context, returned as `Err(KaniOutcome)` by `generate_bounded_kani_corpus_case`,
    with no artifact emitted. This infallible construction route has no outcome-construction
    error branch and requires no `unwrap`,
- id: TC-023
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'IR-621 direct-error procedure: from an external consumer test, explicitly

    assign each public collection, graph and corpus operation''s result to

    `Result<CollectionLowering, KaniOutcome>`, `Result<GraphLowering, KaniOutcome>`

    and `Result<BoundedCorpusCase, KaniOutcome>` respectively. Exercise each retained

    family''s actual profile/dispatch refusal, graph malformed-input and expansion-bound

    refusals, collection bound exhaustion, and the corpus identity/dependency/serialization

    failure controls. Assert the original exact kind and typed code, source identity,

    selected profile/bound context, `boolean'
bindings:
- test_id: public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: FR-015-AC-123
  trace: correct
- test_id: tests::tc_023_large_dependency_graph_refuses_canonical_projection
  ac_id: FR-015-AC-124
  trace: correct
- test_id: public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: FR-015-AC-124
  trace: correct
- test_id: direct_family_errors_preserve_profile_dispatch_and_bound_precedence
  ac_id: FR-015-AC-124
  trace: correct
- test_id: direct_corpus_refusals_preserve_earliest_failure_and_identity_transactions
  ac_id: FR-015-AC-124
  trace: correct
```

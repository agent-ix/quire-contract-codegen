---
id: SR-14500
title: code-review review of IR-615 CG profile cleanup
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-615; frozen parent-to-candidate consumer
  delta; repository matrix
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
---

## Summary

Ticket: IR-615. PR: quire-contract-codegen#485. Review ONLY ec0a2b0640e97f5acb1a6daf62afb3de81e77efd..a307334ede25b9326dbc21e97ac1cd019d0347e6. Foreign-owned parent #484 terminal/spec edits are excluded. That parent subsequently moved; rebasing/retargeting and final aggregate remain lead-owned gates. No applicable AssuranceProfile was found. Reviewer GPT-6 (deployment SKU unavailable, marker gpt-6). No Cargo, fmt, metadata, native, Kani or aggregate was executed by this reviewer. CLI/Git/source inspection and Quire validation only. Scope excerpts over 600 characters are verbatim prefixes.

## Verdict

PASS — no introduced code/Rust defect found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The production delta removes only the obsolete Inconclusive admission arm and corrects profile API documentation; actual Supported and Refused control flow is unchanged. Cargo.lock changes only IR/model to the genuinely merged producer7e1a9e5, with shared source identity preserved. No new pin, vendored implementation, schema duplication, source tracker, public API, unsafe, integer boundary conversion, async/locking branch, waiver, workflow change or relaxed gate. No .github diff.

The new fixture composes a two-family test population and reuses real public IR profile/dispatch/input admission and existing CG public preparations. Searched existing collection/graph fixture and profile helpers: each existing fixture is private and family-specific, not a shared helper accepting both dispatch families; no duplicate production algorithm is introduced. New tests exercise production behavior, not source-text matching or replacement of the unit under test. Success callbacks passed to Result::map are forced by subsequent expect; refusal branches must return actual outcomes, exact kind/cause/source/context/no-Boolean values. No assertion can disappear behind an unasserted guard. All trace attributes are attached to #[test], never proofs/production.

Census strict subset uses distinct refusal causes with a supported entry between refused entries and a reordered request, distinguishing whole-matrix returns and early abort. JSON roundtrip is a genuine owning reader route; otherwise valid custom and actual vacuity codes are both changed to legacy inconclusive and must be rejected. Existing constructors cannot synthesize the removed enum variant. Missing/blank/duplicate request refusal keeps exact codes/context; paired graph/collection positive results and profile refusal-before-empty-dispatch exercise both actual retained paths. Empty and mismatched dispatch checks retain production refusals. Supported family test exercises a real positive-length edge and ordered duplicate-preserving ExistsEqual result; it does not claim all clauses of AC86/88 alone. Existing family-specific controls remain.

Actual classify_success still calls IR proved_from_checks and maps exact KANI_VACUOUS_PROOF to VacuousProof on zero SUCCESS; untouched recorded-real-Kani fixture and synthetic controls check that separation. No profile code becomes run vacuity. FamilyLoweringError::OutcomeConstruction and fallible refusal helper remain existing IR-621 scope, not silently retired. Full Unsupported negotiation remains explicitly planned.

Read raw final-focused.log and final-focused.sh: actual memory.max17179869184/swap0; fmt and locked all-target Clippy pass; nine profile/family tests, four synthetic classifier tests and one recorded-real-Kani vacuity fixture pass (14 actual native executions, zero failed/ignored). The initial wrong zero-test selector is not accepted. Recorded fixture classification is not a new backend invocation or proof qualification. Peak2,552,377,344bytes and all memory events0. Root final aggregate remains pending.

## Examined Scope

```yaml
scope:
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for
    distinct named constructs returns one `CapabilityEntry` per requested construct
    in request order, preserving each admitted `Supported` module or `Refused` cause
    code exactly. The Contract IR profile reader rejects the removed `inconclusive`
    disposition rather than admitting it; no profile disposition or profile-supplied
    code becomes an `Inconclusive` run outcome or asserts proof vacuity.
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition
    census.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a profile/dispatch-admitted request follows only the selected reference field,
    finds a positive-length path in sorted depth-first order, and checks the expansion
    bound before entering each new object; a target edge is found without expanding
    its target.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query
    retains input order and duplicates and returns its truth value and examined-item
    count; a decisive item ends that count, otherwise every item counts.
- id: TC-023
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'The IR-615 producer and consumer

    cleanup is implemented: native reader/census and both retained family preparation

    controls pass; the actual run classifier retains its zero-SUCCESS vacuity rule.'
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: Run bounded Kani obligations, singly or in a batch, and retain their evidence
bindings:
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_census_reports_every_construct_including_ones_after_an_early_refusal
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::profile_reader_rejects_legacy_inconclusive_before_census_admission
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::profile_refusals_preserve_original_causes_and_context_in_both_families
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_missing_construct_is_rejected_with_kani_capability_missing
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::empty_or_duplicate_construct_name_refuses_without_a_partial_census
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::supported_profile_still_prepares_exact_collection_and_graph_results
  ac_id: FR-015-AC-86
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::supported_profile_still_prepares_exact_collection_and_graph_results
  ac_id: FR-015-AC-88
  trace: correct
```

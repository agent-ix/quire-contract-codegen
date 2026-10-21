---
id: SR-14860
title: code-review current-cohort review of IR615 CG
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR615 current main-to-candidate eight-file
  diff; whole repository matrix
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/NFR-006
  type: references
---

## Summary

Ticket: IR-615; CG PR485. Fresh complete review of current main 8ac09bc85f2a34b7b91d36bd59ce8e92e6ced59d through current candidate 2f24b78989f550f3e68e31323cf33782a96caf5d, all eight changed files, including historical review custody. Old SR14500/14501/14502 are historical and supply no fresh acceptance. Reviewer GPT-6 (deployment SKU unavailable; marker gpt-6). No applicable AssuranceProfile exists. No product edit or Cargo/fmt/metadata/rustc/native/Kani/CBMC/goto/aggregate command was run. Scope excerpts longer than600characters are verbatim prefixes.

## Verdict

CONDITIONAL — two medium incomplete-criterion findings and one low missing-trace finding require coder fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The only AC-86 binder exercises one a->b edge, so it does not cover selected-field isolation, sorted depth-first branching, or checking the bound before each new expansion. Removing the field filter passes that binder and the existing branching fixture because every fixture edge uses next. The existing branching test is only TC-tagged. Add genuine multi-field, branching and bound controls that collectively bind the full criterion; a one-edge success is not full AC-86 evidence. | src/kani/generate/lower/mod.rs:270-279 |
| FND-002 | medium | The only AC-88 binder exercises ExistsEqual(3) on [2,2,3], with the decisive item last. Returning the full input length regardless of an early match passes it; no admitted ForAllNonNegative case or unsuccessful ExistsEqual case is covered by an AC-88 binder. The existing collection test is only TC-tagged and also places the successful match last. Add both query kinds, early decisive and no-decisive cases with exact truth, order/duplicates and examined counts, bound to AC-88. | src/kani/generate/lower/mod.rs:270-279 |
| FND-003 | low | The newly added supported_profile_keeps_unowned_and_mismatched_dispatch_refusals test carries only TC-023, which is a test-case ID rather than an acceptance-criterion binder. Its actual non-Boolean refusal assertions exercise the no-profile-to-Inconclusive clause of AC-82, but that coverage is absent from the criterion trace model. Bind the test to the existing criterion it actually asserts; do not add a new requirement or bind unrelated full-corpus behavior. | src/kani/generate/lower/mod.rs:310-313 |

## Coverage

Full current source delta removes the obsolete CapabilityDisposition::Inconclusive arm; no retained supported/refused production algorithm changes. Read the actually selected Cargo Git checkout d41e521: producer enum admits Supported/Refused only, owning KaniProfile reader derives through validation, and real classify preserves requested census and exact malformed-request outcomes. Producer proved_from_checks still maps0 to actual KANI_VACUOUS_PROOF, with positive checks proved. CG classify_success and terminal map preserve actual run Inconclusive/vacuity and checker-absence semantics; OutcomeConstruction helper remains separate IR621 scope. No shim, fallback profile outcome, synthetic proof, new public API, production test-only behavior, unsafe, async/locking or integer/wire conversion was introduced.

Independently diffed lock: only IR/model source entries change from actual baseline to published d41e521, other package bytes/dependencies unchanged. No workflow/CI/waiver/threshold/lint suppression change. New JSON is adversarial test input to the owning typed profile reader, not an emitted production schema copy. Existing family helpers were searched; new combined fixture composes both existing family public seams rather than duplicating a production algorithm. Real test assertions are forced: Result::map callbacks are followed by expect; refusals require exact kind/code/source/context/noBoolean. No tautological or conditionally skipped assertions.

AC82 full request subset/order/distinct causes and legacy reader rejection are evidenced by its three valid binders plus closed production match; AC83 tests missing/blank/duplicate requests return actual typed refusal. AC86/88 new bindings cover only genuine subclauses; the remaining required behaviors are findings below. Bindings marked correct denote valid claimed subclauses, not full criterion completion. A third new dispatch test needs its criterion binding. Existing branching and collection tests were fully read; they do not supply the missing multi-field/early-decision coverage and are TC-only. All actual trace attributes are on tests, never proof or production symbols.

Historical review files are retained unchanged as reports, not new normative tool/version trackers, and do not qualify this new cohort. All current requirements/trace/line assertions above were re-read rather than copied as acceptance.

Independently read current resume/focused.log: actual asserted memory.max17179869184/swap0, fmt-check and locked all-target Clippy PASS,9profile/family+4syntheticclassification+1recordedzero-SUCCESS+2terminal=16actual native tests PASS,0failed/ignored, peak4,935,614,464bytes/events0. Those passing partial controls do not resolve the criterion gaps. Recorded-real-Kani capture classification is not a live backend proof. NFR006 explicitly applies to src/kani and Cargo.lock; required full make kani-gate is PENDING on the final clean/stable candidate and cannot be waived by these reviews or native logs. Root aggregate is also PENDING. No merge-ready or backend-verified conclusion is given.

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
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its
    clean, stable candidate before opening it, and the merger runs it again on the
    clean, stable current candidate before merge; both actual processes exit successfully
    with all expected tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed
    remains unmerged and does not claim real-Kani verification; a written explanation
    does not waive the gate.
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

## Dispositions

Round1 at 55b2da3c53e0e6bbeceb3b3f57131e0bd0afbdbe: all3code findings FIXED after actual source/diff and original raw-control inspection; current source review PASS, conditional on still-pending required full NFR006 real make kani-gate and root aggregate. Initial verdict and original findings remain historical and unchanged. No additional code defect found in this fix.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified the actual fix adds and binds the unsorted branching graph fixture to AC86, including a foreign-field direct-target shortcut and a separately selected other-field positive control; exact DFS order and target-not-expanded assertions plus bound4positive/bound3typed refusal cover the named clauses. Actual field-filter removal compiles and fails the exact ordered expansion assertion ([a] versus [a,b,d,c]); actual > instead of >= bound mutation compiles and fails because four objects are admitted at budget3. Healthy snapshots byte-match current source, and production prefix is unchanged. |
| FND-002 | fixed | Verified the new AC88-bound table has ten literal independently specified cases: both query kinds, decisive positions1/2/3, no decisive item and empty input. Exact returned request checks original order/duplicates; truth values and examined counts are asserted. Actual production full-input-length mutation compiles and fails3versus1 on the first case. Healthy snapshot byte-matches candidate; production unchanged. |
| FND-003 | fixed | Verified the exact dispatch-refusal test now carries FR015AC82. It asserts real Refused kind, original cause/source/context and no Boolean claim for unowned and mismatched dispatch; this genuinely checks the no-profile-to-Inconclusive clause. Fresh matrix includes this exact binder under AC82. |

### Actual focused evidence

fix1/qualification.log reports fmt-check and locked all-target Clippy PASS,10profile/family+4synthetic+1recorded-vacuity+2terminal=17native tests PASS,0failed/ignored. Each of the3red logs shows a compiled single test reaching a substantive assertion failure and exit101, not a compile/setup error. Actual cgroup memory.max17179869184, memory.swap.max0, peak2,402,750,464bytes/events0. The recorded capture remains a native classifier test, not a live backend invocation. The mutation script restores both healthy files byte-for-byte after every red control and before final healthy tests; I independently compared healthy snapshots to the frozen candidate and checked both production prefixes unchanged.

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "references: [\n                (\"a\", \"next\", \"c\"),\n     \
    \           (\"b\", \"next\", \"d\"),\n                (\"c\", \"next\", \"target\"\
    ),\n                (\"a\", \"next\", \"b\"),\n                (\"a\", \"other\"\
    , \"target\"),\n            ]\n            .into_iter()\n            .map(|(source_id,\
    \ field_id, target_id)| FiniteReference {\n                source_id: source_id.to_owned(),\n\
    \                field_id: field_id.to_owned(),\n                target_id: target_id.to_owned(),\n\
    \            })\n            .collect(),\n        }\n        .validate()\n   \
    \     .expect(\"finit"
- fnd: FND-002
  outcome: fixed
  after_excerpt: "let cases = [\n            (QueryKind::ExistsEqual(7), vec![7, 2,\
    \ 2], true, 1),\n            (QueryKind::ExistsEqual(7), vec![2, 7, 2], true,\
    \ 2),\n            (QueryKind::ExistsEqual(7), vec![2, 2, 7], true, 3),\n    \
    \        (QueryKind::ExistsEqual(7), vec![2, 2, 3], false, 3),\n            (QueryKind::ExistsEqual(7),\
    \ vec![], false, 0),\n            (QueryKind::ForAllNonNegative, vec![-1, 2, 2],\
    \ false, 1),\n            (QueryKind::ForAllNonNegative, vec![2, -1, 2], false,\
    \ 2),\n            (QueryKind::ForAllNonNegative, vec![2, 2, -1], false, 3),\n\
    \            (QueryKind::ForAllNonNegative, vec"
- fnd: FND-003
  outcome: fixed
  after_excerpt: "#[trace(\"TC-023\", \"FR-015-AC-82\")]\n    #[test]\n    fn supported_profile_keeps_unowned_and_mismatched_dispatch_refusals()\
    \ {\n        let (profile, _, input) = fixture(|module| CapabilityDisposition"
```

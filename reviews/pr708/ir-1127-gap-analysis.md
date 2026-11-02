---
id: SR-202610310179
title: IR-1127 PR708 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; tests/it/kani_obligations_symbolic_domains.rs; PR708"
review_set: subset
relationships:
  - { target: 'ix://agent-ix/quire-contract-codegen/FR-015', type: references }
---

## Summary

Ticket: IR-1127; related IR-1126. Frozen candidate 3644339ade1e0e7ba2e636c506389d106ef5882c against b9793eea87d89af5005bba4bf01a9dc1542b0d74. One test file, helper plus six call sites; no production, API, spec, dependency or CI change. No applicable AssuranceProfile exists. Source remained read-only; no reviewer Cargo/Kani execution.

## Verdict

CONDITIONAL evidence status: scoped trace/budget delta is clean, but existing Boolean qualification is blocked. This is not a full-repository gap clearance or ticket acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Recomputed quire matrix --scope . --format json; scoped FR-015-AC-96 and AC-99 remain tagged (45 and 20 binders respectively); tags are unchanged. The three installed controls retain correct AC96/AC99 bindings for their partial behavior. Global matrix baseline is retained in matrix.json and is not a fresh repository assurance claim. No new production behavior, reverse ownership gap, stub or inflated evidence introduced. Plan completion: not assessed. Optional whole-repository semantic review not requested and not run. Spec lenses skipped because no spec/plan changes.

Inspected commands-and-results.txt, all four JSON receipts and emitted harnesses, passing-control-summary.json, qualification criterion excerpts, AC96/99/110 trace outputs, and actual ordered-controls/collection-control/focused/fmt/Clippy logs under /home/peter/dev/qualification-logs/titan-ir1127/. Four dispatched proofs use 4294967296 bytes, 120 seconds, unwind 8; actual peaks 201228288–514539520 bytes. Integer OrderedSet and Sequence healthy controls verify; their separately source-authored mutants falsify with in-domain playback and native replay. These are partial controls, not complete AC96/99 qualification. Boolean exact-length-two OrderedSet remains blocked at public IR admission (IllTyped/OperatorIneligible, /semantic_graph/nodes/6/body/result_type), before either Boolean backend runs. Existing upstream intake failure is not introduced by this test-budget helper; no missing Boolean proof credit is granted. IR-1127 and IR-598 remain incomplete. Cancelled diagnostic probe status 143 is not a test result. Author receipts establish fmt and all-target/all-feature Clippy success, default focused 39 passed / 0 failed / 5 ignored. Parent owns final ordinary IT aggregate gate.

## Examined scope

```yaml
scope:
- id: qualified_collection_harness
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: 'memory_bytes: std::num::NonZeroU64::new(4 * 1024 * 1024 * 1024).unwrap(),

    wall_clock: Duration::from_secs(120),'
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 invariant, precondition or postcondition clause, or
    function body, that contains a `quire.op.collection.forall` or `quire.op.collection.exists`
    operation over a selected model collection field is a collection claim. A constraint
    clause follows FR-015-AC-128; it is not a standalone proof obligation. An invariant
    or function-body collection claim uses the direct collection harness. A precondition
    or postcondition collection claim uses its FR-015-AC-38/39/43 operation-clause
    harness, with the same selected symbolic field domain and no second standalone
    collection proof. For a f
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a function-body or invariant collection claim, the direct harness
    asserts the admitted body over symbolic inputs drawn from the selected model field''s
    typed domain and ends with the FR-015-AC-7 cover after its assertions. A precondition
    collection claim is marked as a collection quantifier in its typed V2 clause obligation
    identity and uses the same symbolic field draw in its FR-015-AC-38/39 clause harness:
    an assertion checks that evaluation completes a Boolean on every draw (both true
    and false are allowed), and a separate cover checks that some draw completes true.
    It neither assumes th'
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend
    controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate`
    run on the slice PR's head that is green
- id: tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: fn tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies()
    {
- id: tc_025_real_kani_ordered_set_claim_and_mutant_keep_distinct_active_values
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: fn tc_025_real_kani_ordered_set_claim_and_mutant_keep_distinct_active_values()
    {
- id: tc_025_real_kani_ordered_set_uniqueness_changes_the_verdict
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: fn tc_025_real_kani_ordered_set_uniqueness_changes_the_verdict() {
```

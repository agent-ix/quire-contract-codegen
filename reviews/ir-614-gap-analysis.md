---
id: SR-14361
title: gap-analysis of IR-614 full batch timeout regression
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; src/kani/run/execute.rs; computed repository
  specification matrix
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-017
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-028
  type: references
---

## Summary

Ticket: IR-614. PR: quire-contract-codegen#483. Frozen candidate ce5149e2e0498a35166972817dfcb4248d63ab20. Only the 21-line test addition is under change review. Current main also contains independently reviewed documentation from #482; that separate change is outside this implementation. No applicable AssuranceProfile exists. Scope excerpts longer than 600 characters use a verbatim prefix; complete scoped timing criteria were read. Reviewer: GPT-6 (deployment SKU unavailable; marker uses gpt-6). No source edits, Cargo, rustc, native or Kani runs were performed by this reviewer; IR-615 holds the exclusive heavy slot. The root aggregate gate remains pending on the final reviewed head.

## Verdict

FAIL for the inherited repository-wide strict matrix; scoped IR-614 delta has no introduced gap. FND-001 needs an explicit out-of-scope disposition before merge; the repository is not globally green.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The computed repository matrix contains 291 untagged criteria, including FR-005-AC-3; the repository cannot claim a strict traceability pass. These gaps are inherited: this PR adds only one test and removes no criterion or existing binding. | spec/core/functional/FR-005-cli-conformance.md:49 |

## Coverage

Plan completion: not assessed

No plan was selected. Optional repository-wide semantic review was not requested and was not run. Mechanical criterion/binder audit covers all 869 computed units: 510 tagged, 291 untagged, 50 method-without-symbol and 18 tagged-by-ignored-test. quire 0.36.2 (engine 0.50.2) strict matrix exits 1; installed-module warnings are preserved separately, and computation did produce the complete JSON matrix. Ignored-only bindings are reported as their actual computed state; dedicated-lane execution and semantic adequacy are not inferred. No hand-written matrix status or historical Covered label is accepted as execution evidence. All untagged IDs are recorded in FND-001 artifact_id/related_ids and the raw matrix; mechanically read matrix rows are not presented as individual semantic reviews. Scope lists the criteria whose actual wording was checked; this grouped inherited finding does not claim they are introduced by IR-614. The only new test adds binders to already-tagged FR-017-AC-21 and FR-028-AC-12, confirmed at execute.rs line 3561. No criterion, ignore state or existing binder is removed. Repository-wide PASS is expressly not claimed.

Reverse inventory follows the repository’s core, oracle, publication, routed, replay, evidence, strategy and Kani subsystem owners in spec/spec.md and the existing FR documents. This test introduces zero public behaviors, zero unowned behaviors and no new production implementation; the affected launch/grouping/bounds/report semantics are owned by FR-017 and FR-028. Source/test TODO/todo!/unimplemented!/FIXME scan has nine matches, all test strings or intentionally malformed generated-input fixtures, no actual production placeholder invocation. Focused source and test inspection found zero new source stubs, zero new test stubs and no new coverage inflation. This is not an exhaustive semantic certification of every inherited implementation. Actual Linux execution and call-site mutation receipts are detailed in SR-14360; they establish this local regression, not real Kani/CBMC solver execution. TC-039 and TC-043 provide context for the correct AC bindings. Existing helper-bound arithmetic/overflow tests remain intact.

## Examined Scope

```yaml
scope:
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and
    equal identity `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly
    one launcher process whose argument vector holds one `--harness <module::harness>
    --exact` pair per member in request order, `--harness-timeout` T, then the shared
    options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses
    start fewer than N processes; one harness starts one process with the single-run
    argument vector. Requests whose wall-clock or memory ceilings differ start separate
    processes. The group's proc
- id: FR-028-AC-12
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: In a group of N harnesses (one launcher process of a FR-017 batch) sharing
    a wall-clock budget T, a member the backend cuts off (entry status Failure, no
    checks, exit status `timeout`) is `inconclusive` timed-out naming T and never
    falsified, while members that finished keep their verified or falsified results;
    a group still running at N times T is killed, refused as timed out, and no member
    is classified; an N times T too large to represent never elapses and does not
    panic; a T above 4294967295 whole seconds launches the group without `--harness-timeout`
    and the group still runs, its outer bo
- id: TC-039
  path: spec/kani/matrix/TC-039-bounded-proof-ceilings.md
  role: context_only
  excerpt: "## Expected Results\n\n1. Each ceiling change changes the identity (FR-028-AC-1).\n\
    2. The run is stopped and is `inconclusive` with the timed-out reason naming the\
    \ ceiling, never\n   verified or falsified (FR-028-AC-2).\n3. The run is stopped\
    \ and is `inconclusive` with the memory-exhausted reason naming the ceiling\n\
    \   (FR-028-AC-3).\n4. Each evidence records every argument's bounds and both\
    \ ceilings (FR-028-AC-4).\n5. Each names the family (FR-028-AC-5).\n6. The harness\
    \ records proof subject `production` (FR-028-AC-6).\n7. The result holds a `bounded_shadow`\
    \ harness together with its refinement obligat"
- id: TC-043
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: context_only
  excerpt: "## Expected Results\n\n1. Each over-limit stream is refused with `kani_output_over_limit`\
    \ naming the stream, the limit and\n   the harness count, the group is killed,\
    \ and nothing is classified; exactly the limit completes\n   with its real exit\
    \ status and the whole stream (FR-017-AC-14).\n2. Both are refused with `kani_output_unread`\
    \ and no outcome (FR-017-AC-25).\n3. The grandchild is killed by the time the\
    \ run returns (FR-017-AC-24).\n4. One process for each group, with one `--harness\
    \ <module::harness> --exact` pair per member in\n   request order, `--harness-timeout`\
    \ T and then the shared options; "
- id: a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "    fn a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound()\
    \ {\n        let stand_in = StandIn::running(\n            \"full-member-count-bound\"\
    ,\n            &format!(\"sleep 30\\n{VERIFY_EVERY_HARNESS}\"),\n        );\n\
    \        let evidence = only_group(stand_in.batch(&pair(), Duration::from_secs(20)))\n\
    \            .expect(\"both members must finish before their full 40-second batch\
    \ bound\");\n        assert_eq!(\n            outcomes(&evidence),\n         \
    \   vec![KaniRunOutcome::Verified; 2],\n            \"using one fewer member at\
    \ the batch launch would kill this process before its report\"\n"
- id: FR-005-AC-3
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: For every shared-corpus case on which two backends' normalized outcomes
    differ, the TC-007 Differential Disposition Register holds exactly one row naming
    the case, the backends, and either a regression fixture that exists in the test
    tree or a documented profile/backend distinction; a differing case with no row,
    a row naming a fixture that does not exist, or a row with neither disposition
    fails the check.
```

## Bindings

```yaml
bindings:
- test_id: batch_tests::a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound
  ac_id: FR-017-AC-21
  trace: correct
- test_id: batch_tests::a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound
  ac_id: FR-028-AC-12
  trace: correct
```

## Dispositions

Round 1: independently verified IR-878 is In Progress, its description owns strict-coverage triage, and its posted owner handoff exactly matches the recorded body. Candidate source remains unchanged.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Deferred to existing In Progress owner IR-878 (IR team / luna:IR:plan) by explicit owner handoff f89fc64c-38dd-423c-9a7a-a629eb2d5810. The 291 untagged criteria and 18 ignored-only states predate this test-only change; no criterion or existing binding is removed. Repository strict matrix FAIL remains unchanged. Kani-lane rows retain titan:kani:plan ownership; IR-705/IR-17 remain parked on QSL-924. This is out of scope for the narrow IR-614 call-site mutation regression and is not closure or a global coverage PASS. |

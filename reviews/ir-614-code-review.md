---
id: SR-14360
title: code-review of IR-614 full batch timeout regression
type: SpecReview
analysis: code-review
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

PASS — no introduced code/Rust findings; the focused test kills the real call-site mutant. Final aggregate acceptance remains the lead’s separate gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Code-review includes the Rust review lane. Existing StandIn, pair, only_group, outcomes and VERIFY_EVERY_HARNESS are reused; no duplicate helper, vendored code, source tracker, new production behavior, public API, unsafe, numeric/wire conversion, waiver, workflow change or weakened gate is added. Actual #[trace] is on the Linux #[test], never a proof or production function. The call chain is StandIn::batch -> execute_kani_obligations -> plan_groups -> run_group -> start; the Linux call site passes the actual group.len() to outer_bound. The separate non-Linux report fixture is not exercised or mutated. One group and exactly two Verified outcomes reject missing or partial classifications, and recorded launcher calls equal one. The launcher is the existing external backend seam; process, namespace/procfs supervision, production timeout and report parsing remain real. Fixture scratch directories and cleanup are reused; no port, environment or shared path is introduced.

The 30-second backend sleep supplies the work duration being bounded; the test waits for actual public execution evidence, not sleep-then-observe synchronization or a benchmark elapsed-time assertion. N=2 and T=20 seconds place completion 10 seconds on either side of the candidate limits. As with any real deadline integration test, a host stall exceeding that slack can cause a timeout; no guarantee under arbitrary suspension is claimed. No retry, skip, increased production timeout, clock mock or IR-877 edit is introduced. Overflow and timeout refusal semantics retain their adjacent existing tests.

Read the raw resource logs independently: fmt --all --check exits 0; locked Clippy --all-targets -D warnings exits 0. healthy.log used a wrong exact filter and ran zero tests: it is build-only evidence and is not accepted as a test pass. healthy-focused.log actually ran this one test and passed in 30.01 seconds. mutant.log actually ran the same test and failed with BatchTimedOut { members: 2, timeout: 20s } in 20.02 seconds. restored-focused.log ran that exact test and passed in 30.01 seconds. callsite-control.py replaces only the first of two matching arguments, the real Linux run_group group.len() with group.len()-1; it asserts exactly two original matches and byte-exact restoration. The archived healthy-execute.rs independently byte-matches the candidate source. The helper outer_bound, production semantics, fixtures, lockfile and options are unchanged. Every recorded check ran with actual memory.max=17179869184 and memory.swap.max=0 and no memory events. These are focused test receipts, not a full-suite or real Kani qualification.

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

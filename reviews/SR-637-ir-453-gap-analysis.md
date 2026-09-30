---
id: "SR-637"
title: "CG PR 205 gap analysis: corpus replay rows after retiring the native replay"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@d7a865dc067b1d002c455262eba60b63972b61d9; spec/test-matrix.md, spec/test/TC-023-bounded-kani-profile-corpus.md, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md, spec/test/complete-v1/TC-035-counterexample-envelope-intake.md, src/bounded_kani_corpus.rs, tests/it/bounded_kani_corpus.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
---

# SR-637: CG PR 205 gap analysis

## Summary

Ticket: IR-453. PR: agent-ix/quire-contract-codegen#205, head d7a865d, base main bb8523f. This is
a planless gap analysis scoped to the PR diff. Plan completion: not assessed.

## Method

I ran `quire coverage --scope . --strict --json` (quire 0.33.0) on a detached worktree at main
bb8523f and at the head, and compared the two reports set by set. Both exit 1 with
"66 unbacked row(s) and 0 contradicted status(es)". The `unbacked_rows` sets are identical (66 and
66, none added and none removed). So are `status_lies` (0 and 0), `no_symbol_rows` (11 and 11),
`untracked_symbols` (1 and 1) and `unmatched_tags` (76 and 76). The totals are unchanged at 182
backed of 278. FR-024-AC-5 and the TC-035 row are unbacked both before and after. TC-023's matrix
row (FR-015-AC-23, Planned) is still backed by the `tc_023_*` tests that remain. No deleted test
backed any row that was not backed by another test. No matrix row and no AC row is deleted. The PR
leaves `spec/test-matrix.md` untouched, so both rows stay Planned, and no trace tag is invented.
`make spec` runs `quire validate`, not `coverage --strict`, and passes at the head.

I also compared what each deleted test asserted with the assertions that remain:

- The `bounded_kani_replay` unit test and the two integration replay blocks assert only agreement
  with a constant closure, plus assignment content. The assignment content was packet
  construction and is truthfully dropped with the packet.
- The `arithmetic_assignments` and `collection_assignments` unit tests, and the
  assignment-out-of-range retry test, pin helpers and a refusal code that exist only to build the
  packet. They are truthfully dropped. The case-counter invariant is still covered by the
  dependency-refusal tests.
- `tc_023_false_case_retains_a_replayable_counterexample_packet` also asserted the graph-family
  false classification. That assertion is not packet-related, and it is not preserved (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-023 requires counterexample cases for each semantic family. After this PR, no test asserts a graph-family counterexample in the corpus. The deleted unit test was the only one, and its `Some(false)` assertion did not depend on the packet. Coverage does not register the loss, because TC-023's row stays backed by other `tc_023_*` tags. | src/bounded_kani_corpus.rs:745, spec/test/TC-023-bounded-kani-profile-corpus.md:23-24 |

## Verdict

The spec side of the retirement is honest. It deletes no rows, backs no row with removed tests,
leaves the unbacked set unchanged and invents no tags. Fix FND-001 in this PR. It is the same
defect as SR-636 FND-001, seen from the evidence side.

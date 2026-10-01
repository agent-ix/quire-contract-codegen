---
id: "SR-643"
title: "CG PR 207 gap analysis: TC-023 corpus identity claims against tests and code"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@68caecc4bcd01a0049802275f06f8d5613475461; src/bounded_kani_corpus.rs, spec/test/TC-023-bounded-kani-profile-corpus.md, spec/interface/interface-001-codegen-api.md, spec/test-matrix.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: reviews
---

# SR-643: CG PR 207 gap analysis

## Summary

Ticket: IR-462. PR: agent-ix/quire-contract-codegen#207, head 68caecc. This is a planless gap
analysis of the corpus-identity clauses that the PR adds to TC-023, and of the existing
interface-001 `kani_corpus_identity_collision` semantics, checked against the tagged tests and the
code. Plan completion: not assessed.

## Method

For each clause I found the backing `Trace: TC-023` test and ran mutation probes. Each probe was a
scratch edit in the review worktree, run with `cargo test --locked --lib tc_023` and then reverted
(the tree was confirmed clean afterwards). I also checked that the matrix and the FR-015 ACs were
not changed or over-claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-023 now says "requests differing in any field name different ones", but the distinctness test varies only the arithmetic `source_id`, the arithmetic `right`, the census, the family and the collection `max_items`. I removed one identity component at a time. Of 13 removals, 12 left every TC-023 unit test green: the whole finite input, the arithmetic operator, left, minimum and maximum, all four graph fields (start, target, field, max_expansions), the collection values, the query kind and the collection source id. Only removing the census was caught. The input removal matters most, because the graph oracle is built from the input references. Without the input in the identity, two graph cases over different universes would get one name. The code is correct today, but the clause is mostly untested, and together with SR-642 FND-001 nothing would catch a dropped field. Either add one varied request per family that changes each field (or a table test over single-field variants), or narrow the TC text to what is tested. | src/bounded_kani_corpus.rs:1044-1076 |

## Verdict

Mergeable once FND-001 is addressed, either by covering the other fields or by narrowing the
TC-023 sentence.

Backed and verified:
- Order and run independence. `tc_023_case_identity_is_independent_of_emission_order_and_run`
  compares a forward and a reversed emission through two fresh registries, and checks that each
  proof symbol equals `corpus_case_` plus the harness path's case name. Counter naming would fail
  it, because the arithmetic counter positions differ between the two orders.
- Double emission is refused. `tc_023_a_request_emitted_twice_is_refused_as_an_identity_collision`
  asserts `InvalidInput` and `kani_corpus_identity_collision` on two repeat attempts, and that a
  fresh registry reproduces equal artifacts.
- A refused case claims no identity. Both dependency-refusal tests retry the same request through
  the same registry and are accepted.
- I re-ran the coder's mutation: dropping `max_items` makes the distinctness test fail. I did not
  re-run the counter-naming mutation, but the test text supports that claim.
- Nothing is over-claimed. `spec/test-matrix.md` is unchanged, the TC-023 row is still Planned, and
  no FR-015 AC text changed. interface-001 already described the collision refusal. The corpus
  replay retired in #205 stays retired.

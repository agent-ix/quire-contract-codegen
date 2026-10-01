---
id: "SR-631"
title: "IR-412 slice 1 gap analysis: FR-015-AC-26 to AC-28 against tests and interface"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@01204aba72480578167cc3510ba13a535083f39c (disposition round 1 at fea49c75bfdf51a000b042dfa2735cb7cd4300bd, round 2 at 5879a61858c1cc76f0f14809788efd904ed7ea16); spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/test-matrix.md, spec/test/complete-v1/TC-025-bounded-kani-obligations.md, spec/interface/interface-001-codegen-api.md, src/state_frame.rs, tests/it/kani_obligations_state_frame.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-631: IR-412 slice 1 gap analysis

## Summary

Ticket: IR-412 (slice 1). PR: agent-ix/quire-contract-codegen#203, head 01204ab (rebased on main bb8523f; first reviewed at 7fd5a7b). This is a manual
check that maps acceptance criteria to tests, scoped to the PR diff. Plan completion: not
assessed.

## Method

I mapped FR-015-AC-26, AC-27 and AC-28 to the tests that carry their trace tags. I compared each
case an AC enumerates with the cases the tests exercise. I checked the new public API against
`spec/interface/interface-001-codegen-api.md`, which lists every public operation.

- **FR-015-AC-26** is tagged on
  `tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness` and
  `tc_025_the_frame_harness_follows_the_frame_node_not_the_caller`. Both bindings are correct.
- **FR-015-AC-27** is tagged on `tc_025_shapes_without_a_finite_encoding_are_refused_by_name` and
  `tc_025_malformed_requests_and_non_clause_nodes_are_refused`. The bindings are correct but
  coverage is partial (FND-001).
- **FR-015-AC-28** is tagged on the three `#[ignore]` real-Kani tests. The bindings are correct.
  The mutation controls turn the proofs red for the intended reason, and each counterexample
  names the property.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-27 enumerates refusals that no test exercises: an `invariant` clause, a frame that `deletes`, a `modifies` entry that is a relationship, a foreign-field grant, and a condition that is not a comparison. The tests cover precondition, creates, same-side, differing fields, two ranges, the missing field and all-granted. Yet the matrix row marks AC-26 to AC-28 as fully covered. | tests/it/kani_obligations_state_frame.rs:424-496, spec/test-matrix.md:51 |
| FND-002 | medium | The new public operation `generate_state_frame_obligations` (and the `KaniExecutableHarness::StateFrame` input that `execute_kani_obligation` now accepts) is not listed in interface-001, which lists every public codegen operation, including its inputs, outputs and refusals. | spec/interface/interface-001-codegen-api.md:105-112, src/lib.rs:140-144 |
| FND-003 | low | FR-015-AC-26's identity-scoping claim ("scoped to the operation, anchor, frame and framed object") is tested only as equality of the scope fields with the fixture's. No test shows that a different operation or frame yields a different identity or module. | tests/it/kani_obligations_state_frame.rs:320-326 |

## Verdict

Mergeable once FND-001 and FND-002 are fixed, or once FND-001's matrix row is downgraded to
partial with the untested cases named. The code behind each untested refusal exists
(src/state_frame.rs:633-641 and 673-698) and reads correctly, so the gap is only in the evidence.

The deviation from the plan on lowering profiles is justified. See SR-630 Verdict.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | `tc_025_replay_frame_refuses_an_envelope_that_disagrees_with_its_payload` is tagged FR-015-AC-32. AC-32 claims that the forbidden counterexample reproduces natively and that the allowed run replays as a respected frame. This test exercises a different property: that QSL's own `replay_frame` refuses a mismatched envelope `clause_node` or `occurrence_key`. No FR-015 AC states that property, so the binding is wrong. | tests/it/kani_obligations_state_frame.rs:763-765, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:191 |

## Dispositions

Round 1 at fea49c7. Every refusal that AC-29 enumerates now has a row in
`tc_025_shapes_without_a_finite_encoding_are_refused_by_name`. Removing the deletes, relationship
or foreign-field refusal, the `self`-only read or the bound guard turns that row red (mutants
M1 and M3 to M6 in SR-630).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4af93a1 |
| FND-002 | fixed | 4af93a1 |
| FND-003 | fixed | 4af93a1 |
| FND-004 | fixed | 5879a61 |

Round 2 at 5879a61. The envelope-disagreement test is now traced to TC-025 only. No finding remains open.

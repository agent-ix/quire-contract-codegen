---
id: "SR-1515"
title: "CG PR 279 code review (Rust lane): the StateFrame arm of negotiate and the frame role split"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@c7d6517431a207415d625d35ef2d389e331b511f; src/kani/generate/{frame,negotiate,outcome}.rs, src/lib.rs, tests/it/{kani_obligations_state_frame,kani_obligations,layout}.rs (diff origin/main...HEAD, merge base bcce798)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-005
    type: references
---

# SR-1515: CG PR 279 code review

## Summary

Ticket: IR-461 (code PR 2 of 2). PR: agent-ix/quire-contract-codegen#279 at c7d6517, two commits
on bcce798. This review covers code review and the Rust review lane (`rust-review`) of the PR's
own diff. Gap analysis and the test-oracle strength measurements are in SR-1516. The spec-review
of the spec edits, including the AD-004 amendment, is in SR-1517.

What I measured myself:

- `make ci` on the head with my own target dir: exit 0. It ran fmt-check, spec, clippy
  `-D warnings`, msrv, deny, audit-unsafe and rustdoc `-Dwarnings`, then the tests: 164 lib
  passed, 358 it passed, 23 ignored, 0 failed. The only warnings are the existing FR-017
  EARS warnings and the cargo-deny notices, all from before this PR.
- I did not run `make kani`, because the host's Kani lock is contended. Instead I checked that
  the generated harness text did not change. On base bcce798 and on the head I generated the
  six harnesses (rust and record) of three healthy clauses through
  `generate_state_frame_obligations` and compared the bytes: identical. The Kani lane's inputs
  are unchanged.
- The refusal order of the single-clause entry does change for a clause that both roles refuse
  (FND-001):

  | Clause | base bcce798 | head c7d6517 |
  | --- | --- | --- |
  | negation, and the frame grants every field | `ConditionNotSupported` | `NothingForbidden` |
  | frame grants the only state field, and the clause field is missing from the state | `UnknownStateField { field: "balance" }` | `NothingForbidden` |

- NFR-005: the non-test diff has no `unwrap`, `expect`, `panic!`, `unreachable!` or new
  slicing. The new `states[index]` write in `reject_duplicates_and_mixtures` is inside
  `0..states.len()`, the same form as the existing lines, and is not a site NFR-005 lists.
  Removing the `allow(dead_code)` also removes the old comment that held the word `expect`.
- Public API: `StateFrameRole`, `ObligationItem::StateFrame` and
  `KaniObligationOutcome::Emitted.state_frame_harnesses` are new. The only in-crate consumer
  of `Emitted` outside negotiate is `src/routed/generate.rs:323`, which already uses `..`. No
  sibling checkout under ~/dev names these types. There is no compatibility shim.
- Byte identity through the arm: the arm pushes `generate_state_frame_role`'s harness through
  unchanged (`state_frame_harnesses.push(*harness)`).

## Verdict

APPROVE WITH LOW FINDINGS. The role split is faithful. `prepare` holds the common grounds
(request, lowering, node kind, postcondition, malformed clause); the contract and frame
functions hold their own grounds, as the FR-015 independence paragraph assigns them. The harness
bytes are unchanged. The arm follows the existing negotiate passes: identity-based
`DuplicateItem` that includes the role, a separate `MixedStatePackages` latch, and
`is_invalid` extended to a mapped `invalid_request`. Every mutant of the arm I tried was killed
(SR-1516). The two findings below are low and do not block the merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The comment on `generate_state_frame_obligations` says "The frame is read before the condition, as this entry always has: a clause refused by both roles reports the frame's refusal". The second half was not true of the old entry. The old entry checked the condition before `NothingForbidden`, and the condition field's `UnknownStateField` before the frame's grants-all check. Measured on base and head: negation with grants-all moved from `ConditionNotSupported` to `NothingForbidden`, and grants-all with a missing clause field moved from `UnknownStateField` to `NothingForbidden`. No AC fixes this order. AC-29 asks only for a typed reason, with a malformed request first, and FR-015 says "the first refusal". So this is an undisclosed behaviour change, not a spec violation. The PR's claim that "an existing test requires the frame's refusal before the condition's" also does not hold: swapping the two calls (contract role first) survives the whole `cargo test --all-targets` suite (164 lib, 358 it). Fix: correct the comment so it says the order is frame role then contract role and that it changed for doubly-refused clauses. If the order is meant to be a contract, pin it in FR-015 and in a test; otherwise say in the comment that it is unspecified | src/kani/generate/frame.rs:143-146 |
| FND-002 | low | `Outcome::StateFrameRefused(ObligationDisposition)` can hold a `Supported` disposition by type, and only a doc comment forbids it. `disposition_without_harness` passes it through unchanged, and `is_invalid` matches inside it. Holding the `StateFrameRefusal` and mapping it with `state_frame_disposition` where the record is built (or holding a narrower type) would make the bad state unrepresentable. It is correct today because `state_frame_disposition` never returns `Supported` (the AC-66 table test pins that) | src/kani/generate/negotiate.rs:220-225, src/kani/generate/negotiate.rs:309 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new `first_refusal` doc says it reports "the refusal this entry has always reported first for a clause that both roles refuse, so that splitting the engine into roles does not change which one it names". I found one counterexample. Measured on main 04eb1e6 vs head b1bfa8d: a frame that creates an object (`FrameEffectUnsupported`) combined with a condition whose graph field name is the keyword `type` (`MalformedClause`) refused `FrameEffectUnsupported` on main and refuses `MalformedClause` on head. `prepare` returns the condition's `MalformedClause` before `first_refusal` reads the frame's grants. Head's answer fits FR-015, which lists a malformed clause as a ground common to both roles, so this is wording, not behaviour: qualify the doc ("except that a malformed clause, a ground of both roles, is reported first"), or accept as is. No other doubly-refused case I measured differs: negation with grants-all, grants-all with a missing clause field, and both renders over the ceiling are all identical | src/kani/generate/frame.rs:152-157, src/kani/generate/frame.rs:250-258 |

## Dispositions

Round 1, reviewed at b1bfa8d32587e558ee1b3f1c1bdf6a028a4278ab (fix commit 08ec6a0, then the merge of main 04eb1e6, which contains #278).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 08ec6a0. `first_refusal` restores the old order before the roles run. Measured main vs head: negation with grants-all is `ConditionNotSupported` on both, and grants-all with a missing clause field is `UnknownStateField { field: "balance" }` on both. The new test `tc_025_the_single_clause_entry_keeps_its_first_refusal_when_both_roles_refuse` pins the first case. Dropping the `first_refusal` call (the round-0 behaviour) is killed by it. The literal swap mutant (contract role before frame role) now survives, but it is nearly equivalent: once `first_refusal` has run, only render failures remain, and with both renders over the ceiling the refusal is the same. The residual wording issue is FND-003 |
| FND-002 | fixed | 08ec6a0. `Outcome::StateFrameRefused(StateFrameRefusal)` is mapped by `state_frame_disposition` in `disposition_without_harness`. `is_invalid` is an exhaustive `match` with no wildcard. The arm mutants (Rejected rule, generic reason) are killed |

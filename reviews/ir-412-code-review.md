---
id: "SR-630"
title: "IR-412 slice 1 code review: state-clause operation-contract and frame-effect Kani harnesses"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@01204aba72480578167cc3510ba13a535083f39c (disposition round 1 at fea49c75bfdf51a000b042dfa2735cb7cd4300bd, round 2 at 5879a61858c1cc76f0f14809788efd904ed7ea16); src/state_frame.rs, src/kani_execution.rs, src/kani_obligations.rs, src/lib.rs, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs, tests/state_frame_support/subject.rs, tests/exact_scalar_support/package.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-630: IR-412 slice 1 code review

## Summary

Ticket: IR-412 (slice 1). PR: agent-ix/quire-contract-codegen#203, head 01204ab (rebased on main bb8523f; first reviewed at 7fd5a7b, src unchanged between the two).
This review covers code-review with the rust-review lane folded in. It is scoped to
`git diff origin/main...HEAD`.

## Method

I read every changed file in full. I traced the clause decoder in `src/state_frame.rs` against
QSL's own `state_clause` and `frame` lowering (qsl-semantics `check/lowering/state.rs` at the
locked qsl-replay revision 966e7d2), which shows what the parameter aggregate and the `modifies`
entries hold. I checked the generated harness text for soundness: the direction of the pre/post
comparison, whether the frame is complete, where the cover sits, and which ranges are assumed.
I checked the `KaniExecutableHarness::StateFrame` hook against the cover-based classifier in
`kani_execution.rs`. I checked textual and semantic merges against current main (bb8523f, #201) and
against PR #202's head (5ef88a8). I ran `make ci` with a private TRUSTED_HOME at both 7fd5a7b and 01204ab, and it exited 0 both
times (227 passed and 8 ignored at 01204ab). I also ran the three `#[ignore]` state-frame real-Kani
tests at 01204ab, under the host lock and with `--test-threads=1`, and all 3 passed. I did not
run source mutations. I checked the mutation controls by reasoning over the default-lane
assertions and the seeded subjects.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `field_read` accepts a read through any clause parameter as a read of `self`. QSL's parameter aggregate holds `self`, then `result`, then every operation parameter. A clause such as `self.balance >= pre(other.balance)`, with `other` an operation parameter of the same object type, is therefore rendered as `post.balance >= pre.balance` and can verify a property the clause does not state. Only `parameters[0]` (the `self` slot) should be accepted. | src/state_frame.rs:811-820 |
| FND-002 | medium | The range assumed of the clause field is the one `integer_range` in the whole lowered closure, not the field's own declared range. The closure reaches every member of the framed object, so any object with two distinct integer ranges is refused (`BoundNotResolved`). If the clause field's type is bounded by something other than an `integer_range`, the one range reached belongs to another field and would over-constrain the clause field. `state_domains` already resolves each field's own declared range (`declared`), which could be used for the clause field too, with the closure range only cross-checked. | src/state_frame.rs:432, src/state_frame.rs:859-875, src/state_frame.rs:916-920 |
| FND-003 | low | The `StateFrame` hook reports `kind: None` in the execution view, and so in evidence. A run's evidence cannot tell an operation-contract proof from a frame proof, although the doc comment on `ObligationKind::Frame` now says these harnesses produce frames. | src/kani_execution.rs:211-220, src/kani_obligations.rs:94-96 |
| FND-004 | low | The frame module and record path are keyed on the anchor digest alone (`frame_<anchor12>`). The frame identity also carries `clause`, `state_path`, `subject_path` and `domains`. Two postcondition clauses of one operation therefore produce two different `kani-obligations/frame_<anchor>.json` records at the same path. | src/state_frame.rs:478-487, src/state_frame.rs:967-979 |
| FND-005 | low | The operation name from the graph literal is interpolated into the `assert!` format string. A `{` or `}` in it becomes a format placeholder, so the harness fails to compile. `syn::parse_file` does not parse macro bodies, so the pre-check does not catch this. Braces should be escaped, or the message passed as an argument to `"{}"`. | src/state_frame.rs:1075-1081, src/state_frame.rs:1099-1106 |
| FND-006 | low | The new `UnsupportedObligation::FrameNotClauseRendered` variant replaces an arm that V1 cannot reach (V1 has no frame clause kind). No test asserts it, so nothing would notice if it regressed back to `RenderFailed`. | src/kani_obligations.rs:245-248, src/kani_obligations.rs:1937 |
| FND-007 | low | The native twin is tied to the Kani fixture by names only. Its `modifies: [balance]`, its field set and its `Int[0,1000]` bounds are authored independently of the CG fixture package, and the replay envelope carries the twin's own frame, occurrence and a synthetic `obligation_identity` (`[1; 32]`). The replay therefore confirms QSL's frame verdict on the natively executed values, but not that the replayed obligation is the one Kani proved. Drift between the two fixtures would go undetected. | tests/state_frame_support/native_twin.rs:1-9, tests/state_frame_support/native_twin.rs:185-189, tests/state_frame_support/native_twin.rs:391-395 |

## Verdict

Not mergeable until FND-001 is fixed. The fix is one line: accept only the `self` slot, which is
`parameters.first()`. A test with a second parameter of the object type should back it. FND-002
should be fixed in this PR or deferred with a ticket. The current behaviour refuses realistic
objects, and it relies on the IR `require_bounds` rule to rule out the over-constraint case.

What is right:
- The postcondition harness snapshots `pre`, runs the subject on `post = pre.clone()`, and
  asserts `left <op> right` with the operand sides taken from the graph's `pre(...)` placement.
- The frame is complete. The struct literal names every `state_fields` entry, so rustc rejects a
  field list that differs from the struct. Every field not granted is asserted unchanged.
- Each harness has exactly one `kani::cover!` after the subject call. The classifier treats
  success without every cover satisfied as vacuous for every harness kind, so contradictory
  assumed ranges cannot pass silently.
- Assumptions are applied only to the pre-state, and only for fields the IR bounds. A field with
  no declared range stays fully symbolic, so no bug is hidden by over-constraint, except for the
  FND-002 edge.
- Refusals are typed. There are no `unwrap`, `expect` or `panic` calls in `src/`, and integer
  literals are parsed with `parse().ok()?`.
- The test lanes kill the relevant generator mutants. Swapping or weakening the comparison is
  caught by `deposit_debiting`. Emptying the grants is caught by the regenerated frame. Dropping
  the cover and choosing the wrong field are caught by the default-lane source assertions.

Keeping `CheckedNodeTag::State` out of the scalar, composite and function profiles is correct.
Those generators refuse `State` explicitly with a typed reason (for example
composite_equality.rs:868, exact_function.rs:570 and 639). Adding it would replace that precise
refusal with a generic not-an-expression refusal and serve no consumer. The plan's intent is
met by the dedicated `STATE_FRAME_LOWERING_TAGS` profile.

The constraints hold. IR is untouched and never names QSL. CG reaches QSL only through
qsl-replay. Nothing is vendored. The QSL unit's profile digest is QSL source syntax that
`compile` checks, so it is load-bearing, and it is the same constant as in
`tests/it/skeleton_spine.rs`. There are no compatibility layers.

Merges and the rebased head:
- At 01204ab the branch is rebased on main bb8523f (#201) and merges cleanly. The `src/lib.rs` and
  `spec/test-matrix.md` resolutions keep both sides. The only other change since 7fd5a7b is commit
  01204ab: a `Cargo.lock` bump of the QSL crates only (966e7d2 to c7fd631), the twin selecting
  `deposit` through `resolve_operation`, and `skeleton_spine.rs:283` wrapping
  `WitnessValue::Integer`. `src/` is byte-identical to 7fd5a7b, so every finding above still
  applies at the same lines.
- Against PR #202 (5ef88a8), `kani_execution.rs`, `Cargo.lock` and `Cargo.toml` auto-merge. The
  one conflict is adjacent rows in `spec/test-matrix.md`, which is trivial. #202's
  precondition-only classification branch keys on `Some(Precondition)`, and the `StateFrame` view
  passes `None`, so the two are semantically compatible. #202 will need its own re-lock and
  re-gate against QSL c7fd631 after this lands.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | The braces regression test exercises `assertion_message` on a string the test builds itself, not the generators' output. Reverting either generator to `assert!(cond, {message})`, with the message as the format string, passes every test (mutant M9 survived), so the FND-005 fix is not guarded. | src/state_frame.rs:1099-1108, src/state_frame.rs:1062-1064, src/state_frame.rs:1083 |
| FND-009 | low | Nothing tests the `StateFrame` evidence-kind mapping. Reporting a frame harness as `ObligationKind::Postcondition` passes every default-lane test (mutant M8 survived), and the real-Kani tests read only `evidence.outcome`. | src/kani_execution.rs:215-218 |

## Dispositions

Round 1 at fea49c7 (fixes in 4af93a1 and fea49c7). `make ci` exited 0 with a private TRUSTED_HOME (230 passed, 8 ignored). The three state-frame real-Kani tests passed (3 of 3). I re-checked each finding by mutating the fix back out:

- M1 (`.contains` restored), M2 (the clause field takes another field's range), M3 (the `BoundNotResolved` guard dropped), M4, M5 and M6 (the deletes, relationship and foreign-field refusals dropped) and M7 (the clause digest dropped from the frame path) were all killed by the default lane.
- M8 and M9 survived. See FND-009 and FND-008.

Adding `CheckedNodeTag::Relation` to the state-frame lowering tags is acceptable. The closure reaches a relation node only through a frame's relationship grant, which `frame_grants` then refuses as `Relationship`, or through the condition, which the decoder refuses. A field-only frame on the object therefore admits nothing new.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4af93a1 |
| FND-002 | fixed | 4af93a1 |
| FND-003 | fixed | 4af93a1 |
| FND-004 | fixed | 4af93a1 |
| FND-005 | fixed | 4af93a1 |
| FND-006 | fixed | 4af93a1 |
| FND-007 | fixed | 4af93a1; fea49c7 adds the envelope-identity refusal test. The obligation-identity join remains impossible by QSL API and is documented at native_twin.rs:6-12 |
| FND-008 | fixed | 5879a61 |
| FND-009 | fixed | 5879a61 |

Round 2 at 5879a61, rebased on main dc19928. Range-diff shows the five earlier commits unchanged; the round-1 fix commits 4af93a1 and fea49c7 are now 17e762f and a8cbd18. `make ci` exited 0 with a private TRUSTED_HOME (230 passed, 8 ignored). I re-ran the mutants that survived round 1, and all three are now killed:

- M8 (frame harness reported as `Postcondition`) is killed by `tc_027_a_state_frame_harness_reports_the_kind_of_what_it_proves`.
- M9a (frame message turned back into the format string) is killed by `tc_025_an_operation_name_with_braces_cannot_break_an_assertion`.
- M9b (postcondition message turned back into the format string) is killed by the same test.

No finding remains open.

---
id: SR-1631
title: "IR-241 ceiling slice gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen#295; spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-039-bounded-proof-ceilings.md, src/kani/run/, src/kani/identity.rs, src/kani/classify.rs, src/kani/terminal.rs, src/kani/generate/, src/routed/generate.rs, src/lib.rs, tests/it/"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-039
    type: references
---

# SR-1631: IR-241 ceiling slice gap analysis

## Summary

Ticket: IR-241. PR: quire-contract-codegen#295. The audit was planless and scoped to the
slice the PR claims: FR-028-AC-1 to FR-028-AC-4 and FR-028-AC-21, with FR-028-AC-12 batching
semantics preserved. The PR does not claim IR-241 complete, and this audit does not either.

The computed matrix (`quire matrix --scope . --format tsv`, quire 0.36.1) shows every slice
criterion `tagged`. FR-028-AC-12 keeps its unit-level `tc_043_*` tags; its real-Kani batch test
is in the ignored `make kani` lane. A focused semantic review, which the dispatch explicitly
requested, found one evidence-recording gap and one untested, unowned error path.

## Method

Matrix verification ran `quire matrix --scope . --format tsv` and checked the rows for
FR-028-AC-1, -2, -3, -4, -12 and -21.

The reverse-gap and stub scan covered the new code paths in `memory.rs`, `launch.rs`,
`execute.rs` and `harness.rs`, plus the identity, schema and terminal changes. No `todo!`,
`unimplemented!`, `dbg!` or placeholder return was found.

For the semantic check, each slice criterion's test was traced to the source it exercises, and
each assertion was asked what source change would make it fail. Leader-supplied mutation
receipts were read as supporting evidence only. The initial `classification` mutant survived
and is not counted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-028-AC-4 requires every execution evidence to record "the bounds of each symbolic argument". A state-frame field with no declared IR range is recorded as `SymbolicBounds::Unspecified`, so the evidence carries no bound although the harness draws the field over its whole type. A Boolean state field is recorded as `Unspecified`, not `Boolean`. | src/kani/run/harness.rs:118-137, src/kani/generate/frame.rs:869-889 |
| FND-002 | low | Reverse gap: the mid-run observation failure path (`WaitConclusion::MemoryUnobserved` to `LaunchOutcome::MemoryUnobserved` to `KaniExecutionRefusal::MemoryObservationFailed`) has no owning criterion. FR-028-AC-21 states only refusal before the backend starts. No test drives this path, so a regression that maps it to `Completed` or `TimedOut` passes the suite. | src/kani/run/launch.rs:337-343, src/kani/run/launch.rs:251-252, src/kani/run/execute.rs:674-675 |

### Failure scenarios

- FND-001: a state-frame request over a field whose object member declares no range
  (`state_domains` filters it out). The harness draws `pre.field: kani::any()` over all of
  `i64`, while its evidence reads `{"domain": "unspecified"}`. A consumer comparing evidence
  bounds cannot tell full-range from unknown. Fix: record the type's full range (or `Boolean`)
  from the state field's Rust type, or narrow FR-028-AC-4 to say what `unspecified` means.
- FND-002: mutate `launch.rs:339` to return `WaitConclusion::Completed` on an observation
  error. A run whose procfs read fails mid-run then classifies its report as a verdict with no
  memory enforcement, and nothing in the suite fails. Fix: add a criterion clause (or extend
  FR-028-AC-21) and a test that injects an observation failure after spawn, for example a
  scratch procfs root whose launcher `stat` becomes malformed.

## Verdict

**CONDITIONAL.** Every slice criterion is tagged and backed by a test that exercises real code:

- AC-1 is backed by five identity-change tests covering the contract, scalar, state-frame,
  bundle and corpus identities.
- AC-2 is backed by the identity wall-clock test and the zero-ceiling ordering test.
- AC-3 and AC-21 are backed by the stand-in tree overage, batch refusal, refusal before spawn,
  pid reuse and released-mm tests.
- AC-4 is backed by the evidence ceilings and bounds assertion.
- AC-12 batching is preserved, now also grouped by equal identity ceilings.

The two low findings above remain open. The process-tree coverage gap for descendants that are
never observed is reported once, in SR-1630 FND-002.

## Coverage

- Slice criteria: FR-028-AC-1, -2, -3, -4, -12 and -21 are `tagged`. FR-028-AC-12's real-Kani
  test is `tagged-by-ignored-test`, in the `make kani` lane. The leader receipt for that lane
  reports 25 passed and 0 failed.
- Remaining IR-241 scope, untagged and planned: FR-028-AC-5 to -9, FR-028-AC-13 to -20 and
  FR-028-AC-22 to -24. These cover the shadow, native refinement, proof strength,
  family/proof-subject fields and tool-version evidence. They predate this PR and are out of
  the slice. They are listed here so the slice is not read as IR-241 complete.
- The limit-only mechanism stand-in of TC-039 step 19 is not implemented, and TC-039 Status
  says so.
- Semantic review: run, focused on the slice criteria, as the dispatch requested.
- Plan completion: not assessed

## Dispositions

Round 1. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | PR #295 fix round: `SymbolicBounds::Unspecified` is removed. A Boolean contract draw records `Boolean`; an unbounded `i64` contract draw and an unranged state field record the full `i64` domain. State-frame fields are all `i64` (frame.rs:74). Backed by `unranged_state_draws_record_the_full_i64_domain`. |
| FND-002 | fixed | PR #295 fix round: TC-039 Status ties continuous observation to existing FR-028-AC-21, without new normative text. `observation_failure_after_spawn_refuses_a_valid_report_and_stops_the_run` drives a real mid-run procfs failure beside a valid success report, and the mutant mapping it to `TimedOut` is killed. |

Round-1 verdict: PASS for this method. The slice criteria FR-028-AC-1, -2, -3, -4, -12 and -21,
and now FR-029-AC-3, are tagged. The remaining IR-241 scope is unchanged and still planned.
Plan completion: not assessed.

Round 3 (rebase regression; the exact head is in the private tracker marker): no regression in
this method's scope. Its examined spec and test paths carry the same patch as in round 2. The
rebased matrix keeps `main`'s FR-025-AC-9 row beside the PR's FR-028 rows, and the spec files
validate. The rebase's one compile regression is recorded as SR-1630 FND-014.

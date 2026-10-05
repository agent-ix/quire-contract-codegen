---
id: "SR-1511"
title: "CG PR 278 spec review: the FR-024, FR-029, TC-035, TC-040, matrix and AD edits that land with StateClauseReplay"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@986f23cfa0bd371c73318ff285fbed44049af46d; spec/replay/functional/FR-024-counterexample-envelope-intake.md (AC-11 to AC-19, Current state), spec/replay/matrix/TC-035-counterexample-envelope-intake.md (Status), spec/replay/matrix/tests.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-16, Notes), spec/kani/matrix/TC-040-run-outcome-terminal-record.md (Status), spec/kani/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-004-cg-crate-layout.md (git diff origin/main...HEAD, merge base e526390)"
---

# SR-1511: CG PR 278 spec review

## Summary

Ticket: IR-460. Scope is the spec diff of PR #278 only. The edits are mostly status flips
(removing `PLANNED (IR-460)`), Current-state and Status prose, the AD-002 seam and failure rows,
and the AD-004 tree and move-table rows for `replay/state_clause.rs`. One criterion's text changed
(FR-024-AC-18). The AD-004 rows match the file the PR adds. The AD-002 failure-table row now says
`Document` covers "a fact it is built from is not readable from the supplied package", which matches
the widened `DocumentError`. TC-040 step 15 and the FR-029 Notes describe the unbuilt fault reading
honestly. The status consequence is SR-1510 FND-002. `make spec` passes as part of `make ci`.

## Verdict

CONDITIONAL on this file alone. The AC-18 rewrite is also SR-1510 FND-001 (high), which makes the
PR FAIL overall. Here it is recorded for what it does to the criterion's text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-024-AC-18 rewrite adds a behavior claim no test asserts: "a debit that runs past the floor leaves the model, and QSL's admission refuses the post snapshot (`invalid-value`) instead of settling a verdict". The AC's only test uses the within-range subject, so this sentence is a criterion clause with no backing test. It also narrows the original criterion ("a subject mutated to debit"). Revert to the merged text and record the limit in Current state or in a ticket, pending SR-1510 FND-001 | spec/replay/functional/FR-024-counterexample-envelope-intake.md:219 |
| FND-002 | low | FR-024 Current state says the twin's hand-built frame invocation remains "for an operation that declares a result". This PR removed `returns` from both twin operations and set that invocation's `result` to `null` (`tests/state_frame_support/native_twin.rs:146-150`, `:325`). The sentence is false at the head, and the frame-path twin tests no longer exercise a result-bearing invocation | spec/replay/functional/FR-024-counterexample-envelope-intake.md:238-239 |

## Dispositions

Reviewed at 33d2f1205611b644ff2c41a5c1307bbcbe02159e.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a86a610 | The untested sentence is gone from AC-18, which matches `main` again. TC-035 Status keeps a factual one-line reason for the fixture choice, which is not a criterion |
| FND-002 | fixed a86a610 | FR-024 Current state now reads "its `result` member is now `null`, as the twin's operations declare no result" |

---
id: "SR-711"
title: "IR-489 gap analysis: FR-015 V2 contract and census inputs against the ticket's acceptance"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@2c5291e5b2fc8db4562505732e5088b10f2b7231; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/core/functional/interface-001-codegen-api.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

# SR-711: IR-489 gap analysis

## Summary

Ticket: IR-489. PR: agent-ix/quire-contract-codegen#218, head 2c5291e. The PR is spec only, so
this is an acceptance check of the ticket's "Do" against the diff. It also checks matrix coverage
and conflicts with open PRs. There is no code to audit. Plan completion: not assessed.

## Method

I checked each acceptance item of IR-489 against the diff:

- The contract harness input is a precondition, postcondition or invariant clause claim over a
  `CheckedPackageV2` clause node. Met: Inputs (lines 50-57), Behavior (lines 175-178) and AC-38.
- A V2 census input (AD-004 step 4d) backs AC-22 and AC-25 once `ProofDependencyGraph` retires.
  Met: Inputs (lines 58-60), AC-44, AC-45 and the TM-004 row.
- The existing ACs are verbatim. Met: the FR-015 diff has 0 deletions.
- The new ACs are marked planned. Met: AC-38 to AC-48 each say PLANNED (IR-489), and the matrix
  row is Planned.
- No requirement is removed. Met: no line is deleted in any spec file except the one rewritten
  interface-001 semantics line, which keeps its old text verbatim and appends to it.

I listed the open PRs and fetched draft PR #209's head to look for id collisions.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-38 requires an invariant harness, but no AC defines what an invariant harness asserts or what its subject is. An invariant anchors at an object type and binds `self` alone. Whether the harness checks preservation across an operation, a pre-state or a post-state is left open (the coder's open question 2), so AC-38's invariant case cannot be implemented or tested as specified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:243, spec/kani/functional/FR-015-bounded-kani-obligations.md:248 |
| FND-002 | medium | The ids collide with open draft PR #209 (feat/ir-459-frame-envelope-in-src), which adds its own FR-015-AC-38 (the frame-replay witness), at the pre-reorg path spec/functional/complete-v1/. Whichever PR lands second must renumber. Report only. | spec/kani/functional/FR-015-bounded-kani-obligations.md:243 |
| FND-003 | low | AC-41's real-Kani test depends on building the package through QSL's facade (TC-025 step 11, AD-004 4a). AD-004 allows a CG-side control only if the facade offers that, and this is unverified (the coder's open question 5). If the facade does not, neither AC-41 nor TC-025 says where the criterion is verified instead (for example the quire-integration test under L-5). | spec/kani/functional/FR-015-bounded-kani-obligations.md:246, spec/kani/matrix/TC-025-bounded-kani-obligations.md:119-124 |

## Verdict

The ticket's acceptance is met in form: input, census, verbatim ACs, planned markers and no
removal. Two semantic gaps remain:

- FND-001: the invariant harness shape is unspecified.
- SR-710 FND-001: `self` cannot be bound by `bounded_domain`.

FND-002 is a coordination item with draft PR #209, not a defect in this PR. I could not confirm
that QSL lowering emits precondition and invariant `state_clause` nodes into a `CheckedPackageV2`
the way it emits postconditions; qsl-forms builds the `state_clause` form for all three. That
stays the coder's open question 3.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: new AC-49. A V2 invariant harness draws its arguments within their domains and asserts the clause, with no subject call. Preservation under an operation is explicitly not specified. |
| FND-002 | deferred | The collision with draft PR #209 is recorded in the PR body for the leader to decide. Whichever PR lands second renumbers. This is coordination, not a defect in this PR. |
| FND-003 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: AC-41 and TC-025 step 11 name the quire-integration exemplar (AD-004 L-5) as the real-Kani control when QSL's facade cannot build the package. |

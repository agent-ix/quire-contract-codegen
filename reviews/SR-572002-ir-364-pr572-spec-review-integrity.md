---
id: SR-572002
title: "IR-364 PR #572 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #572 head bc7213ef556911eb71a19e02b83554c8dde4999f from base 1944268f; spec/strategy/functional/FR-011-numeric-harness-campaigns.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/TC-020-numeric-harness-campaigns.md, spec/strategy/matrix/TC-004-proptest-strategies.md, spec/strategy/matrix/tests.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-020
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-004
    type: reviews
---

## Summary

Non-author integrity review (completeness, consistency, atomicity, testability) of draft PR #572
at frozen head bc7213ef, scoped to its diff. Ticket: IR-364. Checks that each new AC is observable
by its TC steps and that no existing AC is narrowed or silently reworded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Binding precedence is untested: no TC-020 or TC-004 step supplies both a binding point and a different caller-context point with the request equal to the caller context; an implementation that accepts a request matching either source passes every listed step | spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:41-44; spec/strategy/matrix/TC-004-proptest-strategies.md:40-42; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:129 |
| FND-002 | medium | FR-011-AC-6 says "an invariant at an initialization or handler anchor", so a test covering one anchor satisfies it and an implementation mapping `Invariant` to that one anchor passes; AC-7's "No point is inferred from ... kind" has no observation of its own (TC-020 step 8's last sentence names no expected result) | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:128-129; spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:44 |
| FND-003 | low | FR-002-AC-12 is narrowed: byte identity was required for the same package, clauses and policy, and now also requires the same two points; the change follows from points becoming inputs but the PR body does not call it out | spec/strategy/functional/FR-002-tristate-proptest.md:103 |
| FND-004 | low | `MissingExecutionPoint` and `ExecutionPointMismatch` are named without an enclosing error type and are absent from FR-011 Outputs (which list only the campaign error type and `ConformanceMismatch`); FR-002-AC-20 does not require the refusal to name the selection, unlike AC-10 for the other selection refusals | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:55-60; spec/strategy/functional/FR-002-tristate-proptest.md:111 |

## Verdict

CONDITIONAL. Traceability is consistent: FR-011-AC-6/AC-7 trace to TC-020 steps 7-8 with matching
expected results; FR-002-AC-19/AC-20 trace to TC-004's new procedure and expected-result
paragraphs; the strategy `tests.md` coverage rows and TC traces list the new ACs, and
`quire validate` passes at the head. TC-004 uses different precondition and postcondition points
and observes each on its own verdict, so a swapped-point implementation would fail. Both TCs
exercise missing request point, missing source and mismatch on each source, for either selection.
AC-1 to AC-5 of FR-011 and AC-1 to AC-11 and AC-13 to AC-18 of FR-002 are unchanged; the one
narrowing is FND-003. The gaps are precedence coverage (FND-001), the anchor-coverage disjunction
(FND-002) and the refusal type (FND-004).

## Dispositions

Round 1, checked against the repository at head fd1de474dd98225d6817e921f4d012e3c3ff9f5f.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Precedence no longer exists: one source. |
| FND-002 | fixed | AC-6 requires initialization, handler and two distinct handler anchors; AC-7 has its own kind-fixed, anchor-varied observation, matched by TC-020 step 7 and its expected result. |
| FND-003 | fixed | FR-002-AC-12 row is byte-identical to base 1944268f (no diff on that row). |
| FND-004 | fixed | Variants sit on existing enums (src/strategy/campaign.rs StrategyErrorCode, src/strategy/harness.rs HarnessErrorCode), are in both Outputs, and FR-002 diagnostics name the selection. |

## Coverage

```yaml
scope:
- id: FR-011-AC-6
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "For a selected clause with an accepted QSL binding point, the V2 request carries that exact typed point; for a selected clause without such a binding point, it carries the caller invocation context's exact typed point. The generated runner passes the corresponding runtime execution point to `construct_verdict` for each case, including an invariant at an initialization or handler anchor."
- id: FR-011-AC-7
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "A missing request point or a missing point in both the binding and caller context yields `MissingExecutionPoint`; a request point differing from an available accepted binding point or from the caller context when no binding point exists yields `ExecutionPointMismatch`. Each case produces no artifact. No point is inferred from `Invariant`, `Precondition`, or `Postcondition` kind."
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities, both explicit typed execution points and campaign policy is byte-identical. The requirement and revision identity that the generated header, `RequirementId` and symbols carry is not an input here and waits on OQ-1."
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). A V2 harness request preserves each selected clause's exact typed execution point from its accepted QSL binding when supplied, or its caller invocation context otherwise; precondition and postcondition points may differ."
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). For either selected clause, a missing request point or missing point in both the accepted binding and caller context yields `MissingExecutionPoint`; a request point mismatched with its available source yields `ExecutionPointMismatch`; each produces no harness artifact."
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: examined
  excerpt: "Generate for accepted QSL bindings at distinct typed execution points, including an invariant bound to initialization and one bound to a handler. Instrument verdict construction and compare its runtime point with the bound request point on every case. For a selection with no binding point, repeat using the caller's typed invocation context."
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: "Generate with each selected clause's point from its accepted QSL binding and, for a selection without a binding point, from its caller's typed invocation context; include different precondition and postcondition points. Instrument verdict construction to observe the precondition point on a rejected case and the postcondition point on a failed case."
- id: tests.md
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: "| FR-011 | FR-011-AC-1 through FR-011-AC-7 | TC-020 | 🚧 V2 planned (V1 tests exist) |"
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: context_only
  excerpt: "The existing generated campaign error type, plus a `ConformanceMismatch` variant carrying the case values, the expected tag, and the observed verdict kind."
```

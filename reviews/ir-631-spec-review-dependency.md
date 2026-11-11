---
id: SR-1605
title: "IR-631 spec review (dependency): FR-032 relationship edges"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen#290; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1605: IR-631 dependency analysis

## Summary

Ticket: IR-631. Reviewer: claude-opus-5-5, run d75eb678-590e-4bb0-b33c-ca7f9c5bde50. I checked FR-032's new `relationships:` edges and TC-047's edges for missing prerequisites, cycles and the split between enablement and feature work. FR-032 is feature work that depends on FR-022 (scalar identity), FR-024 (envelope submission), FR-029 (terminal map) and the external QSL-641 enablement. There is no cycle. One prerequisite edge is missing (FR-015), and one governing architecture decision is cited in the body but not linked (AD-002).

## Verdict

**CONDITIONAL**: one medium finding and one low finding. The declared edges are correct:
- `satisfies` StR-001;
- `depends_on` FR-022, FR-029 and FR-024;
- `references` FR-016 and QSL ADR-013 (which exists on QSL's main branch);
- TC-047 `verifies` FR-032 and FR-029.

The external QSL-641 enablement is correctly stated as a gate, not as an edge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-032 declares no edge or link to FR-015, yet it binds itself to "exactly the current harness assertion". FR-015-AC-37 owns that assertion (Completed with the native value inside the result bound, Refused outside it, any other outcome fails), and FR-015-AC-16 owns the singleton literal rule; FR-032 restates both. A change to FR-015 therefore cannot be traced to FR-032. Add `depends_on` FR-015. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:1, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:107, spec/kani/functional/FR-015-bounded-kani-obligations.md:570 |
| FND-002 | low | AD-002 is cited in the body as the boundary that "remains" and "retains QSL evaluation ownership", but it is absent from `relationships:`. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:43, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:202 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #290. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | `depends_on` FR-015 is added, and the body cites FR-015-AC-37 and AC-16 as the generation authorities. |
| FND-002 | fixed | `references` AD-002 is added to `relationships:`. |

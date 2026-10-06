---
id: SR-1663
title: "IR-639 spec review (dependency): FR-034 and TC-049 relationship edges and ordering"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen#299; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1663: IR-639 spec review, dependency

## Summary

Ticket: IR-639. PR: quire-contract-codegen#299 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 44b8eed1-c733-42e1-b8f3-7818119a17d5. I resolved every `relationships` edge FR-034 and TC-049 add and built the prerequisite graph they imply with FR-017 and FR-028. Two medium findings: one ordering obligation that FR-028 does not state, and one edge whose type and composition rule are under-specified.

## Method

The edges added are:
- FR-034: `satisfies` StR-001, `depends_on` FR-028 and `references` FR-017.
- TC-049: `verifies` FR-034, FR-028 and FR-017.

All four targets exist in this repository at `main` (`spec/core/stakeholder/StR-001-traceable-generation.md` and the three Kani FRs). I compared each edge with the obligations FR-034's prose and criteria actually place on the target. I also checked whether any target needs a reverse edge or text that this PR does not add. The IR-241 / PR #295 mechanism gate is already recorded as SR-1661 FND-001 and is not repeated here.

Classification:

| Requirement | Class | Rationale |
| --- | --- | --- |
| FR-017 | Enablement | The launcher, capture and batching that every Kani run uses; FR-017-AC-14 and FR-017-AC-21 to AC-25 are covered |
| FR-028 (AC-21) | Enablement | The every-run tree ownership and memory mechanism (planned, IR-241) |
| FR-034 | Enablement | The original-caller lifecycle ownership of the AC-21 mechanism; it has no user-visible verdict of its own |
| FR-028 (AC-24) | Feature | The bounded native refinement run and its strength (planned, IR-241) |

Graph (A → B: A is prerequisite to B): FR-017 → FR-028-AC-21 → FR-034 → FR-028-AC-24 (per FR-034-AC-19). Suggested order: FR-017 (in place), then the FR-028-AC-21 mechanism, then FR-034, then the FR-028-AC-24 refinement entry. At criterion granularity this is acyclic. At requirement granularity, FR-034 `depends_on` FR-028 while part of FR-028 depends on FR-034. That is FND-001.

## Verdict

**PASS with two medium findings.** The following were clean:
- Every edge target resolves.
- `satisfies` StR-001 matches the Kani FRs' existing parent.
- `depends_on` FR-028 is the right direction for the AC-21 mechanism.
- TC-049's `verifies` FR-034 is correct.
- The Dependencies prose ordering ("The ceiling namespace implementation must land before the guardian code integrates with it") matches the graph.

No cycle exists at criterion granularity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-19 places an obligation on a requirement this PR does not touch. It reads "Native refinement uses this ownership path when its FR-028 AC-24 typed entry is implemented", which makes FR-034 a prerequisite of FR-028-AC-24. At `main`, FR-028-AC-24 says only that the refinement run "reuses FR-017's launcher and FR-028's ceilings", and FR-028 has no edge or text naming FR-034. An implementer of the refinement entry reading FR-028 alone would launch the native refinement process without caller-death ownership. At requirement level the edges also form a cycle: FR-034 `depends_on` FR-028, while FR-028 (AC-24) needs FR-034. Add the reverse obligation to FR-028-AC-24 (it routes through FR-034's ownership path), with the ordering stated at criterion granularity so the graph stays acyclic. Alternatively, drop the forward claim from FR-034-AC-19 and track it on the IR-241 refinement work. | FR-034-AC-19, FR-028-AC-24, FR-028 |
| FND-002 | medium | The FR-017 edge is under-typed and the batch composition rule is missing. FR-034 cannot be built without FR-017's launcher recipe and capture. AC-17 and AC-18 preserve them, and the guardian must carry that recipe into the namespace, so FR-017 is a prerequisite: `depends_on`, not `references`. FR-034 also says it does not replace "FR-017's batching and capture contract" but defines a "private per-run" endpoint and one guardian, monitor and namespace per run. It never says whether an FR-017 batch launcher, which FR-028-AC-21 holds "as one group", gets one guardian for the whole batch or one per harness. Each choice changes cancellation granularity and resource ownership. Change the edge to `depends_on`, and state the ownership unit for a batch. | FR-034, FR-017, FR-028-AC-21 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #299, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | FR-028 gains a `references` edge to FR-034 and an additive Behavior paragraph: "The typed native refinement entry shall use FR-034's guardian ownership path for each run". It leaves every FR-028 acceptance criterion row unchanged. FR-034's edge to FR-028 is now `references`, with the criterion-level order stated in both files (FR-017 launcher, AC-21 containment slice (PR #295), FR-034, AC-24). No `depends_on` cycle remains at requirement level. |
| FND-002 | fixed | FR-034 now `depends_on` FR-017. Behavior states "One guardian, monitor, namespace, lease and capture set own the entire FR-017 batch launcher, not each harness member" and "shall use one ownership set for the entire batch launcher". TC-049 step 7 exercises whole-batch ownership under the existing ceiling and multiplied deadline. |

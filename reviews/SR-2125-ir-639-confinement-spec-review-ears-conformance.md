---
id: "SR-2125"
title: "IR-639 spec-review/ears-conformance review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2125: IR-639 spec-review/ears-conformance review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 1 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. EARS lens: each new 'shall' sentence in lines 522-574 checked for EARS form, actor and atomicity, and against FR-034's normative statement list (lines 163-238), which earlier amendments extended for their own sections.

## Verdict

**FAIL**: one low finding. The new obligations are prose-only and use an undefined actor.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new obligations (establish isolation before Dispatch, refuse S_IFSOCK stdio, prevent writer export, retain trusted channel lifetimes) appear only as compound prose sentences. None is added to FR-034's atomic EARS statement list (lines 163-238), which earlier amendments, such as the fixture section, did extend. Most use 'the executor', which is not a defined role (the defined actors are the bounded executor C, L, O, M and the guardian I), even though stdin is C's descriptor and the guardian restores it at fd 0 (line 193). Add one atomic EARS bullet per obligation, each naming its defined actor. | spec/kani/functional/FR-034-caller-death-ownership.md:556-559 |

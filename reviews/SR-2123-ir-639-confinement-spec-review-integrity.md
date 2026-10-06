---
id: "SR-2123"
title: "IR-639 spec-review/integrity review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2123: IR-639 spec-review/integrity review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 1 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Integrity lens: consistency of the new section with the unchanged normative recipe (L namespaces line 61, setup capability list line 90, exact M argv lines 108-109), the EARS list, the AC table, and TC-049 Coverage/Procedure/Expected Results.

## Verdict

**FAIL**: one high finding. The new mandatory profile is not allocated to any role and contradicts the still-normative exact recipe.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new IPC profile is mandatory, but no role is assigned to establish it, and it contradicts recipe text that is still normative. Lines 108-109 still require M to run bwrap with exactly '--unshare-user --unshare-pid --as-pid-1 --new-session --bind / / --dev-bind /dev /dev --proc /proc ...': no network namespace, and the whole host root bound. Line 61 has L create only user, PID and mount namespaces, and line 90 lists only NEWUSER/NEWPID/NEWNS as setup capabilities. A host-root bind exposes every host pathname socket (for example /run/user/<uid>/bus and /tmp/.X11-unix) unless some layer curates the view, and no layer is named. Line 540 concedes that the recipe does not establish the profile. But the change neither amends the argv and capability lists nor says which role (L, O or M) creates the network namespace and curates the mount view. It also does not say which capability check produces AC-35's refusal. Allocate both to a named role and update the argv and capability text in the same change. | spec/kani/functional/FR-034-caller-death-ownership.md:535-540 |

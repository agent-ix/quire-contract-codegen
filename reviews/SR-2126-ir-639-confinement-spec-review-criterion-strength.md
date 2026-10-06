---
id: "SR-2126"
title: "IR-639 spec-review/criterion-strength review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2126: IR-639 spec-review/criterion-strength review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 1 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Criterion-strength lens, MANUAL FALLBACK: Jev (typesafe.ai System One) is not available in this environment, and no auth or setup flow was attempted. Each judgment below is a manual, non-calibrated reviewer judgment, not a Jev verdict. Asked of AC-35, AC-36 and AC-37 whether a wrong implementation can still pass. AC-36's exact refusal variant and absence of evidence/outcome/terminal value are strong. AC-37's leaked-control mutant is strong. The one weakness is in AC-35.

## Verdict

**FAIL** (manual, non-calibrated): one low weakness in AC-35. AC-36 and AC-37 can fail on a wrong implementation, apart from the testability issue in SR-2121 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-35 requires connects to 'positively live host pathname/abstract listeners' to fail, but does not require the pathname listener to sit at a path present in the backend's mount view. A listener in a host directory that is not bound into the view fails to connect even with no mount curation at all, so the AC text alone can pass vacuously. TC-049 step 22 adds 'cwd/shared binds', but the criterion is the authority. Require at least one listener at a path that exists in the backend's view (cwd, crate or target directory, or a host-root-bound path). | spec/kani/functional/FR-034-caller-death-ownership.md:672 |

## Dispositions

Round 1, reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 4ade9999-807a-4f26-9625-9b01e9be076f, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: AC-35 now requires the pathname listener at a backend-visible shared prefix and an unconfined positive control of the same real backend connecting there, plus a genuine omission mutant, so an unbound listener can no longer pass vacuously (manual, non-calibrated judgment; Jev unavailable). |

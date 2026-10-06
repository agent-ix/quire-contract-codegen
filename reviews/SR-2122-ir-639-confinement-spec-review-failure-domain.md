---
id: "SR-2122"
title: "IR-639 spec-review/failure-domain review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2122: IR-639 spec-review/failure-domain review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 2 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Failure-domain lens: enumerated, for each new IPC route, which actor can reach a trusted endpoint or export a writer: inherited stdio, host pathname and abstract AF_UNIX, host and contained proc/pidfd access, ptrace/pidfd_getfd within the inner PID namespace, and shared bind paths. Also checked availability side effects of each isolation primitive.

## Verdict

**FAIL**: two medium gaps. The trusted guardian shares the backend's own PID namespace, and a private network namespace silently removes Cargo network access.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The private proc/PID clause (lines 538-540) only excludes access to host process descriptors. But trusted I is PID 1 of the backend's own PID namespace and runs under the same mapped UID in the same user namespace. Unless I is non-dumpable, the backend can open I's pipe and memfd descriptors through /proc/1/fd (PTRACE_MODE_READ, which Yama ptrace_scope=1 does not restrict). On hosts with ptrace_scope 0 it can also take I's lease socket with pidfd_getfd or ptrace. AC-37 requires these endpoints to be unavailable to arbitrary backend code but allocates no mechanism, for example I setting PR_SET_DUMPABLE 0 before spawning the backend. As written, the outcome depends on host ptrace policy, and step 24 may pass on one host and fail on another. Allocate the in-namespace exclusion explicitly. | spec/kani/functional/FR-034-caller-death-ownership.md:674 |
| FND-002 | medium | A private network namespace takes all network access away from the backend, including Cargo's registry and git fetches. The stated availability limit (lines 563-564) covers only socket-backed input and an unavailable isolation capability. A crate whose dependencies are not already in the local Cargo cache builds today; inside the namespace its build fails, and per FR-017 a failed build is an inconclusive NoVerdict run, not a typed refusal. Either state the network loss as an availability limit along with its classification, or require dependencies to be resolved outside the contained tree before Dispatch. | spec/kani/functional/FR-034-caller-death-ownership.md:537-538 |

## Dispositions

Round 1, reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 4ade9999-807a-4f26-9625-9b01e9be076f, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: I now sets and confirms PR_SET_DUMPABLE 0 after final credential transitions and removes backend CAP_SYS_PTRACE in its owning user namespace with no regain; /proc/1/fd, pidfd_getfd and ptrace protection is required independently of ambient Yama, and TC-049 step 24 adds an omission mutant. |
| FND-002 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: Network loss is now an explicit caller availability limit; missing registry/git inputs after admitted Dispatch keep FR-017 Inconclusive NoVerdict for single and batch runs, with no prefetch or weaker mode. |

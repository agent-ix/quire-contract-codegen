---
id: "SR-2121"
title: "IR-639 spec-review/base review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2121: IR-639 spec-review/base review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 2 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Base checklist: ID format and contiguity (AC-35..37 follow AC-34; steps 22-24 follow step 21), link integrity (man7 links, FR/TC cross-refs), AC-to-TC coverage (Coverage, Procedure and Expected Results rows exist for each new AC), hash/digest/pin antipattern (none introduced), and old-criteria preservation (additions only).

## Verdict

**FAIL**: one medium untestable clause and one low ambiguity. IDs, coverage rows and preservation of AC-1..34 pass, and no hash, digest or pin is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-36 requires 'each stdin/stdout/stderr socket case' to refuse, and TC-049 step 23 presents 'a real socket separately at each backend stdio admission position'. But FR-034 states that normal backend stdout/stderr remain capture pipes (line 558) and the original bounded captures (line 284), and the executor creates them itself (launch.rs:245-247, Stdio::piped). No ordinary caller input can place a socket at backend fd 1 or fd 2, and steps 22-24 forbid a new fixture hook. So the stdout and stderr cases cannot be produced, and the AC cannot be fully tagged. Restrict the Test clause to stdin, or state that the fd 1/fd 2 check is an internal assertion verified by Analysis. | spec/kani/functional/FR-034-caller-death-ownership.md:673 |
| FND-002 | low | On a closed stdin, fstat(0) fails with EBADF, which is also what a failed type inspection looks like. The spec requires closed stdin to be admitted unchanged and a failed inspection to refuse, but does not say how the executor tells them apart (the caller's explicit closed-stdin request representation, or a specific errno). Two implementers could choose differently: one refuses every closed-stdin run, the other admits every EBADF. Define the discriminator. | spec/kani/functional/FR-034-caller-death-ownership.md:560-561 |

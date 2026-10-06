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

## New findings (disposition pass 1)

Reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The fix for FND-002 introduces OriginalStdin::{Open, Closed}, but no artifact defines the type or its source. Line 262 says Closed is 'explicitly supplied', while lines 625-626 say it is 'captured before controls exist'. The current public KaniExecutionRequest (src/kani/run/execute.rs:52-61) has no stdin field, so a caller cannot supply it today, and no API amendment is planned for it the way one is for KaniStartupAdmissionCause. If instead C captures it by probing fd 0 at entry, that probe is itself an errno observation (EBADF), which line 628 ('errno alone shall not manufacture Closed admission') appears to forbid. State whether the tag is a planned public request field or C's entry-time capture, and how the capture is made. | spec/kani/functional/FR-034-caller-death-ownership.md:260-262 |

## Dispositions

Round 1, reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 4ade9999-807a-4f26-9625-9b01e9be076f, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: AC-36 and TC-049 step 23 no longer require caller stdout/stderr socket cases; fd1/fd2 are verified as executor-created capture pipes by mapping/inventory Analysis, and AC-36's verification method is now Test, Analysis. |
| FND-002 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: Closed stdin is an explicit OriginalStdin::Closed tag captured before controls exist, and an inspection error on OriginalStdin::Open (EBADF included) refuses; the two cases no longer share one observable. The definition of the tag itself is raised separately as FND-003. |

Round 2, reviewed at 004c864626720341f3e06994f3d17494bbd6aa44 (prior 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run f90d3cf1-e644-4114-8208-665e32c34e50, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 004c864626720341f3e06994f3d17494bbd6aa44: OriginalStdin is now a planned internal bootstrap value captured by C, explicitly not a public KaniExecutionRequest field. Line 262 now reads 'When C captures OriginalStdin::Closed at entry'. Initial authoritative absence (EBADF) or an original CLOEXEC fd0 at C's capture boundary is Closed. A later inspection error on a captured Open pin refuses. A failed or unstable capture refuses. The two observations no longer conflict, and safe capture is stated as an UNRUN CODE gate, not current support. |

Round 5, reviewed at 0696d1bc567f1d5717b9a54cdda828664211f9d5 (prior 412f056e814882eb94b9e9e65977d6993326f3aa, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 7533286d-299c-4811-af95-031c491e2c1e, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 0696d1bc567f1d5717b9a54cdda828664211f9d5: Public rustdoc for execute_kani_obligation, execute_kani_obligations (src/kani/run/execute.rs:321 and 483) and any other public bounded entry must now document the fd0..2 stability precondition, its setup window, the caller-contract-breach consequence and the observed-only refusal limit. AC-36, AC-38 and TC-049 steps 23/25 inspect it. |
| FND-005 | fixed | 0696d1bc567f1d5717b9a54cdda828664211f9d5: lstat of /proc/self/fd/N is now only a presence/magic-link probe and is never compared with the pin's fstat. Followed stat and the pin's fstat must agree on S_IFMT type, st_dev and st_ino. Original F_GETFL/F_GETFD are compared separately, excluding the pin's own CLOEXEC. TC-049 step 23 adds an Open positive control that must admit despite differing link metadata. |

## New findings (disposition pass 4)

Scoped round 4 on the caller-stdio-stability delta only; reviewed at 412f056e814882eb94b9e9e65977d6993326f3aa (prior 3368df1392c87f255772af1c1f7511fa3240a506, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run f4448661-73ba-448f-98c0-0a990c619cce, model claude-opus-5-5. All earlier findings keep their latest outcome (fixed).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new trusted-caller precondition (no concurrent close, rebind or replacement of the caller's fd0/fd1/fd2 during setup) binds library consumers, but nothing requires telling them about it. FR-034 imposes documentation obligations for helper delivery (line 221) and feature exclusion (lines 248, 544-545), but none for this contract. The public rustdoc of execute_kani_obligation (src/kani/run/execute.rs:313-320) lists only host prerequisites. A multi-threaded embedder, such as one with a thread that reopens or closes stdin, cannot know it is breaching a contract whose breach the spec moves outside the guaranteed fault domain. Require the public API documentation to state the precondition and its setup window. | spec/kani/functional/FR-034-caller-death-ownership.md:643-647 |
| FND-005 | low | 'Inconsistent lstat/fstat metadata' names neither the lstat target nor the fields compared. The authenticated self-proc observation (line 666) presumably calls lstat on /proc/self/fd/N. That returns the magic link itself: S_IFLNK, with a proc inode and mode bits that reflect the descriptor's access mode. It never equals fstat's file type or inode. A literal lstat-versus-fstat comparison would therefore refuse every Open capture, while another implementer might compare only access mode or presence. TC-049 step 23 repeats the phrase. State the observed object and which fields must agree (for example presence, and access mode against F_GETFL), or use stat (following the link) for identity. | spec/kani/functional/FR-034-caller-death-ownership.md:647-649 |

## New findings (disposition pass 5)

Scoped round 5 on the stdio-capture fix and the new settlement/kernel-fault/API delta only; reviewed at 0696d1bc567f1d5717b9a54cdda828664211f9d5 (prior 412f056e814882eb94b9e9e65977d6993326f3aa, base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 7533286d-299c-4811-af95-031c491e2c1e, model claude-opus-5-5.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The fix for FND-005 also compares original F_GETFL status flags across capture observations and refuses on any change. File status flags such as O_NONBLOCK and O_APPEND live on the shared open file description, not the caller's descriptor table. Any other process sharing that description can change them during setup: a parent shell, a sibling in a pipeline, or another program on the same terminal. Such a change is not the embedding caller or its threads, so it is not the caller contract breach the precondition describes, yet it triggers refusal. Only the F_GETFD exec flag and the F_GETFL access mode are within the caller's own control. Either limit the comparison to access mode and FD_CLOEXEC, or state that external status-flag changes on a shared description also refuse, as an availability limit. | spec/kani/functional/FR-034-caller-death-ownership.md:664-667 |

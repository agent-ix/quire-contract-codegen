---
id: "SR-2124"
title: "IR-639 spec-review/scope-boundary review of report threat boundary and backend IPC confinement"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-codegen@b0cb2737eaea22aa1f58970931d92d4f55e3868e; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (unopened PR; diff against base cd9fdaa46fec2ae8f7cdc8c9bc4e1c32bce81770)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2124: IR-639 spec-review/scope-boundary review of report threat boundary and backend IPC confinement

## Summary

Ticket: IR-639. PR: quire-contract-codegen#unopened at b0cb2737eaea22aa1f58970931d92d4f55e3868e. Reviewer model claude-opus-5-5, run dbb8a12e-b532-45a6-a5bd-451efbb27322.
Scope units examined: FR-034 (new section lines 522-574 and Dependencies lines 734-741),
FR-034-AC-35, FR-034-AC-36, FR-034-AC-37, TC-049 (Coverage rows, steps 22-24, Expected Results
rows). Context only: FR-034-AC-4, FR-034 recipe lines 61, 90, 108-109, 157, 280-284. 2 finding(s).

## Method

Read the full base..head diff (+110/-0 lines; git diff --numstat shows 66 and 44 insertions and zero deletions, so FR-034-AC-1..34 and TC-049 steps 1..21 are byte-unchanged at head). Grounded the cited CG facts in source at the frozen head: src/kani/run/execute.rs start (BoundedLaunchError::Unavailable -> KaniExecutionRefusal::MemoryMechanismUnavailable, lines 348-371), KaniExecutionRefusal::code (lines 138-151, None for that variant), src/kani/terminal.rs run_terminal_value(&KaniRunOutcome, ..) (line 82), src/kani/run/launch.rs run-stage error classification (lines 224-229) and stdio setup (lines 245-247), src/kani/run/namespace.rs bwrap recipe (lines 129-142). No test, build, Kani or gate was run; all new criteria and TC-049 steps 22-24 are PLANNED/UNRUN and no tag for FR-034-AC-35..37 exists in src/ or tests/. quire validate on the two changed files exited 0 with ambient warnings recorded in the review manifest. Scope-boundary lens: the new fault-domain exclusion versus every guarantee that the same host-side capability could defeat (exclusive lease, AC-4 same-UID foreign actor, AC-37 trusted channels), and the in/out boundary for shared writable paths.

## Verdict

**FAIL**: two medium boundary inconsistencies.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new exclusion covers only a host peer obtaining report authority through host-side descriptor access. The same capability (pidfd_getfd or ptrace by a same-UID host peer, under permissive dumpability and Yama settings) also defeats 'Only C ever holds the original guardian-lease writer' (line 157), AC-4's foreign-actor guarantee, which TC-049 step 9 exercises with same-UID processes, and AC-37's trusted channels. As written, those guarantees remain asserted against a peer that the new section admits is outside what the executor can stop. State one fault domain for every authority (report, lease, bootstrap, final control), or explain why lease authority is held against host-side descriptor access when report authority is not. | spec/kani/functional/FR-034-caller-death-ownership.md:524-529 |
| FND-002 | medium | The backend must write the host-shared crate and Cargo target directories (FR-017 request inputs), and a network namespace does not isolate AF_UNIX pathname connects. So a same-UID host peer can bind a listening socket inside those directories at any time, including after the pre-Dispatch check. The spec requires that no host pathname socket be connectable 'through shared bind paths', yet places the cooperating host peer outside the fault domain. It does not say which applies to a socket a peer creates in a required shared path. If that is an export the executor must prevent, a one-time pre-Dispatch check cannot do it, and blanket sendmsg denial is excluded. If it is an excluded host-peer action, AC-35 overstates the guarantee. TC-049 step 22 tests only listeners that exist before the attempt. State the rule for required writable shared paths and for listeners created after Dispatch. | spec/kani/functional/FR-034-caller-death-ownership.md:672 |

## Dispositions

Round 1, reviewed at 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee (base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60); session dbb8a12e-b532-45a6-a5bd-451efbb27322, run 4ade9999-807a-4f26-9625-9b01e9be076f, model claude-opus-5-5.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: One fault domain now covers report, original lease, bootstrap, ownership and final-control authority; ordinary foreign actors without stolen authority stay under the unchanged AC-4 rejection criteria. |
| FND-002 | fixed | 622ecfb2cd8ea9fb9489e4839686f2f258f2bfee: Exclusion of dynamic listeners in shared paths is now continuous: backend-only seccomp denies socket(AF_UNIX) and datagram socketpairs (with socketcall/ABI/io_uring closure) for the whole run, covering listeners created after Dispatch in admitted shared source/target/cwd paths; TC-049 step 22 adds a post-Dispatch listener case. |

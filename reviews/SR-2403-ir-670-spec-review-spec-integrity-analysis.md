---
id: "SR-2403"
title: "spec-integrity-analysis of IR-670 FR-034 AC-39 and TC-049 step 26"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2403: spec-integrity-analysis of IR-670 FR-034 AC-39 and TC-049 step 26

## Summary

Ticket: IR-670. Consistency and completeness of the FR-034 and TC-049 additions against the unchanged text they touch: the TC-049 slice-allocation and Expected Results tables, the steps 22-25 summary, the FR-034 privilege/owner-protection clauses, the KaniStartupCapability allocation and the hand-maintained matrix index. Five defects, three medium.

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new FR-034-AC-39 row is in the evidence-slice table (columns 'Slice 1: ordinary-seam Test evidence' / 'Slice 2: IR-655 owed Test evidence'), but its third cell lists regressions caught, the Expected Results format; the Expected Results table (L709 onward, rows up to AC-38) has no AC-39 row, and AC-39's slice 2 / IR-655 allocation is never stated. | spec/kani/matrix/TC-049-caller-death-ownership.md:105; spec/kani/matrix/TC-049-caller-death-ownership.md:81; spec/kani/matrix/TC-049-caller-death-ownership.md:709; spec/kani/matrix/TC-049-caller-death-ownership.md:728 |
| FND-002 | medium | AC-39 and FR-034:637-638 send every policy installation failure through BackendIpcExclusion, but FR-034:578-579 installs 'seccomp IPC and privilege restrictions' at one boundary, and FR-034:670-675 makes backend CAP_SYS_PTRACE removal an owner-protection duty whose failure refuses admission, with TrustedOwnerProtection allocated at L768; the spec does not say which capability a privilege-restriction failure inside the installer names. | spec/kani/functional/FR-034-caller-death-ownership.md:1073; spec/kani/functional/FR-034-caller-death-ownership.md:637; spec/kani/functional/FR-034-caller-death-ownership.md:578; spec/kani/functional/FR-034-caller-death-ownership.md:674; spec/kani/functional/FR-034-caller-death-ownership.md:768 |
| FND-003 | medium | FR-034:643 requires only that an unsupported architecture or x32 alias 'not reach an allowed native-syscall action', which an errno return (EPERM/ENOSYS) satisfies, while TC-049 step 26 requires 'actual policy termination' as the oracle and FR-034:660 presumes a 'syscall-policy ABI termination'; the action the policy must take for an ABI mismatch is not fixed. | spec/kani/functional/FR-034-caller-death-ownership.md:643; spec/kani/functional/FR-034-caller-death-ownership.md:660; spec/kani/matrix/TC-049-caller-death-ownership.md:618 |
| FND-004 | low | The hand-maintained Kani matrix index still lists FR-034-AC-1 through FR-034-AC-30 for TC-049 (rows L54 and L85); the diff adds FR-034-AC-39 without registering it there. AC-31 to AC-38 were already missing before this diff. | spec/kani/matrix/tests.md:54; spec/kani/matrix/tests.md:85 |
| FND-005 | low | Step 26 is appended after the closing 'Steps 22-25 allocate ...' paragraph rather than before it, and that paragraph is not extended, so step 26 does not get its rules (no fixture DTO, rights, hook or coordination cap; missing ordinary-seam evidence recorded for the IR-655 SPEC-before-fixture-CODE process). | spec/kani/matrix/TC-049-caller-death-ownership.md:599; spec/kani/matrix/TC-049-caller-death-ownership.md:604 |

## Finding detail

### FND-001

Confidence: high. Check: coverage. Unit: TC-049 (spec/kani/matrix/TC-049-caller-death-ownership.md:105).

Failure scenario: A tester reading Expected Results finds no required observation or regression list for AC-39, and a planner reading the slice table takes the regression list as IR-655 fixture evidence owed by slice 2, which TC-049 nowhere allocates.

### FND-002

Confidence: medium. Check: ambiguous. Unit: FR-034-AC-39 (spec/kani/functional/FR-034-caller-death-ownership.md:1073).

Failure scenario: The installer fails while removing CAP_SYS_PTRACE from the bounding set before applying the seccomp filter. One implementer reports BackendIpcExclusion, as AC-39 literally requires; another reports TrustedOwnerProtection, as the owner-protection clause implies. Both claim conformance, and the AC-37 and AC-39 tests expect different variants for the same event.

### FND-003

Confidence: medium. Check: ambiguous. Unit: FR-034 (spec/kani/functional/FR-034-caller-death-ownership.md:641-645).

Failure scenario: A policy that returns ENOSYS to every compat syscall meets the FR statement but fails step 26's termination oracle; conversely, a test that only checks the syscall failed would accept a policy that never terminates. Implementer and tester disagree on what conformance is.

### FND-004

Confidence: high. Check: trace. Unit: spec/kani/matrix/tests.md (spec/kani/matrix/tests.md:54).

Failure scenario: A reader using spec/kani/matrix/tests.md to find what TC-049 covers does not see AC-39 (or AC-31..38); only the computed quire matrix lists it.

### FND-005

Confidence: high. Check: other. Unit: TC-049 (spec/kani/matrix/TC-049-caller-death-ownership.md:599-604).

Failure scenario: A coder implementing step 26 who finds no ordinary production seam has no stated route for recording owed evidence, and the summary paragraph tells the reader the new-test list ended at step 25.

## Scope

- `TC-049 L105` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 L105 (column 3)` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L604-609` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L616-622` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `FR-034-AC-39` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034-AC-39 (middle)` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L635-639` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L641-645` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L645-648` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L660-664` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L664-667` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `TC-049 L81` (spec/kani/matrix/TC-049-caller-death-ownership.md) — context_only
- `TC-049 L707-711` (spec/kani/matrix/TC-049-caller-death-ownership.md) — context_only
- `TC-049 L728` (spec/kani/matrix/TC-049-caller-death-ownership.md) — context_only
- `TC-049 L599-602` (spec/kani/matrix/TC-049-caller-death-ownership.md) — context_only
- `FR-034 L578-581` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L670-675` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L766-768` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L770-772` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L592` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `spec/kani/matrix/tests.md L54` (spec/kani/matrix/tests.md) — context_only

## Verdict

The new FR text is internally consistent with the FR-034 Dispatch transition table (L589-593) and the FR-017 NoVerdict mapping, and it does not weaken the IPC, descriptor, owner-protection, capture, deadline or settlement obligations. The defects are placement and mapping gaps in how AC-39 is wired into TC-049 and into the existing capability allocation.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@aef3d5715543db202d37d28c5fa4f422b99dc369 (fix diff 24b4135..aef3d57; 79a1a9a only adds the seven original SR files under reviews/, byte-identical to this file's original prefix). Reviewer session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run fea14e45-7035-49c6-9f42-ef4ddc5a1514. Every outcome was re-measured against the fix head; the author's fix map was read as data only. No build, test, Kani or full gate was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | aef3d57: The slice row (TC-049:105) now holds ordinary-seam evidence in column 2 and states 'No new IR-655 fixture facility is allocated' with owed-evidence routing in column 3; a new Expected Results row FR-034-AC-39 (TC-049:749) carries required observation and regressions caught. |
| FND-002 | fixed | aef3d57: FR-034:639-645 separates IPC/seccomp filter installation failure (BackendIpcExclusion) from privilege installation or owner-protection verification failure, including NNP/CAP_SYS_PTRACE reported via a filter library (TrustedOwnerProtection), decided by the typed operation; AC-39 and TC-049 step 26 match. |
| FND-003 | fixed | aef3d57: FR-034:668-671 now fixes the action as the seccomp process-kill before the syscall executes, matching TC-049 step 26's termination oracle and the Expected Results regression 'errno-only denial instead of process-kill'. |
| FND-004 | fixed | aef3d57: spec/kani/matrix/tests.md:54 now reads FR-034-AC-1 through FR-034-AC-39 and the TC-049 summary row (L85) lists AC-31 through AC-39; quire matrix at the fix head computes FR-034-AC-39 untagged with no binder, as a PLANNED criterion should. |
| FND-005 | fixed | aef3d57: The closing paragraph now follows step 26 (TC-049:648-652) and reads 'Steps 22-26', carrying the no-fixture-DTO/hook and IR-655 owed-evidence rules to step 26, and adds 'Analysis is never runtime Test credit'. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The fix requires the private refusal transport to bind provenance to the 'original stop stamp' (FR-034:659), and TC-049 step 26 inspects 'original stamp custody' (L612), but neither term is defined anywhere in the spec (the only other 'stamp' text is 'without timestamps' at FR-034:492), and 'stop' has no meaning for a pre-Dispatch admission refusal in this file, whose only stop concepts are the AC-38 workdeadline/FIRST stop trigger. | spec/kani/functional/FR-034-caller-death-ownership.md:659; spec/kani/matrix/TC-049-caller-death-ownership.md:612 |

Failure scenario (FND-006, confidence medium): One coder binds the refusal to a monotonic timestamp taken at refusal, another to the AC-38 FIRST stop-trigger deadline, a third to nothing because no stop occurred before Dispatch; the step 26 Analysis has no fixed thing to check, so any of the three can be called conformant.

---
id: SR-1660
title: "IR-639 spec review (failure domain): guardian placement, guardian death and unconfirmed group cancellation"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen#299; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1660: IR-639 spec review, failure domain

## Summary

Ticket: IR-639. PR: quire-contract-codegen#299 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 44b8eed1-c733-42e1-b8f3-7818119a17d5. I enumerated the ways each owner in the proposed topology (original caller, guardian, bubblewrap monitor, namespace INIT) can die or be signalled, at each lifecycle stage (pre-initialization, gated before INIT claim, claimed but not dispatched, dispatched), and checked that FR-034 assigns a live owner that keeps the startup gate closed or kills the owned tree in every case it claims. Two high findings and one medium finding.

## Method

For each stage I asked what holds the gate writer, what pins the target process group or INIT, and who can still act if one process dies. I crossed that with the common real-world caller-death modes: SIGKILL of the caller alone, a process-group kill (`timeout -s KILL`, CI job cancellation, a terminal hangup or interrupt to the foreground job), abort and OOM kill. I then compared the result with the scope statement in FR-034 Dependencies ("Simultaneous host failure or forced killing of all owners is outside this lifecycle claim. With a live caller, guardian failure is detected and refused with owned cleanup"). I also checked the overlap between the pre-Dispatch and post-claim cancellation rules, and what observation can confirm cancellation of a pinned group whose INIT has not yet been claimed. The research note at `/tmp/ix-handoff/ir241-caller-death-research.md` was read as design context only. It is not evidence for any finding below. Each finding follows from FR-034's own text: AC-2 and AC-7 themselves treat gate closure as the event that releases the backend.

## Verdict

**FAIL: two high findings and one medium finding.** The following were clean:
- The core ordering for caller death is correct: kill the pinned group before gate closure or monitor reaping (AC-2), then claimed-INIT teardown after Dispatch (AC-3).
- Identity verification refuses arbitrary PPid, reused identity and dead INIT (AC-5).
- Descendant ownership through kernel INIT teardown, not PID lists or host scans, is required (AC-8).
- Evidence is never fabricated for a dead caller (AC-3, Outputs).
- Deadline continuity holds (AC-20, AC-21).
- Mutants and emergency cleanup are ordered so cleanup cannot mask a regression (AC-24).

No delivered guardian is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-034 places the guardian only "outside the backend startup process group" and never requires it to leave the caller's process group, session or job-control signals. The guardian is spawned by the caller, so by default it shares the caller's process group. The most common way a caller dies is a group-wide signal: `timeout -s KILL`, CI job cancellation, or a terminal hangup or interrupt to the foreground job. That signal kills the caller and the guardian together, while the monitor in its own group survives. The guardian's retained gate writer then closes with no kill, which releases an unowned backend. Dependencies excludes "forced killing of all owners", so this exclusion silently covers the commonest caller-death mode, and the hole reopens. Require the guardian to run in its own process group and session, outside both the caller's and the backend's. Require it to ignore or survive terminal job-control and hangup signals. Add a TC-049 scenario that kills the caller's whole process group at each barrier. | spec/kani/functional/FR-034-caller-death-ownership.md:46-47, spec/kani/functional/FR-034-caller-death-ownership.md:102-104 |
| FND-002 | high | The guardian's own death while the caller is alive moves the original hole rather than closing it. Between monitor creation and INIT claim, the guardian alone holds the gate writer and the unreaped monitor. If the guardian is SIGKILLed or OOM-killed there, the writer closes and the gate releases the backend. The monitor is reparented away from any owner, so its numeric group is no longer pinned. The caller holds no pidfd, no pin and no gate copy. Dependencies nonetheless claims "With a live caller, guardian failure is detected and refused with owned cleanup", and AC-11 bounds only the evidence (typed refusal, never verified/falsified), not the release of an unowned backend. Specify a mechanism that keeps the gate closed or the group pinned for a live caller when the guardian dies before claim. Alternatively, narrow the Dependencies claim and add an AC that names this residual window explicitly, and say why it is accepted. | spec/kani/functional/FR-034-caller-death-ownership.md:102-105, spec/kani/functional/FR-034-caller-death-ownership.md:78 |
| FND-003 | medium | "Confirmed pinned startup-group cancellation" (AC-2, AC-7, Behavior bullet 8) has no defined observation. Before INIT is claimed, the guardian has identified only the monitor. TC-049 step 3 itself says "Do not infer cleanup from monitor exit alone", but neither FR-034 nor TC-049 says what does confirm cancellation of an unclaimed group member: kill(2) success on the pinned PGID, a waitid on the monitor, or a pidfd on INIT obtained from the info channel. Two implementers would build different confirmations, and AC-2 cannot fail deterministically without one. Also, the windows "before Dispatch" (kill the pinned group) and "after INIT is claimed" (kill the claimed INIT) overlap between claim and Dispatch. Behavior does not say which rule governs there, or whether both apply. State the confirming observation for each stage, and make the stage rules disjoint or explicitly cumulative. | spec/kani/functional/FR-034-caller-death-ownership.md:53-57, spec/kani/functional/FR-034-caller-death-ownership.md:69, spec/kani/functional/FR-034-caller-death-ownership.md:74, spec/kani/matrix/TC-049-caller-death-ownership.md:41-44 |

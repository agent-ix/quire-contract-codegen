---
id: SR-1943
title: "IR-652 lifecycle spec review (failure-domain)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-049-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

# SR-1943: IR-652 lifecycle spec review, failure-domain

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080.
Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run
5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. I looked for unstated failure modes, identity confusion and
trust-boundary gaps in the C, L, O, M, I chain and the pipe-to-memfd report path. Two medium and
one low finding.

## Method

Walked each role's death (C or its group, C's spawning thread, L, O, M, I, backend descendants)
before and after parent-death arm, using the PR_SET_PDEATHSIG and pid_namespaces man-page
semantics that FR-034 itself cites. Traced every holder of the report-pipe writer from creation
to EOF, and the memfd's crossing from O to C. Scope units examined: FR-034 Outer containment
(lines 74 to 107), the Bootstrap stage row, Run artifact and report lifetime (lines 466 to 497),
FR-034-AC-2, AC-12, AC-33 and AC-34, and FR-017 lines 101 to 116. Clean on examination:
- Pre-arm L loss reaches bounded EOF refusal with no inner child.
- Post-arm O death and L death both reach kernel outer teardown.
- Credential and mapping changes require rearm and revalidation.
- The lease, bootstrap and final-report EOF identities are separated.
- Pre-Completed reader failure cancels O and I.

## Verdict

**FAIL: two medium findings and one low finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | PR_SET_PDEATHSIG fires when the parent thread that created the child exits, not when the parent process exits. C is the multi-threaded library caller. If the thread that spawned L exits while C lives (a pool or scoped thread), L is signalled and the whole outer tree is torn down under a live C. The spec does not require the spawning thread to outlive the run, and does not classify that teardown. State the spawning-thread lifetime obligation, or a dedicated long-lived spawner, and the typed outcome when it is violated. | spec/kani/functional/FR-034-caller-death-ownership.md:74-77 |
| FND-002 | medium | The writer set for EOF leaves out non-inner holders. If O creates the pipe and the writer passes through bwrap to I and the backend, then O's own pre-spawn copy and the bwrap monitor M's inherited copy also hold the writer. bwrap does not close arbitrary inherited descriptors in its monitor. AC-33 names only "confirmed inner teardown closing ALL pipe writers, including reopened procfd and descendant copies". Require O to close its writer after spawn, and require M's exit or reap (or a delivery path M never holds) before the drain. Otherwise the drain waits on a holder the ordering does not name. | spec/kani/functional/FR-034-caller-death-ownership.md:488-490, spec/kani/functional/FR-034-caller-death-ownership.md:536 |
| FND-003 | low | The memfd crosses from O, which runs in the outer user namespace above the backend tree, to C. The spec has O apply the WRITE, GROW and SHRINK seals but never requires C to check them (F_GET_SEALS, and whether SEAL_SEAL is set) on the received descriptor before reading. A stable read then depends on O's correctness alone. Add the consumer-side seal check and a typed refusal when it fails. | spec/kani/functional/FR-034-caller-death-ownership.md:488-493, spec/kani/functional/FR-017-kani-execution-evidence.md:111-116 |

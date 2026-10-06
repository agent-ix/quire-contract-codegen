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

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Creating thread must outlive L settlement; async adapter keeps a joined dedicated spawner; violation is typed ownership failure after cleanup. |
| FND-002 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: O closes its spawn writer, C/L hold none, M termination/reap precedes the EOF drain (also FR-034-AC-33). |
| FND-003 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: O applies WRITE/GROW/SHRINK/SEAL; C checks F_GET_SEALS on the received descriptor and refuses before read. |

## Round 3 scoped delta

Round 3 scoped delta reviewed 0de3e8823f0cae6382fd8d465fd0a787d5109b35 on branch ir652-lifecycle-spec (fresh main fcf7f6a415a31a80824eafbe95b64bf977555c38; normative rebased equivalent a54cd1c1b880947487bee7c2e29382366318a03b of 6f552cd97c8a6915d999e03d49129ebc3c198195, the four normative files byte-identical across the rebase; PR not open). Scope: only the evidence-allocation and one-PR sequencing delta a54cd1c1b880947487bee7c2e29382366318a03b..0de3e8823f0cae6382fd8d465fd0a787d5109b35 in FR-034 (Dependencies-adjacent staging paragraph), TC-027 (staging paragraph) and TC-049 (Evidence delivery allocation); no criterion row, id, Trace, Rust, test or review artifact changed. Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 0f55a2f0-235f-4999-b03a-1a17c640a3df. Earlier interrupted attempt at 1f81f04 produced no verdict and no records. All prior findings keep their latest outcome; no disposition row is added.

**Round 3 verdict: one medium new finding.** Unavailable ordinary-seam predicates move to stage 2 rather than being waived, and a single M or I pin is explicitly not O-death authority.

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | IR-655 SPEC may be grounded, and its coordination cap measured, on O source from the unmerged stage-1 branch, then merged before the CODE PR. Nothing requires that schedule proof and cap to be re-established against the final CODE PR head before it merges. Stage-1 source will change through review fixes and rebases (this branch was just rebased), so a merged IR-655 could rest on a sample schedule the shipped O no longer has. Require the source grounding and cap measurement to be re-verified at the exact CODE PR head, refusing merge on any divergence. | spec/kani/matrix/TC-049-caller-death-ownership.md:54-59, spec/kani/functional/FR-034-caller-death-ownership.md:609-611 |

## Dispositions, round 4

Round 4 reviewed d031e73aed42d35a063986fb703cea0ebe264c6e (previous 0de3e8823f0cae6382fd8d465fd0a787d5109b35; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run c9608ac7-dc6f-4390-9d08-6f6afcedc331. Only the disposition-pass-3 findings were open; every original finding's latest row is already fixed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | d031e73aed42d35a063986fb703cea0ebe264c6e: TC-049 and the FR-034 IR-655 Dependencies entry now require the O/C schedule proof and cap evidence to be re-grounded at the exact final frozen CODE source; relevant changes invalidate earlier proof and force re-measurement, any amended IR-655 SPEC merges before the changed fixture code, and divergence blocks merge. |

## Round 5 scoped amendment

Round 5 scoped amendment reviewed a7389cc1af4562e05b545bdd73fef24ef39177c3 (prior 37d93da9a78a1bd2e844a0dde4d669e386f00f88; fresh main fcf7f6a415a31a80824eafbe95b64bf977555c38; PR not open). Scope: only the FR-034 delta 37d93da9a78a1bd2e844a0dde4d669e386f00f88..a7389cc1af4562e05b545bdd73fef24ef39177c3 (39 added, 3 removed lines): the corrected nested bwrap argv (`--as-pid-1`, `--new-session`), the safe I report-writer entry prerequisite, and the added source-grounding paragraph. All 34 FR-034 criterion rows are byte-identical, and no TC, Rust, test, Trace or review artifact changed. Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 1a9e76d9-fc6c-47a7-b7e5-a9dd973d87e0. Source facts are grounding only, never runtime acceptance. All prior findings keep their latest outcome; no disposition row is added.

**Round 5 verdict: one low new finding.** Reopening via /proc/self/fd/N yields a new owned File rather than adopting the inherited descriptor; the no-rebind window, one-time close and bounded cancellation on failure are stated.

## New findings (disposition pass 5)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | I must validate the reopened File's "device/inode identity ... against that mapping", but the spec never says where the expected identity comes from. Reopening /proc/self/fd/N reaches the same pipe inode as slot N, so comparing the new File with slot N (or its proc link) always matches and cannot detect a wrong or substituted descriptor at N. Require O to deliver the pipe's expected device/inode, with N, over the authenticated control channel, and I to compare against that. | spec/kani/functional/FR-034-caller-death-ownership.md:128-138 |

---
id: SR-1760
title: "IR-639 guardian-test-support spec review (failure domain)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@7742a43f62fc3feddde61a529a2bab37b4d0a67b; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md, spec/tests.md (PR #305 diff against main 28553daeb1e9cfd88bb6620125bb1df27273c68e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1760: IR-639 guardian-test-support spec review, failure domain

## Summary

Ticket: IR-639. PR: quire-contract-codegen#305. Reviewer: claude-opus-5-5, session
9072f908-e626-4176-ae33-69e582990445. I traced each way the new opt-in fixture operation can
fail or be misbuilt and checked that FR-034 and TC-049 assign a defined, realizable outcome to
each. The cases were: lost, overflowed or failed observation; failed pending-Dispatch
coordination; cleanup after a failed oracle; a helper and library built with different feature
sets in either direction; and the operation reaching a production build. Two findings: one high
and one medium.

## Method

I read the new "Opt-in guardian fixture observation" section, AC-24 and AC-27 to AC-30, and
TC-049 steps 12 to 14. For the pre-Dispatch scenario I asked which process holds the queued
Dispatch and what orders guardian receipt after the lease closes. I checked whether that ordering
is possible without a feature-dependent branch in a production stage and without a timing race.
For the identity rules I enumerated the four combinations of helper and library feature state and
checked which combinations a criterion covers. The merged containment launcher
(`src/kani/run/namespace.rs`) was read as context. Scope units examined: the new FR-034
section, FR-034-AC-24, AC-27, AC-28, AC-29 and AC-30, and TC-049 steps 12 to 14.

## Verdict

**FAIL: one high finding and one medium finding.** The following were clean:
- Overflow, lost observation and failed coordination record a typed failed/inconclusive result
  and still clean up (AC-28, step 14).
- The sealed oracle cannot be rewritten by later cleanup, so an ignored-EOF mutant cannot pass
  because escalation later kills the worker (AC-24, AC-28).
- No lease, process handle or cleanup-deferring callback is returned (AC-27).
- Urgent resource and identity cancellation stays authoritative.
- A helper built without the feature is refused against a feature-enabled fixture (AC-29).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The pre-Dispatch AC-24 scenario needs a valid Dispatch to be pending at the guardian until the lease closes, then the guardian to observe EOF before acting on it. The spec does not say which process holds it or how the ordering is enforced. It forbids any feature branch inside a production stage, yet grants the feature "bounded positive coordination for pending Dispatch" that "advances without controller permission". The guardian wakes as soon as Dispatch bytes arrive, so ordering needs either a guardian-side hold in the InitReady/Dispatch stage (a forbidden stage branch, and AC-29 implies the helper itself is feature-built) or an unordered race (forbidden as timing-based success, and a lost race leaves the mutant passing as a legitimate Dispatch). State where the coordination lives (executor or guardian), what holds the Dispatch, and how ordering is made deterministic without a stage branch. Otherwise, name the unconditional production barrier it reuses. | spec/kani/functional/FR-034-caller-death-ownership.md:189-204, spec/kani/matrix/TC-049-caller-death-ownership.md:122-128 |
| FND-002 | medium | AC-29 requires only one direction of feature mismatch to be refused: a helper built without guardian-test-support against a feature-enabled fixture. The riskier reverse case is unspecified and untested: a feature-enabled helper supplied to a feature-off production library, which would put the test-support build into production. Require refusal in both directions, or state that the helper's identity is independent of the feature and why that is safe. AC-29 also says the helper "refuses" the fixture's identity, while AC-25 assigns identity refusal to the bounded executor's handshake check. | spec/kani/functional/FR-034-caller-death-ownership.md:272, spec/kani/matrix/TC-049-caller-death-ownership.md:140-143 |

## New findings (disposition pass 1)

Reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d. Found while confirming FND-001's fix.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The publication-before-close mutant is detected only by timing. Under that mutant the continuation sees publication and sends SIGCONT before the lease closes. If the close still lands before the resumed guardian reads its stream, the guardian sees EOF, rejects Dispatch and the mutant passes. The spec requires this mutant to fail but names no raw fact that separates it deterministically. When the continuation observes publication, it should record whether the guardian's control endpoint already shows peer hangup (for example a non-blocking HUP/RDHUP poll of a pidfd_getfd duplicate taken while INIT is stopped). Publication observed without closure should be a failed predicate. | spec/kani/functional/FR-034-caller-death-ownership.md:226-238, spec/kani/matrix/TC-049-caller-death-ownership.md:143-144 |
| FND-004 | low | The pre-Dispatch scenario queues "otherwise valid Dispatch on the ordinary control stream" while INIT is stopped. The spec does not say whether the unchanged production Dispatch send emits it or the fixture writes the frame itself; the kernel probe wrote the frame directly. A fixture-written frame is a synthetic Dispatch path. If the production send waits for any guardian acknowledgement, it blocks against the stopped INIT until the cap expires, and the scenario can only record coordination failure. State that the unchanged private production Dispatch send emits it without waiting on the stopped guardian, or name the private send step used. | spec/kani/functional/FR-034-caller-death-ownership.md:224-225, spec/kani/matrix/TC-049-caller-death-ownership.md:133-134 |

## New findings (disposition pass 2)

Reviewed at d5d9d63b600b51a98b66db7b0b2396e6aa5ebb4a. Found while confirming FND-003's fix.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | A FR-034 prose line added this round is 126 characters (line 247: "snapshot after a later close. If publication is duplicated ..."), and the preceding line 242 breaks after only 71. The rest of FR-034's prose wraps at 100, and the author receipt reports "prose<=100 outside tables: PASS", which this line contradicts. Rewrap the paragraph to 100 columns. | spec/kani/functional/FR-034-caller-death-ownership.md:242-248 |

## New findings (amended scope)

Reviewed at 893ac1afb5a851c9ccc02528489af700b198c737. The same PR's spec owner amended AC-23/26/27/28 and TC-049 (exact early-stage witness allocation).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The amended early-stage witness list (BeforeMonitor, ClaimedGated, ClaimedBootstrap, InitReady) skips the FR-034 stage table's Bootstrap row: monitor spawned, INIT not yet claimed, gate retained. Caller death there is the window FR-034 was written for: the caller-owned gate closes before the parent-death chain is armed, and gate EOF is the only release path. The previous TC-049 step 1 required a "Bootstrap before claim" barrier (line 34 at 52bca0d). No step now kills the caller at that stage; step 5 covers it only with a live caller. AC-23's "gated" now means ClaimedGated, which is after claim. Add an exact Bootstrap prefix whose witness carries a cloned owned monitor pidfd (INIT typed unclaimed, no fabricated INIT pin) and asserts no backend marker and bounded guardian exit after gate EOF. Otherwise state which scenario exercises caller death in that window. | spec/kani/matrix/TC-049-caller-death-ownership.md:37-40, spec/kani/functional/FR-034-caller-death-ownership.md:138 |
| FND-007 | medium | The report socketpair is described as CLOEXEC but may be "mapped ... to the fixture's reporting stdio". A descriptor at fd 0-2 is inherited across exec, so it reaches the bwrap monitor, INIT, guardian and backend unless every spawn explicitly overrides that stdio slot. FR-034 forbids this (line 258: the report channel shall never "enter the helper/backend inheritance set"), but TC-049 never checks it: step 9 (line 122) verifies only the caller lease endpoint and bootstrap-control writer. A leaked copy also keeps the harness's receive side from seeing EOF and lets the guardian or backend write into the witness channel. Add the report channel to step 9's runtime inheritance checks for the monitor, INIT, guardian, backend and an unrelated exec. Either require a non-stdio CLOEXEC auxiliary descriptor, or require that every child's stdio slot holding it is overridden. | spec/kani/functional/FR-034-caller-death-ownership.md:250-258, spec/kani/matrix/TC-049-caller-death-ownership.md:122-124 |

## Dispositions

Round 1, reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: FR-034 now places the pending-Dispatch hold in the fixture owner: owned-pidfd SIGSTOP of the verified INIT with a positive T-state check, Dispatch queued on the ordinary stream, unchanged close_lease_and_observe, an unconditional read-only LeaseClosing publication after the actual close, then a fixture-only SIGCONT. This removes both the guardian-side stage branch and the race. |
| FND-002 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: Both mismatch directions now require refusal by the bounded executor before Dispatch, and TC-049 step 13 tests both. |

Round 2, reviewed at d5d9d63b600b51a98b66db7b0b2396e6aa5ebb4a. FND-001 and FND-002 already read fixed after round 1.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | d5d9d63b600b51a98b66db7b0b2396e6aa5ebb4a: The executor records a close-completion ordinal only after the consumed CallerLease close returns. Publication seals an immutable snapshot holding Option<close ordinal> and the publication ordinal, with no late fill. The harness requires the close ordinal to be present and below the publication ordinal, so early publication fails deterministically whatever the scheduling. The actual INIT/marker/worker predicates are still required. |
| FND-004 | fixed | d5d9d63b600b51a98b66db7b0b2396e6aa5ebb4a: Dispatch goes through the unchanged private production frame-send step: the actual frame and rights, bounded nonblocking, returning a pending-frame state with no ACK wait. Fixture-written bytes are forbidden, send failures are typed coordination failures, and the ACK wait is a separate bounded transition. |

Round 3, reviewed at 52bca0d64f2404aa8c23528a41090f2b91dc4f21.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 52bca0d64f2404aa8c23528a41090f2b91dc4f21: The close-completion/publication paragraph is rewrapped to 100 columns with identical words (whitespace-only diff from d5d9d63). The 126-character line and the short 71-character break are gone, and no prose line in the paragraph exceeds 100. |

Round 4, reviewed at c1a8764b3acd1250475e11a083dc3c0ce9f2965d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | c1a8764b3acd1250475e11a083dc3c0ce9f2965d: Bootstrap is restored as an exact prefix right after monitor spawn, before any claim or gate release. Its witness carries one owned monitor pidfd and typed InitUnclaimed, with no INIT pin fabricated. The Test asserts only stage facts, caller/monitor death, closed gate/lease ownership and no marker; it never infers INIT death from EOF. The present PR #295 monitor-PDEATH/child_wait leak is named and allocated to IR-652 (exists, Backlog, blocks IR-639). Cleanup assurance is kept as separately required Analysis/repair, and later claimed-INIT Tests are unchanged. |
| FND-007 | fixed | c1a8764b3acd1250475e11a083dc3c0ce9f2965d: The report socket now enters only through startup stdout. That is safely duplicated to a >=3 CLOEXEC OwnedFd auxiliary, and the original stdout is marked CLOEXEC before any spawn. Every child configures explicit stdio that excludes both descriptors. TC-049 step 9 checks both descriptors' absence from monitor, INIT, guardian, backend and unrelated exec by socket identity, plus report EOF. It adds separate flag-removal and stdio-exclusion mutants. No raw-fd adoption. |

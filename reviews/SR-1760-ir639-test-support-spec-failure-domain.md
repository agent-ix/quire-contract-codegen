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

## Dispositions

Round 1, reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: FR-034 now places the pending-Dispatch hold in the fixture owner: owned-pidfd SIGSTOP of the verified INIT with a positive T-state check, Dispatch queued on the ordinary stream, unchanged close_lease_and_observe, an unconditional read-only LeaseClosing publication after the actual close, then a fixture-only SIGCONT. This removes both the guardian-side stage branch and the race. |
| FND-002 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: Both mismatch directions now require refusal by the bounded executor before Dispatch, and TC-049 step 13 tests both. |

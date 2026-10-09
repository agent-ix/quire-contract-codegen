---
id: SR-3203
title: "IR-694 spec review (failure domain): queued-claim admission failures, cancellation ordering and phase scope"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen PR #327, branch spec/ir694-queued-claim-cleanup (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md lines 430-459 and AC-94; spec/kani/matrix/TC-049-caller-death-ownership.md lines 1133-1156 and the AC-94 Expected Results row; context: FR-034 stage table (ClaimedGated parent/start/namespace admission, ClaimedBootstrap), AC-77 concurrent I and M cancellation; published IR-639 guardian review-source backup ref: caller_bootstrap.rs advance_startup (InnerClaimed validates I as child of the retained monitor; GateReleased requires live I)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3203: IR-694 spec review (failure domain)

## Summary

Ticket: IR-694. This pass enumerates how C's admission of a queued claim can fail during AC-77
cancellation, and checks whether the new text and the TC decide each case. Every case resolves
fail-closed: no positive authority is granted on any path. The findings concern outcome fidelity
(original cause versus CleanupUnconfirmed or refusal) and untested branches, not unsafe release.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-77 cancels claimed I and retained M concurrently, and their order is not fixed. ClaimedGated admission verifies I's parent, so if M dies or is reaped first, the queued claim fails admission while I is still alive and before any I death or I-lease EOF. The new section names only "AC-77-caused I death or original I-lease EOF" as the condition under which provisional receipt is retained. It is therefore undecided whether an M-first failure enters close-only receipt (keeping the original cause) or is refused. The TC procedure positively establishes only I death or lease EOF, so an M-first schedule is never exercised. | spec/kani/functional/FR-034-caller-death-ownership.md:432-436; spec/kani/matrix/TC-049-caller-death-ownership.md:1138-1140 |
| FND-002 | medium | "merely because its claimed I can no longer pass live-child admission" implies that only death-related admission failure qualifies. A complete, authenticated, well-formed claim whose right names a live process with a mismatched start, namespace or parent is an identity defect, not a cancellation race. The text does not say that this case keeps the existing refusal. The TC substitutes wrong phase, sender or run, malformed or partial claims, and extra rights, but has no live-identity-mismatch control. The mutant "any admission failure enters provisional receipt" survives, reclassifying a genuine identity defect as O's original negative cause. | spec/kani/functional/FR-034-caller-death-ownership.md:434-436; spec/kani/matrix/TC-049-caller-death-ownership.md:1147-1149 |
| FND-003 | low | The exception is limited to a queued InnerClaimed, and other prior phases are "wrong-phase ... prior traffic". The spec does not bound "claimed-startup", so it does not say whether O can commit the claimed-startup Failure after a GateReleased reply that C has not yet read. In the review source, that reply also requires live I at C. If O can, the same race ends in CleanupUnconfirmed instead of the original cause. That is fail-closed, but the scope limit should be stated deliberately, or the window shown to be impossible. | spec/kani/functional/FR-034-caller-death-ownership.md:442-445 |

## Verdict

**Changes requested (two medium, one low).** Each medium can be closed with a sentence in the
section plus a TC control:

- FND-001: state that any admission failure of the queued claim caused by owned AC-77 cancellation
  of I or M qualifies, or that M-first is excluded and why. Add an M-first schedule to the TC.
- FND-002: state that a live identity mismatch keeps the existing refusal. Add that control and
  name its mutant.

Recorded clean, examined:

- Missing, partial, malformed or misbound following Failure gives CleanupUnconfirmed.
- An expired original cutoff gives CleanupUnconfirmed.
- An unsettled chain gives CleanupUnconfirmed.
- An EOF is never treated as a terminal frame.
- No second parser, splice or numeric PID reopen is allowed.
- The raw right grants no signalling or termination-observation authority.
- The bounded pending storage is one prior frame plus its declared right, under the existing named
  accounting.

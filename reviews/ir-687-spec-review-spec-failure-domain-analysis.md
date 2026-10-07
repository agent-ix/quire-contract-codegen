---
id: SR-3063
title: "IR-687 spec review (failure domain): pre-Armed negative forwarding, pidfd authority and claimed-startup ordering"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir687-prearm-negative-custody (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md lines 272-362 and AC-57..AC-65; context: published guardian review-source backup ref (the older one, whose head commit is 'Retain original outer input on pre-arm split failure'): launcher_owner.rs publish_outer_arm and confirm_arm, caller_bootstrap.rs advance_startup, advance_outer_arm and receive_startup_control, control.rs rights-count check, helper_entry.rs run_outer"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
---

## Summary

Ticket: IR-687. This pass looks for unstated failure modes, identity confusion and timing edges in the new forwarding and transaction rules, measured against the published guardian review source.

Recorded clean:

- **A one-right negative context fits the receive machinery.** control.rs computes the expected rights count from a per-receive closure over the decoded control and refuses a mismatch while retaining the received descriptors. A state-selected count of one for the L→C AwaitArm negative is expressible, and the existing exact-frame refusal and owned cleanup cover AC-59.
- **The cloned pidfd cannot reach M, I or the backend.** It travels only on the authenticated L→C socketpair, and received descriptors are CLOEXEC.
- **C gains no Child or waitid authority.** C is not O's parent, so the transfer gives it neither.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "Positive Armed" goes to two recipients in sequence: publish_outer_arm sends to L first, then to C (launcher_owner.rs:653 and :658). The spec does not define the split case where L has accepted Armed but the send to C failed or is partial. In that case L has left confirm_arm, C is in AwaitArm, possibly holding a partial Armed frame and its right, and O currently returns a helper error. The forwarding trigger is also stated in C's state, "while C awaits arm publication (AwaitArm)", which L cannot observe. Implementers will differ on whether L forwards, and with what right, once it is itself Armed. | spec/kani/functional/FR-034-caller-death-ownership.md:274-276; spec/kani/functional/FR-034-caller-death-ownership.md:281-283 |
| FND-002 | medium | In AwaitArm, C already reads O's direct channel for Armed and refuses EOF there (caller_bootstrap.rs advance_outer_arm). Once launcher_ready is set, C does not read the L channel during AwaitArm at all (caller_bootstrap.rs:730-741). When O exits after sending its negative to L, C can observe O-channel EOF before L's forwarded negative arrives. It then fails startup with a control or EOF cause, never takes the pidfd, and loses the original cause. The spec gives no precedence or ordering between O-channel EOF and the authenticated L-forwarded negative during AwaitArm, and does not require C to service the L channel in that state. | spec/kani/functional/FR-034-caller-death-ownership.md:281-291; spec/kani/functional/FR-034-caller-death-ownership.md:303-310 |
| FND-003 | low | "First independently established" is not tied to an instant for a resource candidate. The text does not say whether a complete exhausting tick is established at its sampling start, its completion or its evaluation, so the comparison with the failure's producer stop stamp is undefined when a tick straddles the failure. The rule asks for a "successful fresh complete observation" but names no ordering instant. | spec/kani/functional/FR-034-caller-death-ownership.md:321-327 |
| FND-004 | low | The pidfd authority text lists what C may do (termination observation and owned-cancellation signalling) and names some exclusions. It does not exclude pidfd_getfd, setns through the pidfd, or process_madvise. C owns O's user namespace, so these are kernel-permitted, and setns into O's namespaces would reach M and I. TC-049 step 2 inspects only "that cancellation uses only the retained O capability". The same surface already exists for C's post-Armed O pin, so this is not new escalation, but AC-60's "permits only" has no enumerated exclusion to inspect. | spec/kani/functional/FR-034-caller-death-ownership.md:293-296; spec/kani/matrix/TC-049-caller-death-ownership.md:980-981 |

## Verdict

**Changes requested.** FND-001 and FND-002 are real startup races or splits that the new route creates and the text leaves open. Fix them by doing three things:

- Key L's forwarding to L-observable state.
- Define the L-Armed/C-not-Armed split.
- Give the authenticated forwarded negative an ordering rule against O-channel EOF while C is in AwaitArm.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | After COMMIT, the text says later complete exhaustion "shall not ... stop enforcement", but it never says what enforcement is in that window. Before COMMIT, exhaustion selects OwnerStop and O's existing owned cancellation (outer_sampling.rs exhaustion handling at :2345 and :2380). After COMMIT, it is unspecified whether a late ceiling breach makes O cancel the still-live claimed I and M at once, or whether O keeps waiting for C to close the I lease within the cutoff. One implementer adds an immediate cancellation; another records the fact and waits, leaving the whole-run memory ceiling exceeded until lease close or cutoff. No Test separates the two. | spec/kani/functional/FR-034-caller-death-ownership.md:356-362 |
| FND-006 | low | The post-L-Armed rule covers a partial or failed direct O→C Armed, but not a partial O→L Armed followed by O failure (L-side partial frame then EOF). It also does not say whether "preserve actual partial bytes/rights solely for cleanup" lets C signal a pidfd received with an unauthenticated partial Armed frame, or only close it. | spec/kani/functional/FR-034-caller-death-ownership.md:289-299 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | With AC-77, the claimed I can die from O's post-COMMIT cancellation while O's committed frame is still being delivered and before C has closed the I lease. Delivery itself is safe: the cancellation touches neither O's channel to C nor its committed bytes, nor the exact-EOF check on that channel. But the text gives C no rule that claimed-I death or I-lease EOF observed in this window must not pre-empt completing the authenticated O frame within the original cutoff. An implementation that fails first on the I-death observation loses the original cause and reports a startup or guardian-death refusal instead. The result is still a refusal with CleanupUnconfirmed precedence, so there is no false verdict. | spec/kani/functional/FR-034-caller-death-ownership.md:384-389; spec/kani/functional/FR-034-caller-death-ownership.md:1791 |

## Dispositions

Round 1, re-checked at the branch's round-1 fix head (the commit after an ordinary main merge, subject 'Clarify negative startup publication and settlement contracts'; head named in the Linear marker only), against the newer published guardian review-source backup ref (the one whose head commit is 'Retain producer clock failure with borrowed outer setup custody'). Static, read-only; no build, test, Kani or replay run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': forwarding is keyed to L-observable state ('while L has not accepted a complete positive Armed'); after L accepts Armed, a zero/partial/failed direct O→C Armed is a startup-protocol failure with the original cause honestly possibly unavailable, partial bytes/rights kept for cleanup only, and a partial-Armed-as-positive mutant must fail (FR-034:280-299) |
| FND-002 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': in AwaitArm C services both sources; direct O EOF is only a provisional source-closure fact, a full valid L negative within the original cutoff keeps cause/clock/capability, and an earlier valid negative is not discarded because EOF arrived first (FR-034:300-307, AC-67, TC step 4) |
| FND-003 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': resource exhaustion is established at completion/evaluation of every named observation and checked sum; sampling start and history establish nothing (FR-034:339-341) |
| FND-004 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': FR-034:308-313 and AC-68 exclude pidfd_getfd, setns through the pidfd, process_madvise, other-actor authority and numeric-PID reopening, with an Inspection row |

Round 2, re-checked at the branch's round-2 fix head (after an ordinary main merge; subject 'Clarify negative terminal cancellation and atomic evidence obligations'; head named in the Linear marker only), against the newer published guardian review-source backup ref. Static, read-only; make spec passes; no build, test, Kani or replay run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | fix commit 'Clarify negative terminal cancellation and atomic evidence obligations': FR-034:384-389 and AC-77 require O, on the completed exhausting tick after COMMIT, to begin cancelling the live claimed I through the retained I pin and the retained M through its real Child, before waiting for C lease closure and clipped to the original cutoff, with committed bytes unchanged; TC step 3 adds a wait-for-lease mutant that must fail, and honestly records the actor/order witness as owed |
| FND-006 | fixed | fix commit 'Clarify negative terminal cancellation and atomic evidence obligations': FR-034:289-294 says a partial O→L Armed is kept only for disposal, with no Armed acceptance, splice or later negative; FR-034:302-305 says C closes a pidfd received with an unauthenticated partial Armed frame and gains no signalling or termination-observation authority from it; TC pre-Armed step 3 covers both |

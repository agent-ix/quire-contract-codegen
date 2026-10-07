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

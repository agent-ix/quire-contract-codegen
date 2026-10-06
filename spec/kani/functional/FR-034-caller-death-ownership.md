---
id: FR-034
title: "Retain backend ownership when the original caller dies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---
# FR-034: Retain backend ownership when the original caller dies

## Description

When the original codegen caller terminates during startup or execution, the bounded executor shall cancel its owned backend namespace without releasing an unowned backend through startup EOF. This lifecycle requirement closes the caller-death window in the Linux PID-namespace
mechanism selected for [FR-028](./FR-028-bounded-proof-ceilings.md) AC-21. It does not replace
that requirement's memory ceiling, wall-clock ceiling, observation semantics or native refinement
contract (AC-24), or [FR-017](./FR-017-kani-execution-evidence.md)'s batching and capture contract.

Planned (IR-639): the first-party guardian and production fixtures are not implemented. The
preceding ceiling candidate defines gated namespace ownership for in-process conclusions; its
caller-owned gate can close on SIGKILL, abort or OOM death before the parent-death chain is
armed. A passing research topology probe does not establish production coverage. This
requirement is not complete until the real packaged helper and regression mutants are verified.

## Inputs

- The existing bounded execution request, original harness identity, backend command recipe and
  recorded ceilings, including the original monotonic wall deadline.
- An explicit first-party guardian executable installation path, or discovery of the executable
  installed from this same Cargo package.
- A private per-run Unix-stream endpoint, connecting only the caller and its owned guardian,
  carrying typed bounded control messages and the original caller's liveness lease.

## Outputs

- For a live caller, the existing typed backend outcome or typed setup, observation or cleanup
  refusal, with the existing evidence semantics.
- For a dead caller, owned namespace cancellation and cleanup; no fabricated run result or
  execution evidence attributed to that caller.

## Behavior

- The bounded executor shall launch `quire-kani-guardian` as a first-party executable of this
  Cargo package, outside the backend startup process group.
- The guardian shall establish the original caller's exclusive liveness lease through the private Unix stream before creating a backend monitor.
- The guardian shall own the unreaped bubblewrap monitor child and the retained startup-gate
  writer from monitor creation through cancellation or confirmed completion cleanup.
- The caller shall verify the connecting peer against its owned unreaped guardian child and verify the actual guardian-owned monitor and namespace INIT chain, including process start identity, INIT pidfd, INIT parent, namespace identity and namespace PID 1, before sending typed Dispatch.
- The executor shall establish the owned memory observer's readiness before typed Dispatch.
- If caller-lease EOF occurs before Dispatch, then the guardian shall kill the pinned startup
  group before closing the gate writer or reaping the monitor.
- If caller-lease EOF occurs after INIT is claimed, then the guardian shall kill the claimed INIT
  and confirm its termination before completing cleanup.
- When a run is cancelled or completed, the guardian shall confirm owned INIT teardown when INIT has been claimed and settle pinned startup-group cancellation before gate closure and monitor reaping when it has not.
- If the guardian or ownership observation fails while the caller is alive, then the executor shall refuse a proof conclusion unless owned teardown is confirmed.
- The executor shall preserve the original command's argument bytes, stdin, stdout, stderr,
  environment inheritance and overrides, and working directory.
- The executor shall charge guardian connection, startup, identity verification and Dispatch to
  the original wall deadline rather than beginning another budget after setup.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-034-AC-1 | Before the guardian establishes the original caller's exclusive lease and monitor ownership, no backend monitor is created. SIGKILL, abort or OOM-equivalent forced process termination of the original caller before guardian initialization completes therefore dispatches no backend. | Test |
| FR-034-AC-2 | While INIT is gated, original-caller lease EOF triggers SIGKILL of the pinned dedicated startup group with the monitor still unreaped and gate writer retained. Group cancellation is confirmed before gate closure or monitor reaping, and no backend marker appears. | Test |
| FR-034-AC-3 | Immediately after Dispatch and throughout execution, original-caller lease EOF triggers cancellation through the claimed INIT pidfd and confirmed INIT termination, without fabricating a run outcome or evidence for the dead caller. | Test |
| FR-034-AC-4 | The private Unix-stream peer credentials identify the caller's actual unreaped guardian Child. A different peer refuses before Dispatch. | Test |
| FR-034-AC-5 | The monitor is the guardian's actual unreaped Child; INIT's pidfd/start identity, parent, namespace identity and namespace PID 1 agree with it. A stale/reused identity, arbitrary reported parent, incomplete identity or dead INIT refuses before Dispatch. | Test |
| FR-034-AC-6 | The owned tree memory observer is ready before typed Dispatch; failed observer preparation releases no backend instruction. | Test |
| FR-034-AC-7 | Malformed/closed startup information, unexpected monitor exit and startup refusal retain the gate writer and unreaped monitor until confirmed pinned startup-group cancellation. Once INIT is claimed, every completion or cancellation path confirms its teardown before releasing namespace ownership. | Test |
| FR-034-AC-8 | After Dispatch, original-caller death cancels double-fork/reparented, setsid, late-born and nested-PID-namespace descendants through kernel-owned INIT teardown, including descendants absent from previous samples. Observed-PID lists or normal-operation host scans are not cancellation authority. | Test |
| FR-034-AC-9 | Cancelling one owned namespace leaves concurrent independent runs and unrelated host children unaffected. | Test |
| FR-034-AC-10 | Normal backend completion, backend exec failure, explicit cancellation, timeout, memory excess, capture failure and observation failure each require confirmed owned teardown before any proof conclusion. Backend exit or valid success output alone never authorizes acceptance. | Test |
| FR-034-AC-11 | With a live caller, guardian failure, missing ownership or unconfirmed cleanup yields a typed refusal and never verified/falsified evidence, including beside an otherwise valid success report. | Test |
| FR-034-AC-12 | Every startup refusal, completion and cancellation removes the run's private socket/artifacts and stops its guardian and monitor after owned cleanup. A live caller reaps its guardian; after original-caller death the guardian exits after monitor/namespace cleanup without claiming reaping by the dead caller. | Test |
| FR-034-AC-13 | The executor uses this package's actual Cargo executable quire-kani-guardian through an explicit installation path or this package's installation. A missing, non-executable or unusable helper gives a typed setup refusal before Dispatch, with no copied executable, shell substitute or alternate launcher. | Test |
| FR-034-AC-14 | Setup documentation identifies the package helper and Linux/procfs children and RSS, pidfd, bubblewrap namespace/info/gate features and namespace permissions required by the existing ceiling mechanism, without requiring host-policy changes or an ad hoc global installation. | Inspection |
| FR-034-AC-15 | Typed private controls reject malformed and unknown fields and enforce finite encoded-byte, pending-message and startup-work bounds. Invalid/overlimit control or EOF cancels or refuses rather than authorizing Dispatch. | Test |
| FR-034-AC-16 | Only the original caller retains its liveness endpoint. Guardian, monitor, backend descendants and unrelated concurrent execs cannot inherit an endpoint that keeps it alive. Guardian info/gate pipes are CLOEXEC in their creator and mapped only to the intended child using a safe child-only descriptor API, without raw inherited-FD adoption or weakening the unsafe prohibition. | Test |
| FR-034-AC-17 | The same backend receives unchanged raw argument bytes, inherited stdin, inherited and overridden environment, and working directory. | Test |
| FR-034-AC-18 | Guardian diagnostics and control use separate channels from backend stdout/stderr captures and cannot contaminate reports; existing bounded capture and capture-failure behavior remain authoritative. | Test |
| FR-034-AC-19 | Original identity ceilings and existing outcome classifications remain authoritative after confirmed cleanup: memory excess and wall expiry stay distinct, ambiguous live-worker RSS refuses, and ordinary completed/refused/falsified reports retain their meanings. Native refinement uses this ownership path when its FR-028 AC-24 typed entry is implemented, with unchanged refinement class/evidence rules. | Test |
| FR-034-AC-20 | Guardian connection, startup, identity verification and Dispatch use the original monotonic identity deadline without resetting it. An already expired deadline causes no Dispatch; expiry during setup or execution settles timed out. | Test |
| FR-034-AC-21 | Guardian connection and handshake have a finite setup cap within the remaining original deadline. Cap expiry while that deadline remains live is a typed setup refusal, distinct from identity-deadline timeout. | Test |
| FR-034-AC-22 | Control/capture shutdown and cleanup observation waits have finite bounds. Unconfirmed termination refuses with a live caller rather than accepting a proof or claiming physical disappearance of an uninterruptible task. | Test |
| FR-034-AC-23 | Caller-death fixtures invoke the real guardian built from this package in the owning target directory and observe pre-initialization, gated and immediate post-Dispatch stages through positive handshakes and owned identities/pidfds, without production test bypasses, sleep-based success or wide host-scan authority. | Test |
| FR-034-AC-24 | Removing lease cancellation, removing retained gate ownership or closing/reaping before pinned cancellation fails the targeted premature-marker or surviving-owned-process assertion. Emergency fixture cleanup occurs after recording the ownership observation and cannot turn failure into a pass; restored production controls pass. | Test |

## Dependencies

- [FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 owns the every-run tree memory mechanism;
  AC-2/3 own resource outcomes, and AC-24 owns bounded native refinement. This requirement adds
  independent original-caller lifecycle ownership, not another resource model.
- [FR-017](./FR-017-kani-execution-evidence.md) owns backend reports, capture and batching.
- [TC-049](../matrix/TC-049-caller-death-ownership.md) describes planned production verification.
  The ceiling namespace implementation must land before the guardian code integrates with it.

The guarantee covers original-caller loss while the independent guardian and kernel facilities
remain operational. Simultaneous host failure or forced killing of all owners is outside this
lifecycle claim. With a live caller, guardian failure is detected and refused with owned cleanup;
this does not assert an impossible guarantee for simultaneous caller and guardian death.

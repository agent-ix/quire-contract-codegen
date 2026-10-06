---
id: FR-034
title: "Retain backend ownership when the original caller dies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
---
# FR-034: Retain backend ownership when the original caller dies

## Description

If the original caller dies, then the namespace guardian shall cancel its backend descendants.
The guardian shall prevent startup EOF from authorizing a production backend instruction.

This requirement extends the planned Linux PID-namespace containment code slice (PR #295),
which is OPEN and unmerged. [FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 on main remains
mechanism-neutral. Guardian CODE integration is gated on that slice's delivery; this SPEC can
merge independently. It does not depend on all parent IR-241 work, which IR-639 itself blocks.

Planned (IR-639): no first-party guardian or production fixtures are implemented. The unmerged
containment candidate's caller-owned gate can close on SIGKILL, abort or OOM death before its
parent-death chain is armed. Here the only program that such EOF may start is the first-party
guardian bootstrap, never the production backend. The guardian is namespace PID 1, so its own
termination tears down the namespace through the kernel. Positive backend authorization remains
separate from the bubblewrap bootstrap gate. Research probes establish design feasibility, not
production coverage. The real packaged helper and regression mutants must be verified.

## Inputs

- The existing bounded execution request, original harness identity, backend command recipe and
  recorded ceilings, including the original monotonic wall deadline.
- A required explicit executable path to `quire-kani-guardian`, built with the library from the
  same Cargo package source and build inputs. A library consumer deliberately builds and
  supplies this artifact; Cargo does not build or install a dependency's binary for it.
- A private per-run Linux abstract Unix-stream endpoint carrying typed bounded control and the
  original caller's exclusive liveness lease; a fresh private run nonce names the endpoint.

## Outputs

- For a live original caller, the existing typed backend outcome or typed setup, observation or
  cleanup refusal, with existing evidence semantics.
- For a dead original caller, namespace cancellation and guardian cleanup; no fabricated run
  result or execution evidence attributed to that caller.

## Behavior

The **bounded executor** runs in the original caller process. The **guardian** is the actual
namespace INIT, executed by bubblewrap's PID-1 command mode. The **monitor** is the bounded
executor's direct bubblewrap Child, retained unreaped until owned cleanup. One guardian, monitor,
namespace, lease and capture set own the entire FR-017 batch launcher, not each harness member.
FR-028 AC-21's whole-group memory ceiling and AC-12's batch deadline remain authoritative.

- The bounded executor shall launch only the first-party guardian as namespace PID 1.
- The bounded executor shall use one ownership set for the entire batch launcher.
- The guardian shall remain INIT while supervising and reaping its backend descendants.
- The guardian shall run in a new session and process group outside the original caller's.
- The guardian shall remain outside original-caller terminal job-control and hangup delivery.
- The bounded executor shall verify session isolation before authorizing backend Dispatch.
- The guardian shall wait for an exclusive live caller lease before accepting control.
- The guardian shall require positive typed Dispatch before creating a production backend.
- The guardian shall reject bootstrap-gate EOF as production backend authorization.
- The bounded executor shall verify the guardian peer against the owned monitor/INIT chain.
- The bounded executor shall establish memory-observer readiness before backend Dispatch.
- The bounded executor shall require a configured explicit helper path with no PATH discovery.
- If the helper is missing or unusable, then the bounded executor shall refuse setup.
- The bounded executor shall match the helper identity to the actual running library build.
- If helper identity mismatches, then the bounded executor shall refuse before backend Dispatch.
- The guardian shall enforce finite control-byte, pending-message and startup-work bounds.
- The guardian shall reject malformed controls and unknown fields.
- If caller-lease EOF occurs, then the guardian shall exit as namespace INIT.
- While Bootstrap is unclaimed, the bounded executor shall retain its gate during cancellation.
- When Bootstrap is cancelled, the bounded executor shall signal its pinned startup group.
- While INIT is claimed, the bounded executor shall confirm its pidfd termination on cleanup.
- The bounded executor shall keep the original caller's lease endpoint exclusive to the original
  caller.
- The bounded executor shall map CLOEXEC control descriptors only into their intended child.
- The bounded executor shall use safe descriptor APIs without relaxing its unsafe prohibition.
- The guardian shall preserve the original backend argv, stdin, environment and cwd.
- The guardian shall separate control and diagnostics from backend captures and reports.
- The bounded executor shall retain existing capture, resource and verdict classifications.
- The bounded executor shall charge all setup and execution to the original identity deadline.
- If that deadline expires, then the bounded executor shall classify the run as FR-028 AC-2 states.
- The bounded executor shall apply a finite setup cap within the remaining identity deadline.
- If only the setup cap expires, then the bounded executor shall return a typed setup refusal.
- The bounded executor shall bound control, capture and cleanup observation waits.
- If guardian failure occurs, then the bounded executor shall return a typed refusal.
- If owned teardown is unconfirmed, then the bounded executor shall refuse any proof conclusion.
- The guardian shall restrict cancellation to its own namespace descendants.
- The bounded executor shall use a kernel-lifetime endpoint without a filesystem socket artifact.
- A surviving run owner shall clean the temporary reports and artifacts assigned to that owner.
- The bounded executor shall reap its monitor after confirmed owned cleanup while it remains live.
- Setup documentation shall explain explicit matched helper delivery for library consumers.
- Production verification shall invoke the real package helper and observable lifecycle barriers.
- Regression verification shall fail ownership mutants before separate emergency fixture cleanup.

### Startup and termination observations

The stage boundaries and cancellation observations are cumulative only where explicitly stated:

| Stage | Authority and cancellation | Confirming observation |
|---|---|---|
| Bootstrap: monitor created; guardian INIT not yet claimed; no backend Dispatch | bounded executor retains unreaped monitor and bootstrap-gate writer. On live-caller startup failure, retain the gate and signal the pinned startup group before close/reap. INIT has not entered its new session while gated. Recover owned member identities through the exceptional pinned-group observation defined below, or claim exact INIT from bounded info/owned-child observation. Caller loss may close the bootstrap gate, but starts only trusted guardian bootstrap; absent lease, invalid control or startup deadline causes guardian exit without backend spawn. | Every discovered member pidfd signals termination and a fresh complete pinned-group membership observation establishes no live member; or a recovered exact INIT pidfd signals termination with startup-group death confirmed. Monitor exit or signal success alone is insufficient. Ambiguous membership or termination returns typed cleanup refusal. |
| ClaimedBootstrap: INIT pidfd/start/parent/namespace verified; gate released only for trusted bootstrap; no backend Dispatch | bounded executor retains monitor and INIT handles while guardian enters its new session and connects the lease. Failed connection, guardian death or caller loss never authorizes a backend. | Claimed INIT pidfd signals termination on cancellation; no production backend marker. |
| InitReady: guardian peer/build/session verified; observer ready; no backend Dispatch | bounded executor retains monitor and INIT handles. Lease EOF causes guardian exit; live bounded executor can cancel through the claimed INIT pidfd. Guardian death itself is namespace-INIT death. | Claimed INIT pidfd signals termination before a live caller accepts cleanup; no backend marker. |
| Dispatched: guardian received positive typed authorization with live lease | Guardian supervises the whole backend namespace. Caller loss causes guardian exit after owned cancellation; guardian death invokes kernel namespace teardown. Live bounded executor cancels through the claimed INIT pidfd on every completion or failure. | Claimed INIT pidfd signals termination before any live-caller proof conclusion; descendants are cancelled through kernel INIT teardown, not an observed-PID list. |

Bootstrap cancellation uses an exceptional bounded host-procfs membership walk keyed only to
the retained monitor's pinned PGID. Each discovered member retains its start identity and pidfd;
after signalling that group, a fresh complete membership walk must show no live member and every
retained member pidfd must report termination. Inaccessible or malformed potential membership,
changed identities that cannot be resolved, live threads with ambiguous leader state and an
observation deadline expiry are unconfirmed cleanup, never absence. Monitor exit, a successful
SIGKILL call and control EOF are not confirming observations. The gate remains retained through
this check so startup INIT cannot enter its new session. After INIT claim, its pidfd replaces
this exceptional group observation; ordinary execution never scans host processes for authority.

The abstract Unix endpoint stays in the same network namespace; this containment design does
not unshare networking. Its private run nonce distinguishes concurrent runs. Host-side Unix peer
credentials bind the connecting guardian to the actual claimed host INIT PID/start/pidfd and
namespace chain. A caller PID observed inside the child PID namespace is not assumed to equal
the host caller PID. Safe address/stream APIs create and connect the endpoint; no filesystem
socket path, copied descriptor adoption or independent cleanup daemon is required. Kernel
endpoint lifetime ends when its final owner closes, including abrupt process death.

Temporary run reports/artifacts have an explicit bounded executor/guardian cleanup owner before
their
creation. The surviving bounded executor cleans them on guardian failure; the isolated guardian
cleans
them on original-caller loss. Guardian bootstrap creates no persistent private files before it
connects the lease. If every file-cleanup owner dies, temporary files can remain; no dead actor
is claimed to unlink them. Kernel namespace cancellation and no unauthorized backend creation
still hold in that case, independently of filesystem cleanup and without fabricated evidence.

The guardian has a finite bootstrap connection/control deadline, even if the original caller
vanishes before it connects. No EOF, empty socket, failed connection, stale helper, failed
identity observation or refused startup permits backend creation. Session isolation precedes
Ready and survives signals directed to the original caller's whole process group/session. Direct
guardian SIGKILL/abort/OOM also cannot release a backend during bootstrap or retain descendants
after Dispatch: guardian death is PID-1 death, not closure of a sole external gate owner.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-034-AC-1 | Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer, observer readiness and positive typed Dispatch are established, no production backend instruction is created. Caller SIGKILL, abort or OOM-equivalent forced death during initialization may start only bounded guardian bootstrap, which exits without backend on absent lease or authorization. | Test |
| FR-034-AC-2 | While Bootstrap is unclaimed, the live bounded executor retains its unreaped monitor and bootstrap gate, signals the pinned startup group before close/reap, and confirms cancellation through the specified bounded member-identity observation. An exact recovered INIT pidfd can confirm INIT termination but never replaces required startup-group observation; monitor exit or successful group signalling alone never confirms cleanup. Bootstrap gate EOF can start only the trusted guardian; it cannot create a production backend. | Test |
| FR-034-AC-3 | In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF causes guardian cancellation/exit and confirmed INIT termination; Dispatched descendants are torn down by the kernel. The test controller observes dead-caller cleanup without manufacturing that caller's outcome or evidence. | Test |
| FR-034-AC-4 | Unix peer credentials identify the actual guardian INIT through the bounded executor's owned unreaped monitor and claimed INIT identity. A different peer refuses before backend Dispatch; the guardian need not be the bounded executor's direct Child. | Test |
| FR-034-AC-5 | The monitor is the bounded executor's actual unreaped Child; guardian INIT's pidfd/start identity, parent, namespace identity and namespace PID 1 agree with it. A stale/reused identity, arbitrary reported parent, incomplete identity or dead INIT refuses before Dispatch. | Test |
| FR-034-AC-6 | The owned tree memory observer is ready before typed Dispatch; failed observer preparation releases no backend instruction. | Test |
| FR-034-AC-7 | Startup refusal and malformed/closed startup information use Bootstrap's pinned-group member identity/pidfd observation; inaccessible/ambiguous live membership or unconfirmed termination yields typed cleanup refusal. In ClaimedBootstrap, InitReady and Dispatched, every cancellation/completion confirms claimed INIT termination before releasing ownership. Stage rules have no implicit claim-to-Dispatch gap. | Test |
| FR-034-AC-8 | After Dispatch, original-caller death cancels double-fork/reparented, setsid, late-born and nested-PID-namespace descendants through kernel-owned INIT teardown, including descendants absent from previous samples. Observed-PID lists or normal-operation host scans are not cancellation authority. | Test |
| FR-034-AC-9 | Cancelling one owned namespace leaves concurrent independent runs and unrelated host children unaffected. | Test |
| FR-034-AC-10 | Normal backend completion, backend exec failure, explicit cancellation, timeout, memory excess, capture failure and observation failure each require confirmed owned teardown before any proof conclusion. Backend exit or valid success output alone never authorizes acceptance. | Test |
| FR-034-AC-11 | Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady or immediately after Dispatch cannot release an unowned production backend: the guardian is INIT, and its death tears down that namespace. With a live original caller, guardian failure always yields a typed refusal and never verified/falsified evidence, even after confirmed teardown and beside a valid success report. | Test |
| FR-034-AC-12 | Every startup refusal, completion and cancellation stops the guardian/monitor after owned cleanup. The private abstract endpoint has no persistent socket artifact and ends with its final owner; bootstrap creates no persistent private files before lease connection. Temporary reports/artifacts have explicit cleanup ownership: a surviving bounded executor cleans after guardian failure and a surviving isolated guardian cleans after caller loss. All-owner death can leave temporary files but never waives kernel INIT descendant cancellation. A live bounded executor reaps its monitor Child; no dead caller is claimed to reap or unlink. | Test |
| FR-034-AC-13 | The bounded executor requires an explicit path to this package's actual Cargo executable quire-kani-guardian, built and supplied with the library from the same package source/build inputs. A missing, non-executable or unusable helper gives a typed setup refusal before backend Dispatch, with no PATH/global discovery, copied executable, shell substitute or alternate launcher. Cargo library dependency resolution alone is not helper delivery. | Test |
| FR-034-AC-14 | Setup documentation identifies the package helper and Linux/procfs children and RSS, pidfd, bubblewrap PID-1/new-session namespace/info/gate features and namespace permissions required by the planned containment code slice (PR #295), without requiring host-policy changes or an ad hoc global installation. | Inspection |
| FR-034-AC-15 | Typed private controls reject malformed and unknown fields and enforce finite encoded-byte, pending-message and startup-work bounds. Invalid/overlimit control or EOF cancels or refuses rather than authorizing Dispatch. | Test |
| FR-034-AC-16 | Only the original caller retains its liveness endpoint. Guardian, monitor, backend descendants and unrelated concurrent execs cannot inherit an endpoint that keeps it alive. bounded executor-created bootstrap/info pipes are CLOEXEC in their creator and mapped only to the intended child using a safe child-only API; Unix control needs no raw inherited-FD adoption or weakening of unsafe prohibition. | Test |
| FR-034-AC-17 | The same backend receives unchanged raw argument bytes, inherited stdin, inherited and overridden environment, and working directory. | Test |
| FR-034-AC-18 | Guardian diagnostics and control use separate channels from backend stdout/stderr captures and cannot contaminate reports; existing bounded capture and capture-failure behavior remain authoritative. | Test |
| FR-034-AC-19 | Original identity ceilings and existing outcome classifications remain authoritative after confirmed cleanup: memory excess and wall expiry stay distinct, ambiguous live-worker RSS refuses, and ordinary completed/refused/falsified reports retain their meanings. Native refinement uses this ownership path when its FR-028 AC-24 typed entry is implemented, with unchanged refinement class/evidence rules. | Test |
| FR-034-AC-20 | Guardian connection, startup, identity verification and Dispatch use the original monotonic identity deadline without resetting it. An already expired deadline causes no Dispatch and settles inconclusive with the timed-out reason as FR-028 AC-2 states; actual identity-deadline expiry during setup or execution uses the same classification. | Test |
| FR-034-AC-21 | Guardian connection and handshake have a finite setup cap within the remaining original deadline. Cap expiry while that deadline remains live is a typed setup refusal, distinct from identity-deadline timeout. | Test |
| FR-034-AC-22 | Control/capture shutdown and cleanup observation waits have finite bounds. Unconfirmed termination refuses with a live caller rather than accepting a proof or claiming physical disappearance of an uninterruptible task. | Test |
| FR-034-AC-23 | Caller-death fixtures invoke the real guardian built from this package in the owning target directory and observe pre-initialization, gated and immediate post-Dispatch stages through positive handshakes and owned identities/pidfds, without production test bypasses, sleep-based success or wide host-scan authority. | Test |
| FR-034-AC-24 | Removing positive Dispatch authorization, replacing the PID-1 guardian with a non-INIT watcher, removing session isolation or closing/reaping before pinned startup confirmation fails the corresponding premature-marker, surviving-descendant, measured-session or startup-order assertion. Emergency fixture cleanup occurs after recording the ownership observation and cannot turn failure into a pass; restored production controls pass. | Test |
| FR-034-AC-25 | Before backend Dispatch, the helper handshake matches the actual running CG library's build, protocol and lifecycle-capability identity against the actual invoked first-party executable. A stale helper or changed lifecycle implementation refuses, even if a caller supplies a matching version label or digest. Expected identity derives from actual library/helper build artifacts, not caller assertions or manually maintained tracking pins. | Test |
| FR-034-AC-26 | The guardian's host session and process group are distinct from the original caller's before Ready and backend Dispatch, with no controlling-terminal job-control delivery from that caller's session. Killing the caller's whole group at pre-Ready, InitReady and immediate post-Dispatch stages leaves guardian cleanup operational; a directly killed guardian still triggers kernel namespace teardown. | Test |

## Dependencies

- [FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 owns the every-run tree memory mechanism;
  AC-2/3 own resource outcomes, and AC-24 owns bounded native refinement. This requirement adds
  independent original-caller lifecycle ownership, not another resource model.
- [FR-017](./FR-017-kani-execution-evidence.md) owns backend reports, capture and batching.
- [TC-049](../matrix/TC-049-caller-death-ownership.md) describes planned production verification.
  Guardian CODE is gated on the preceding ceiling containment code slice (PR #295), not on
  this SPEC merging or completion of all parent IR-241 work. Criterion-level implementation
  order is FR-017 launcher → FR-028 AC-21 containment slice → FR-034 guardian ownership →
  FR-028 AC-24 native refinement; no whole-requirement or parent-ticket cycle is introduced.

The lifecycle claim includes signals directed to the original caller's process group/session
and direct guardian death. Host failure or loss of the kernel's namespace facilities cannot be
turned into a confirmed cleanup observation. Such unavailable observation with a live caller
refuses; it does not waive caller-group or guardian startup death protection.

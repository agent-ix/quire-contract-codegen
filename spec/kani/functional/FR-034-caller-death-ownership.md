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

If the original caller dies, then the namespace guardian shall cancel its backend descendants. The
guardian shall prevent startup EOF from authorizing a production backend instruction.

This requirement extends the Linux PID-namespace containment delivered by PR #295.
[FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 retains its resource authority. The preceding
containment code is merged; guardian ownership and its verification remain planned. This requirement
does not depend on completing all parent IR-241 work, which IR-639 itself blocks.

Planned (IR-639): no first-party guardian or production fixtures are implemented. The existing
containment launcher's caller-owned gate can close on SIGKILL, abort or OOM death before its
parent-death chain is armed. Here the only program that such EOF may start is the first-party
guardian bootstrap, never the production backend. The guardian is namespace PID 1, so its own
termination tears down the namespace through the kernel. Positive backend authorization remains
separate from the bubblewrap bootstrap gate. Research probes establish design feasibility, not
production coverage. The real packaged helper and regression mutants must be verified.

## Inputs

- The existing bounded execution request, original harness identity, backend command recipe and
  recorded ceilings, including the original monotonic wall deadline.
- A required explicit executable path to `quire-kani-guardian`, built with the library from the same
  Cargo package source and build inputs. A library consumer deliberately builds and supplies this
  artifact; Cargo does not build or install a dependency's binary for it.
- A private per-run anonymous connected Unix-stream pair carrying typed bounded control and the
  original caller's exclusive liveness lease, plus its separately owned backend stdin.

## Outputs

- For a live original caller, the existing typed backend outcome or typed setup, observation or
  cleanup refusal, with existing evidence semantics.
- For a dead original caller, namespace cancellation and guardian cleanup; no fabricated run result
  or execution evidence attributed to that caller.

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
- The guardian shall authenticate the original caller through its exclusive connected lease.
- The guardian shall reject a creator UID outside the verified original-caller UID mapping.
- The bounded executor shall authenticate guardian Ready using kernel sender credentials.
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
- Explicit cancellation shall consume the private caller lease while retaining run ownership.
- The bounded executor shall record lease-close observation before independent INIT escalation.
- While Bootstrap is unclaimed, the bounded executor shall retain its gate during cancellation.
- When Bootstrap is cancelled, the bounded executor shall signal its pinned startup group.
- While INIT is claimed, the bounded executor shall confirm its pidfd termination on cleanup.
- The bounded executor shall keep the original caller's lease endpoint exclusive to the original
  caller.
- The bounded executor shall map CLOEXEC control descriptors only into their intended child.
- The guardian shall restore separately received original stdin as backend fd 0 before spawn.
- If lease EOF is observable, then the guardian shall reject pending Dispatch and cancel.
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
- The bounded executor shall use an anonymous pair without a publicly bindable rendezvous.
- A surviving run owner shall clean the temporary reports and artifacts assigned to that owner.
- The bounded executor shall reap its monitor after confirmed owned cleanup while it remains live.
- Setup documentation shall explain explicit matched helper delivery for library consumers.
- Production verification shall invoke the real package helper and observable lifecycle barriers.
- Regression verification shall fail ownership mutants before separate emergency fixture cleanup.
- The CG library shall omit guardian-test-support from default features.
- Where guardian-test-support is enabled, the CG library shall export one fixture operation.
- The fixture operation shall record immutable raw observations before independent INIT escalation.
- The live-caller fixture operation shall invoke unchanged production cleanup immediately after
  observation.
- The bounded executor shall publish BeforeMonitor, Bootstrap, ClaimedGated, ClaimedBootstrap and
  InitReady unconditionally.
- Production and fixture execution shall select one shared private stage sequence as data.
- The intentional-death fixture shall report its exact prefix boundary before self-killing.
- The intentional-death fixture shall transfer only its actual owned monitor or validated INIT pin
  to its harness.
- The fixture operation shall exclude pass/fail oracle evaluation from library execution.
- The CG verification harness shall judge the defined oracle from pre-escalation raw observations.
- The bounded executor shall publish private LeaseClosing state after actual caller-lease closure.
- The bounded executor shall publish that state unconditionally through a monotonic read-only value.
- The bounded executor shall seal close/publication ordering at the actual publication boundary.
- The private Dispatch sender shall queue the unchanged production frame without waiting for ACK.
- The bounded executor shall wait for Dispatch acknowledgement in a separate bounded transition.
- The fixture continuation shall resume only its deliberately stopped and pinned INIT.
- When LeaseClosing is positively observed, the fixture continuation shall send owned-pidfd SIGCONT.
- The fixture operation shall reject unavailable continuation as passing EOF-cancellation evidence.
- The fixture build shall select the actual package helper through the consumer manifest.
- The CG verification harness shall build feature-off and feature-on configurations separately.
- If helper and library feature identities differ, then the bounded executor shall refuse Dispatch.
- CG documentation shall identify the downstream production-feature exclusion owned by IR-649.

### Startup and termination observations

The stage boundaries and cancellation observations are cumulative only where explicitly stated:

| Stage | Authority and cancellation | Confirming observation |
|---|---|---|
| BeforeMonitor: no monitor or INIT spawned | The same shared sequence has not invoked monitor creation; the original caller owns only its actual setup/lease state. No absent child has a process identity or pidfd. | Typed NoInit witness, no backend marker and closed actual setup/lease ownership after caller death. |
| Bootstrap: monitor created; guardian INIT not yet claimed; no backend Dispatch | The bounded executor retains its unreaped monitor and bootstrap-gate writer. Recover exact INIT through bounded startup information or direct owned-monitor child/identity observation. If recovery is reliable, claim its pidfd and use that authority. Otherwise signal only the pinned startup group before close/reap, close the exclusive lease and refuse unconfirmed cleanup. Gate EOF starts only trusted bootstrap; it cannot supply a replacement caller authority. | Exact recovered INIT pidfd termination confirms namespace teardown. A live child/identity observation must verify parent/start/namespace before claim. If monitor death/reparenting or unavailable data prevents recovery, no group signal, monitor exit or host scan is accepted as confirmation: return typed unconfirmed-cleanup refusal. |
| ClaimedGated: INIT pidfd/start/parent/namespace verified and observer bound; gate retained | The same unconditional claim transition retains the validated INIT pin before the separate gate-release transition. Caller/group death cannot authorize a backend; trusted guardian bootstrap still requires the exclusive lease and Dispatch. | Claimed INIT pidfd termination and no production backend marker; gate remains retained at the exact fixture prefix. |
| ClaimedBootstrap: INIT pidfd/start/parent/namespace verified; gate released only for trusted bootstrap; no backend Dispatch | bounded executor retains monitor and INIT handles while guardian enters its new session and authenticates the exclusive pair. Failed lease authentication, guardian death or caller loss never authorizes a backend. | Claimed INIT pidfd signals termination on cancellation; no production backend marker. |
| InitReady: guardian peer/build/session verified; observer ready; no backend Dispatch | bounded executor retains monitor and INIT handles. Lease EOF causes guardian exit; live bounded executor can cancel through the claimed INIT pidfd. Guardian death itself is namespace-INIT death. | Claimed INIT pidfd signals termination before a live caller accepts cleanup; no backend marker. |
| Dispatched: guardian received positive typed authorization with live lease | Guardian supervises the whole backend namespace. Caller loss causes guardian exit after owned cancellation; guardian death invokes kernel namespace teardown. Live bounded executor confirms claimed INIT termination on every completion or failure. Explicit cancellation first closes the lease and observes guardian exit; bounded escalation uses the claimed INIT pidfd. | Claimed INIT pidfd signals termination before any live-caller proof conclusion; descendants are cancelled through kernel INIT teardown, not an observed-PID list. |

Bootstrap cancellation gives exact INIT ownership precedence. The bounded executor may recover INIT
from bounded startup information or a bounded walk of the live owned monitor's direct task children,
validating parent, start identity and namespace before opening the pidfd. If the monitor dies and
children reparent before recovery, or observation is unavailable, no complete cleanup is claimed:
signal the still-pinned owned group while retaining the gate, close the exclusive lease, then return
typed unconfirmed-cleanup refusal. The trusted guardian can only exit on that closed lease. No
host-wide procfs membership walk occurs in startup or steady state. A recovered exact INIT pidfd's
termination suffices for namespace teardown; monitor reaping remains separate. Signal success,
monitor exit and stream EOF never prove teardown.

The bounded executor creates the anonymous connected Unix-stream pair before monitor spawn. Both
ends are CLOEXEC; only the original caller retains the executor end. Safe child-only mapping gives
the guardian end to fd 0 through bubblewrap. The guardian safely borrows stdin's owned descriptor
for control receive/send; it never adopts an arbitrary inherited raw fd. There is no pathname,
abstract name, nonce-derived secret, public listener or connect/rebind step. An unrelated process
cannot replace the original caller by binding a discovered address. Pair creation or mapping failure
is typed setup refusal, never a rendezvous fallback.

The guardian checks kernel peer credentials of the pair's creator against the actual mapped
original-caller UID. Its view of the outside creator PID can be 0 and is not used as a host PID. The
non-public authority is possession of this specific already-connected executor endpoint, which never
passes to a monitor, backend or unrelated exec. Neither argv, environment nor a public socket-name
listing confers that authority. The bounded executor enables kernel sender credentials and binds
Ready's actual sender PID to its claimed host INIT pidfd/start/parent/namespace chain: socketpair's
creator credentials alone cannot authenticate guardian messages. Controls from another pair,
mismatched mapped UID or stale/replayed run authority refuse.

The guardian receives the original backend stdin separately as a bounded typed control's owned
descriptor through safe ancillary-rights APIs. Received descriptors are CLOEXEC, with exactly the
declared count/type; unknown ancillary records, extra/missing descriptors and byte/control
truncation refuse and close every received descriptor. Safe conversion to backend stdio restores
that same stdin at fd 0; the closed-stdin case remains closed. The guardian's control fd is excluded
from backend inheritance, and stdout/stderr remain the original bounded captures. Invalid control,
EOF and peer/build mismatch refuse before Dispatch. Observable lease EOF takes precedence over
buffered or pending Dispatch; an earlier valid authorization remains owned by INIT, which cancels
when it observes EOF. No atomic prediction of future caller death is claimed.

### Production lease-close cancellation

Private typed ownership separates a non-clonable `CallerLease`, which owns only the executor stream,
from `RunOwner`, which retains the unreaped monitor Child, claimed INIT pidfd/identity, captures and
original deadline. Both stay in the original caller; this separation neither exports the lease nor
transfers monitor ownership. Explicit cancellation in InitReady or Dispatched invokes the production
`close_lease_and_observe` operation: it consumes and closes `CallerLease`, disables further caller
Dispatch, and retains `RunOwner` while observing claimed INIT termination. No independent INIT
cancellation or escalation signal is sent during this LeaseClosing phase. Pending guardian
authorization still obeys observable EOF precedence. The same operation handles cancellation before
and after Dispatch, not a test-only hook.

LeaseClosing has a finite observation cap clamped to the remaining original identity deadline. Its
private typed `LeaseCloseObservation` distinguishes confirmed guardian termination, escalation
required while INIT remains live, and unavailable termination observation. The operation records and
returns this observation before any independent INIT cancellation or escalation signal; it never
reports termination from stream EOF alone. The normal production cancellation driver immediately
uses the observation: confirmed termination proceeds to bounded capture settlement and monitor
reaping; the other cases signal the claimed INIT, perform bounded confirmation/cleanup and refuse
unconfirmed teardown. This operation boundary lets the opt-in fixture operation record the actual
pre-escalation observation while its live original caller retains monitor/INIT ownership, then
immediately execute the same production cleanup phase. It exports no lease or cancellation handle
and never waits for a test controller.

Resource breach, original-deadline expiry and observation/identity failure retain their urgent
claimed INIT cancellation authority; they do not wait out a new lease grace period or reset a
deadline. LeaseClosing expiry records escalation before signalling, never a successful EOF
cancellation. `RunOwner` remains an ownership guard throughout both phases: dropping or abandoning a
live owned run initiates pinned cancellation; only explicit bounded cleanup can confirm teardown,
reap and settle a result. Dropping the separate lease cannot drop or reap the monitor. No proof is
accepted during LeaseClosing or from an escalation-required/unavailable observation.

### Opt-in guardian fixture observation

The `guardian-test-support` Cargo feature shall be off by default and absent from default features.
It shall expose exactly one documented fixture operation for TC-049's private-boundary scenarios.
The operation shall invoke the same private production transitions selected by its typed scenario.
Its live-caller form shall execute unchanged lease-close and cleanup operations unconditionally. The
feature shall add no branch inside a production stage and no substitute namespace INIT, worker,
identity check or cleanup implementation. Its additional fixture-only authority is recording real
typed observations, arranging bounded positive coordination for pending Dispatch and reporting an
exact early-stage death witness. Ordinary public execution and positive post-Dispatch caller-death
fixtures remain available without this feature.

The live-caller form shall retain real `RunOwner` handles in the original caller while recording raw
typed stage, `LeaseCloseObservation`, pinned worker and backend-marker observations before
independent INIT escalation. The operation shall seal these observations without evaluating a
pass/fail oracle. The operation shall immediately invoke unchanged production cleanup on success,
refusal and observation-error paths. The operation shall return immutable raw observations and a
separate cleanup result only after that cleanup. The operation shall return no lease, process handle
or cleanup-deferring callback. If observation is lost, overflowed or unavailable, then the fixture
operation shall record typed failure. The fixture operation shall reject that failure as confirmed
termination. Urgent resource and identity cancellation remains authoritative.

For exact early-stage caller deaths, the executor shall use one shared private stage sequence.
Production shall select the complete sequence and the fixture shall select a bounded prefix as
ordinary typed data. Both selections shall invoke the same functions in the same order; no copied
fixture orchestrator, production feature branch, hook or replacement transition is permitted.
BeforeMonitor, Bootstrap, ClaimedGated, ClaimedBootstrap and InitReady shall publish their actual
stage through the same private monotonic read-only mechanism as LeaseClosing. Publication shall add
no I/O, callback, blocking handoff or controller pause. The feature shall expose this read access
only inside the single fixture operation, not as another public item or production process handle.

The shared sequence shall claim the gated INIT and bind its observer before a separate unconditional
gate-release transition. ClaimedGated retains the gate; ClaimedBootstrap has released it. A fixture
shall never label the released gate as retained or bypass actual identity/observer checks.

The harness shall map its anonymous report socketpair only into the first-party fixture's startup
stdout. Before any process spawn, the fixture shall safely borrow stdout, duplicate it into an
OwnedFd auxiliary descriptor numbered at least 3 with CLOEXEC, and mark the original stdout CLOEXEC.
It shall then use only the auxiliary descriptor for witness reporting. This safe standard-descriptor
entry shall not adopt an arbitrary inherited raw fd. Every later spawn shall configure stdio
explicitly and exclude both report descriptors from its child mapping. Neither descriptor may enter
the monitor, INIT, guardian, backend or unrelated exec. No child stdio option may clear CLOEXEC or
reuse the reporter as a capture. Failure to establish this exclusion shall fail before monitor
spawn.

At the selected boundary, the intentional-death fixture shall seal the actual published stage and
owned caller/monitor/claimed-INIT start and namespace identities, including the actual transferred
descriptor identity, before invoking the next shared transition. It shall send one bounded typed
witness through that auxiliary report descriptor. At ClaimedGated or later, the message shall carry
exactly one clone of the already-validated owned INIT pidfd through safe ancillary-rights APIs. At
Bootstrap immediately after monitor spawn but before any INIT claim or gate release, it shall
instead carry exactly one clone of the actual owned monitor pidfd and typed InitUnclaimed. INIT may
already exist; InitUnclaimed shall never mean NoInit, nor fabricate an INIT identity or pin. At
BeforeMonitor, typed NoInit shall record actual no-spawn/setup state and carry no descriptor. An
unavailable owned monitor pin or required claimed INIT pin shall fail, never establish a passing
stage witness. The fixture shall never open a replacement pidfd from a reported PID after death. The
report channel shall never carry the original caller lease or become backend capture.

After the bounded complete send, the same fixture process shall immediately SIGKILL itself or its
positively owned dedicated caller group, before any next transition. No harness acknowledgement or
external controller permission shall delay that death. A partial send, absent/overflowed stage,
missing pin or failed coordination shall fail the fixture and invoke unchanged owned cleanup while
the caller lives; no timed success or passing platform skip is permitted. All reporting work remains
within the original deadline. Reporting and self-kill exist only inside the opt-in fixture
operation; ordinary production transitions perform no report I/O or intentional death. This
operation does not return after successful self-kill and supplies no fabricated execution result for
its dead caller.

The surviving harness shall retain its actual original-caller Child unreaped and its start/pidfd
identity until authenticating the report's kernel sender credentials. It shall receive exactly one
CLOEXEC owned pidfd for Bootstrap or claimed INIT and match its actual descriptor identity and
reported pin kind to the sealed predeath pin. It shall reject malformed, truncated, wrong-sender,
wrong-type or extra/missing rights and close all rejected descriptors. NoInit shall carry no right.
For a live claimed INIT, the harness shall verify the transferred pin against sealed start/namespace
identity. After INIT death, that predeath validated pin remains termination authority; a reused
/proc PID cannot substitute. ClaimedGated and later require confirmed caller and INIT death plus no
pre-Dispatch backend marker. BeforeMonitor instead requires actual no-INIT setup observations,
closed actual lease/pair ownership and no backend marker, plus termination of any positively owned
setup child.

Bootstrap shall positively witness monitor creation, no INIT claim, retained gate and no Dispatch
before self-kill. Its runtime assertion shall require caller and pinned monitor termination, closed
original-caller gate/lease ownership and no backend marker within bounded observation. It shall not
infer INIT termination or guardian exit through EOF from monitor readiness, stream EOF or a reported
PID. No descendant snapshot or after-death scan may invent missing INIT authority: the held gate
does not prevent all bubblewrap setup forks. The underlying no-unowned-backend and owned-cleanup
guarantees remain mandatory. Their Bootstrap verification shall separately include source/lifecycle
Analysis of trusted-only gate-EOF bootstrap, exclusive caller-lease loss, bounded guardian refusal
and namespace-INIT teardown; parent-death handling shall not be claimed armed before its actual
installation. Analysis shall include INIT creation before startup information and the internal
map/setup handoff before the public gate; it shall not presume gate EOF reaches guardian when the
monitor dies before releasing that handoff. Any unresolved path remains an unmet CODE guarantee, not
passing stage-only evidence. That argument is not a measured INIT-death Test at this boundary.
ClaimedGated and later retain positive INIT-pidfd termination Tests, and unconfirmed live-caller
cleanup still refuses.

Independent emergency cleanup of only fixture-owned processes shall follow recorded assertions
without masking them. Structural inspection shall establish shared stage functions/order, safe
reporter exclusion and feature item exposure only; mandatory AC-24 EOF mutants retain behavioral
parity evidence. Neither stage-only Bootstrap facts nor structural inspection satisfies the complete
namespace-teardown oracle or supplies executable coverage for the Analysis obligation.

A Linux queued-descriptor probe measured one validated INIT pidfd surviving a separate reporter's
SIGKILL, received CLOEXEC both while INIT was live and after its death. This establishes only the
queued-pin facility: its host parent owned the actual monitor/INIT/control, not the production
original-caller Rust path. Actual shared-prefix stage reporting, caller/group death, sender binding,
NoInit/InitUnclaimed observations and the production guardian oracle remain CODE acceptance tests.

A separate Python/kernel reporter probe measured the standard-stdout entry, non-stdio CLOEXEC
auxiliary duplication and original-stdout CLOEXEC marking before spawn. The actual socket identity
was absent from the unrelated exec, bubblewrap monitor, namespace PID-1 helper and backend. Removing
either mark failed its actual pre-spawn descriptor-flag assertion; removing the auxiliary mark also
leaked the socket into unrelated exec. Explicit stdout replacement prevented a leak in the
original-mark mutant, so flag and inheritance assertions remain distinct. Safe Rust descriptor APIs
were source-grounded; no arbitrary inherited-fd adoption is required. This facility probe is not
production Rust reporter acceptance, gated-INIT coverage or a passing Bootstrap cleanup Analysis.

For deterministic pending-Dispatch coordination, the feature-gated fixture owner shall stop its
verified InitReady INIT through that owned pidfd. The owner shall positively verify stopped state
`T` against the same live INIT/start/namespace identity before queueing Dispatch through the
unchanged private production frame-send step on the ordinary control stream. This step shall
serialize and send the actual production authorization and ancillary rights, not fixture-written
bytes. The send shall use bounded nonblocking transport within the original deadline. The send shall
return a typed pending-frame state only after the complete frame is queued. If the bounded send
fails or transport is unavailable, then the fixture operation shall record typed coordination
failure. The fixture operation shall invoke cleanup after that failure. The pending-frame state
proves neither guardian receipt nor backend creation. The normal executor shall await any guardian
acknowledgement in a separate bounded transition. The stopped-INIT fixture shall not enter that
acknowledgement wait before closing its lease. The owner shall keep `RunOwner` and the original
caller alive while invoking the unchanged synchronous `close_lease_and_observe` operation. After
actual `CallerLease` closure, that production operation shall publish its private LeaseClosing stage
unconditionally through a monotonic read-only value before waiting for termination. Publication
shall add no callback, blocking handoff, extra I/O or feature-keyed branch to any production stage.

The bounded executor shall record a close-completion ordinal only after consuming the exclusive
`CallerLease` and returning from its actual owned-endpoint close. At actual LeaseClosing
publication, the executor shall assign a publication ordinal and seal one immutable snapshot
containing the optional completed-close ordinal and publication ordinal. A bounded per-run sequence
orders these events without timestamps. Publication shall never late-fill or rewrite that snapshot
after a later close. If publication is duplicated or ordering is unavailable, then the fixture
operation shall record typed observation failure. The continuation shall read this exact publication
snapshot, not a later mutable closed flag. The external harness shall require a present
close-completion ordinal strictly before the publication ordinal. Early publication therefore fails
even if a later close occurs before the resumed guardian reads its stream. These ordering facts
shall complement actual namespace/EOF observations, never replace them or grant a synthetic passing
value.

The fixture owner shall use an internal owned continuation thread to observe that positive
publication and send SIGCONT through its positively pinned INIT pidfd. Only that fixture-only
continuation may resume its deliberately stopped INIT; no external controller grants permission.
SIGCONT is a resumption signal, not cancellation or escalation, and shall never establish the EOF
oracle. The guardian shall observe the already closed stream before accepting its buffered Dispatch.
The fixture continuation shall use bounded observation within the original deadline and join during
unconditional cleanup. Failed stop/state validation, publication, continuation or resume shall
record typed coordination failure and trigger owned cleanup, never a passing fixture or platform
skip. Urgent deadline/resource/identity cancellation retains its authority while INIT is stopped.

Linux namespace/pidfd/stream probes measured owner stop, positive `T` state, queued Dispatch
followed by exclusive lease close, resume, EOF alongside buffered bytes and confirmed INIT
termination. These facility observations establish feasibility only; the unconditional Rust
publication/continuation and production guardian oracle remain unimplemented and require actual CODE
verification.

The external test harness owns the AC-24 predicate and evaluates it after the operation returns
following unconditional cleanup, using only sealed observations taken before escalation. For
pre-Dispatch it requires the sealed close-before-publication ordering, confirmed INIT termination
before escalation and no production backend marker. For post-Dispatch it requires confirmed INIT
termination and a dead pinned worker which positively acknowledged startup, both observed before
escalation. Escalation-required, unavailable observation and a live worker fail the predicate;
subsequent cleanup success cannot change those observations. The library supplies raw facts, not a
fixture verdict or fabricated proof evidence.

The packaged caller fixture, library and real helper shall use the same normal, non-`cfg(test)`
library artifact. The fixture build shall select the package helper from the consumer manifest using
`cargo -p`. The fixture build shall match target, profile, feature selection and compiler flags. The
bounded executor shall require actual artifact identity without an epoch, version or digest
override. If a feature-off helper is supplied to a feature-on fixture, then the bounded executor
shall refuse before Dispatch. If a feature-on helper is supplied to a feature-off production caller,
then the bounded executor shall refuse before Dispatch. Feature-off builds shall expose no fixture
operation. The CG verification harness shall check both mismatch refusals and feature-off export
absence using actual artifacts.

The CG verification harness shall use separate named Cargo invocations: `guardian-feature-off` for
normal feature-off library/helper/caller builds and AC-23/26 public positive post-Dispatch fixtures,
and `guardian-feature-on` for normal feature-enabled library/helper/caller builds, exact early-stage
AC-23/26 death witnesses and AC-24 observation.
These names identify verification configurations, not Cargo profiles or new build directories. The
harness shall not use a self dev-dependency that unifies guardian-test-support into the feature-off
configuration. Neither configuration uses a `cfg(test)` library to impersonate its normal helper.

CG documentation shall mark guardian-test-support as test-only. CG documentation shall identify
matched-artifact delivery and separate feature-off/on verification. CG documentation shall allocate
production-driver dependency-edge exclusion to the QSL production-driver work owned by
[IR-649](https://linear.app/agent-ix/issue/IR-649). That downstream work must reject the feature in
all driver production-build profiles, including transitive Cargo feature unification. IR-639 enables
that work; its downstream delivery is not a prerequisite for delivering the CG observation export.
CG inspection checks publication of this contract, not the driver's implementation or a CG-only
surrogate of it. The downstream dependency-edge assertion remains a separate planned gate.

### Run artifact and bootstrap cleanup

Temporary run reports/artifacts have an explicit bounded executor/guardian cleanup owner before
their creation. The surviving bounded executor cleans them on guardian failure; the isolated
guardian cleans them on original-caller loss. Guardian bootstrap creates no persistent private files
before lease authentication. If every file-cleanup owner dies, temporary files can remain; no dead
actor is claimed to unlink them. Kernel namespace cancellation and no unauthorized backend creation
still hold independently of filesystem cleanup and without fabricated evidence.

The guardian has a finite bootstrap connection/control deadline, even if the original caller
vanishes before lease authentication. No EOF, missing control capability, stale helper, failed
identity observation or refused startup permits backend creation. Session isolation precedes Ready
and survives signals directed to the original caller's whole process group/session. Direct guardian
SIGKILL/abort/OOM also cannot release a backend during bootstrap or retain descendants after
Dispatch: guardian death is PID-1 death, not closure of a sole external gate owner.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-034-AC-1 | Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer, observer readiness and positive typed Dispatch are established, no production backend instruction is created. Caller SIGKILL, abort or OOM-equivalent forced death during initialization may start only bounded guardian bootstrap, which exits without backend on absent lease or authorization. | Test |
| FR-034-AC-2 | While Bootstrap is unclaimed, bounded startup information or direct owned-monitor child/identity observation may establish exact INIT ownership. Its validated pidfd takes precedence and termination confirms namespace teardown. If monitor death/reparenting or unavailable identity prevents recovery, signal only the pinned owned group while retaining the gate, close the exclusive lease and return typed unconfirmed-cleanup refusal. No host scan, successful signal or monitor exit establishes a passing cleanup claim. | Test |
| FR-034-AC-3 | In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF causes guardian cancellation/exit and confirmed INIT termination; Dispatched descendants are torn down by the kernel. The test controller observes dead-caller cleanup without manufacturing that caller's outcome or evidence. | Test |
| FR-034-AC-4 | Mutual authority is the specific anonymous connected pair: guardian checks the actual mapped creator UID and only the original caller holds its executor endpoint. The bounded executor binds kernel credentials of Ready's actual sender to claimed host INIT/start/pidfd/parent/namespace, not socketpair creator credentials alone. Guardian's creator PID0 is not a host identity. A foreign actor cannot rebind a public name or supply replacement lease/Dispatch; another pair, mismatched UID or replayed authority refuses. | Test |
| FR-034-AC-5 | The monitor is the bounded executor's actual unreaped Child; guardian INIT's pidfd/start identity, parent, namespace identity and namespace PID 1 agree with it. A stale/reused identity, arbitrary reported parent, incomplete identity or dead INIT refuses before Dispatch. | Test |
| FR-034-AC-6 | The owned tree memory observer is ready before typed Dispatch; failed observer preparation releases no backend instruction. | Test |
| FR-034-AC-7 | Startup information failure uses bounded exact INIT recovery through owned information/children, with exact pidfd authority first. Unrecoverable identity or unconfirmed termination yields typed cleanup refusal, with no host-wide scan. ClaimedBootstrap, InitReady and Dispatched cancel through claimed INIT and confirm termination before release of ownership. Stage rules have no implicit claim-to-Dispatch gap. | Test |
| FR-034-AC-8 | After Dispatch, original-caller death cancels double-fork/reparented, setsid, late-born and nested-PID-namespace descendants through kernel-owned INIT teardown, including descendants absent from previous samples. Observed-PID lists or normal-operation host scans are not cancellation authority. | Test |
| FR-034-AC-9 | Cancelling one owned namespace leaves concurrent independent runs and unrelated host children unaffected. | Test |
| FR-034-AC-10 | Normal backend completion, backend exec failure, explicit cancellation, timeout, memory excess, capture failure and observation failure each require confirmed owned teardown before any proof conclusion. Backend exit or valid success output alone never authorizes acceptance. | Test |
| FR-034-AC-11 | Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady or immediately after Dispatch cannot release an unowned production backend: the guardian is INIT, and its death tears down that namespace. With a live original caller, guardian failure always yields a typed refusal and never verified/falsified evidence, even after confirmed teardown and beside a valid success report. | Test |
| FR-034-AC-12 | Every startup refusal, completion and cancellation stops the guardian/monitor after owned cleanup. The anonymous pair has no public name or persistent socket artifact and ends with its final owner; bootstrap creates no persistent private files before lease authentication. Surviving executor and guardian own temporary artifact cleanup on opposite-owner failure. All-owner death may leave temporary files but never waives kernel descendant cancellation; no dead caller is claimed to reap or unlink. | Test |
| FR-034-AC-13 | The bounded executor requires an explicit path to this package's actual Cargo executable quire-kani-guardian, built and supplied with the library from the same package source/build inputs. A missing, non-executable or unusable helper gives a typed setup refusal before backend Dispatch, with no PATH/global discovery, copied executable, shell substitute or alternate launcher. Cargo library dependency resolution alone is not helper delivery. | Test |
| FR-034-AC-14 | Setup documentation identifies the package helper and Linux/procfs children and RSS, pidfd, bubblewrap PID-1/new-session namespace/info/gate features and namespace permissions required by the planned guardian (IR-639); the merged containment slice (PR #295) instead uses user/PID namespace, die-with-parent, info-fd and block-fd flags, without requiring host-policy changes or an ad hoc global installation. Fixture documentation marks guardian-test-support as test-only and identifies matched-artifact delivery, feature-off verification and production-driver exclusion. | Inspection |
| FR-034-AC-15 | Typed private controls reject malformed and unknown fields and enforce finite encoded-byte, pending-message and startup-work bounds. Ancillary descriptor count/type is exact, received descriptors are CLOEXEC, and unknown/extra/truncated ancillary data refuses while closing all received descriptors. Invalid/overlimit control or EOF cancels or refuses rather than authorizing Dispatch. | Test |
| FR-034-AC-16 | Only the original caller holds the pair's executor endpoint. Guardian receives only its control end as fd0 through safe CLOEXEC child-only mapping, borrows it with a safe descriptor API and receives actual backend stdin separately as OwnedFd through safe ancillary rights. Backend fd0 restores that original stdin or closed state, while the control/lease descriptor is excluded from backend and unrelated exec inheritance. No arbitrary raw-fd adoption or unsafe exception occurs. | Test |
| FR-034-AC-17 | The same backend receives unchanged raw argument bytes, inherited stdin, inherited and overridden environment, and working directory. | Test |
| FR-034-AC-18 | Guardian diagnostics and control use separate channels from backend stdout/stderr captures and cannot contaminate reports; existing bounded capture and capture-failure behavior remain authoritative. | Test |
| FR-034-AC-19 | Original identity ceilings and existing outcome classifications remain authoritative after confirmed cleanup: memory excess and wall expiry stay distinct, ambiguous live-worker RSS refuses, and ordinary completed/refused/falsified reports retain their meanings. Native refinement uses this ownership path when its FR-028 AC-24 typed entry is implemented, with unchanged refinement class/evidence rules. | Test |
| FR-034-AC-20 | Guardian connection, startup, identity verification and Dispatch use the original monotonic identity deadline without resetting it. An already expired deadline causes no Dispatch and settles inconclusive with the timed-out reason as FR-028 AC-2 states; actual identity-deadline expiry during setup or execution uses the same classification. | Test |
| FR-034-AC-21 | Guardian connection and handshake have a finite setup cap within the remaining original deadline. Cap expiry while that deadline remains live is a typed setup refusal, distinct from identity-deadline timeout. | Test |
| FR-034-AC-22 | Control/capture shutdown and cleanup observation waits have finite bounds. Unconfirmed termination refuses with a live caller rather than accepting a proof or claiming physical disappearance of an uninterruptible task. | Test |
| FR-034-AC-23 | Caller-death fixtures invoke the real matching package guardian from the owning target directory. Feature-on fixtures select shared private prefixes as data at BeforeMonitor/NoInit, Bootstrap/InitUnclaimed, ClaimedGated, ClaimedBootstrap and InitReady before exact-boundary caller/group self-kill. Bootstrap transfers one actual monitor pin and Tests only its stage, caller/monitor death, closed gate/lease ownership and no marker; separate source/lifecycle Analysis remains required for its unchanged INIT-cleanup guarantee, never inferred from monitor termination. Claimed prefixes transfer one validated INIT pin and require actual INIT termination; NoInit fabricates none. Feature-off public fixtures use a positive production backend marker for immediate post-Dispatch death. No production bypass, sleep-based success or host-scan authority participates. | Test, Analysis |
| FR-034-AC-24 | Removing lease-EOF cancellation fails pre-Dispatch closed-lease pending-authorization and post-Dispatch surviving-descendant assertions driven by production close_lease_and_observe. Its consumed CallerLease closes independently of live RunOwner monitor/INIT handles. The guardian-test-support fixture operation seals raw LeaseCloseObservation and owned worker/marker observations at the real private boundary before unconditional production cleanup or emergency cleanup. The test harness requires pre-Dispatch confirmed INIT termination with no marker, or post-Dispatch confirmed INIT termination with a dead pinned positively acknowledged worker, all observed before escalation; escalation-required is a failed EOF-cancellation oracle even if later cleanup kills the worker. Pending authorization is ordered by fixture-owned pidfd SIGSTOP and positive T state, queued Dispatch, actual lease closure with unconditional private LeaseClosing publication, then owned continuation SIGCONT. SIGCONT never satisfies the EOF predicate. The sealed publication snapshot must contain an actual completed-close ordinal strictly below its publication ordinal; absent/inverted order fails independent of continuation scheduling. Publication-before-close, missing-publication and skipped-lease-close mutants must fail their named ordering/observation/EOF predicates. The unchanged production frame-send step queues bounded nonblocking Dispatch separately from ACK waiting; fixture-written frames are forbidden. Separate mutants remove positive Dispatch, replace PID1 with a non-INIT watcher, remove session isolation and break startup close/reap ordering. Restored controls pass. | Test |
| FR-034-AC-25 | Before backend Dispatch, the helper handshake matches the actual running CG library's build, protocol and lifecycle-capability identity against the actual invoked first-party executable. A stale helper or changed lifecycle implementation refuses, even if a caller supplies a matching version label or digest. Expected identity derives from actual library/helper build artifacts, not caller assertions or manually maintained tracking pins. | Test |
| FR-034-AC-26 | The guardian's host session and process group are distinct from the original caller's before Ready and backend Dispatch, with no controlling-terminal job-control delivery from that caller's session. Feature-on exact-prefix fixtures kill the caller's whole group at pre-Ready and InitReady; feature-off public fixtures kill it immediately after positive post-Dispatch startup. Pre-session gated INIT remains owned by the startup group; after isolation guardian lease cleanup remains operational. A directly killed guardian still triggers kernel namespace teardown. | Test |
| FR-034-AC-27 | guardian-test-support is off by default, absent from default features, and exposes exactly one documented fixture operation only when explicitly enabled. A feature-off consumer cannot use that operation. No public lease, process-ownership handle, cancellation entry or cleanup-deferring callback is exported. The private initialized death witness transfers exactly one actual owned monitor or validated INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup stdout is safely duplicated into a non-stdio CLOEXEC OwnedFd and itself marked CLOEXEC before any spawn; every child stdio/mapping excludes both report descriptors. | Test, Inspection |
| FR-034-AC-28 | The live-caller fixture seals raw stage/lease-close and pinned-worker/marker observations before immediate unchanged cleanup on success, refusal and observation failure. The intentional-death form seals/transfers its exact-prefix typed witness before self-kill and does not return; the surviving harness judges it before separate emergency cleanup. One shared all-stages/prefix-as-data sequence invokes the same functions/order. BeforeMonitor/Bootstrap/ClaimedGated/ClaimedBootstrap/InitReady/LeaseClosing publication is unconditional, monotonic and read-only, adding no production I/O, callback, pause or feature branch. LeaseClosing seals actual close/publication ordinals without late fill and follows actual close. Lost/overflowed observations, report-inheritance exclusion failure and failed coordination are typed failures. No library oracle, inferred Bootstrap INIT death or later cleanup masks either harness predicate. | Test, Inspection |
| FR-034-AC-29 | The packaged caller fixture and real helper link the same normal library artifact through consumer-manifest package selection with matching target/profile/features/compiler flags. Separate named feature-off/on invocations avoid self dev-dependency feature unification. No cfg-test library or identity override is accepted. The bounded executor refuses feature mismatch in both directions before Dispatch; feature-off consumer compilation verifies absence of the fixture operation. | Test |
| FR-034-AC-30 | CG publishes the test-only feature contract and allocates production-driver dependency-edge exclusion to IR-649's QSL driver work. The contract requires all downstream production-build profiles to reject direct or transitively unified guardian-test-support. CG inspection verifies the published allocation and checks; downstream assertion evidence is owned by IR-649. | Inspection |

## Dependencies

- [FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 owns the every-run tree memory mechanism;
  AC-2/3 own resource outcomes, and AC-24 owns bounded native refinement. This requirement adds
  independent original-caller lifecycle ownership, not another resource model.
- [FR-017](./FR-017-kani-execution-evidence.md) owns backend reports, capture and batching.
- Downstream [IR-649](https://linear.app/agent-ix/issue/IR-649) owns QSL production-driver
  dependency-edge exclusion after IR-639 delivers this contract; it is not a CG acceptance test or
  an upstream prerequisite for the observation export.
- [TC-049](../matrix/TC-049-caller-death-ownership.md) describes planned production verification.
  The preceding containment code slice (PR #295) is merged; guardian CODE remains planned,
  independently of completion of all parent IR-241 work. Criterion-level implementation order is
  FR-017 launcher → FR-028 AC-21 containment slice → FR-034 guardian ownership → FR-028 AC-24 native
  refinement; no whole-requirement or parent-ticket cycle is introduced.

The lifecycle claim includes signals directed to the original caller's process group/session and
direct guardian death. Host failure or loss of the kernel's namespace facilities cannot be turned
into a confirmed cleanup observation. Such unavailable observation with a live caller refuses; it
does not waive caller-group or guardian startup death protection.

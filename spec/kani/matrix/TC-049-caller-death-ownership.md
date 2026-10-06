---
id: TC-049
title: "Verify original-caller death cannot release an unowned backend"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-049: Verify original-caller death cannot release an unowned backend

## Description

Planned production scenarios for [FR-034](../functional/FR-034-caller-death-ownership.md). Guardian
CODE builds on the merged containment code slice (PR #295) and remains planned; this amendment does
not require completion of all parent IR-241 work. No executable coverage of the new allocation is
claimed. Use the real
Cargo guardian as namespace PID 1, the real bounded executor, and explicit helper configuration as a
library consumer. Tiny research probes are not production fixtures. AC-23/26 exact early-stage
caller-death scenarios use the opt-in fixture operation with shared
private sequence prefixes; positive post-Dispatch scenarios use the ordinary feature-off public API.
The mandatory AC-24 private lease-EOF oracle uses that same documented opt-in fixture operation.
[TC-039](./TC-039-bounded-proof-ceilings.md) retains its broader ceiling/refinement scenarios.

The merged PR #295 recipe fails the actual post-clone/pre-`child_wait` monitor-death boundary;
IR-652 specifies outer kernel containment and bounded unnamed report storage to repair it.
Nine scratch cases do not cover actual original-caller death/exclusive lease, exact internal bwrap
boundary, production observer/deadline, cargo report inheritance or all-owner storage release.
AC-31 through AC-34 remain mandatory UNRUN CODE gates, with no implementation coverage claimed.

## Test Procedure

1. Use the separate named guardian-feature-off Cargo invocation to build the normal package library
   and real `quire-kani-guardian` from matching source/build inputs in the owning target directory.
   Do not enable the feature through a self dev-dependency. Supply the required explicit helper path
   to the original-caller fixture using the ordinary public API. Keep that caller in its own
   positively verified dedicated session/group. Observe a positive production backend startup marker
   and pin its actual escaped worker before immediate post-Dispatch caller death; no sleep or host
   scan establishes success. Feature-on early-stage cases require the matched build from step 12
   before invoking the single operation, not unavailable private reads from the feature-off API.
2. In separate feature-on cases select shared-sequence prefixes as data at BeforeMonitor, Bootstrap
   immediately after actual monitor spawn and before any INIT claim or gate release, ClaimedGated,
   ClaimedBootstrap and InitReady. Production traverses the same functions/order: verify armed outer
   INIT before spawning monitor,
   claim gated inner INIT and bind observer, release gate, authenticate Ready. Require unconditional
   monotonic read-only publication, no copied orchestrator, feature branch, hook or production
   report I/O. Map the harness-created anonymous socketpair only into fixture startup stdout; safely
   duplicate stdout as a >=3 CLOEXEC OwnedFd auxiliary and mark original stdout CLOEXEC BEFORE any
   spawn. Use only auxiliary for witness reporting and configure every child's stdio/mapping
   explicitly to exclude both report descriptors. At the exact prefix seal actual published stage,
   caller/owned monitor/claimed INIT start and namespace identities and transferred descriptor
   identity. Bootstrap carries ONE clone of the actual owned monitor pidfd and typed InitUnclaimed:
   INIT may exist, but no claim or gate release occurred; fabricate no INIT pin. ClaimedGated and
   later instead carry ONE cloned validated INIT pidfd. BeforeMonitor carries typed
   NoInit/no-spawn/setup facts and no rights. After complete bounded send, self-SIGKILL the original
   caller or its positively owned dedicated group before the next transition; no ACK/controller
   pause. Authenticate sender credentials against the harness's actual unreaped caller Child/start/
   pidfd and match received safe CLOEXEC descriptor identity and pin kind to the sealed original.
   Reject wrong sender/type, unknown/truncated control or extra/missing rights. Test claimed-INIT
   receipt both live and already dead; verify live identity when available and retain the
   transferred validated pin after death, never reopen a reused PID. ClaimedGated and later require
   confirmed caller/INIT death and no backend marker. NoInit requires actual no INIT/marker, closed
   actual lease/pair and termination of any other positively owned setup child. At Bootstrap instead
   require sealed monitor-created/no-claim/gate-held/no-Dispatch facts, caller and pinned monitor
   death, closed original-caller gate/lease ownership and no backend marker under bounded
   observation. Do not infer guardian EOF exit or INIT death from monitor readiness or pipe EOF.
   Record the present PR #295 monitor-PDEATH/internal-child_wait leak as the named IR-652 gap; these
   facts do not measure INIT teardown or satisfy its repair. Separately analyze the unchanged
   cleanup guarantee under IR-652 through actual trusted gate-EOF bootstrap, exclusive lease loss,
   bounded guardian refusal and namespace-INIT teardown, including INIT creation, internal map/setup
   handoff before the public gate and actual parent-death installation order. Do not assume monitor
   death releases that internal wait. Held gate does not establish a stable descendant snapshot
   because bwrap may fork during setup; no after-death PID reopen/host scan supplies missing INIT
   authority. This is stage-only Test plus separately required Analysis, not full namespace-teardown
   coverage. ClaimedGated positively retains gate; ClaimedBootstrap does not label its released gate
   retained. Record failure before independent owned emergency cleanup. Missing stage/pin, partial
   send or failed exclusion/coordination fails and invokes unchanged cleanup while caller lives,
   within the original deadline. Repeat feature-off public post-Dispatch caller/group SIGKILL with
   step 1's positive marker. Retain abort and forced process-kill OOM-model cases without host
   memory pressure. Absent/closed lease and connection cap cause bounded helper refusal; gate EOF
   never authorizes production Dispatch. Verify actual SID/PGID isolation before Ready/Dispatch and
   isolated guardian lease cleanup after caller-group death; pre-session gated INIT remains
   startup-group owned. The successful self-kill operation cannot return or fabricate a dead caller
   result. The surviving harness judges raw witness/pins/marker before its own emergency cleanup.
   The queued-pidfd probe establishes only the Linux descriptor facility; actual caller Rust stage
   coverage, reporter inheritance exclusion, positive INIT tests and Bootstrap lifecycle
   repair/Analysis under IR-652 remain CODE gates.
3. Kill the actual guardian INIT through its pinned pidfd before its peer/Ready claim, in InitReady,
   and immediately after Dispatch. Require no pre-Dispatch backend marker and kernel cancellation of
   post-Dispatch descendants. With a live original caller, even confirmed teardown beside valid
   success output must produce the typed guardian-failure refusal. Do not model the helper as a
   fictional direct bounded executor Child.
4. Substitute another actual sender/control capability and require authority rejection. Alter the
   monitor/INIT start identity, parent, namespace identity and namespace PID independently; provide
   incomplete startup data and a positively dead INIT. Check refusal before Dispatch. Verify kernel
   Ready sender PID equals the claimed INIT chained to the actual unreaped monitor, and observer
   readiness precedes Dispatch. Substitute a stale helper built with different lifecycle
   capabilities and source/build inputs, even with a caller-supplied matching version or digest
   label. Require mismatch against the actual running library's identity and actual invoked
   executable, not a path inspected before replacement.
5. Before M creation, positively verify O actual outer PID 1, private proc/mount identity, mapping,
   parent-death arm and original-exclusive bootstrap authority. Fail each capability separately:
   require typed refusal before M, without host mount/profile/sysctl changes. Kill L before O arm;
   require no inner child and bounded exclusive EOF refusal. Kill L after arm and O during M spawn,
   gated I and live descendants; require actual owned outer pin termination and kernel whole-tree
   cancellation. Fail/malformed/overlimit inner info and reparent M children before claim: outer
   authority cancels without host scan or fabricated inner pin. Exercise actual original C death
   at the precise installed bwrap post-clone/pre-child_wait boundary and natural M/info-reader loss.
   Require the old merged PR #295 control to FAIL this named teardown assertion, then the new
   production allocation to pass; public gate timing or elapsed sleeps do not identify that window.
6. After Dispatch, create double-fork/setsid descendants which reparent before observation,
   positively acknowledged descendants born during cancellation, and nested PID namespaces where the
   required facilities permit them. Kill original caller or guardian separately and verify all owned
   descendants terminate through INIT teardown, including unsampled ones. Run a second independent
   namespace and unrelated fixture child beside the target; they stay live until their own cleanup.
   Record unsupported platform refusal rather than a passing skip.
7. Exercise whole-batch ownership with multiple FR-017 harness members in one launcher. Verify one
   guardian, monitor, namespace and lease own the entire group under its existing memory ceiling and
   multiplied wall deadline. Normal completion with adopted background work, nonzero/signal exit,
   exec failure, explicit cancellation, timeout, memory excess, capture overflow/read failure and
   observation failure all require confirmed teardown before outcomes. Close backend streams and
   fail the helper separately; require bounded capture settlement, appropriate typed refusal, no
   proof accepted from success output beside unconfirmed cleanup, monitor reaping with a live
   caller, and surviving-owner cleanup of other assigned artifacts after caller loss or helper
   failure. The
   report expectation uses FR-017's unnamed bounded pipe-to-memfd storage, not guardian Drop/unlink.
   Kill all actual report owners and verify final kernel descriptor/backing release without any
   persistent report pathname. Other assigned artifacts retain truthful surviving-owner cleanup
   and all-owner-death limits. No process-only fixture or emergency deletion proves report lifetime.
8. Supply a missing, non-executable or unusable explicit helper path. Require typed refusal and no
   backend marker, with no PATH/global discovery, copied fixture binary or alternate launcher.
   Inspect setup docs for deliberate matching library/helper delivery, and the Linux, procfs
   children/RSS and pidfd prerequisites. Distinguish merged containment user/PID namespace,
   die-with-parent/info/gate flags from planned guardian PID-1/new-session prerequisites.
9. Create the private anonymous connected pair with exclusive original-caller ownership. Verify the
   guardian's mapped creator UID and the host's kernel Ready sender PID against actual INIT; do not
   equate guardian-side PID0 or creator credentials to actual guardian sender identity. Kill the
   original caller before lease authentication. Have foreign same- and different-UID fixture
   processes bind publicly discoverable names and supply plausible live lease/Dispatch; the guardian
   never connects to them, and no backend marker appears. Try another pair, mismatched mapped UID,
   replayed control and leaked lease rights; each refuses. Pair creation failure is setup refusal
   with no listener/path fallback. Send malformed, unknown-field, oversized and excessive pending
   control records, plus unknown, extra/missing, wrong-type and truncated ancillary
   rights/credentials; received rights must be CLOEXEC and closed on refusal. Close the lease.
   Require bounded cancellation/refusal, never Dispatch from EOF or bootstrap-gate bytes. Race an
   unrelated exec and verify it and backend descendants inherit neither a caller lease endpoint nor
   bootstrap-control writer. In the feature-on fixture verify BOTH report descriptors are absent
   from the real monitor, gated INIT, execed guardian, backend/descendants and unrelated exec; check
   the actual report socket identity, not only its fd number. Require the harness report stream's
   EOF after fixture death without waiting for a leaked child writer. Explicit child stdio must
   never restore reporter inheritance or use the report as capture. Positively inspect both actual
   reporter descriptors' CLOEXEC flags before the first spawn. Remove each flag independently and
   require its named pre-spawn FD-flag assertion to fail, even if another exclusion defense prevents
   a leak. Mutate child stdio/mapping reporter exclusion separately and require its
   inherited-reporter or extra-writer/EOF assertion to fail before emergency cleanup closes leaked
   copies. Restore controls. Inspect safe standard-stdout AsFd duplication to OwnedFd auxiliary,
   original CLOEXEC marking BEFORE spawn and every child stdio/mapping; no arbitrary inherited-FD
   adoption or unsafe exception participates.
10. Run a backend echoing raw non-UTF8 argv, stdin, overridden/inherited environment and cwd, plus
    distinct stdout/stderr markers. Verify exact recipe/capture separation. Feed actual ordinary
    completed/refused/falsified reports, wall/memory stops and ambiguous live-worker RSS through the
    existing classifiers. When FR-028 AC-24 native refinement is implemented, repeat its
    resource-stop control through this ownership path and assert the original class/evidence.
11. Distinguish an already expired original deadline, expiry during handshake, and helper-cap expiry
    while that deadline remains live. The first causes no Dispatch and inconclusive with the
    timed-out reason; actual deadline expiry during setup has the same classification; helper-cap
    expiry is typed setup refusal. Verify no deadline reset, bounded shutdown and observation, and
    refusal on unconfirmed teardown without a physical-disappearance claim.
12. Before using the operation, build the normal library, packaged caller fixture and real helper
    with guardian-test-support enabled through the separate named guardian-feature-on Cargo
    invocation. Select the package helper from the consumer manifest with cargo -p and match
    target/profile/features/flags, verifying actual library/helper identity. This is separate from
    step 1's feature-off invocation, with no cfg-test library or self dev-dependency unification.
    First remove guardian lease-EOF cancellation. In the live original fixture caller invoke the
    single `guardian-test-support` fixture operation around the real private production
    `close_lease_and_observe` cancellation boundary after Dispatch. It consumes only `CallerLease`;
    retain `RunOwner`'s unreaped monitor and claimed INIT handles and an escaped worker's positive
    handshake/pidfd. Observe its actual bounded LeaseClosing phase, then capture its typed
    `LeaseCloseObservation` and the worker's pinned state as immutable raw facts before immediately
    and unconditionally invoking production escalation/cleanup. The test harness requires confirmed
    INIT termination and a dead positively acknowledged pinned worker in those pre-escalation raw
    observations; the ignored-EOF mutant records escalation-required with live INIT/worker and fails
    the harness-owned surviving-descendant predicate. Later successful escalation cannot turn that
    record into a passing EOF-cancellation oracle. Keep the original caller alive: no parent-death
    kill, resource deadline or controller INIT signal establishes this observation. Choose an
    original deadline with remaining observation budget; expiry or unavailable observation is a
    failed/inconclusive fixture, never a passing mutant result. No sleep or elapsed-time threshold
    establishes success. Before Dispatch, the fixture operation itself sends SIGSTOP through its
    claimed INIT pidfd after InitReady, positively verifies state T plus unchanged start/namespace
    identity, and invokes the unchanged private production Dispatch frame-send step while retaining
    live original-caller RunOwner ownership. Require the complete actual production frame and rights
    queued through its bounded nonblocking transport, not fixture-written serialization. The typed
    pending-frame result is separate from ACK wait; do not wait for stopped INIT acknowledgement.
    Partial-send failure, would-block beyond the original bounded budget or unavailable transport
    must record coordination failure and cleanup, never establish a passing fixture. Invoke
    unchanged synchronous close_lease_and_observe. Its actual lease close precedes unconditional
    private monotonic read-only LeaseClosing publication; at actual publication seal the optional
    actual-close completion ordinal and publication ordinal in one immutable per-run snapshot. An
    internal owned continuation thread reads that snapshot and sends SIGCONT through only that
    pinned pidfd. The harness requires a present completed-close ordinal strictly below publication;
    later closure cannot fill an earlier snapshot. This raw ordering assertion fails early
    publication independently of which thread is scheduled first, even when guardian termination and
    the marker oracle otherwise pass. Publication adds no production-stage branch, callback,
    blocking handoff or extra I/O. The fixture-owned continuation uses no external-controller
    permission. Capture publication/coordination facts and raw termination/marker observations
    before cancellation escalation. The correct guardian confirms INIT termination without a backend
    marker; ignored EOF fails the closed-lease authorization predicate or records
    escalation-required. The external harness evaluates these sealed pre-escalation facts only after
    unconditional cleanup returns. SIGCONT itself cannot satisfy the EOF predicate. Require named
    assertions to fail when publication precedes actual close, publication is removed, or actual
    lease close is skipped. Evaluate the retained early-publication snapshot after the later close
    and cleanup complete; its original missing/inverted close ordinal must remain unchanged and fail
    the ordering assertion. No scheduling restriction is needed to distinguish it from
    close-before-publication. Neither SIGCONT nor valid-looking ordinals alone satisfy the actual
    INIT/marker/worker predicate. A missing publication or failed owned continuation records
    coordination failure, not a passing platform skip, and cleanup joins/resumes or cancels only the
    fixture's pinned INIT. This checks observable EOF precedence, not future-death prediction. The
    normal production driver immediately follows this same typed observation boundary with bounded
    cleanup; the fixture uses the opt-in observation operation at that boundary without a cfg-test
    close hook, synthetic ownership, exported run handles or controller-blocked escalation. After
    the operation returns following unconditional cleanup, evaluate the AC-24 predicate from the
    immutable pre-escalation raw record, separately from the returned cleanup result; the library
    evaluates no pass/fail oracle. Never infer the predicate from the eventually dead worker.
    Independently remove positive Dispatch authorization, replace the PID-1 guardian with a non-INIT
    watcher, remove session isolation, and close/reap before pinned startup confirmation. Require
    the corresponding premature-backend-marker, surviving-descendant, actual SID/PGID or
    startup-order assertion to fail, rather than compilation or fixture setup. Record the ownership
    observation before emergency cleanup of only the fixture's pinned namespace/group. Restore
    production and require focused controls to pass. Trace each actual asserted AC.
13. Use the actual matched artifacts from steps 1 and 12 to test both feature-mismatch directions.
    Supply the feature-off helper to the feature-on caller, then the feature-on helper to the normal
    feature-off production caller. The bounded executor must refuse actual artifact identity before
    backend Dispatch in both cases; no supplied epoch/version/digest may bless a pair. Build a
    normal feature-off consumer in its separate invocation and require a compile-fail check
    specifically for the absent fixture operation. Run ordinary public caller-death controls for
    positive post-Dispatch death against the actual feature-off library. Exact early-stage controls
    use the feature-on shared-prefix operation; a feature-on library avoiding its operation cannot
    replace the feature-off controls. Inspect exactly one documented feature-on fixture operation
    and the absence of public lease, process-ownership, cancellation or cleanup-deferring callback
    exports. Inspect shared private production stage/cleanup paths for one shared sequence and
    identical functions/order selected as all-stages or prefix data, and unconditional read-only
    stage publication. Only the single fixture item may be feature-gated; no production feature
    branch or copied orchestrator may implement early-stage tests. Keep this structural evidence
    alongside actual runtime observations and the mandatory AC-24 mutants. Record structural
    inspection separately from runtime/compile Tests; no feature-dependent stage bypass or
    resource/identity weakening is permitted.
14. Through the feature-on operation, fail stage observation, overflow bounded observation storage
    and fail owned stop/T-state/publication/resume coordination separately. Verify publication
    cannot precede actual lease close and that SIGCONT alone yields no passing EOF predicate. Apply
    the missing-publication and skipped-close mutants beside the primary ignored-EOF mutant; each
    must fail its named raw-observation/EOF assertion before cleanup can mask the recorded state.
    Each raw result records failure/unavailability and cleanup still executes. The harness must
    reject these records as passing AC-24 evidence; retain pinned INIT/monitor observations for
    confirmed cleanup or typed unconfirmed refusal. Inspect CG's test-only publication and
    allocation of downstream production-feature exclusion to
    [IR-649](https://linear.app/agent-ix/issue/IR-649), owned by the QSL driver lane. CG inspection
    passes only the published contract; no CG-only runtime check satisfies the downstream gate.
    IR-649 must separately verify the driver's dependency edges for all its production-build
    profiles, reject direct and transitive feature unification, and pass the feature-off controls.
    Keep that gate pending until its owner delivers actual driver evidence.

15. Run FR-017 AC-19's actual cargo/Kani 0.68 descriptor export, hard writer cap, concurrent pipe/
    kernel-backing accounting, slow collector, actual EOF/all writer copies, sealing, final handoff
    and all-owner death scenarios. O concurrently services controls/report/deadline/accounting; no
    completion wait cycle or deadline reset is allowed. Confirm overflow yields single-run
    KaniRunOutcome::Inconclusive with MemoryExhausted; batches keep whole-batch memory-exhausted
    refusal with no member classified, while FR-029 maps ResourceExhausted, never Failed or
    truncated acceptance. Pipe incompatibility stops CODE and requires measured spec revision; never
    select another storage mode at runtime.

16. Positively validate actual host/outer/inner pidfd identity translation, original caller-exclusive
    lease liveness immediately before
    spawn/Dispatch, credential changes/rearm, real observer and original ceiling/deadline across
    every role. C/group, L, O and I death each has an owned bounded settlement path; actual C death
    must not be replaced by controller bootstrap EOF. During live-C AC-24 observation retain O/L
    handles and separate report control, withholding outer escalation until immutable raw
    observations are sealed. Require the ignored-EOF mutant to fail unchanged. The nine successful
    scratch cases cover feasibility only; these production gates are UNRUN.

17. Measure actual ordinary-caller inherited profile/unshare/mount permission independently of the
    codex-node scratch label; record typed refusal where bwrap-only execution formerly succeeded.
    Keep the actual C spawning thread alive through L settlement; exit it while C lives and require
    owned cleanup and typed ownership failure, never successful evidence. Verify O closes its report
    spawn writer, C/L inherit none, and M termination closes any inherited writer before final EOF.
    Inspect the exact nested flags and fd3/fd4/N mapping/owners. Attempt pipe growth past reserved
    capacity; missing enforceable bound gives capability refusal before Dispatch. Apply each missing
    consumer F_GET_SEALS seal and require refusal before read. These tests remain UNRUN.

## Expected Results

| Authority | Required observation | Regression caught |
|---|---|---|
| FR-034-AC-1/2/3/7 | Trusted bootstrap is distinct from production Dispatch; verified outer authority covers unclaimed inner stages; inner claim precedes Dispatch | EOF starts subject; signal/monitor exit falsely confirms teardown; claim-to-Dispatch gap |
| FR-034-AC-4/5/6 | Exclusive original-caller pair and actual kernel Ready sender/INIT chain precede Dispatch | Public rendezvous capture; wrong pair or creator/sender credentials; arbitrary parent |
| FR-034-AC-8/9 | Kernel INIT teardown cancels escaped, late-born and nested descendants, preserving other runs | Kill only sampled PIDs; cancel another run |
| FR-034-AC-10/11/12/26 | Later caller-group/guardian death confirms namespace teardown and lease closure; live-caller helper failure refuses; unnamed report backing ends on final close and exact pre-handoff teardown is independently tested; process-only success establishes no storage claim | External sole gate owner dies; accept proof after helper failure; leak helper/socket |
| FR-034-AC-13/14/25 | Explicit real package helper matches actual running library build/capabilities; setup docs and missing-helper refusal are accurate | PATH/copy fallback; stale helper blessed by caller label |
| FR-034-AC-15/16 | Invalid bounded controls refuse; exclusive lease and safe child-only mapping | EOF authorizes Dispatch; descendants inherit caller lease |
| FR-034-AC-17/18/19; FR-028-AC-2/3/21/24; FR-017-AC-14/24/25 | Whole-batch ownership, exact recipe, separate captures and existing resource/refinement outcomes | One helper per member; diagnostics become report; weaken ceilings; ambiguous RSS becomes zero |
| FR-034-AC-20/21/22 | Original deadline and expired-deadline outcome persist; setup cap distinct; bounded observation refuses ambiguity | Reset deadline; setup refusal falsely timed out; hang capture |
| FR-034-AC-23/24 | Real helper, exact preclaim Bootstrap stage facts distinguished from separate cleanup Analysis, positive claimed-INIT pins and production typed lease-close boundary before INIT escalation | Ignored EOF keeps worker or accepts closed-lease Dispatch; emergency teardown masks failure |
| FR-034-AC-27/28/29/30 | Opt-in single observation operation, immutable pre-escalation raw facts and harness predicate, unconditional cleanup, same normal artifacts and CG publication and separately owned driver exclusion | Default export; controller pause; changed production stage; false oracle after cleanup; cfg-test epoch override; feature enabled in production |
| FR-034-AC-31 | Actual outer pin and whole-tree termination at exact bwrap handoff failure; ordinary caller capabilities and creating-thread lifetime observed | Old295 orphaned unclaimed INIT; profile-only availability assumption; parent-thread exit misclassified as success |
| FR-034-AC-32 | Hard writer retention bound and defined conservative charge; actual Kani pipe roundtrip; slow/overflow cases stop under original deadline | Unmapped backing counted as zero; resizable pipe exceeds reservation; backend/collector deadlock; report cap becomes Failed or truncated pass |
| FR-034-AC-33 | O spawn writer closed, M and inner writers terminated, actual EOF, verified four seals and final descriptor read; final-close backing reclamation | Extra monitor writer prevents EOF; forged seal claim; report residue after all owners die |
| FR-034-AC-34 | Separate lease/report controls and live C retained outer ownership; ignored-EOF fails before outer escalation | Outer kill masks mandatory EOF mutant; deadline/ceiling reset or missing charge accepted |

AC-31 through AC-34 and rewritten FR-017 AC-19 are UNRUN and must compute untagged until actual
production assertions are implemented. Scenario prose or research probes establish no executable
coverage. Native refinement remains planned until its actual typed entry is delivered. Host
destruction and uninterruptible tasks cannot justify fabricated teardown; live-caller unavailable
confirmation refuses. Caller-group signals and direct guardian death are included lifecycle cases,
not excluded double faults.
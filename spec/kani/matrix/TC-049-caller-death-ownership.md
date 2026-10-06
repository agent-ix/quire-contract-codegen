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
not require completion of all parent IR-241 work. No executable coverage is claimed. Use the real
Cargo guardian as namespace PID 1, the real bounded executor, and explicit helper configuration as a
library consumer. Tiny research probes are not production fixtures. AC-23/26 caller-death scenarios
use the ordinary public execution API without `guardian-test-support`; the mandatory AC-24 private
lease-EOF oracle uses the separate documented opt-in fixture operation.
[TC-039](./TC-039-bounded-proof-ceilings.md) retains its broader ceiling/refinement scenarios.

## Test Procedure

1. Use the separate named guardian-feature-off Cargo invocation to build the package library and
   real `quire-kani-guardian` from the same source/build inputs in the owning target directory with
   guardian-test-support compiled off. Do not enable it through a self dev-dependency; AC-23/26 run
   against this normal feature-off library. Supply the required explicit helper executable path to a
   separate original-caller fixture. Keep that caller in its own positively verified fixture
   session/group so group signals reach no other work. Retain controller-owned monitor/INIT
   identities and pidfds. Use observable production barriers at Bootstrap before claim,
   ClaimedBootstrap before lease connection, InitReady before Dispatch and immediately after
   Dispatch; no production bypass, sleeps or host-wide scan establishes a successful assertion.
2. At each barrier, kill only the original fixture caller, then its whole fixture process group in a
   separate run. Use SIGKILL, abort and a forced process-kill model of OOM without host memory
   pressure. Before Dispatch no production backend marker appears, even if gate EOF starts guardian
   bootstrap. Absent/closed lease and connection deadline cause bounded helper exit and artifact
   cleanup. After session separation, observe the guardian remains outside caller job-control/hangup
   delivery and handles lease loss. Verify its SID/PGID isolation at Ready and retain the positive
   group-death cleanup observation.
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
5. While the bootstrap gate is retained, inject partial/malformed/overlimit info, monitor exit and
   setup failure. Recover exact INIT through bounded info or direct owned-monitor task children with
   actual parent/start/namespace checks. Exact pidfd termination takes precedence; no wider
   membership scan is required. Reparent before recovery, make owned data unavailable, and change
   start identity separately. If recovery is impossible, require pinned-group signal before
   close/reap, exclusive lease closure and typed unconfirmed-cleanup refusal, never a passing claim
   based on kill success, monitor exit or a host-wide scan. Verify no production backend marker even
   if gate EOF starts guardian bootstrap after this refusal.
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
   caller, and surviving-owner temporary artifact cleanup after caller loss or helper failure.
   Verify the anonymous pair ends on final close without a filesystem socket artifact; bootstrap
   creates no persistent private files before the lease. Kill both cleanup owners separately to
   confirm kernel process containment without falsely claiming that dead actors remove remaining
   temporary reports.
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
   bootstrap-control writer. Inspect safe CLOEXEC child-only mapping alongside runtime inheritance
   checks; no raw inherited-FD adoption or unsafe exception participates.
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
    specifically for the absent fixture operation. Run ordinary public caller-death controls against
    the actual feature-off library, not a feature-on library which simply avoids calling the
    operation. Inspect exactly one documented feature-on fixture operation and the absence of public
    lease, process-ownership, cancellation or cleanup-deferring callback exports. Inspect shared
    private production stage/cleanup paths for unconditional behavior alongside runtime
    observations. Record structural inspection separately from runtime/compile Tests; no
    feature-dependent stage bypass or resource/identity weakening is permitted.
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

## Expected Results

| Authority | Required observation | Regression caught |
|---|---|---|
| FR-034-AC-1/2/3/7 | Trusted bootstrap is distinct from production Dispatch; exact claimed-pidfd termination or truthful unrecoverable-identity refusal covers every stage | EOF starts subject; signal/monitor exit falsely confirms teardown; claim-to-Dispatch gap |
| FR-034-AC-4/5/6 | Exclusive original-caller pair and actual kernel Ready sender/INIT chain precede Dispatch | Public rendezvous capture; wrong pair or creator/sender credentials; arbitrary parent |
| FR-034-AC-8/9 | Kernel INIT teardown cancels escaped, late-born and nested descendants, preserving other runs | Kill only sampled PIDs; cancel another run |
| FR-034-AC-10/11/12/26 | Caller-group death preserves isolated guardian cleanup; guardian death kills namespace; live-caller helper failure always refuses; monitor/artifact cleanup confirmed | External sole gate owner dies; accept proof after helper failure; leak helper/socket |
| FR-034-AC-13/14/25 | Explicit real package helper matches actual running library build/capabilities; setup docs and missing-helper refusal are accurate | PATH/copy fallback; stale helper blessed by caller label |
| FR-034-AC-15/16 | Invalid bounded controls refuse; exclusive lease and safe child-only mapping | EOF authorizes Dispatch; descendants inherit caller lease |
| FR-034-AC-17/18/19; FR-028-AC-2/3/21/24; FR-017-AC-14/24/25 | Whole-batch ownership, exact recipe, separate captures and existing resource/refinement outcomes | One helper per member; diagnostics become report; weaken ceilings; ambiguous RSS becomes zero |
| FR-034-AC-20/21/22 | Original deadline and expired-deadline outcome persist; setup cap distinct; bounded observation refuses ambiguity | Reset deadline; setup refusal falsely timed out; hang capture |
| FR-034-AC-23/24 | Real helper, positive observations and production typed lease-close boundary before INIT escalation | Ignored EOF keeps worker or accepts closed-lease Dispatch; emergency teardown masks failure |
| FR-034-AC-27/28/29/30 | Opt-in single observation operation, immutable pre-escalation raw facts and harness predicate, unconditional cleanup, same normal artifacts and CG publication and separately owned driver exclusion | Default export; controller pause; changed production stage; false oracle after cleanup; cfg-test epoch override; feature enabled in production |

Scenario prose or research probes establish no executable coverage. Native refinement remains
planned until its actual typed entry is delivered. Host destruction and uninterruptible tasks cannot
justify fabricated teardown; live-caller unavailable confirmation refuses. Caller-group signals and
direct guardian death are included lifecycle cases, not excluded double faults.

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

1. Without guardian-test-support, build the package library and real `quire-kani-guardian` from the
   same source/build inputs in the owning target directory. Supply the required explicit helper
   executable path to a separate original-caller fixture. Keep that caller in its own positively
   verified fixture session/group so group signals reach no other work. Retain controller-owned
   monitor/INIT identities and pidfds. Use observable production barriers at Bootstrap before claim,
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
   children/RSS, pidfd, bubblewrap PID-1/new-session/info/gate and namespace prerequisites.
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
12. First remove guardian lease-EOF cancellation. In the live original fixture caller invoke the
    single `guardian-test-support` fixture operation around the real private production
    `close_lease_and_observe` cancellation boundary after Dispatch. It consumes only `CallerLease`;
    retain `RunOwner`'s unreaped monitor and claimed INIT handles and an escaped worker's positive
    handshake/pidfd. Observe its actual bounded LeaseClosing phase, then capture its typed
    `LeaseCloseObservation` and the worker's pinned state and seal the named oracle before
    immediately and unconditionally invoking production escalation/cleanup. The correct guardian
    terminates INIT and the worker; the ignored-EOF mutant records escalation-required with live
    INIT/worker and fails the named surviving-descendant assertion. Later successful escalation
    cannot turn that record into a passing EOF-cancellation oracle. Keep the original caller alive:
    no parent-death kill, resource deadline or controller INIT signal establishes this observation.
    Choose an original deadline with remaining observation budget; expiry or unavailable observation
    is a failed/inconclusive fixture, never a passing mutant result. No sleep or elapsed-time
    threshold establishes success. Before Dispatch, queue an otherwise valid authorization behind a
    positively acknowledged production-stage barrier, invoke the same InitReady cancellation
    operation, and establish observable EOF before pending authorization continues. Correct guardian
    rejects it with no marker and confirmed termination. The ignored-EOF mutant fails the named
    closed-lease authorization/premature-marker assertion or records escalation-required instead of
    guardian termination; seal these observations and the failed oracle before independent INIT
    cleanup. This tests observable EOF precedence, not an atomic prediction of future caller death.
    The normal production driver immediately follows this same typed observation boundary with
    bounded cleanup; the fixture uses the opt-in observation operation at that boundary without a
    cfg-test close hook, synthetic ownership, exported run handles or controller-blocked escalation.
    Assert its immutable pre-cleanup oracle and the separately returned cleanup result; never infer
    the oracle from the eventually dead worker. Independently remove positive Dispatch
    authorization, replace the PID-1 guardian with a non-INIT watcher, remove session isolation, and
    close/reap before pinned startup confirmation. Require the corresponding
    premature-backend-marker, surviving-descendant, actual SID/PGID or startup-order assertion to
    fail, rather than compilation or fixture setup. Record the ownership observation before
    emergency cleanup of only the fixture's pinned namespace/group. Restore production and require
    focused controls to pass. Trace each actual asserted AC.
13. Explicitly enable `guardian-test-support` for the packaged caller fixture, actual CG library and
    real package helper using consumer-manifest `cargo -p` delivery. Match target, profile, features
    and compiler flags; verify actual artifact identity, not a supplied epoch or version label.
    Build a real helper without the feature and require identity refusal before backend Dispatch.
    Build a feature-off normal consumer and require a compile-fail check specifically for the absent
    fixture operation; its ordinary public-run caller-death controls must still run. Verify exactly
    one documented fixture operation is exported when enabled, with no cancellation or owned-handle
    export. Inspect unconditional shared production stage/cleanup paths beside runtime observations;
    feature selection must not bypass a stage, identity verification or resource ceiling.
14. Through that operation, fail stage observation, overflow its bounded observation storage and
    fail pending-Dispatch coordination separately. Each result is failed/inconclusive and cleanup
    still executes; retain pinned INIT/monitor observations to verify cleanup or typed unconfirmed
    refusal. Require bounded automatic coordination without controller release. Restore and rerun
    the AC-24 mutant controls. Verify test-only setup documentation and a production-driver
    dependency-edge assertion for every production profile, including transitive feature
    unification. A profile enabling the feature must fail that assertion; the normal feature-off
    profiles pass. Record unavailable driver verification as undelivered evidence, never a passing
    CG-only check.

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
| FR-034-AC-27/28/29/30 | Opt-in single observation operation, immutable pre-cleanup oracle, unconditional cleanup, same normal artifacts and production-driver exclusion | Default export; controller pause; changed production stage; false oracle after cleanup; cfg-test epoch override; feature enabled in production |

Scenario prose or research probes establish no executable coverage. Native refinement remains
planned until its actual typed entry is delivered. Host destruction and uninterruptible tasks cannot
justify fabricated teardown; live-caller unavailable confirmation refuses. Caller-group signals and
direct guardian death are included lifecycle cases, not excluded double faults.

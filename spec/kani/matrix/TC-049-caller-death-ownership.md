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

Planned production scenarios for [FR-034](../functional/FR-034-caller-death-ownership.md).
Guardian CODE is gated on the preceding containment code slice (PR #295); this SPEC may merge
independently and does not require all parent IR-241 work. No executable coverage is claimed.
Use the real Cargo guardian as namespace PID 1, the real bounded executor, and explicit helper
configuration as a library consumer. Tiny research probes are not production fixtures.
[TC-039](./TC-039-bounded-proof-ceilings.md) retains its broader ceiling/refinement scenarios.

## Test Procedure

1. Build the package library and real `quire-kani-guardian` from the same source/build inputs in
   the owning target directory. Supply the required explicit helper executable path to a
   separate original-caller fixture. Keep that caller in its own positively verified fixture
   session/group so group signals reach no other work. Retain controller-owned monitor/INIT
   identities and pidfds. Use observable production barriers at Bootstrap before claim,
   ClaimedBootstrap before lease connection, InitReady before Dispatch and immediately after
   Dispatch; no production bypass, sleeps or host-wide scan establishes a successful assertion.
2. At each barrier, kill only the original fixture caller, then its whole fixture process group
   in a separate run. Use SIGKILL, abort and a forced process-kill model of OOM without host
   memory pressure. Before Dispatch no production backend marker appears, even if gate EOF
   starts guardian bootstrap. Absent/closed lease and connection deadline cause bounded helper
   exit and artifact cleanup. After session separation, observe the guardian remains outside
   caller job-control/hangup delivery and handles lease loss. Verify its SID/PGID isolation at
   Ready and retain the positive group-death cleanup observation.
3. Kill the actual guardian INIT through its pinned pidfd before its peer/Ready claim, in
   InitReady, and immediately after Dispatch. Require no pre-Dispatch backend marker and kernel
   cancellation of post-Dispatch descendants. With a live original caller, even confirmed
   teardown beside valid success output must produce the typed guardian-failure refusal.
   Do not model the helper as a fictional direct bounded executor Child.
4. Connect another actual process to the private endpoint and require peer rejection. Alter the
   monitor/INIT start identity, parent, namespace identity and namespace PID independently;
   provide incomplete startup data and a positively dead INIT. Check refusal before Dispatch.
   Verify the guardian peer PID equals the claimed INIT chained to the actual unreaped monitor,
   and observer readiness precedes Dispatch. Substitute a stale helper built with different
   lifecycle capabilities and source/build inputs, even with a caller-supplied matching version
   or digest label. Require mismatch against the actual running library's identity and actual
   invoked executable, not a path inspected before replacement.
5. While the bootstrap gate is retained, inject partial/malformed/overlimit info, monitor exit
   and setup failure. Keep the monitor unreaped and signal its pinned group before close/reap.
   Exercise the exceptional bounded PGID membership/identity observation: retain each member's
   start identity and pidfd, require all retained pidfds terminated plus a complete post-signal
   membership observation with no live member. Include a member reparented after monitor death,
   unavailable/malformed membership, changed start identity and ambiguous live-thread state.
   Unavailable or unconfirmed observation refuses cleanup; kill success or monitor exit cannot
   pass. After claim, verify cancellation by INIT pidfd without normal-operation host scanning.
6. After Dispatch, create double-fork/setsid descendants which reparent before observation,
   positively acknowledged descendants born during cancellation, and nested PID namespaces
   where the required facilities permit them. Kill original caller or guardian separately and
   verify all owned descendants terminate through INIT teardown, including unsampled ones.
   Run a second independent namespace and unrelated fixture child beside the target; they stay
   live until their own cleanup. Record unsupported platform refusal rather than a passing skip.
7. Exercise whole-batch ownership with multiple FR-017 harness members in one launcher. Verify
   one guardian, monitor, namespace and lease own the entire group under its existing memory
   ceiling and multiplied wall deadline. Normal completion with adopted background work,
   nonzero/signal exit, exec failure, explicit cancellation, timeout, memory excess, capture
   overflow/read failure and observation failure all require confirmed teardown before outcomes.
   Close backend streams and fail the helper separately; require bounded capture settlement,
   appropriate typed refusal, no proof accepted from success output beside unconfirmed cleanup,
   monitor reaping with a live caller, and surviving-owner temporary artifact cleanup after caller
   loss or helper failure. Verify the abstract endpoint ends on final close without a filesystem
   socket artifact; bootstrap creates no persistent private files before the lease. Kill both
   cleanup owners separately to confirm kernel process containment without falsely claiming that
   dead actors remove remaining temporary reports.
8. Supply a missing, non-executable or unusable explicit helper path. Require typed refusal and
   no backend marker, with no PATH/global discovery, copied fixture binary or alternate launcher.
   Inspect setup docs for deliberate matching library/helper delivery, and the planned Linux,
   procfs children/RSS, pidfd, bubblewrap PID-1/new-session/info/gate and namespace prerequisites.
9. Connect the fresh private-nonce abstract endpoint in the same network namespace and verify
   actual host INIT peer credentials; do not equate a namespace-local caller PID to a host PID.
   Send malformed, unknown-field, oversized and excessive pending control records and close the
   lease. Require bounded cancellation/refusal, never Dispatch from EOF or bootstrap-gate bytes.
   Race an unrelated exec and verify it and backend descendants inherit neither a caller lease
   endpoint nor bootstrap-control writer. Inspect safe CLOEXEC child-only mapping alongside
   runtime inheritance checks; no raw inherited-FD adoption or unsafe exception participates.
10. Run a backend echoing raw non-UTF8 argv, stdin, overridden/inherited environment and cwd, plus
    distinct stdout/stderr markers. Verify exact recipe/capture separation. Feed actual ordinary
    completed/refused/falsified reports, wall/memory stops and ambiguous live-worker RSS through
    the existing classifiers. When FR-028 AC-24 native refinement is implemented, repeat its
    resource-stop control through this ownership path and assert the original class/evidence.
11. Distinguish an already expired original deadline, expiry during handshake, and helper-cap
    expiry while that deadline remains live. The first causes no Dispatch and inconclusive with
    the timed-out reason; actual deadline expiry during setup has the same classification;
    helper-cap expiry is typed setup refusal. Verify no deadline reset, bounded shutdown and
    observation, and refusal on unconfirmed teardown without a physical-disappearance claim.
12. Independently remove positive Dispatch authorization, replace the PID-1 guardian with a
    non-INIT watcher, remove session isolation, and close/reap before pinned startup confirmation.
    Require the corresponding premature-backend-marker, surviving-descendant, actual SID/PGID
    or startup-order assertion to fail, rather than compilation or fixture setup. Record the
    ownership observation before emergency cleanup of only the fixture's pinned namespace/group.
    Restore production and require focused controls to pass. Trace each actual asserted AC.

## Expected Results

| Authority | Required observation | Regression caught |
|---|---|---|
| FR-034-AC-1/2/3/7 | Trusted bootstrap is distinct from production Dispatch; defined group/claimed-pidfd termination observations cover every stage | EOF starts subject; signal/monitor exit falsely confirms teardown; claim-to-Dispatch gap |
| FR-034-AC-4/5/6 | Actual monitor/INIT/peer chain and observer readiness precede Dispatch | Arbitrary parent accepted; stale identity or unrelated peer accepted |
| FR-034-AC-8/9 | Kernel INIT teardown cancels escaped, late-born and nested descendants, preserving other runs | Kill only sampled PIDs; cancel another run |
| FR-034-AC-10/11/12/26 | Caller-group death preserves isolated guardian cleanup; guardian death kills namespace; live-caller helper failure always refuses; monitor/artifact cleanup confirmed | External sole gate owner dies; accept proof after helper failure; leak helper/socket |
| FR-034-AC-13/14/25 | Explicit real package helper matches actual running library build/capabilities; setup docs and missing-helper refusal are accurate | PATH/copy fallback; stale helper blessed by caller label |
| FR-034-AC-15/16 | Invalid bounded controls refuse; exclusive lease and safe child-only mapping | EOF authorizes Dispatch; descendants inherit caller lease |
| FR-034-AC-17/18/19; FR-028-AC-2/3/21/24; FR-017-AC-14/24/25 | Whole-batch ownership, exact recipe, separate captures and existing resource/refinement outcomes | One helper per member; diagnostics become report; weaken ceilings; ambiguous RSS becomes zero |
| FR-034-AC-20/21/22 | Original deadline and expired-deadline outcome persist; setup cap distinct; bounded observation refuses ambiguity | Reset deadline; setup refusal falsely timed out; hang capture |
| FR-034-AC-23/24 | Real package helper, positive observations and killed mutants before emergency cleanup | Test-only bypass; sleep-vacuous pass; cleanup masks survival |

Scenario prose or research probes establish no executable coverage. Native refinement remains
planned until its actual typed entry is delivered. Host destruction and uninterruptible tasks
cannot justify fabricated teardown; live-caller unavailable confirmation refuses. Caller-group
signals and direct guardian death are included lifecycle cases, not excluded double faults.

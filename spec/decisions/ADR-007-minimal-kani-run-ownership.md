---
id: ADR-007
title: "Minimal Kani run ownership: process group, wall-clock timeout, environment-owned memory cap"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/ADR-005
    type: supersedes
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: relates_to
---
# ADR-007: Minimal Kani run ownership: process group, wall-clock timeout, environment-owned memory cap

## Status

Accepted (owner decision, IR-639). Supersedes
[ADR-005](./ADR-005-original-guardian-gate-ownership.md), which stays as the record of the earlier
design. The source behaviors this decision removes are PLANNED changes owned by the code slice
IR-1510, not delivered by this record.

## Context

IR-639 asked for a caller-death supervisor: a guardian that keeps the Kani backend owned when the
original caller dies. ADR-005 recorded that the original guardian-gate design could not establish a
safe descriptor owner or a finite kernel bound, and listed eight options, none chosen. The
requirement it fed (the former FR-034 and its test case TC-049, about one hundred criteria) described
a PID-namespace guardian, a gate protocol, unnamed kernel report storage and a stored-certificate
history. The merged runner already carries part of that machinery: `src/kani/run/namespace.rs`
(bubblewrap `--unshare-user --unshare-pid`, init pidfd owner), a procfs resident-memory sampler in
`src/kani/run/memory.rs` and the `MemoryMechanismUnavailable` refusal. That path works only on Linux
with bubblewrap, user namespaces, procfs `children` files and pidfd, and a host policy such as
AppArmor can deny it.

The deployment already bounds memory outside the runner: the owner's Kani lane runs under
`systemd-run` with `MemoryMax=16G`. A verification tool that wraps a solver is expected to start it,
bound its time and clean up its processes; it is not expected to be a sandbox.

## Decision

The runner is minimal and sandbox-free.

1. CG starts Kani as the leader of its own process group, keeps the wall-clock timeout, and on
   timeout, output overflow or normal exit kills that process group, signalling it before the
   launcher is reaped so that the unreaped launcher still reserves the group id. The kill reaches
   the processes in that group. This is the behavior [FR-017](../kani/functional/FR-017-kani-execution-evidence.md)
   already states (the launcher group kill, `kani_output_over_limit`, no survivors).
2. CG reports the peak memory of a run only when the environment provides it, and reports none
   otherwise.
3. CG leaves any memory cap to the caller's environment (a cgroup, `systemd-run` or a container).
   The runner performs no cap check, refuses no run for lack of a memory mechanism, and kills no run
   for memory itself ([FR-028](../kani/functional/FR-028-bounded-proof-ceilings.md) AC-21).
4. The path works on macOS as on Linux: process group and timeout only, with no Linux-only
   dependency (no bubblewrap, namespace, procfs or pidfd).
5. The former FR-034 and TC-049 are retired. The criteria that remain real requirements already live
   in FR-017 and FR-028 and stay; new criteria for the minimal design are FR-017-AC-65 to
   FR-017-AC-68 and FR-028-AC-25, FR-028-AC-21 is reworded as an environmental requirement, and
   FR-028-AC-3 is retired with FR-028-AC-25 as its replacement.

Two costs are accepted. First, caller death is not guarded: a caller killed with SIGKILL after the
launch leaves its Kani process group running, and any bound on that orphan is the caller's
environment's (a single run has no Kani-side time bound, and a batch's `--harness-timeout` is per
member). Second, a descendant that calls `setsid` or `setpgid` leaves the process group and survives
the group kill; nothing in Kani's process tree does so today. When an environment memory kill also
takes `kani-driver` down, no report is written, the run is `NoVerdict` and its terminal value is
`Failed`, not `Incomplete(ResourceExhausted)`.

## Options not taken

| Option | Why not |
| --- | --- |
| Guardian process or binary outside the backend group (IR-639 as ticketed; ADR-005 options 2 to 4) | A second trusted process, an authenticated control channel and an audited unsafe owner for a guarantee a verification tool does not owe. |
| PID/user namespace with bubblewrap (the merged path) | Linux-only, needs host permission for user namespaces, does not run on macOS, and host policy such as AppArmor can deny the namespaces it needs. |
| cgroup watchdog or runner-side memory cap check | The caller's environment already owns the cap; a runner-side copy duplicates it, needs Linux cgroup access and creates a second source of truth for the memory ceiling. |
| Orphan time bound (a deadline after which orphaned backends are reaped by a helper) | Needs a surviving owner, which is the guardian again. |
| Narrow the guarantee only (ADR-005 option 8) | Kept as the direction, but not enough alone: the namespace path and its refusal would remain. |

## Consequences

- FR-034, TC-049 and their matrix rows are removed; ADR-005's mentions of them are history.
- FR-017-AC-19 returns to the launch-unique named report file in the target directory, which is the
  current code, and loses the unnamed `/proc/self/fd/N` storage that only the guardian design needed.
  FR-017-AC-54 to FR-017-AC-56 (diagnostics of the namespace owner) are retired with the namespace
  owner.
- FR-028-AC-3 is retired with replacement FR-028-AC-25: a memory stop is the environment's and is
  recognised only from the backend's report, so the runner-measured `Inconclusive` with
  `MemoryExhausted` path goes away. The statements that assumed it are amended (FR-017 Behavior,
  AC-21 and AC-58; FR-028 Behavior and AC-24; FR-029-AC-34; TC-027, TC-039, TC-043).
- The harness identity keeps recording the memory ceiling (FR-028-AC-1), now a declared value the
  environment is expected to enforce.
- The runner no longer binds a prepared source tree read-only for the backend: it has no sandbox,
  so a read-only mount is the caller's environment's. The staged-snapshot comparison in `scalar.rs`
  (before and after the run) remains the guard that a run left the tree unchanged.
- IR-1510 owns the code: AC-25 as the only memory classification; removing `MemoryExhausted`,
  `BatchMemoryExhausted`, `MemoryObservationFailed`, `MemoryMechanismUnavailable` and `LinuxPidNamespaceProcfsTreeRss`, and building no
  `MemoryExhausted` outcome for the planned playback entry (FR-017-AC-58); deleting the namespace/bubblewrap
  path and the procfs sampler; retagging or deleting the tests tagged FR-028-AC-3, FR-028-AC-21 and
  FR-017-AC-55; and the structural scan of FR-017-AC-67 in `make ci`. IR-639, IR-652 and IR-655
  are the superseded design.

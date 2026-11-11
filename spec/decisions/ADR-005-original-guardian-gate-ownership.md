---
id: ADR-005
title: "Original guardian-gate ownership: failed premises and the product architecture choice"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
---
# ADR-005: Original guardian-gate ownership: failed premises and the product architecture choice

## Status

Proposed, awaiting a product architecture choice. This is the IR-717 decision record, not an
implemented gate, a conformance finding, or permission to activate IR-652 or IR-655. IR-717 could
**not** establish a supported safe descriptor owner or a finite kernel/native bound for the
original-gate design. The exact failed premises are stated below and the choice they force is in
[Decision required](#decision-required). [FR-034](../kani/functional/FR-034-caller-death-ownership.md)
and [TC-049](../kani/matrix/TC-049-caller-death-ownership.md) are not amended by this record: no
decision is established that would support an amendment, and the committed caller/group-death,
early-release and confirmation-before-close guarantees are not narrowed.

## Context

FR-034 gives O the exclusive block-gate pipe writer and M the blocking reader (gate reader fd4).
Main's `src/kani/run/namespace.rs` creates the gate as a CLOEXEC `pipe_with` pair and a gate EOF
before release aborts without dispatching the backend (its test
`gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend`). C's cached phase
cannot establish that O still holds the gate if O exits at C's reporting boundary, and FR-034
already marks the exact fatal-prefix gate predicate UNBACKED.

The candidate replacement keeps the release endpoint S and the original caller lease in C's
physical custody, transfers only the blocking receiver R through the existing authenticated C to L
and L to O Start carriers, and lets C issue one checked release after O's ordinary tick and
permission over a one-way connected Unix stream with per-send NOSIGNAL. Genuine I Ready stays a
later authenticated condition for Backend. No new acknowledgement, pause, stored certificate,
fixture-only authority or post-return custodian is allocated.

## Custody walk-through (conditional on P1)

This is a structural argument over the candidate graph. It holds only if S and the lease have no
alias, which is the premise that fails (P1). It is not evidence of any running mechanism.

| Event | Owner of S | Gate predicate if S is unaliased | Gap |
| --- | --- | --- | --- |
| Creation before L, two Start transfers of R (C to L, L to O) | C | S open in C, R only in transit or in L, O, M. No byte written, so the gate is held. | Between socket creation and C's close of its own R copy, any fork from C's shared table also holds R. A held R alias makes a later C release succeed into the queue with no live reader, so peer loss on the NOSIGNAL send is not detected. S and the lease are not the only aliasable authorities. |
| Fatal O before C's held-gate observation | C | S stays open in C. M, I and O's R copies are torn down by outer containment, so C's release returns EPIPE (no SIGPIPE under NOSIGNAL) and positive admission requires the identity check, not S possession. | Needs the R-alias exclusion above and an unrun control. |
| Live early close | C | Only C closes S. A close before actual I confirmation is the AC-54 mutant. | Control unrun. |
| Caller or group death | kernel closure of C's table | S closes, R reads EOF, backend is never dispatched (existing abort semantics). The lease closes and I cancels. | A fork child that retained S or the lease keeps the gate open and defeats FR-034-AC-83 after C is gone, and a fork child can write a release byte for a dead caller. |
| Original-cutoff unavailable confirmation | C | AC-38 CleanupUnconfirmed; last close is abandonment with no order credit (FR-034, unavailable-confirmation fault domain). | Unchanged by this record. |

## Premise results

| Premise | Result | Exact grounding |
| --- | --- | --- |
| P1. S, R (C copy) and the lease are excluded from every foreign fork, including other embedding-C threads and the owner thread's signal handler, through a **safe** API | **FAILED.** No supported safe Rust or OS API upholds it. | See P1 evidence. |
| P2. Live release and the original caller/group-death path preserve the held-gate and teardown predicates through fatal O | **Holds structurally only if P1 holds** (walk-through). Not established. | Depends on P1. |
| P3. A finite additive stream, ancillary, kernel and native reserve is derivable from real APIs before exposure | **FAILED for the Unix-stream transport.** Partially derivable (see inventory). | See P3 evidence. |

### P1 evidence

Linux fork copies the forking task's descriptor table, and threads created by `pthread_create` or
`std::thread` share one table. A fork from any embedding-C thread therefore copies S, the lease and
C's R copy until the child execs, and CLOEXEC does not act before exec. A fork without exec
(daemonizing, multiprocessing fork) keeps the alias indefinitely. The installed userspace headers
here define no `O_CLOFORK`, so no per-descriptor fork exclusion exists.

The only per-thread isolation of the table is `unshare(CLONE_FILES)` on a dedicated owner thread.
In the pinned `rustix 1.1.5`, the safe `thread::unshare` is `#[deprecated]` with the comment that it
"should be unsafe", and the supported entry is `unshare_unsafe`, whose contract is that callers
"must ensure that threads never observe file descriptors from unshared tables". `OwnedFd` carries
no table affinity, so moving or dropping any descriptor across threads after the unshare closes or
uses a different table's entry of the same number. This is the IO-safety unsoundness that the
library names, not a missing wrapper. The crate sets `#![forbid(unsafe_code)]` in `src/lib.rs`, the
workspace denies undocumented unsafe blocks, and the ticket excludes an unsafe shortcut, so this
route is unavailable without a product decision.

A thread-private table still permits a fork from a handler running on the owner thread. Denying that
needs one of: a per-thread seccomp filter (rustix 1.1.5 exposes `no_new_privs` setting but has no
seccomp filter installation), or a signal mask. A mask is safe through `nix::sys::signal::pthread_sigmask`
but `nix` is not a dependency, and a mask cannot exclude synchronous fault signals (SIGSEGV, SIGBUS,
SIGFPE, SIGILL, SIGSYS), whose handlers, such as crash reporters, commonly fork. A mask is a
narrowing, not an enforced exclusion. The seccomp route would also have to deny `clone3`
entirely because its flags are not visible to a filter, and it makes no_new_privs and the filter
irreversible for that thread.

The enabled `rustix` features are `std, event, process, time, pipe, fs`. The safe transport
primitives themselves exist in `rustix 1.1.5` and are not the blocker: `net::socketpair`,
`net::sendmsg` and `net::recvmsg` with `SendAncillaryBuffer` and `RecvAncillaryBuffer`,
`SendFlags::NOSIGNAL`, `RecvFlags::CMSG_CLOEXEC` and `sockopt::socket_peercred`. Using them needs
the `net` feature, which is a dependency change, not an unsafe one.

### P3 evidence

Derivable from real APIs before exposure (all UNRUN until a control reads them back):

- Control-message buffer bytes: `cmsg_space!` for the exact right count (the common ancillary
  maximum, currently four rights and five in the candidate Start).
- Per-direction queued-byte cap: `set_socket_send_buffer_size` then `socket_send_buffer_size`
  read-back, the stream analogue of `F_GETPIPE_SZ`.
- Descriptor-table slots: `RLIMIT_NOFILE` and the exact count of S, R, lease and Start rights per
  table.

Not derivable from any userspace API before exposure: the kernel socket object and its slab
rounding for both endpoints, `sk_buff` truesize and allocation rounding, the per-message in-flight
right list the kernel allocates for each ancillary message, and the in-flight accounting for
queued rights. `SO_MEMINFO` and cgroup `memory.stat` report these only after the objects exist.
The in-flight right count is additionally compared with a per-user counter that C does not own,
so a pre-exposure reservation cannot own it and its exhaustion must be a bounded refusal
(statement from kernel source, not verified locally). A pipe gate would have a real pre-exposure
reserve (`F_GETPIPE_SZ`, already used by `pipe_reserve`), but it cannot carry per-send NOSIGNAL,
so SIGPIPE handling then needs a separate safe design.

Native accounting terms with no derivation: owner-thread stack, TLS and runtime frames beyond the
requested stack size, and any private-table preparation. FR-034 already forbids a requested stack
size or RSS proxy to stand in for them.

## Resource inventory at the maximum simultaneous configuration

Maximum simultaneous configuration: gate endpoints S and R, two Start transfers each carrying R
with the existing Start rights (the candidate fifth right), the lease, the existing C-L, L-O and
report controls, and the existing pipe and memfd reservations. Status is per term.

| Term | Source | Status |
| --- | --- | --- |
| Gate endpoint descriptors and file objects | exact counts in C, L, O, M tables | Count derivable; kernel object size not |
| Control-message bytes per Start transfer | `cmsg_space!` for the exact right count | Derivable; five rights exceeds the four-right common maximum, so both Start counts and the common maximum change together |
| Queued and partial rights after refusal, setup failure or cancellation | receiver-owned `RecvAncillaryBuffer` drain with `CMSG_CLOEXEC` | Ownership derivable; kernel in-flight cost not |
| Stream queued bytes | send-buffer read-back | Derivable as a cap; per-skb overhead not |
| Socket objects and skb/slab rounding | none before exposure | **Not derivable** (P3) |
| Native stack, TLS, runtime, private-table preparation | none | **Not derivable** |
| Existing `owned_RSS`, `caller_run_buffers`, `pipe_reserve`, `memfd_reserve` | FR-034 charge oracle | Unchanged; the new terms are additive and replace none of them |

No chosen RSS ceiling, no unmeasured estimate and no invented reserve is substituted for the two
non-derivable rows. No measurement was run by this record (no Cargo, native probe or kernel read).

## Falsifying controls

All controls are **UNRUN**. Each uses the original C/L/O/M/I roles and actual descriptors, must be
able to fail its own predicate before emergency cleanup, and includes a restored healthy control.
An omitted or unavailable construction earns no Test credit.

| Control | Mutant that must fail | Predicate | Run state |
| --- | --- | --- | --- |
| Foreign-fork alias | Other C thread forks without exec while S, R and the lease are held; a handler on the owner thread forks; owner guard omitted | The child cannot retain or operate the original authorities; actual C or group death still tears down I. The shared-table mutant fails. | UNRUN; cannot be constructed safely while P1 fails |
| Fatal O before C's held-gate observation | O exits between I claim and C's observation; O or C releases or shuts down early | C custody survives O loss, a dead or mismatched identity gains no positive admission, early gate progress fails with no backend marker | UNRUN |
| Live early close | C closes S before actual I confirmation, or omits confirmation | Cooperative I-before-last-close passes; each mutant fails; withheld confirmation to the original cutoff yields only bounded AC-38 abandonment | UNRUN |
| Peer loss on the NOSIGNAL release | Peer disappears during C's send; second successful send; partial permission; NOSIGNAL omitted | EPIPE with no SIGPIPE, no second release, original stop and cutoff remain active, permission is neither release nor Ready. A held R alias in C's window makes the send succeed and must fail this control. | UNRUN |
| Omitted native or kernel charge | One socket, skb, in-flight right, stack or table term dropped from the peak, or one Start right wrong, extra or truncated | The pre-exposure comparison fails rather than borrowing another ceiling; every received or pending right is disposed after refusal | UNRUN; the two non-derivable terms have no oracle to compare against |

## Decision required

Product choice, not a technical default. Each option has a cost recorded here; none is selected by
this record, and none is implemented or evaluated beyond the grounding above.

| Option | What it changes | Open cost |
| --- | --- | --- |
| 1. Retain the hold | Original gate stays as committed (O-held writer). IR-652 production and IR-655 fixtures stay inactive. | The fatal-O held-gate predicate stays UNBACKED. This is the status quo, stated openly, not a narrowing of the guarantee. |
| 2. Authorize an audited unsafe owner boundary | Permit a reviewed unsafe-allowing owner (dedicated thread with `unshare_unsafe(FILES)` plus a fork-denial mechanism) outside the `forbid(unsafe_code)` crate. | Revises the unsafe policy; descriptor affinity, synchronous-fault handlers, seccomp coverage and the P3 rows still open. |
| 3. Authorize a separate release custodian process | Move S and the lease to a process with its own table. | Changes C custody and death semantics (the ticket excludes it as a hidden custodian); needs its own death and teardown contract. |
| 4. Authorize a process-wide no-fork rule for embedding C | Embedding C must not fork while a guardian run is live. | Changes ordinary embedding behavior; the ticket forbids assuming it. |
| 5. Authorize a non-descriptor lease and a measured gate profile | Replace the fd lease by an identity not duplicated by fork, and pin a measured kernel profile for the gate. | New caller-death mechanism and a profile-restricted support claim; the measurement is itself unrun. |
| 6. Narrow the guarantee explicitly | Amend FR-034 AC-54/83/87 to drop the strong held-gate predicate. | Requires an explicit reviewed amendment; never done silently. |

Until one is chosen, the strong original-gate construction remains **UNBACKED**.

## Consequences

IR-717 stays open as a prerequisite to IR-652 and the later completion audit. IR-652 owns the
actual C/L/O/M/I and transport implementation; IR-655 owns genuine fixture evidence and stays
downstream of an approved operational contract. The five-right Start and the stream transport do
not enter the normative protocol. No compatibility route, copied dependency file, fabricated
reserve, stored certificate or source-only success claim follows from this record.

## Not grounded by this record

- No measurement: no Cargo build, native probe, kernel read or control was run.
- Kernel facts (fork copying the table, the in-flight right accounting and per-message right list,
  skb overhead, per-user counter) are stated from kernel behavior and were not verified on the
  supported kernel here.
- Whether any supported embedding actually forks from a signal handler, and which supported kernel
  and build profile applies, are not established.
- No packet existed at `/tmp/ix-handoff/ir717-operational-gate-contract.md`; the private design
  review and feasibility report cited in the ticket were not available to this record.

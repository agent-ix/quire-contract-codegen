---
id: ADR-005
title: "Original guardian-gate ownership: failed premises and the product architecture choice"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/TC-049
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
original-gate design. The exact failed premises are stated in [Decision](#decision) and the choice
they force is in [Product architecture choice](#product-architecture-choice).
[FR-034](../kani/functional/FR-034-caller-death-ownership.md) and
[TC-049](../kani/matrix/TC-049-caller-death-ownership.md) are not amended by this record: no
decision is established that would support an amendment, and the committed caller/group-death,
early-release and confirmation-before-close guarantees are not narrowed.

## Context

FR-034 gives O the exclusive block-gate pipe writer and M the blocking reader (gate reader fd4).
Main's `src/kani/run/namespace.rs` creates the gate as a CLOEXEC `pipe_with` pair and releases it
with a write. Its test `gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend`
kills INIT first and only then lets the gate reach EOF, so it does not show that gate EOF alone
prevents dispatch. Whether EOF alone does is **unestablished** here: bubblewrap's `--block-fd`
waits for data and, from upstream source recalled and not verified locally, may treat EOF as
unblock. What prevents dispatch on caller death is teardown of I and the absence of a positive
Dispatch authorization (FR-034), not gate EOF. C's cached phase cannot establish that O still holds
the gate if O exits at C's reporting boundary, and FR-034 already marks the exact fatal-prefix gate
predicate UNBACKED.

The candidate replacement keeps the release endpoint S and the original caller lease under C's
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
| Creation before L, two Start transfers of R (C to L, L to O) | C | S open in C, R only in transit or in L, O, M. No byte written, so the gate is held. | Between socket creation and C's close of its own R copy, any fork from C's shared table also holds R. A held R alias makes a later C release succeed into the queue with no live reader, so peer loss on the release is not detected. S and the lease are not the only aliasable authorities. |
| Fatal O before C's held-gate observation | C | S stays open in C. M, I and O's R copies are torn down by outer containment, so C's release returns EPIPE (no SIGPIPE when the write suppresses it) and positive admission requires the identity check, not S possession. | Needs the R-alias exclusion above and an unrun control. |
| Live early close | C | Only C closes S. A close before actual I confirmation is the AC-54 mutant. | Control unrun. |
| Caller or group death | kernel closure of C's table | S closes and R reads EOF; the lease closes and I cancels. Whether EOF alone withholds dispatch is unestablished (Context); teardown of I and absent Dispatch authorization are what FR-034 relies on. | A fork child that retained S or the lease keeps the gate open and defeats FR-034-AC-83 after C is gone, and a fork child can write a release byte for a dead caller. |
| Original-cutoff unavailable confirmation | C | AC-38 CleanupUnconfirmed; last close is abandonment with no order credit (FR-034, unavailable-confirmation fault domain). | Unchanged by this record. |

## Decision

The strong original-gate construction is held **UNBACKED**; this record selects no architecture.
FR-034-AC-38, AC-54, AC-83, AC-87 and AC-94 stay as committed, and a product choice below does not
by itself authorize implementation: any option that changes custody or protocol contradicts
AC-54's "actual original O gate owner" and needs an explicit, reviewed FR-034/TC-049 amendment
evaluated by the separate specification reviewer before IR-652 implementation.

| Premise | Result | Exact grounding |
| --- | --- | --- |
| P1. S, R (C copy) and the lease are excluded from every foreign fork, including other embedding-C threads and the owner thread's signal handler, through a **safe** API | **FAILED.** No supported safe Rust or OS API upholds it. | P1 evidence |
| P2. Live release and the original caller/group-death path preserve the held-gate and teardown predicates through fatal O | **Holds structurally only if P1 holds** (walk-through). Not established. | Depends on P1 |
| P3. A finite additive stream/pipe, ancillary, kernel and native reserve is derivable from real APIs before exposure | **FAILED for the Unix-stream transport.** Partially derivable; a pipe gate may narrow the gap (option 6) but is unprobed. | P3 evidence |

### P1 evidence

Linux fork copies the forking task's descriptor table, and threads created by `pthread_create` or
`std::thread` share one table (kernel behavior, not verified here; see Not grounded). A fork from
any embedding-C thread therefore copies S, the lease and C's R copy until the child execs, and
CLOEXEC does not act before exec. A fork without exec (daemonizing, multiprocessing fork) keeps the
alias indefinitely. The installed userspace headers here define no `O_CLOFORK`, so no
per-descriptor fork exclusion is available.

The only per-thread isolation of the table is `unshare(CLONE_FILES)` on a dedicated owner thread.
In rustix 1.1 (locked at 1.1.6; the quoted text is identical in 1.1.5) the safe `thread::unshare`
is `#[deprecated]` with the comment that it "should be unsafe", and the supported entry is
`unshare_unsafe`, whose contract is that callers "must ensure that threads never observe file
descriptors from unshared tables". `OwnedFd` carries no table affinity, so moving or dropping any
descriptor across threads after the unshare closes or uses a different table's entry of the same
number. This is the IO-safety unsoundness that the library names, not a missing wrapper. The crate
sets `#![forbid(unsafe_code)]` in `src/lib.rs`, the workspace denies undocumented unsafe blocks,
and the ticket excludes an unsafe shortcut, so this route is unavailable without a product
decision.

A thread-private table still permits a fork from a handler running on the owner thread. Routes to
deny that, each rejected as an enforced exclusion:

- A per-thread seccomp filter: rustix exposes safe `no_new_privs` setting but no filter
  installation. The filter would also have to deny `clone3` entirely because its flags are not
  visible to a filter, and `no_new_privs` and the filter are irreversible for that thread.
- Seccomp strict mode: rustix exposes safe `thread::set_secure_computing_mode` with
  `SecureComputingMode::Strict`. Strict mode would deny fork, but it permits only read, write,
  `_exit` and `sigreturn` (no close, `sendmsg` or poll) and is irreversible, so it cannot host the
  owner.
- A signal mask: safe through `nix::sys::signal::pthread_sigmask`, but it cannot exclude
  synchronous fault signals (SIGSEGV, SIGBUS, SIGFPE, SIGILL, SIGSYS) whose handlers, such as crash
  reporters, commonly fork. A mask is a narrowing, not an enforced exclusion.

Dependency cost of the routes: `unshare_unsafe`, `set_no_new_privs` and `set_secure_computing_mode`
sit in rustix's `thread` feature, which is not enabled (enabled: `std, event, process, time, pipe,
fs`), and the socket primitives need the `net` feature. `nix` 0.31.3 is already resolved through
the direct `command-fds` dependency but without its `signal` feature, so `pthread_sigmask` needs a
direct `nix` dependency with `signal`. These are dependency changes; only the unshare route is also
an unsafe-policy change.

The safe transport primitives themselves exist in rustix 1.1 and are not the blocker:
`net::socketpair`, `net::sendmsg` and `net::recvmsg` with `SendAncillaryBuffer` and
`RecvAncillaryBuffer`, `SendFlags::NOSIGNAL`, `RecvFlags::CMSG_CLOEXEC` and
`sockopt::socket_peercred`.

### P3 evidence

Derivable from real APIs before exposure (all UNRUN until a control reads them back):

- Control-message buffer bytes: `cmsg_space!` for the exact right count. The current common
  ancillary maximum is carried from the earlier ADR as "four rights"; no FR-034 clause or `src/`
  constant states it, so it is an unsourced assumption here, and the candidate fifth right exceeds
  it.
- Per-direction queued cap on a stream: `set_socket_send_buffer_size` then
  `socket_send_buffer_size` read-back, the stream analogue of `F_GETPIPE_SZ`.
- Per-pipe buffer capacity: `F_GETPIPE_SZ`, already the basis of the FR-034 `pipe_reserve`.
- Descriptor-table slots: `RLIMIT_NOFILE` and the exact count of S, R, lease and Start rights per
  table.

Not derivable from any userspace API before exposure: the kernel socket (or pipe inode) object and
its slab rounding for both endpoints, the per-message in-flight right list the kernel allocates for
each ancillary message, and the in-flight accounting for queued rights. `SO_MEMINFO` and cgroup
`memory.stat` report these only after the objects exist. The in-flight right count is additionally
compared with a per-user counter that C does not own, so a pre-exposure reservation cannot own it
and its exhaustion must be a bounded refusal (kernel documentation and recollection; no kernel
source is installed here and nothing was verified).

Pipe alternative. The installed uapi `linux/fs.h` defines `RWF_NOSIGNAL` (0x100, "prevent pipe and
socket writes from raising SIGPIPE") and includes it in `RWF_SUPPORTED`, and rustix offers the safe
`io::pwritev2(fd, bufs, offset, ReadWriteFlags)` whose flags type accepts unnamed bits, so a safe
route to a per-write SIGPIPE suppression on a pipe exists in the headers and crate. Support on the
running or any supported kernel, the minimum kernel version, and `pwritev2` behavior on a pipe at
the current-position offset were **not probed**. A pipe gate would keep the `F_GETPIPE_SZ`
reserve and remove the stream socket and skb terms for the gate. It does not remove the extra
ancillary right on the existing Start carriers, the pipe inode and file objects, the in-flight
right list, or the native terms, and it does not address P1. See option 6.

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
| Control-message bytes per Start transfer | `cmsg_space!` for the exact right count | Derivable; five rights exceeds the assumed four-right maximum, so both Start counts and the common maximum change together |
| Queued and partial rights after refusal, setup failure or cancellation | receiver-owned `RecvAncillaryBuffer` drain with `CMSG_CLOEXEC` | Ownership derivable; kernel in-flight cost not |
| Stream queued charge | send-buffer read-back | Derivable as a cap; whether it includes per-skb overhead (and by how much of one message it can be exceeded) is an unverified kernel recollection |
| Gate pipe buffer | `F_GETPIPE_SZ` | Derivable (same basis as `pipe_reserve`); applies only to a pipe gate |
| Socket or pipe object and slab rounding | none before exposure | **Not derivable** (P3) |
| Native stack, TLS, runtime, private-table preparation | none | **Not derivable** |
| Existing `owned_RSS`, `caller_run_buffers`, `pipe_reserve`, `memfd_reserve` | FR-034 charge oracle | Unchanged; the new terms are additive and replace none of them |

No chosen RSS ceiling, no unmeasured estimate and no invented reserve is substituted for the
non-derivable rows. No measurement was run by this record (no Cargo, native probe or kernel read).

## Falsifying controls

All controls are **UNRUN**. Each uses the original C/L/O/M/I roles and actual descriptors, must be
able to fail its own predicate before emergency cleanup, and includes a restored healthy control.
An omitted or unavailable construction earns no Test credit.

| Control | Mutant that must fail | Predicate | Run state |
| --- | --- | --- | --- |
| Foreign-fork alias | Other C thread forks without exec while S, R and the lease are held; a handler on the owner thread forks; owner guard omitted | The child cannot retain or operate the original authorities; actual C or group death still tears down I. | UNRUN. The failing mutant (shared-table fork against the current design) can be built in an integration-test crate outside `forbid(unsafe_code)` and is owed as a demonstration that the current design fails. Only the passing arm has no safe construction while P1 fails. |
| Fatal O before C's held-gate observation | O exits between I claim and C's observation; O or C releases or shuts down early | C custody survives O loss, a dead or mismatched identity gains no positive admission, early gate progress fails with no backend marker | UNRUN |
| Live early close | C closes S before actual I confirmation, or omits confirmation | Cooperative I-before-last-close passes; each mutant fails; withheld confirmation to the original cutoff yields only bounded AC-38 abandonment | UNRUN |
| Peer loss on the release | Peer disappears during C's send; second successful send; partial permission; suppression flag omitted (MSG_NOSIGNAL, or RWF_NOSIGNAL on a pipe) | EPIPE with no SIGPIPE, no second release, original stop and cutoff remain active, permission is neither release nor Ready. A held R alias in C's window makes the send succeed and must fail this control. The omitted-flag mutant runs in a host whose SIGPIPE disposition is the default (a non-Rust host, or a child exec'd with SIGPIPE at SIG_DFL), because the Rust runtime ignores SIGPIPE before `main` and the mutant would otherwise still pass with EPIPE; it must fail by observed SIGPIPE termination of C. | UNRUN |
| Omitted native or kernel charge | One socket, pipe, skb, in-flight right, stack or table term dropped from the peak, or one Start right wrong, extra or truncated | The pre-exposure comparison fails rather than borrowing another ceiling; every received or pending right is disposed after refusal | UNRUN; the non-derivable terms have no oracle to compare against |

## Product architecture choice

Product choice, not a technical default. Each option has its cost recorded here; none is selected
by this record, and none is implemented or evaluated beyond the grounding above. Options address
P1 and P3 separately: the "P1" and "P3" columns say what each option does to each failed premise,
and options can be combined (the P3 option 6 with a P1 option 2 or 3, for example). Every option
that changes custody or protocol (2 to 7) still requires the explicit reviewed FR-034/TC-049
amendment named in [Decision](#decision) before implementation, because it contradicts AC-54's O
gate owner; option 8 is itself such an amendment.

| Option | What it changes | P1 | P3 | Open cost |
| --- | --- | --- | --- | --- |
| 1. Retain the hold | Original gate stays as committed (O-held writer). IR-652 production and IR-655 fixtures stay inactive. | Stays failed for S, R and the lease | Not applicable (no new transport) | The fatal-O held-gate predicate stays UNBACKED, and committed FR-034-AC-83 stays unbacked as well, because the lease sits in C's shared table today. Status quo stated openly, not a narrowing. |
| 2. Authorize an audited unsafe owner boundary | A reviewed unsafe-allowing owner (dedicated thread with `unshare_unsafe(FILES)` plus a fork-denial mechanism) outside the `forbid(unsafe_code)` crate. | Possibly addressed if descriptor affinity and fork denial hold, including handlers on the owner thread | Stays failed | Revises the unsafe policy; rustix `thread` (and `net`) features; synchronous-fault handlers and seccomp coverage still open. |
| 3. Hold the release endpoint in the existing launcher L | L is an existing single-thread process (C's unreaped Child) with its own table, PR_SET_PDEATHSIG on C's creating thread, whole-run lifetime and an authenticated C-to-L channel. L would create S and R after its own spawn of O, send R once to O, and release on C's instruction. | S is outside every foreign C fork with no unsafe code. The lease stays in C's shared table, so AC-83 is not addressed unless the lease also moves, which changes "only C holds the lease". | Stays failed (one fewer Start transfer of R) | Not a new custodian, but C-to-L release instruction is a new message on an existing channel and C would need a witness of L's actual close for AC-54/87 order without a new acknowledgement or pause; L exit or an L-to-C message are the only candidates and neither is established. PDEATHSIG tracks C's creating thread, not C process death, so L loss with C alive is a distinct case. Not evaluated against AC-54/87 beyond this. |
| 4. Authorize a new separate release custodian process | A new role with its own table holds S and the lease. | Addressed for the holder's own table | Stays failed | The ticket excludes a hidden or new custodian/channel; needs its own death, teardown and ordering contract and a changed C custody. |
| 5. Authorize a process-wide no-fork rule for embedding C | Embedding C must not fork while a guardian run is live. | Addressed by policy, not enforced | Stays failed | Changes ordinary embedding behavior; the ticket forbids assuming it. |
| 6. Pipe gate with `RWF_NOSIGNAL` writes | Gate endpoints are a pipe pair; the release is `io::pwritev2` with `RWF_NOSIGNAL`; reserve is `F_GETPIPE_SZ` like `pipe_reserve`. | Not addressed (combine with 2 or 3) | Narrows: removes the gate stream socket and skb terms with a derivable reserve; leaves the extra Start right, pipe object, in-flight list and native terms | Kernel support, minimum kernel version and pipe behavior at the current-position offset are **not probed**; an unsupported kernel must refuse, with no compatibility path. |
| 7. Non-descriptor caller liveness and a measured gate profile | Meet AC-83 without a descriptor the C table can alias, using the existing PDEATHSIG and pidfd mechanisms, and pin a measured kernel profile for the gate. | Possibly addressed for the lease only | Addressed only by a pinned measured profile (unrun) | Whether existing PDEATHSIG (creating-thread semantics) and held pidfds satisfy AC-83's "exclusive lease" is not established; S would still be a descriptor. A profile-restricted support claim. |
| 8. Narrow the guarantee explicitly | Amend FR-034-AC-54 and AC-87 to drop the held-gate custody and cooperative-order predicate; separately, AC-83 only if the lease predicate is dropped. | Not applicable | Not applicable | Requires an explicit reviewed amendment; never done silently. |

Until one is chosen, the strong original-gate construction remains **UNBACKED**.

## Consequences

IR-717 stays open as a prerequisite to IR-652 and the later completion audit. IR-652 owns the
actual C/L/O/M/I and transport implementation; IR-655 owns genuine fixture evidence and stays
downstream of an approved operational contract. The five-right Start and the stream transport do
not enter the normative protocol. No compatibility route, copied dependency file, fabricated
reserve, stored certificate or source-only success claim follows from this record.

## Alternatives Considered

- **Shared-table custody.** Unrelated C forks inherit S or the original lease in the shared table.
  Rejected as the strong held-gate construction.
- **O-only custody (current).** O-only release loses the live gate when O exits before C's
  observation. Rejected for the strong predicate; retained as option 1's status quo.
- **Thread-private table with owner-local fork denial.** Evaluated as option 2; no safe API.
- **Separate or existing process holder, no-fork rule, pipe transport, non-descriptor liveness,
  explicit narrowing.** Options 3 to 8 above.

## Not grounded by this record

- No measurement: no Cargo build, native probe, kernel read or control was run.
- Kernel behavior stated as fact but not verified on the supported kernel here: fork copying the
  descriptor table, threads sharing one table, CLOEXEC acting only at exec, the in-flight right
  accounting and per-message right list, skb overhead and whether it counts against the send
  buffer, and the per-user in-flight counter. No kernel source is installed.
- `RWF_NOSIGNAL` is grounded in the installed uapi headers and rustix source only; runtime support,
  minimum kernel and `pwritev2` behavior on a pipe are unprobed.
- Rust std setting SIGPIPE to ignored before `main`, and its `Command` child resetting it to
  default, are recalled and unverified here.
- bubblewrap `--block-fd` behavior on EOF and PR_SET_PDEATHSIG creating-thread semantics are from
  upstream documentation and recollection.
- Whether any supported embedding actually forks from a signal handler, and which supported kernel
  and build profile applies, are not established.
- The ticket named a private design review and feasibility report as inputs; they were not
  available to this record, whose source is the IR-717 ticket and its comments.

---
id: SR-17940486709564847047617
title: "IR-717 PR #775 ADR-005 dependency analysis"
type: SpecReview
analysis: dependency
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #775 @ 141760f0; IR-717; spec/decisions/ADR-005-original-guardian-gate-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/ADR-005
    type: reviews
---

## Summary

Ticket: IR-717. Spec-only PR #775 (head 141760f0, one commit): rewrites spec/decisions/ADR-005-original-guardian-gate-ownership.md from the #345 hold into a decision record (P1 safe alias exclusion FAILED, P2 conditional on P1, P3 finite bound FAILED for the Unix-stream transport, five UNRUN controls, six product options with none selected; FR-034/TC-049 unamended). Checked against: rustix 1.1.5 and 1.1.6 vendored source (src/thread/setns.rs unshare/unshare_unsafe, src/thread/prctl.rs seccomp/no_new_privs, src/net send_recv/msg.rs cmsg_space/sendmsg/recvmsg, sockopt.rs send buffer and peercred, socketpair.rs, io/read_write.rs pwritev2, backend io/types.rs ReadWriteFlags); nix 0.31.3 signal.rs; /usr/include and linux-headers-7.0.0-38 uapi (no O_CLOFORK; RWF_NOSIGNAL present); Cargo.toml/Cargo.lock (rustix 1.1.6 locked, nix via command-fds); src/lib.rs forbid(unsafe_code); src/kani/run/namespace.rs gate pipe and abort test; FR-034 Behavior and AC-38/54/83/87/94; TC-049; the #345 ADR on origin/main; IR-717 ticket acceptance and comments. quire validate and quire matrix pass. No Cargo, build, probe or kernel read was run.

## Verdict

PASS WITH LOWS. FR-034/FR-028/FR-017 edges are unchanged from main and resolve. IR-652/IR-655 ordering (IR-717 prerequisite; IR-655 downstream of an approved contract) matches the ticket. The feature/dependency inventory for options 2 and P3 is incomplete, and TC-049 is not a relationship.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The dependency inventory is incomplete. thread::unshare_unsafe, set_no_new_privs and set_secure_computing_mode are in rustix's `thread` feature, which is not enabled (enabled: std, event, process, time, pipe, fs), so option 2 also needs a `thread` feature change, not only `net`. 'nix is not a dependency' is imprecise: nix 0.31.3 is already resolved through the direct dependency command-fds (features fs, process). nix::sys::signal::pthread_sigmask (safe, nix-0.31.3 src/sys/signal.rs:1045) needs a direct nix dependency with the `signal` feature. | spec/decisions/ADR-005-original-guardian-gate-ownership.md:90-94 |
| FND-002 | low | The Status says the record deliberately leaves TC-049 unamended and links it in the body, but relationships list only FR-034, FR-028 and FR-017. Add relates_to ix://agent-ix/quire-contract-codegen/TC-049 so the decision shows up in TC-049's blast radius. | spec/decisions/ADR-005-original-guardian-gate-ownership.md:5-11 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| ADR-005-relationships | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:5-11 | relationships:   - target: ix://agent-ix/quire-contract-codegen/FR-034     type: relates_to   - target: ix://agent-ix/quire-contract-codegen/FR-028     type: relates_to   - target: ix://agent-ix/quire-contract-codegen/FR-017     type: relates_to |
| ADR-005-P1-transport | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:90-94 | The enabled `rustix` features are `std, event, process, time, pipe, fs`. The safe transport primitives themselves exist in `rustix 1.1.5` and are not the blocker: `net::socketpair`, `net::sendmsg` and `net::recvmsg` with `SendAncillaryBuffer` and `RecvAncillaryBuffer`, `SendFlags::NOSIGNAL`, `RecvFlags::CMSG_CLOEXEC` and `sockopt::socket_peercred`. Using them needs the `net` feature, which is a dependency change, not an unsafe one. |
| ADR-005-P1-unshare | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:71-79 | The only per-thread isolation of the table is `unshare(CLONE_FILES)` on a dedicated owner thread. In the pinned `rustix 1.1.5`, the safe `thread::unshare` is `#[deprecated]` with the comment that it "should be unsafe", and the supported entry is `unshare_unsafe`, whose contract is that callers "must ensure that threads never observe file descriptors from unshared tables". `OwnedFd` carries no table affinity, so moving or dropping any descriptor across threads after the unshare closes or uses a different table's entry of the same number. This is the IO-safety unsoundness that the library names, |
| ADR-005-P1-handler | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:81-88 | A thread-private table still permits a fork from a handler running on the owner thread. Denying that needs one of: a per-thread seccomp filter (rustix 1.1.5 exposes `no_new_privs` setting but has no seccomp filter installation), or a signal mask. A mask is safe through `nix::sys::signal::pthread_sigmask` but `nix` is not a dependency, and a mask cannot exclude synchronous fault signals (SIGSEGV, SIGBUS, SIGFPE, SIGILL, SIGSYS), whose handlers, such as crash reporters, commonly fork. A mask is a narrowing, not an enforced exclusion. The seccomp route would also have to deny `clone3` entirely be |
| ADR-005-consequences | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:172-176 | IR-717 stays open as a prerequisite to IR-652 and the later completion audit. IR-652 owns the actual C/L/O/M/I and transport implementation; IR-655 owns genuine fixture evidence and stays downstream of an approved operational contract. The five-right Start and the stream transport do not enter the normative protocol. No compatibility route, copied dependency file, fabricated reserve, stored certificate or source-only success claim follows from this record. |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in ac5beeeb |
| FND-002 | fixed | fixed in ac5beeeb |

---
id: SR-17940486709564847047615
title: "IR-717 PR #775 ADR-005 control-strength analysis (manual)"
type: SpecReview
analysis: criterion-strength
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

CHANGES REQUIRED (manual: Jev client unavailable). All five controls are honestly UNRUN, each names a mutant and a predicate, and the omitted-charge control states openly that the two non-derivable terms have no oracle. The NOSIGNAL-omitted mutant cannot fail in a Rust process that ignores SIGPIPE (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The 'NOSIGNAL omitted' mutant cannot fail in a Rust process. Rust's std runtime sets SIGPIPE to SIG_IGN before main (test harness binaries included), so an omitted MSG_NOSIGNAL still yields EPIPE with no SIGPIPE death and passes the predicate 'EPIPE with no SIGPIPE'. The control must say how a SIGPIPE-default C is constructed: restore SIG_DFL in the control process before the send, or embed C in a non-Rust host. Otherwise the mutant is vacuous. | spec/decisions/ADR-005-original-guardian-gate-ownership.md:151 |
| FND-002 | low | The run state 'cannot be constructed safely while P1 fails' conflates the mutant and the healthy arm. The shared-table mutant (another C thread forks without exec) can be built in the integration-test crate, which is outside src/lib.rs's forbid(unsafe_code); the workspace only denies undocumented unsafe. Only the passing arm has no safe construction. Say which arm is unconstructible, so the mutant is still owed as a demonstration that the current design fails. | spec/decisions/ADR-005-original-guardian-gate-ownership.md:148 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| ADR-005-control-fork | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:148 | \| Foreign-fork alias \| Other C thread forks without exec while S, R and the lease are held; a handler on the owner thread forks; owner guard omitted \| The child cannot retain or operate the original authorities; actual C or group death still tears down I. The shared-table mutant fails. \| UNRUN; cannot be constructed safely while P1 fails \| |
| ADR-005-control-fatal-O | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:149 | \| Fatal O before C's held-gate observation \| O exits between I claim and C's observation; O or C releases or shuts down early \| C custody survives O loss, a dead or mismatched identity gains no positive admission, early gate progress fails with no backend marker \| UNRUN \| |
| ADR-005-control-early-close | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:150 | \| Live early close \| C closes S before actual I confirmation, or omits confirmation \| Cooperative I-before-last-close passes; each mutant fails; withheld confirmation to the original cutoff yields only bounded AC-38 abandonment \| UNRUN \| |
| ADR-005-control-peer-loss | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:151 | \| Peer loss on the NOSIGNAL release \| Peer disappears during C's send; second successful send; partial permission; NOSIGNAL omitted \| EPIPE with no SIGPIPE, no second release, original stop and cutoff remain active, permission is neither release nor Ready. A held R alias in C's window makes the send succeed and must fail this control. \| UNRUN \| |
| ADR-005-control-charge | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:152 | \| Omitted native or kernel charge \| One socket, skb, in-flight right, stack or table term dropped from the peak, or one Start right wrong, extra or truncated \| The pre-exposure comparison fails rather than borrowing another ceiling; every received or pending right is disposed after refusal \| UNRUN; the two non-derivable terms have no oracle to compare against \| |

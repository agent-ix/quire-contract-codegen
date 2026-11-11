---
id: SR-17940486709564847047616
title: "IR-717 PR #775 ADR-005 EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
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

PASS. The diff touches only an ADR. It adds no FR/NFR/StR requirement statement and no SHALL clause (the 'must' occurrences are quoted rustix text, a kernel statement, and a control-table header), so no EARS grammar applies. The record correctly makes no FR-034 statement edits.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| ADR-005-status | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:17-24 | Proposed, awaiting a product architecture choice. This is the IR-717 decision record, not an implemented gate, a conformance finding, or permission to activate IR-652 or IR-655. IR-717 could **not** establish a supported safe descriptor owner or a finite kernel/native bound for the original-gate design. The exact failed premises are stated below and the choice they force is in [Decision required](#decision-required). [FR-034](../kani/functional/FR-034-caller-death-ownership.md) and [TC-049](../kani/matrix/TC-049-caller-death-ownership.md) are not amended by this record: no decision is establis |
| ADR-005-P3-nonderivable | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:107-115 | Not derivable from any userspace API before exposure: the kernel socket object and its slab rounding for both endpoints, `sk_buff` truesize and allocation rounding, the per-message in-flight right list the kernel allocates for each ancillary message, and the in-flight accounting for queued rights. `SO_MEMINFO` and cgroup `memory.stat` report these only after the objects exist. The in-flight right count is additionally compared with a per-user counter that C does not own, so a pre-exposure reservation cannot own it and its exhaustion must be a bounded refusal (statement from kernel source, not  |
| ADR-005-option-4 | examined | spec/decisions/ADR-005-original-guardian-gate-ownership.md:164 | \| 4. Authorize a process-wide no-fork rule for embedding C \| Embedding C must not fork while a guardian run is live. \| Changes ordinary embedding behavior; the ticket forbids assuming it. \| |

---
id: ADR-005
title: "Hold original guardian-gate activation pending a supported descriptor owner and finite reserve"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
---
# ADR-005: Hold original guardian-gate activation pending a supported descriptor owner and finite reserve

## Status

Proposed. This is an architecture decision for IR-717, not an implemented gate, a conformance
finding, or permission to activate IR-652 or IR-655. The supported owner API and complete finite
reserve described below are **UNBACKED**. FR-034 and TC-049 remain the normative contracts.

## Context

[FR-034](../kani/functional/FR-034-caller-death-ownership.md) currently gives O the exclusive
block-gate pipe writer and M the blocking reader. C's cached phase cannot establish that O still
holds the gate if O exits at C's reporting boundary. Main's
`src/kani/run/namespace.rs` likewise creates the gate pipe in its namespace owner and releases it
with a write. That source predates the planned C/L/O/M/I construction and supplies no contrary
custody evidence. The existing requirement explicitly marks the exact fatal-prefix gate predicate
UNBACKED.

The proposed replacement would keep the original release endpoint S and the original caller lease
under C's physical custody, transfer only blocking receiver R through the existing authenticated
C→L and L→O Start carriers, and let C issue one checked release after O's ordinary tick and
permission. The proposed one-way connected Unix stream permits a per-send NOSIGNAL operation;
genuine I Ready remains a later, separately authenticated condition for Backend. This is a
candidate operational graph, not a supported mechanism. No additional acknowledgement, controller
pause, stored gate certificate, fixture-only authority or post-return custodian is allocated.

## Decision

Hold the strong original-gate construction **UNBACKED**. Do not amend FR-034 or TC-049 to claim
the proposed replacement as supported, and do not activate IR-652 production or IR-655 fixture
claims from endpoint possession, an O permission, a phase, an old receipt, a Boolean or eventual
absence of a backend marker. An operational amendment requires an independently reviewed owner
contract and an additive finite reserve matching the actual supported build and kernel profile.

Three premises remain open:

| Premise | Current result | Required operational evidence |
| --- | --- | --- |
| Exclusive original S and caller lease through all supported C forks | **UNBACKED.** A shared C file table lets an unrelated embedding thread fork a child retaining S or the lease before exec. CLOEXEC does not remove that pre-exec alias. A thread-private table excludes forks from other tables but still permits a handler or callback on its own thread to fork. | Name the actual safe Rust/OS owner API and audit its complete descriptor affinity, table preparation, signal/native callback and Drop paths. Show that S and lease never enter another table, including a fork from the owner thread's signal handler, while ordinary forks from other C threads remain supported. L must finish its own fork/exec before S or lease creation in the private table, or an equally supported exclusion must be shown. |
| C-held gate through fatal O and live close | **UNBACKED.** O's present writer can disappear before C's held-gate observation. A changed C owner could survive O exit, but its release, actual last close and original caller/group-death behavior have not been established. | Identify each real C/L/O/M/I owner and close operation across both Start transfers, queued/partial rights, setup failure, ordinary release, fatal O/C, cancellation and cleanup. Prove O's complete ordinary tick and stop precedence before permission, C's one successful bounded NOSIGNAL byte, genuine I Ready before Backend, and actual I confirmation before successful cooperative last close. Preserve the original cutoff and cause precedence. |
| Finite simultaneous stream and native reserve | **UNBACKED.** A one-byte payload, SO_SNDBUF, sampled RSS or requested thread-stack size does not bound socket objects, queues, ancillary references, fd tables and native frames. The candidate fifth Start right exceeds the current four-right transport maximum. | For the exact supported build/kernel/runtime, enumerate and bound both gate endpoints and file/socket objects, queue/skb allocation and rounding, sender/receiver SCM rights and partial transfers, both Start phases, the private table and its preparation, native stack/TLS/runtime work, and every overlapping existing control/report/capture allocation. Reserve their checked additive maximum before L exposure, update the common ancillary maximum and both Start counts together, preserve every other message's exact count, and prove error disposal under the original cutoff. |

FR-034-AC-38's genuine unavailable-confirmation result remains bounded CleanupUnconfirmed without
evidence or cleanup credit. FR-034-AC-54/87 require actual original-I confirmation before the
successful cooperative retained-gate close; the separate original-cutoff abandonment path earns no
operation-order credit. FR-034-AC-83 still requires an exclusive original caller lease, and
FR-034-AC-94 still applies to pending or partial startup receipts. A permission cannot be treated
as GateReleased, I Ready or Backend to evade that boundary. TC-049's early-close,
omitted-confirmation, lease-alias and observation-withholding controls remain independently owed.
No error label converts a live early close into a confirmed cooperative close.

## Evidence needed to choose a supported architecture

All controls in this table are **UNRUN**. Each must use the original C/L/O/M/I roles and actual
descriptors, fail its own predicate before emergency cleanup, and include a restored healthy
control. An omitted or unavailable construction earns no Test credit.

| Falsifying control | Predicate it must distinguish |
| --- | --- |
| Other C thread forks an unexeced child while S and the lease are held; an application signal handler on the actual owner thread also forks | The child cannot retain or operate either original authority; the shared-table or omitted-owner-guard mutant fails. Actual C/group death still tears down original I and its descendants. |
| O exits fatally between original I claim and C's held-gate observation; C or O sends/shuts down the gate early | Actual C custody survives O loss, but a dead/mismatched identity cannot gain positive admission. Early gate progress fails even if no backend marker appears. |
| Complete O tick and permission, one C release, genuine I Ready; peer disappears during C's NOSIGNAL send | Permission is neither release nor Ready. A second successful send, partial permission, premature Ready, omitted NOSIGNAL or SIGPIPE-driven C death fails independently; original stop/cutoff remains active. |
| Live C closes early or omits actual I confirmation; observation is separately withheld to the original cutoff | Healthy cooperative I-before-last-close passes; each early-close mutant fails. Genuine unavailable confirmation yields only bounded AC-38 abandonment without order, cleanup, kernel-fault or fresh-deadline credit. |
| One Start right is wrong, extra, truncated or left in a queued/partial transfer; one kernel/native/ancillary term is omitted from the peak | Every received or pending right is owned and disposed after refusal. The omitted charge fails the pre-exposure comparison rather than borrowing an unrelated ceiling. |

An architecture reviewer must inspect the safe API and source-derived bound, then the separate
specification reviewer must evaluate the exact FR-034/TC-049 amendment before implementation. The
former is currently unavailable: no existing safe owner boundary has been established for this
complete operation graph, and no numerical kernel/native maximum has been derived. This is a
product architecture choice if the owner context or supported profile cannot be provided. The
choice is to allocate and prove that context and reserve, or retain this explicit hold; it is not
an implicit narrowing of the current caller/group-death guarantee.

## Consequences

IR-717 remains an open prerequisite to IR-652 and the later completion audit. IR-652 owns the
actual C/L/O/M/I and transport implementation; IR-655 owns genuine fixture evidence. The proposed
five-right Start and stream transport do not enter the normative protocol while the premises above
are unbacked. No compatibility route, copied dependency file, fabricated reserve or source-only
success claim follows from this decision.

## Alternatives Considered

- **A: private descriptor table on an audited C owner thread.** This can exclude ordinary forks
  from other C threads only if the complete owner runtime is safe and all operations stay on that
  thread. Signal masking by itself leaves reserved native signals and callbacks; fd-affinity and
  bounded teardown are also open. No current safe API for the whole boundary is established.
- **A-prime: A with owner-local no-fork enforcement.** A kernel policy might also exclude handler
  forks on the owner, while allowing other C threads to fork. It still needs a supported policy
  installation, complete syscall coverage, descriptor affinity and all native/resource bounds.
  It is not selected or measured.
- **Shared-table or O-only custody.** Unrelated C forks can inherit S or the original lease in a
  shared table. O-only release loses the live gate when O exits before C's observation. Neither
  supports the strong held-gate predicate.
- **A separate release custodian or a process-wide no-fork rule.** These change original C custody,
  death semantics or ordinary embedding behavior and require an explicit product decision. They
  are not allocated by this ADR.

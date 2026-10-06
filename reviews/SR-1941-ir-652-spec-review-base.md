---
id: SR-1941
title: "IR-652 lifecycle spec review (base)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-049-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1941: IR-652 lifecycle spec review, base

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080.
Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run
5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. Overall consistency and correctness review of the outer
containment allocation (C, L, O, M, I) and the unnamed report storage. The author receipt and
ticket text were treated as claims and re-measured. One high and three medium findings.

## Method

Read the full diff and the surrounding text of FR-017 (Behavior, Acceptance Criteria) and FR-034
(Description, Behavior, Stage table, Run artifact and report lifetime, Acceptance Criteria,
Dependencies). Compared the new norms with unchanged FR-017 text, FR-028-AC-3, current code
(`src/kani/run/report_file.rs` REPORT_LIMIT and TooLarge refusal, `src/kani/run/namespace.rs`
bwrap flags) and the scratch probe receipts under /tmp/ix-handoff/ir652-lifecycle-probe. Read the
host values `apparmor_restrict_unprivileged_userns=1` and `max_user_namespaces=256363`. Ran
`quire validate` on each changed file: all pass, with only catalog diagnostics and one unchanged
FR-017 line-167 EARS warning.

Scope units examined: FR-017 Behavior bullets at lines 96 to 116, FR-017-AC-18, FR-017-AC-19,
FR-034 Description (lines 25 to 39), Behavior and Outer containment (lines 64 to 107), the
Bootstrap stage row, Run artifact and report lifetime (lines 466 to 497), FR-034-AC-2, 5, 7, 12,
14, 17, 23, 26 and 31 to 34, TC-027 Unnamed report storage, and TC-049 steps 5, 7, 15 and 16.
Context only: FR-017 line 235 and FR-028-AC-3.

Clean on examination:
- Single ownership of each role.
- O armed before any M spawn, and a pre-arm L loss gives exclusive bootstrap EOF refusal.
- Kernel outer teardown independent of the inner claim.
- No host scan, reused PID or monitor proxy.
- AC-24's ignored-EOF mutant is kept and protected from outer-kill masking (FR-034-AC-34).
- Explicit statement that the pipe is incompatible with no runtime fallback, compatibility
  reader or named mode.
- No new hash, pin, vendored source or unsafe exception.

## Verdict

**FAIL: one high and three medium findings.** The topology and refusal model are coherent, and
the UNRUN gates are labelled honestly. The report-overflow outcome contradicts unchanged FR-017
text and current behaviour. Absent-report semantics, the nested bwrap recipe and the availability
cost of caller-side user namespaces are not specified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new FR-017 bullet and AC-19 make an over-16-MiB report yield `Incomplete(ResourceExhausted)`. Unchanged FR-017 line 235 says a report "over the read bound" is "a typed refusal with a stable cause and never an outcome". The current code refuses with `KaniReportRefusal::TooLarge`, and the old AC-19 said "refused and not truncated". The author receipt does not list this as a norm delta. It also names a QSL final outcome rather than a `KaniRunOutcome` and reason at FR-017's layer: FR-028-AC-3 says the run is Inconclusive with a reason, and FR-029 maps that reason to `Incomplete(ResourceExhausted)`. State one rule: either keep refusal, or define the run outcome and reason and amend line 235. | spec/kani/functional/FR-017-kani-execution-evidence.md:106-110, spec/kani/functional/FR-017-kani-execution-evidence.md:235, spec/kani/functional/FR-017-kani-execution-evidence.md:278, spec/kani/functional/FR-034-caller-death-ownership.md:484-485 |
| FND-002 | medium | Absent-report semantics are undefined for the pipe carrier. Today "absent" means the file does not exist: a successful exit is then refused and an unsuccessful exit is `NoVerdict`, under FR-017-AC-18, AC-21 and line 235. A pipe always exists. The spec does not say that zero bytes at actual EOF is "exported no report", nor how a partial write before backend death is classified. | spec/kani/functional/FR-017-kani-execution-evidence.md:101-105, spec/kani/functional/FR-017-kani-execution-evidence.md:278 |
| FND-003 | medium | The nested bwrap recipe for M is not normative. #295 runs bwrap with `--die-with-parent`, `--info-fd` and `--block-fd` and C as parent (src/kani/run/namespace.rs:132-144). In the new topology O is M's parent. Line 26 says the inner bubblewrap is "unchanged", while AC-14 dropped the #295 flag list. Nothing states whether `--die-with-parent` stays, or whether C or O owns the info and block descriptors (the "info-reader" whose failure the Bootstrap row names). Two implementers could build different recipes. State the argv, the descriptor owners and the cost of each flag. | spec/kani/functional/FR-034-caller-death-ownership.md:25-33, spec/kani/functional/FR-034-caller-death-ownership.md:189, spec/kani/functional/FR-034-caller-death-ownership.md:516 |
| FND-004 | medium | The availability cost is not stated. In #295, bwrap creates the user namespace under bwrap's own AppArmor profile. Under the new allocation the caller-side L must unshare NEWUSER and NEWNS and mount proc itself. This host has `apparmor_restrict_unprivileged_userns=1`. The scratch probes ran only under the agent-specific `codex-node` label, not as an ordinary unconfined process. Where that restriction applies to the caller, every bounded run would get the typed refusal where #295 runs today. The refusal is correct, but the spec and setup criterion should state this regression and its cost, and a real ordinary-caller measurement is needed before CODE. | spec/kani/functional/FR-034-caller-death-ownership.md:86-99, spec/kani/functional/FR-034-caller-death-ownership.md:516 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The zero-byte EOF rule keeps an independently established resource or deadline stop ahead of the missing-report rule, but the nonempty rule says a partial or malformed write "refuses regardless of process exit" with no such precedence. A deadline or memory-ceiling kill that lands while Kani writes its report (once, at the end) leaves partial bytes, and the rules disagree: timed-out or memory-exhausted Inconclusive versus report refusal. State that the resource or deadline stop takes precedence, as the zero-byte rule does. | spec/kani/functional/FR-017-kani-execution-evidence.md:104-110 |
| FND-006 | low | The normative recipe maps the report writer at N >= 5 because fd 3 and fd 4 carry info and gate, but the FR-017 locator bullet, FR-017-AC-19 and FR-034-AC-17 still say N >= 3. An implementer following FR-017 may pick 3 or 4 and collide with the info or gate mapping. Make one bound normative everywhere, or state that the backend-side N is independent of M's descriptor table. | spec/kani/functional/FR-034-caller-death-ownership.md:119-120, spec/kani/functional/FR-017-kani-execution-evidence.md:96-100, spec/kani/functional/FR-034-caller-death-ownership.md:568 |

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Overflow is single-run KaniRunOutcome::Inconclusive/MemoryExhausted (exists in src/kani/classify.rs:49), batch keeps FR-028-AC-21 whole-batch memory-exhausted refusal, FR-029 maps ResourceExhausted; line 235 no longer lists over-read-bound refusal and names the delta. |
| FND-002 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Zero-byte EOF is no exported report (success refuses, failure NoVerdict); nonempty partial or malformed bytes refuse. |
| FND-003 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Normative nested argv matches namespace.rs minus --die-with-parent, with its reason; O owns info reader and gate writer; fd 3/4 and report N mapping stated. |
| FND-004 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Availability regression stated; ordinary-caller label/errno measurement is a required CODE gate (FR-034-AC-14, AC-31, TC-049 step 17). |

Round 2 reviewed 6f552cd97c8a6915d999e03d49129ebc3c198195 (previous a661f2f297f5e9860c25e9995226d07332426ab2; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 04443041-0ffa-49c7-a688-2004c61268ae. Only the disposition-pass-1 findings were open; every original finding's latest row is already fixed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 6f552cd97c8a6915d999e03d49129ebc3c198195: Nonempty partial/malformed bytes now yield to an independently established memory/resource or deadline stop, keeping the single-run outcome and the whole-batch refusal with no member classified; FR-017-AC-19, FR-034 report-lifetime prose and TC-027 agree. |
| FND-006 | fixed | 6f552cd97c8a6915d999e03d49129ebc3c198195: Every report-writer bound is N >= 5 (FR-017 bullet and AC-19, FR-034 lines 119 and 495, FR-034-AC-17, TC-027); the only remaining >= 3 is the unrelated fixture auxiliary descriptor (FR-034:328, TC-049:49). |

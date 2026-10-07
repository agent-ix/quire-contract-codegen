---
id: SR-3102
title: "IR-689 spec review (integrity): feature-only stage-2 O observation transport"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir689-stage2-observation (frozen head named in the Linear marker; one author commit plus an ordinary merge of main); spec/kani/functional/FR-034-caller-death-ownership.md section 'Feature-only stage-2 observation transport' (lines 1453-1549) and AC-78..AC-82; spec/kani/matrix/TC-049-caller-death-ownership.md (allocation lines 183-199, step 29, expected rows 1266-1274); spec/kani/matrix/tests.md; context: FR-034 opt-in fixture section (lines 512-541), stage-2 section (lines 1262-1361), run artifact charge formula (lines 1390-1405), supported-build native workspace (lines 1551-1591), AC-8, AC-10, AC-27, AC-29, AC-30, AC-41..AC-77"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-689. This pass checks the new slice for completeness, consistency with merged FR-034 text and traceability integrity.

Recorded clean:

- **Ids.** FR-034-AC-78..AC-82 are new and disjoint from AC-51..AC-77. No other document on main or on the guardian review source uses them; the FR-015 AC-78..81 ids are a different FR.
- **Matrix delta.** The computed matrix TSV grows from 615 to 620 criterion records, additions only. AC-78..AC-81 compute `untagged`. AC-82 computes `method-without-symbol`, the same as the existing Analysis-only rows.
- **TC rows.** AC-78..AC-82 appear in both TC-049 tables where AC-41..AC-50 and AC-57..AC-77 appear (the evidence allocation and Expected Results), in the procedure (step 29a-e), and in both tests.md rows (FR coverage and TC-049 index).
- **No forbidden content.** The three changed documents have no 7+ hex identifier, local path or conflict marker. `quire validate` on the three files and `make spec` both pass. The only `make spec` warnings are two pre-existing EARS warnings at FR-017 line 174.
- **Consistency with IR-687 and IR-682.** The binding envelope is a separate feature-only envelope with exactly one write right and no read right. It leaves the IR-687 exact-rights frames, the O-to-L zero-right negative and the L-to-C pidfd clone untouched. AC-82's "supported consumer proof configuration" is the IR-682 feature-off configuration (FR-034:1569-1570). AC-8 and AC-10 are restated as unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The slice contradicts merged normative text it does not amend. FR-034:517-519 says "The feature shall add no branch inside a production stage". FR-034:536-537 forbids any "production feature branch, hook or replacement transition". FR-034:539-540 says stage publication "shall add no I/O, callback, blocking handoff". The new slice requires O to write a fixed frame at the actual I poll-IN, M reap, seal, gate-close and ordinary-tick boundaries (FR-034:1490-1504), and those boundaries are inside O's production stages. FR-034:1541-1543 then concedes that "producer/reader branches" exist and must be absent only when the feature is off. The only qualification offered (FR-034:1466-1467) covers the external-right exclusion, not the no-branch or no-I/O rules. As written, a conforming implementation cannot satisfy both sections. | spec/kani/functional/FR-034-caller-death-ownership.md:1490-1504; spec/kani/functional/FR-034-caller-death-ownership.md:517-519; spec/kani/functional/FR-034-caller-death-ownership.md:536-540; spec/kani/functional/FR-034-caller-death-ownership.md:1541-1543 |
| FND-002 | medium | AC-81 requires charging the actual native workspace of the new observation storage and transport before exposure. That charge happens only in the feature-on configuration. The IR-682 supported-build native-workspace proof is scoped to feature-off production, and its initial input names "no verification feature" (FR-034:1569-1577). TC-049's AC-81 evidence boundary says "no guessed capacity or native-stack proof" (TC-049:198), and step 29d gives no native-workspace measurement route. AC-81's native-workspace term therefore has no evidence route. It can only stay UNPROVEN, or be closed by a guess the spec forbids. | spec/kani/functional/FR-034-caller-death-ownership.md:1905; spec/kani/functional/FR-034-caller-death-ownership.md:1531-1534; spec/kani/matrix/TC-049-caller-death-ownership.md:198 |
| FND-003 | medium | "Actual pipe reservation under the existing formula" names no formula term. The existing charge is `owned_RSS + caller_run_buffers + pipe_reserve + memfd_reserve`. `pipe_reserve` is defined as the report pipe's page-rounded F_GETPIPE_SZ measured before writer exposure (FR-034:1390-1397). One implementer adds the observation pipe to `pipe_reserve`, another folds it into `caller_run_buffers` as a named per-run C control, and a third treats it as an unnamed extra term, which the "no new resource allowance" rule forbids. The spec also does not say that this feature-on-only charge may move a boundary run into MemoryExhausted only in feature-on builds. That is a test-versus-production divergence the harness should expect, not discover. | spec/kani/functional/FR-034-caller-death-ownership.md:1531-1535; spec/kani/functional/FR-034-caller-death-ownership.md:1390-1397 |

## Verdict

**Changes requested (high).** Explicitly amend FR-034:517-519, 536-537 and 539-540 for the feature-on O emission sites. Either define emission so that production stages hand an unconditional typed boundary fact to a cfg-gated sink outside the stage, or state the exception and its bounds. Give AC-81's native-workspace term a real feature-on evidence route, or drop it from AC-81 with an explicit UNPROVEN allocation. Name the formula term that charges the observation pipe.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | Merged AC-24 says "fixture-written frames are forbidden". The slice now has C, in fixture mode, write a new Stage2ObservationBinding frame, carrying a right, on the EXISTING trusted C/O control transport. The slice qualifies the DTO/right exclusion and the no-branch/no-I/O rules, but not AC-24. Read literally, the two conflict. A CODE reviewer enforcing AC-24 would reject the binding frame, and one enforcing the new section would accept it. | spec/kani/functional/FR-034-caller-death-ownership.md:1472-1475; spec/kani/functional/FR-034-caller-death-ownership.md:1892 |
| FND-005 | medium | The new exception is too narrow for what the slice requires of O. It allows "a cfg-gated nonblocking observation sink" at "these named event boundaries only" and calls itself "the sole exception" to the no-production-stage-feature-branch rule. The slice also requires other feature-on branches inside O's production stages: (1) decoding and authenticating the binding envelope in O's existing C/O receive cursor while I lives; (2) installing the feature-only SIGPIPE suppression before the first write; and (3) the parent-side M-reap probes (SR-3100 FND-004). None of these is a sink at an event boundary. FR-034:517-519 therefore still contradicts the required O receive and setup branches. | spec/kani/functional/FR-034-caller-death-ownership.md:1460-1465; spec/kani/functional/FR-034-caller-death-ownership.md:1480-1481; spec/kani/functional/FR-034-caller-death-ownership.md:1499-1503; spec/kani/functional/FR-034-caller-death-ownership.md:1558-1562; spec/kani/functional/FR-034-caller-death-ownership.md:517-519 |

## Dispositions

Round 1 was re-checked at the branch's round-1 fix head (subject 'Resolve IR689 observation selection and independent evidence obligations'; head named in the Linear marker only). The check was static and read-only. make spec passes, and the computed matrix grows from 615 to 631 records, additions only.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': FR-034:1460-1465 explicitly excepts cfg-gated nonblocking sinks at the named event boundaries from the earlier no-production-stage-feature-branch and no-I/O rules. Stage publication stays unconditional and without I/O, feature-off stages and frames are unchanged, and no sink may change control, cancellation, sampling or cutoffs. The residual scope gap is raised as FND-005. |
| FND-002 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': FR-034:1577-1583 and the new AC-93 (TC step 29p) require a separate feature-on paired-artifact native-workspace Analysis with an independently declared charge, without expanding IR-682's supported configuration. Missing capacity or highwater leaves it UNPROVEN. |
| FND-003 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': in feature-on bound scenarios, pipe_reserve is the sum of the report-pipe reserve and the observation pipe's page-rounded F_GETPIPE_SZ, under the same no-growth obligation. C storage goes in caller_run_buffers, O storage in owned_RSS, and there is no separate term. A feature-on-only MemoryExhausted shift is stated and may not be hidden (FR-034:1569-1575). |

Round 2 was re-checked at the branch's round-2 fix head: six commits ahead of main, with subject 'Clarify stage-two parent observations and binding prerequisites'. The head is named in the Linear marker only. The check was static and read-only. make spec exits 0, and its only warnings are the two older ones at FR-017 line 174. The computed matrix has 631 records: the 16 additions plus the intended AC-24 text amendment, which stays untagged. No 7+ hex string, local path or timestamp appears in the committed diff.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | fix commit 'Clarify stage-two parent observations and binding prerequisites': AC-24 is the only merged base row amended, and it is amended in place. Fixture-written frames stay forbidden except the explicitly allocated guardian-test-support Stage2ObservationBinding on the existing C/O control transport. That exception 'cannot fabricate or replace the original I/C Dispatch frame', so the prohibition on fixture-written Dispatch is not weakened. The other 76 base rows are unchanged (FR-034:1913). |
| FND-005 | fixed | fix commit 'Clarify stage-two parent observations and binding prerequisites': the sole feature-on exception now lists exactly what it covers: C's binding emission, O's binding decode, authentication, receive and right retention, SIGPIPE setup, O's parent-side M checks, and event capture and nonblocking writes with the fault latch. All of these are absent from feature-off consumers and helpers, and none may alter cancellation, sampling, classification, ownership, cutoffs or control flow (FR-034:1459-1468). It grants no feature-off effect and no cancellation authority. |

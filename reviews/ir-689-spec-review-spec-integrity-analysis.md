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

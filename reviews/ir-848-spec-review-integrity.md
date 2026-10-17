---
id: SR-12191
title: "Integrity review of IR-848 FWC cause in the FR-029 terminal map"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@9bdac7e40c35f73d42cb37fd02617a8dabe741c5 (PR 422, diff origin/peter/ir-827-cbmc-failure-cause...HEAD); spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-848. PR agent-ix/quire-contract-codegen#422 at head 9bdac7e40c35f73d42cb37fd02617a8dabe741c5, stacked on #410 (peter/ir-827-cbmc-failure-cause); only the stacked diff is reviewed. Planner rulings (option b, G1, G2, G4) are taken as the intent.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The exhaustiveness rule is not extended to the new input. FR-029 Behaviour says the one function's `match` over "the pair (`KaniRunOutcome` with `KaniInconclusiveReason`, replay settlement) has no wildcard arm". The map now also matches `Option<&KaniFailureCause>`, and nothing requires the FWC arm to name every `KaniFailureCause` variant. `KaniFailureCause` is not `non_exhaustive` (report.rs:383), so a wildcard in the FWC arm would silently send a future variant to `Failed` or `Incomplete`. State that the FWC arm matches the cause exhaustively with no wildcard (the other arms may ignore it, per AC-34), and have AC-35's exhaustive table fail on a new variant. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:116-118, :382; src/kani/output/report.rs:383 | missing-requirement |
| FND-002 | medium | FR-029-AC-35 is false as worded. It says "over every pair of outcome and cause ... no result is `Refuted`", but `falsified` with a reproduced settlement is `Refuted` whatever cause is passed (terminal.rs:136; AC-7 row). The settlement axis is unstated: with none, falsified is `MissingSettlement`; with reproduced, it is `Refuted`. Restate as: for every outcome, settlement and cause, the value equals the value with no cause except for the FWC rows of AC-32/AC-33, so no cause creates or removes a `Proved` or `Refuted`. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:382; src/kani/terminal.rs:136-142 | wrong-requirement |
| FND-003 | low | Trace edges added on one side only. FR-029-AC-33 and the new tests.md row cite TC-043, but TC-043 has no step or expected result for FR-029-AC-33 (it is not in this diff). Amended FR-017-AC-40 cites TC-040, but TC-040 names no FR-017 criterion and its tests.md summary row (:112) lists none of FR-029-AC-32..35. Add the TC-043 step (or drop TC-043 from AC-33 since TC-040 step 5 builds the batch evidence), and add AC-32..35 and FR-017-AC-40 to TC-040's summary row. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:380; spec/kani/matrix/tests.md:59, :112; spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md | wrong-requirement |
| FND-004 | low | Branch order differs from the pre-scope. The pre-scope said the IR-848 spec PR goes after #418 merges because both edit FR-017, interface-001 and TC-027. This branch is stacked on #410 without #418; `git merge-tree` with #418 (d9cf2b0) conflicts in FR-017's AC table where AC-40 (amended here) sits beside AC-41..AC-44 (added there). The resolution is mechanical (keep both), but the last-merged PR must re-check that #418's FR-017 :654-661 wording still agrees with FR-029's rationale after the rebase. | spec/kani/functional/FR-017-kani-execution-evidence.md:654 | wrong-requirement |

## Scope examined

Examined (role examined): FR-029 Inputs cause bullet (:81-91), the no-settlement table (:136-146), the rationale bullets (:155-183), the bounded-shadow restatement (:240-246), FR-029-AC-5, AC-25, AC-32 to AC-35, the IR-848 status note (:412-414); FR-017 behaviour bullet (:565-571), rationale (:596-607), FR-017-AC-40; interface-001 run_terminal_value inputs and failed_without_counterexample_cause; TC-040 steps/expected 5 and Status; TC-027 coverage note (:387-392); tests.md FR-029 AC-32..35 row. Context only: src/kani/terminal.rs:90-148 (run_terminal_value, FWC arm :127-135), src/kani/classify.rs:127-223 (classify_kani_run, classify_member timeout path :172-176, classify_report), src/kani/output/report.rs:383-424 (KaniFailureCause: PropertiesFailed, Timeout, OutOfMemory, ExitCode, Unknown; not non_exhaustive), src/kani/run/execute.rs:495-525 (cause retained only for FWC), every run_terminal_value caller (tests/it/terminal_map.rs:76,82,89,266,271; kani_obligations.rs:2236,2378; kani_obligations_state_clause_replay.rs:688,1151; kani_obligations_state_frame.rs:2884,3062; no src caller), /home/peter/dev/quire-driver/src/provider.rs:89 (read-only), CG #418 branch peter/ir-847-native-error-discriminant-spec@d9cf2b0 FR-017 :654-661 and AC-42/AC-44 (class table).

## Verdict

Needs changes (medium). The new table, AC-5, AC-25 and the bounded-shadow text are mutually consistent and match FR-017's amended bullet and interface-001's signature (outcome, cause, SUCCESS count, settlement, matching the planned signature). The structural gaps are the unstated no-wildcard rule for the cause, AC-35's settlement axis, and one-sided trace edges.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-005 | medium | The new signature takes the SUCCESS-check count twice. `run_terminal_value(evidence: &KaniExecutionEvidence, success_checks, settlement)` reads the outcome and cause from the evidence but keeps `success_checks` as a separate argument, although FR-017 :347 requires the evidence to carry that count and `KaniExecutionEvidence.success_checks` exists (src/kani/run/execute.rs:276-280; the driver and the IR-827 tests already pass `evidence.success_checks`). The fix-round note's reason that the evidence does not carry it is not true at this head. With two sources, a caller can pass a count that disagrees with the evidence, and `Proved { success_checks }` then states a count the evidence contradicts: the same hazard the evidence input was added to remove for the cause. Read the count from the evidence too (`run_terminal_value(evidence, settlement)`), or state why a separate count is needed and require it to equal `evidence.success_checks`. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:83-89; spec/core/functional/interface-001-codegen-api.md:149; spec/kani/functional/FR-017-kani-execution-evidence.md:347; src/kani/run/execute.rs:276-280 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | d536e94: FR-029 :120-122 requires the FWC arm to match `KaniFailureCause` (or its absence) exhaustively with no wildcard arm, so a new cause is a compile error. |
| FND-002 | fixed | d536e94: AC-35 states the settlement axis ("with the replay settlement each outcome takes: a falsified outcome with a reproduced replay is Refuted whatever the cause; the cause never turns a non-Refuted result into Refuted"); TC-040 step 5 adds falsified+reproduced beside each cause. |
| FND-003 | fixed | d536e94: TC-043 adds step 13 and expected result 13 for c::check/d::check (FR-029-AC-33); the TC-040 summary row lists FR-029-AC-32..35 and FR-017-AC-40; AC-36 is Inspection with no TC, as the FR-029 row says. |
| FND-004 | fixed | d536e94: rebased onto #418 (d9cf2b0) and the PR base changed to it. No conflict markers remain; FR-017 AC-40 (amended) sits at :730 directly before #418's AC-41..44 (:731-734); #418's Scalar native error discriminant section is byte-identical to d9cf2b0. Its class rule agrees with FR-029's terminal map; the remaining wording gap is SR-12192 FND-003. |

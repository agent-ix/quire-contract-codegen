---
id: SR-12190
title: "Spec review (base) of IR-848 FWC cause in the FR-029 terminal map"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@9bdac7e40c35f73d42cb37fd02617a8dabe741c5 (PR 422, diff origin/peter/ir-827-cbmc-failure-cause...HEAD); spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-848. PR agent-ix/quire-contract-codegen#422 at head 9bdac7e40c35f73d42cb37fd02617a8dabe741c5, stacked on #410 (peter/ir-827-cbmc-failure-cause); only the stacked diff is reviewed. Planner rulings (option b, G1, G2, G4) are taken as the intent.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-017 behaviour bullet contradicts itself. Its unchanged first sentence says the generator computes "the FR-029 terminal value exactly as the rules above state, before and independently of the cause and the diagnostic record"; the new sentences added in the same bullet say "Its FR-029 terminal value is read from the cause by the terminal map alone". Both cannot hold. Drop "and the FR-029 terminal value" from the independence list (keep outcome, checks, SUCCESS count, playback, refusals) and keep the new sentence as the sole rule for the terminal value. | spec/kani/functional/FR-017-kani-execution-evidence.md:565-571 | wrong-requirement |
| FND-002 | high | TC-027 expected result still asserts the old value. Its CBMC-failure-cause expected result says the three runs differing in cause (exit_code_64, out_of_memory, cbmc_exploded) have "FR-029 terminal value `Failed` with and without the two members". Amended FR-017-AC-40 now requires out_of_memory to give `Incomplete(ResourceExhausted)`. Only the coverage note (:387-392) was updated; the expected result a tester checks against was not. Rewrite :247-248 to match AC-40 (exit_code_64 and cbmc_exploded `Failed`, equal with and without the members; out_of_memory `Incomplete(ResourceExhausted)`). | spec/kani/matrix/TC-027-kani-execution-evidence.md:243-249, :387-392; spec/kani/functional/FR-017-kani-execution-evidence.md:654 | wrong-requirement |
| FND-003 | medium | The matrix still marks FR-017-AC-40 `✅ Covered (IR-827)` against TC-027 only. The amended AC-40 asserts out_of_memory gives `Incomplete(ResourceExhausted)` and cites TC-027 and TC-040, while the bound test `tc_027_the_terminal_value_of_a_cbmc_failure_ignores_its_cause` (tests/it/kani_obligations.rs:2326-2378) asserts `Failed` for out_of_memory and calls `run_terminal_value` with no cause. The row claims coverage of a criterion its test contradicts. Split AC-40 out as 🚧 Partial/Planned (IR-848) naming the terminal half, as the new FR-029 row does, and list TC-040. TC-027's own Status (:382) should say the same. | spec/kani/matrix/tests.md:47, :104; tests/it/kani_obligations.rs:2326-2378 | wrong-requirement |
| FND-004 | low | Planned status is not marked where the convention puts it. FR-029-AC-32 to AC-35 carry no `PLANNED (IR-848).` prefix, unlike AC-21..AC-31's `PLANNED (IR-806)`, although the status note (:412) and the tests.md row call them planned. The Inputs bullet (:80-82) also says `run_terminal_value` "takes ... today" the CBMC failure cause, which it does not at this head (terminal.rs:90-94). Add the prefix and say "after IR-848" instead of "today". | spec/kani/functional/FR-029-run-outcome-terminal-record.md:80-82, :379-382; src/kani/terminal.rs:90-94 | wrong-requirement |

## Scope examined

Examined (role examined): FR-029 Inputs cause bullet (:81-91), the no-settlement table (:136-146), the rationale bullets (:155-183), the bounded-shadow restatement (:240-246), FR-029-AC-5, AC-25, AC-32 to AC-35, the IR-848 status note (:412-414); FR-017 behaviour bullet (:565-571), rationale (:596-607), FR-017-AC-40; interface-001 run_terminal_value inputs and failed_without_counterexample_cause; TC-040 steps/expected 5 and Status; TC-027 coverage note (:387-392); tests.md FR-029 AC-32..35 row. Context only: src/kani/terminal.rs:90-148 (run_terminal_value, FWC arm :127-135), src/kani/classify.rs:127-223 (classify_kani_run, classify_member timeout path :172-176, classify_report), src/kani/output/report.rs:383-424 (KaniFailureCause: PropertiesFailed, Timeout, OutOfMemory, ExitCode, Unknown; not non_exhaustive), src/kani/run/execute.rs:495-525 (cause retained only for FWC), every run_terminal_value caller (tests/it/terminal_map.rs:76,82,89,266,271; kani_obligations.rs:2236,2378; kani_obligations_state_clause_replay.rs:688,1151; kani_obligations_state_frame.rs:2884,3062; no src caller), /home/peter/dev/quire-driver/src/provider.rs:89 (read-only), CG #418 branch peter/ir-847-native-error-discriminant-spec@d9cf2b0 FR-017 :654-661 and AC-42/AC-44 (class table).

## Verdict

Needs changes. The mapping itself (option b, classify.rs unchanged, out_of_memory to ResourceExhausted, timeout to TimedOut, everything else Failed) matches the planner rulings and the code shape, and the new FR-029 table, bounded-shadow text and AC-25 agree with each other. Two high defects are stale or self-contradicting statements left beside the amendment in FR-017 and TC-027, and the matrix overclaims AC-40. #418's section is untouched (it is not in this branch's ancestry; a trial merge conflicts only on the adjacent AC-40/AC-41 rows, see SR-12191 FND-004).

---
id: SR-12193
title: "Evidence review of IR-848 FWC cause in the FR-029 terminal map"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@9bdac7e40c35f73d42cb37fd02617a8dabe741c5 (PR 422, diff origin/peter/ir-827-cbmc-failure-cause...HEAD); spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-848. PR agent-ix/quire-contract-codegen#422 at head 9bdac7e40c35f73d42cb37fd02617a8dabe741c5, stacked on #410 (peter/ir-827-cbmc-failure-cause); only the stacked diff is reviewed. Planner rulings (option b, G1, G2, G4) are taken as the intent.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | FR-029-AC-35's SIGKILL clause names an obligation a Test cannot discharge as stated. "an `out_of_memory` cause from any `SIGKILL` of CBMC is `Incomplete(ResourceExhausted)` with no corroboration" can only be exercised with a stand-in that writes the string `out_of_memory`, which is already AC-32; no stand-in exhibits who sent the signal. Keep the limit in the rationale (:184-187) and either drop the clause from the AC or verify it by Inspection that the FWC arm reads nothing but the cause. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:382, :184-187 | wrong-requirement |

## Scope examined

Examined (role examined): FR-029 Inputs cause bullet (:81-91), the no-settlement table (:136-146), the rationale bullets (:155-183), the bounded-shadow restatement (:240-246), FR-029-AC-5, AC-25, AC-32 to AC-35, the IR-848 status note (:412-414); FR-017 behaviour bullet (:565-571), rationale (:596-607), FR-017-AC-40; interface-001 run_terminal_value inputs and failed_without_counterexample_cause; TC-040 steps/expected 5 and Status; TC-027 coverage note (:387-392); tests.md FR-029 AC-32..35 row. Context only: src/kani/terminal.rs:90-148 (run_terminal_value, FWC arm :127-135), src/kani/classify.rs:127-223 (classify_kani_run, classify_member timeout path :172-176, classify_report), src/kani/output/report.rs:383-424 (KaniFailureCause: PropertiesFailed, Timeout, OutOfMemory, ExitCode, Unknown; not non_exhaustive), src/kani/run/execute.rs:495-525 (cause retained only for FWC), every run_terminal_value caller (tests/it/terminal_map.rs:76,82,89,266,271; kani_obligations.rs:2236,2378; kani_obligations_state_clause_replay.rs:688,1151; kani_obligations_state_frame.rs:2884,3062; no src caller), /home/peter/dev/quire-driver/src/provider.rs:89 (read-only), CG #418 branch peter/ir-847-native-error-discriminant-spec@d9cf2b0 FR-017 :654-661 and AC-42/AC-44 (class table).

## Verdict

Acceptable with one low. AC-32 is testable with the existing single-run launcher stand-in pattern of tests/it/kani_obligations.rs:2330-2378 (execute_kani_obligation, then pass evidence.failure_cause.as_ref()); the exit statuses it lists are all in FR-017-AC-37/AC-38's stand-in set. AC-33's batch c::check and d::check are reachable through the public execute_kani_obligations with the AC-39 report shape (in-crate fixture at src/kani/run/execute.rs:4864-4930). AC-34 and AC-35 are pure-map table tests in tests/it/terminal_map.rs. Test is the right method for AC-32..34.

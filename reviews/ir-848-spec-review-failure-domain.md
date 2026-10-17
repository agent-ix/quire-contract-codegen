---
id: SR-12192
title: "Failure-domain review of IR-848 FWC cause in the FR-029 terminal map"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@9bdac7e40c35f73d42cb37fd02617a8dabe741c5 (PR 422, diff origin/peter/ir-827-cbmc-failure-cause...HEAD); spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-848. PR agent-ix/quire-contract-codegen#422 at head 9bdac7e40c35f73d42cb37fd02617a8dabe741c5, stacked on #410 (peter/ir-827-cbmc-failure-cause); only the stacked diff is reviewed. Planner rulings (option b, G1, G2, G4) are taken as the intent.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The rationale's justification for absent-cause `Failed` is factually wrong for a case the spec itself tests. It says an FWC whose cause is `properties_failed` or absent keeps `Failed` because "Kani reported a failed property and the backend printed no playback, which is a genuine failed-property verdict". classify_report gives FWC for any `Failure` that is not an unwind failure and has no playback for a failed property, including a `Failure` with no checks at all (classify.rs:216-219; failed_property() false). AC-32 exercises exactly that: a no-checks report with a null status or no `error_details` gives `Failed`. No property was reported failed there. The mapping (`Failed`) and #418's class (`CompletionRefusal`) are untouched by this; only the stated reason over-reaches. Split the reason: `properties_failed` is a failed property without a playback; an absent cause gives no evidence of a resource stop, so the run keeps `Failed`. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:164-168, :379; src/kani/classify.rs:206-223 | wrong-requirement |
| FND-002 | medium | Nothing requires a caller to pass the cause it holds. `run_terminal_value(outcome, None, ...)` compiles, so a caller holding `KaniExecutionEvidence` with `failure_cause: Some(OutOfMemory)` can still get `Failed`: the two-values split the ticket closes reappears at the caller. The pre-scope rejected an evidence-only variant for this reason. The downstream fix (quire-driver provider.rs:89) is routed to draco with no stated contract. Add a statement (and an AC on a CG-side helper or the driver ticket) that a caller with FR-017 evidence passes `evidence.failure_cause.as_ref()`, or provide a `&KaniExecutionEvidence` entry point so None cannot be passed by mistake. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:89-97; /home/peter/dev/quire-driver/src/provider.rs:89; src/kani/run/execute.rs:302 | missing-requirement |

## Scope examined

Examined (role examined): FR-029 Inputs cause bullet (:81-91), the no-settlement table (:136-146), the rationale bullets (:155-183), the bounded-shadow restatement (:240-246), FR-029-AC-5, AC-25, AC-32 to AC-35, the IR-848 status note (:412-414); FR-017 behaviour bullet (:565-571), rationale (:596-607), FR-017-AC-40; interface-001 run_terminal_value inputs and failed_without_counterexample_cause; TC-040 steps/expected 5 and Status; TC-027 coverage note (:387-392); tests.md FR-029 AC-32..35 row. Context only: src/kani/terminal.rs:90-148 (run_terminal_value, FWC arm :127-135), src/kani/classify.rs:127-223 (classify_kani_run, classify_member timeout path :172-176, classify_report), src/kani/output/report.rs:383-424 (KaniFailureCause: PropertiesFailed, Timeout, OutOfMemory, ExitCode, Unknown; not non_exhaustive), src/kani/run/execute.rs:495-525 (cause retained only for FWC), every run_terminal_value caller (tests/it/terminal_map.rs:76,82,89,266,271; kani_obligations.rs:2236,2378; kani_obligations_state_clause_replay.rs:688,1151; kani_obligations_state_frame.rs:2884,3062; no src caller), /home/peter/dev/quire-driver/src/provider.rs:89 (read-only), CG #418 branch peter/ir-847-native-error-discriminant-spec@d9cf2b0 FR-017 :654-661 and AC-42/AC-44 (class table).

## Verdict

Needs changes (medium). Clean: the SIGKILL known limit is stated (any SIGKILL accepted as ResourceExhausted, no corroboration, raw cause kept, no verdict claimed); the single-run vs batch timeout split is stated (G4), including d::check (FWC+timeout with a failed check gives Incomplete(TimedOut)) as AC-B required; the cause is retained only for FWC evidence (execute.rs:513-516), so a batch TimedOut member carries none and the c::check row stays on the timed-out arm; NoVerdict and MissingCoverSummary stay Failed; FR-030 needs no row (no FWC kind in the IR outcome). Consistency with #418: FR-029's mapping is consistent with the #418 class table (properties_failed/none CompletionRefusal; timeout, out_of_memory, exit_code, unknown EnvironmentFault); exit_code/unknown give Failed as the author stated. The only tension is the over-broad reason in FND-001, which #418's wording shares; #418 is not edited here.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-003 | medium | The two specs now describe a no-cause FailedWithoutCounterexample run differently, and each describes only one of its two shapes. A no-cause FWC is either a `Failure` that lists a failed check, where the cause is absent because `error_details` is missing, names another harness, has two entries or has a null status (FR-017-AC-38's shapes), or a `Failure` with no checks and no exit status (AC-32's case). FR-029 :174-176 (new) describes only the second shape ("a `Failure` report with no checks and no exit status is no evidence of a verdict ... the tool broke its output contract"). #418's FR-017 :659-662 describes only the first ("a genuine property or proof verdict ... a failed property"). The class (CompletionRefusal) and the terminal value (Failed) agree, so nothing behaves differently, but G1 says #418's reading wins and FR-029 now gives a different reason for the same input. State both shapes in the FR-029 bullet (a failed check listed with no cause is a failed property CG cannot replay; no checks is no evidence of a verdict), and either narrow #418's parenthetical the same way in this PR or route that to the planner. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:171-176; spec/kani/functional/FR-017-kani-execution-evidence.md:659-662, FR-017-AC-38 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | d536e94: the false claim is gone. FR-029 :171-176 separates `properties_failed` (a failed property without a playback, CompletionRefusal) from "no cause at all" (a no-checks Failure: no evidence of a verdict). The new bullet's narrowness and its disagreement with #418's prose are new finding FND-003. |
| FND-002 | fixed | d536e94: `run_terminal_value` takes `&KaniExecutionEvidence` and reads the outcome and failureCause from it, so a caller cannot omit the cause (FR-029 :83-93, interface-001 :149). The driver contract is stated: pass the KaniExecutionEvidence it already holds (provider.rs:89 holds `evidence`). No src caller exists. The test callers that map bare outcomes (terminal_map.rs, state_clause_replay.rs:688,1151, state_frame.rs:2884,3062) can build evidence because every field of KaniExecutionEvidence is pub and the struct is not non_exhaustive. The duplicated success_checks argument is SR-12191 FND-005. |

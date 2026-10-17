---
id: SR-11998
title: "Failure-domain review of IR-827 CBMC failure cause"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@1ed68d0931d45988fb46b5d7a03c000f8c4657f8 (PR 409); spec/kani/functional/FR-017-kani-execution-evidence.md (CBMC failure cause, AC-37 to AC-40), spec/kani/functional/FR-029-run-outcome-terminal-record.md (context)"
review_set: subset
---

## Summary

Ticket: IR-827. Failure-domain lens: identity of the same physical event across paths, edges of the
exit-status vocabulary, and inputs the closed vocabulary does not admit. The intent says
classification must not change, so the findings below ask for the inconsistencies to be named, not
fixed here.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | One physical event gets two FR-029 values and the spec names neither the split nor a follow-up. Memory exhaustion seen by CG's own meter is `MemoryExhausted` -> `Incomplete(ResourceExhausted)` (terminal.rs:121-126). The same exhaustion, when a cgroup OOM killer (systemd `MemoryMax`, as this lane runs) SIGKILLs only cbmc, makes Kani export `out_of_memory`, which is `FailedWithoutCounterexample` -> `Failed`. AC-40 now asserts `Failed` for `out_of_memory` and so pins it in a test. FR-029's own rationale (lines 147-150) says FWC is `Failed` because "the tool broke its output contract", which does not hold for a resource stop. Scenario: a harness under a 16 GiB scope is OOM-killed and QSL records a tool failure, not an incomplete resource result. Add a "Rationale, not a requirement" note in the CBMC failure cause section that names this as a known gap and cites a follow-up ticket, and keep AC-40 asserting that the value is unchanged. | spec/kani/functional/FR-017-kani-execution-evidence.md:550-553, :531-532, :620; spec/kani/functional/FR-029-run-outcome-terminal-record.md:128-133, :147-150; src/kani/terminal.rs:114-135 | missing-requirement |
| FND-002 | low | The single-run `timeout` behaviour is stated ("stays FailedWithoutCounterexample with the cause timeout") but the consequence is not. One harness stopped by Kani's `--harness-timeout` is `TimedOut`/`Incomplete(TimedOut)` in a batch and `FailedWithoutCounterexample`/`Failed` alone. CG adds `--harness-timeout` only for a batch (execute.rs:705), so a single run reaches this only through a caller option, which keeps the impact low. Add one sentence saying so and that the single/batch split in FR-029 is deliberate, or that it is deferred. | spec/kani/functional/FR-017-kani-execution-evidence.md:559-560; src/kani/run/execute.rs:705; src/kani/classify.rs:172-176 | wrong-requirement |
| FND-003 | low | The rule "never refuse a report or a run for an exit status it does not know" cannot hold for every JSON string. A lone-surrogate escape (`"exit_status": "\ud800"`) is valid JSON grammar, but serde_json fails to read it into `RawErrorDetail.exit_status: Option<String>`, so the whole report is refused as `Malformed`. The `unknown.exitStatus` value is also carried without the redaction and ANSI stripping that the record beside it gets. Either name a non-UTF-8-representable string as malformed, or deserialize leniently. Then state that `exitStatus` is not redacted, or redact it. | spec/kani/functional/FR-017-kani-execution-evidence.md:537-539, :566-567; src/kani/output/report.rs:329-333 | missing-requirement |

## Scope examined

Examined: the one-entry rule (522-526) against duplicate, absent, null and other-harness entries
(AC-38 covers each), mapping rules 1-5 (527-539), retention and diagnostics gating (540-549),
independence (550-553), batch timeout interaction (554-560; matches `classify_member` at
classify.rs:172-176 and `parse_batch` at report.rs:450-453, including the two-entry case in AC-39),
numeric `exit_status` refusal (561-562; already refused today because `exit_status` is
`Option<String>`), AC-37 to AC-40. Clean: the duplicate-entry asymmetry between the one-entry rule
(no cause) and the timeout rule (any `timeout` entry) is stated and tested, and the timeout rule's
check-count guard means a timed-out member never also gets a cause.

## Verdict

Needs changes, documentation only: name the OOM/timeout terminal-value split as a known limitation
with a follow-up, and close the surrogate/redaction edge.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-004 | low | Rule 5 applies 'excerpt steps 2 and 3 of Excerpts below', but Excerpts is above (line 429). Its parenthetical sums up step 3 as 'redaction against the request's directories', while step 3 (lines 435-437) also redacts RFC 3339 date-times, durations and process identities. AC-37 tests only the crate-directory path and the ANSI escape, so an implementation that redacts only directories passes. Scenario: an unknown status `killed pid 4242` comes out as `killed pid <PID>` from one implementation and verbatim from another. Change 'below' to 'above', make the parenthetical name all of step 3 (or restrict it to paths on purpose), and add a duration or PID case to AC-37. | spec/kani/functional/FR-017-kani-execution-evidence.md:540-544, 429-438 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 8ad2cc6: rationale at FR-017:590-599 names the out_of_memory Failed vs MemoryExhausted Incomplete split. Naming it was what this spec finding asked for; the classification question is routed to the planner (no ticket id is cited in the spec, which is acceptable for a rationale note). |
| FND-002 | fixed | 8ad2cc6: the same rationale names the single-run/batch timeout split and says a single run reaches it only through a caller option. |
| FND-003 | fixed | 8ad2cc6: the refusal rule (:571-575) now says a number or a lone-surrogate string is refused as malformed. That is honest: serde_json 1.0.151 parse_unicode_escape errors with LoneLeadingSurrogateInHexEscape when deserializing into String. AC-38 tests \ud800. The unknown exitStatus now gets excerpt steps 2-3 (:540-544), and AC-37 tests a path and an ANSI case. Residual wording is FND-004. |

---
id: SR-792002
title: "IR-1514 PR #792 gap-analysis of FR-045-AC-7 / TC-059 step 8"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #792 at e27b95b7; IR-1514; src/routed/smt/refinement.rs, src/routed/smt/mod.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-045
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-059
    type: reviews
---

## Summary

Ticket: IR-1514. This is a planless gap analysis (Plan completion: not assessed). It checks FR-045-AC-7 and TC-059 step 8 against the computed matrix, the tagged tests and the runner code, for a test-only PR. `quire matrix --scope .` binds FR-045-AC-7 to eight `#[trace]`-tagged tests, including `non_proof_solver_outcomes_refuse_refinement_handoff` (refinement.rs:646), whose tag is unchanged.

TC-059 step 8 asks for `Sat`, `Unknown`, absent response, malformed response, process failure and reached-limit solver results, each giving typed non-proof evidence and no handoff. After the PR the test still drives every one of these and reaches each deterministically:
- `Sat` and `Unknown`.
- Absent response: `cat >/dev/null`, giving `MalformedResponse`.
- Malformed response: `invalid`, giving `MalformedResponse`.
- Process failure: `exit 3`, giving `ProcessFailed`.
- Reached limit: output limit 2, giving `Stopped(OutputBytes)`.

For each case it asserts both the status and `NonProof(status)`. Before the PR, the race could produce `ProcessFailed` in the process-failure case through InputUnread instead of the exit code. The PR removes that alias, so the process-failure case is now tied to the exit code. The reviewer's reproduction is in SR-792001: main 0/30 under saturated cores, head 100/100, and the single-script mutation 0/30.

## Verdict

**MERGEABLE; one low coverage gap, not blocking.** The PR makes no criterion unreachable and weakens no assertion.

The `InputUnread` to `ProcessFailed` mapping (src/routed/smt/mod.rs:212-214) is production behaviour. A solver that answers `unsat` without reading the query must not count as proof evidence. It is covered only at the process layer (`child_exit_before_consuming_input_refuses_as_unread_stdin`, src/kani/run/process.rs:424-436, FR-045-AC-3). No test checks it at the `run_solver_query` / handoff layer.

The old test never asserted this mapping either: the race only produced spurious failures. So no coverage was lost. The gap was pre-existing and is now visible.

Ruling: accept for this PR. Add a small deterministic test as a follow-up, or in this PR if the lead prefers. The test should send a 256 KiB query (larger than the pipe buffer) to `/bin/sh -c "printf 'unsat\n'"` with `input_bytes` of 256 KiB. It should assert that `process().outcome` is `InputUnread`, that the status is `ProcessFailed` (not `Unsat`), and that the handoff is `Err(NonProof(ProcessFailed))`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The `run_solver_query` mapping `ProcessOutcome::InputUnread` to `SolverStatus::ProcessFailed` has no solver-level test. A fake solver that prints `unsat` and exits without reading its query is never checked to stay non-proof at the `run_solver_query` / refinement-handoff layer. Only the process layer has a test (process.rs:424-436, `/usr/bin/false` with 256 KiB of stdin). The gap is pre-existing, not introduced by this PR. Before the PR, the race only made the test fail; it never asserted the mapping. Suggested follow-up: a deterministic test with a 256 KiB query, `/bin/sh -c "printf 'unsat\n'"` and input limit 256 KiB, asserting outcome `InputUnread`, status `ProcessFailed` and `Err(NonProof(ProcessFailed))`. | src/routed/smt/mod.rs:212-214; src/kani/run/process.rs:424-436; src/routed/smt/refinement.rs:644-708 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-045-AC-7 | examined | spec/routed/functional/FR-045-hyper-smt-induction-map.md:212 | An FR-315 refusal, a Sat or Unknown answer, a missing or malformed response, a failed solver invocation or a reached limit remains typed non-proof evidence and produces no proof-bearing handoff. |
| TC-059 step 8 | examined | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:114-123 | Separately exercise an FR-315 query refusal and Sat, Unknown, absent response, malformed response, process failure and reached-limit solver results ... Assert typed non-proof evidence and no proof-bearing handoff (FR-045-AC-7). |
| FR-045-AC-3 | context_only | spec/routed/functional/FR-045-hyper-smt-induction-map.md:208 | CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock and captured-output limits |
| tests::non_proof_solver_outcomes_refuse_refinement_handoff | examined | src/routed/smt/refinement.rs:644-708 | six scripted outcomes, status and NonProof handoff asserted |

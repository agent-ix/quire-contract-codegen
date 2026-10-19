---
id: SR-13286
title: "IR-890 CG PR 457 spec-review EARS: FR-016 and FR-024 edited statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@b8e2a28e8c2672090718bcb81d9617a060cf73dc vs origin/main e45b9d2f; spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md"
review_set: subset
---

## Summary

Ticket: IR-890. Checked the requirement statements the PR edits or removes for EARS form.
`quire validate` reports no EARS warning in either file. The removed FR-024 "When a counterexample
did not come from a backend transcript" statement leaves no dangling reference among the
statements.

## Examined criteria

- FR-016:91-92 (examined): "If native replay runs and disagrees with the harness result, then the generator shall report a typed mismatch rather than a failure." Unwanted-behaviour form, one response. Clean.
- FR-016:93-95 (examined): "If the executor reports an internal fault, then the generator shall retain it as `SpineReplayError::Refused(ReplayRefusal::Fault)` and map the run outcome to `Failed` under FR-029-AC-10."
- FR-024:176-179 removed statement (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The fault statement has two responses joined by "and", and the second ("map the run outcome to `Failed`") is not this generator's: the terminal map is `run_terminal_value` in `src/kani/terminal.rs` under FR-029. Fix: end the statement at "retain it as `SpineReplayError::Refused(ReplayRefusal::Fault)`" and move the terminal reading to a plain cross-reference sentence ("FR-029-AC-10 reads it as `Failed`"). | spec/replay/functional/FR-016-witness-native-replay.md:93-95 |

## Verdict

**PASS WITH NITS.** One low compound-response finding.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 835d61d9: FR-016:93-95 now ends at "retain it as `SpineReplayError::Refused(ReplayRefusal::Fault)`." followed by the separate sentence "FR-029-AC-10 reads that fault as terminal `Failed`." |

---
id: SR-11900
title: "Base review of IR-809 NoVerdict diagnostics (FR-017-AC-32 to AC-36)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md (PR #403, head 91c7c67)"
review_set: subset
---

## Summary

Ticket: IR-809. Base checklist over the new FR-017 "NoVerdict diagnostics" subsection, its
Outputs bullet, FR-017-AC-32 to AC-36, the TC-027 and TC-043 additions and the two new matrix
rows. IDs are well formed and contiguous (AC-32 to AC-36 follow AC-31), each new AC has a Test
method and a TC that `verifies` FR-017, and the matrix rows mark them PLANNED/UNRUN with no
coverage claimed. Classification independence is stated consistently with FR-017-AC-14, AC-18,
AC-21 and AC-25: the excerpt is taken only from a stream read whole, over-limit and unread streams
stay refused with no evidence, and `NoVerdict` stays `NoVerdict`. Three findings: a checklist-
mandated version-in-prose hit, and two AC-36 clauses that a correct implementation cannot satisfy
as written.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Checklist hash/digest/pin rule: the normative phase rule and the CBMC-failure statement are defined "measured on Kani 0.68 with CBMC 6.11" in requirement prose (and the tail-size rationale cites "Kani 0.68"). Exact tool versions belong in Cargo/lockfiles or a run report, not in a shall-statement; drop the versions from the rule and keep the measured provenance in the ticket/PR. | spec/kani/functional/FR-017-kani-execution-evidence.md:366, :378, :386, :413 | wrong-requirement |
| FND-002 | medium | AC-36 says "a Success member in a non-zero exit with no Failure entry is NoVerdict with phase unknown, while its neighbours' evidence carries no record". With no Failure entry in the report, `process_succeeded` is false for every member (src/kani/run/execute.rs:790-793), so every Success neighbour is also NoVerdict and must carry a record; the clause is unsatisfiable within one group. TC-043 step 11 instead uses a separate run with a Failure entry; align AC-36 to that. | spec/kani/functional/FR-017-kani-execution-evidence.md:471; spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md:46,87-89 | wrong-requirement |
| FND-003 | medium | AC-36 requires two NoVerdict runs with equal exit, no report and different streams to "have equal evidence in every field but diagnostics". Evidence carries `memory.peak_resident_bytes` (src/kani/run/memory.rs:36), an observed RSS peak that differs between any two real or stand-in runs, so a correct implementation fails this assertion (or the test must silently exclude a field the AC does not exclude). Name the excluded observation fields, and state what "the FR-029 terminal value ... is unchanged" is compared against. | spec/kani/functional/FR-017-kani-execution-evidence.md:471 | wrong-requirement |

## Checklist and coverage

Examined: FR-017 Outputs bullet (line 86), every bullet of the NoVerdict diagnostics subsection
(lines 344-428), AC-32 to AC-36 (lines 467-471), TC-027 procedure/expected/implementation
additions, TC-043 description/step 11/expected 11/implementation note, and the four tests.md rows.
Context only: FR-017-AC-14, AC-18, AC-21, AC-22, AC-25 and the classifier in src/kani/classify.rs
and src/kani/run/execute.rs.

Clean units: AC-32 is falsifiable with the named non-compiling harness; the reviewer reproduced it
on Kani 0.68.0/CBMC 6.11.0 (exit 1, no report, stdout `error[E0425]` and
`error: Failed to execute cargo (exit status: 101). Found 1 compilation errors.`, stderr
`error: could not compile`), so every marker AC-32 names is real. AC-33 and AC-34 are falsifiable
over stand-ins and the excerpt builder. AC-36's over-limit/unread clause restates AC-14/AC-25
exactly. The AC-14 "never truncated and then classified" rule is not contradicted, because the
excerpt is a copy cut from a stream already read whole and never classified. No tracking
artifact, hash or digest is introduced.

## Validation

`QSPEC_DIR=/home/peter/dev/quire-specification make spec` in the worktree exited 0. The only
FR-017 warnings (line 248, ears:unclassifiable / missing-subject) are pre-existing text shifted by
the three-line Outputs insert (origin/main line 245).

## Verdict

Changes requested: FND-001 (checklist-mandated) and the two AC-36 clauses. The rest of the base
checklist passes.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | interface-001's `kani_obligation_execution_slice.evidence` enumerates the evidence fields and the optional `batch` wire member, and `execute_kani_obligation(s)` list the request inputs, but neither names the new optional `diagnostics` wire member (with `KaniFailurePhase`) or the new `KaniExecutionRequest::home_directory` input. A consumer reading the interface contract will not know the member exists. AD-004's tables are a move plan of pre-existing items (MemoryObservation and KaniGroupRun are absent too), so AD-004 needs no update; interface-001 should get one line in this PR or a tracked follow-up. | spec/core/functional/interface-001-codegen-api.md:125-131, :351; spec/kani/functional/FR-017-kani-execution-evidence.md:77-80 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (fe3a4e1): fe3a4e1 removes every Kani/CBMC version from the new subsection (phase rule, launcher rule, CBMC rationale, tail rationale); remaining Kani 0.68 mentions are pre-existing FR-017 text outside this PR. |
| FND-002 | fixed | Round 1 (fe3a4e1): AC-36 now uses a separate run with a Failure entry (no member NoVerdict, no record), matching execute.rs:790-793 and TC-043 step 11. |
| FND-003 | fixed | Round 1 (fe3a4e1): AC-36 excludes `diagnostics` and `memory` and compares the FR-029 value to `Failed` (FR-029-AC-4, terminal.rs:128-135), with and without the record. |

---
id: SR-11997
title: "Spec review of IR-827 CBMC failure cause"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@1ed68d0931d45988fb46b5d7a03c000f8c4657f8 (PR 409, diff vs origin/peter/ir-809-run-diagnostics); spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md"
review_set: subset
---

## Summary

Ticket: IR-827. Base spec-review pass over the new "CBMC failure cause" section, FR-017-AC-37 to
AC-40, the AC-32/AC-36 amendments to IR-809's unmerged criteria, TC-027/TC-043 additions, the
interface-001 semantics line and the FR-017-AC-19 status correction in tests.md. Failure-domain
lens is in SR-11998; evidence, integrity and EARS lenses are folded into SR-11999.

Checked against code at the head: `src/kani/output/report.rs:320-460`, `src/kani/classify.rs:160-225`,
`src/kani/terminal.rs:100-140`, `src/kani/run/execute.rs:700-715, 955-968, 3375-3435, 4100-4410`,
`src/kani/run/report_file.rs:20-35`, `src/kani/test_support.rs:60-140`.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Failure-phase rule 1 is extended to "a report whose entry states a CBMC failure" but keeps the justification "the report shows the backend finished, so no stage is named". For a CBMC failure the report shows the opposite: CBMC (the backend/solver stage) is the stage that failed. The new section then attaches this record, type and slice still named NoVerdict diagnostics (`no_verdict_diagnostics`, `KaniNoVerdictDiagnostics`), to non-NoVerdict evidence via "rule 1", while the rule set's lead-in still scopes the rules to "each NoVerdict member". A consumer reading `phase: unknown` plus the stated rationale is told the backend finished on a run where CBMC exited 64. Reword rule 1's justification for the CBMC case (phase is `unknown` because `failureCause` already names the failed stage), state the phase directly in the new section rather than by reference to a NoVerdict-scoped rule, and say whether the record type is renamed or kept. | spec/kani/functional/FR-017-kani-execution-evidence.md:385-390, :545-549; spec/core/functional/interface-001-codegen-api.md:352 | wrong-requirement |
| FND-002 | low | TC-027 Implementation still names `tc_027_execution_reads_only_the_report_its_own_run_exported`, the report_file and classify tests "for FR-017-AC-18 and FR-017-AC-19", while tests.md (this PR) now says no test carries FR-017-AC-19 and the code passes a named path. The two TC-027 paragraphs and the matrix disagree on whether any test backs AC-19. Drop AC-19 from that sentence. | spec/kani/matrix/TC-027-kani-execution-evidence.md:340-347; spec/kani/matrix/tests.md:41 | wrong-requirement |
| FND-003 | low | The AC-36 amendment narrows the IR-809 clause to "a Failure entry with no exit status", matching the IR-809 fixture (`entry()` writes `has_errors: false`) rather than real Kani, whose failed-assertion entry carries `properties_failed`. No AC now pins the realistic batch case: a member `Failure` with a failed assertion and `properties_failed` beside exit 1 has no record. AC-38 covers it only for a single run and AC-39's d::check uses `timeout`. Add a `properties_failed` member to AC-39 (or to AC-36) asserting no `diagnostics`. | spec/kani/functional/FR-017-kani-execution-evidence.md:616, :619; src/kani/run/execute.rs:4386-4393 | correct-requirement-no-evidence |

## Scope examined

Examined: FR-017 Outputs bullet (lines 94-96), failure-phase rule 1 (385-390), IR-809 rationale
(419-424), batch record rule (505-509), CBMC failure cause section (514-574), AC-32, AC-36, AC-37,
AC-38, AC-39, AC-40 (612-620), TC-027 procedure/expected/implementation additions, TC-043 step 12,
tests.md rows 40-48 and 101-106, interface-001 lines 127 and 352-353. Context only: FR-017-AC-19 and
AC-22 (599, 602), FR-029 terminal table (FR-029 lines 124-150).

Clean: the AC-32 and AC-36 amendments are consistent with IR-809's tests at this head. AC-32's
"other inconclusive" control (`execute.rs:4160`, `failed-bare`) uses the single-run `report()` with no
`error_details`, so it keeps no record under AC-38; AC-36's Failure-entry control (`execute.rs:4386`)
uses `entry()` with no exit status. The IR-277 test with `d::check` `timeout` plus checks asserts
only the outcome, which AC-39 keeps. The FR-017-AC-19 correction is accurate: no `.rs` file carries
the FR-017-AC-19 tag, and `execute.rs:964-965` (single), `:711-712` (batch) and
`report_file.rs:25-31` build a named target-directory path. The interface-001 line matches the
section. The cause is kept out of classification, refusal and FR-029.

## Verdict

Needs changes (one medium on how the reused record is justified, two lows). The design (closed
vocabulary, one-entry rule, serialize-only, batch interaction) holds together and matches the code it
describes.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 8ad2cc6: phase rule lead-in now covers FWC members with a CBMC-failure cause (FR-017:386-388); rule 1 gives separate justifications, and for a CBMC failure says failureCause names the stage (:390-393); record type and member kept unrenamed, stated in the section (:556-560) and interface-001. |
| FND-002 | fixed | 8ad2cc6: TC-027 Implementation now reads 'for FR-017-AC-18 (none of them covers FR-017-AC-19, which is Planned)'. |
| FND-003 | fixed | 8ad2cc6/a90c8a6: AC-39 and TC-043 step 12 add e::check (failed assertion, properties_failed) with no diagnostics. AC-36 reverts to IR-809 wording; the prose bullet at FR-017:550-554 limits it to an entry with no exit status. No contradiction: AC-36 describes a concrete fixture, and the batch rule (:510-512) is consistent. |

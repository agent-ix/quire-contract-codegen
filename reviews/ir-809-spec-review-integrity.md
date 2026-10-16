---
id: SR-11902
title: "Integrity review of IR-809 NoVerdict diagnostics"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md (Inputs, Outputs, NoVerdict diagnostics, AC-32 to AC-36), spec/kani/matrix/tests.md (PR #403)"
review_set: subset
---

## Summary

Ticket: IR-809. Checked completeness and consistency of the new subsection against FR-017's
Inputs/Outputs and existing rules. The Outputs bullet is added, the wire shape is stated once and
matches AC-35, the "read Kani's output in one place" rule is extended rather than contradicted,
and the batch rule agrees with the existing no-report group rule. One finding: the excerpt is
declared a function of an input FR-017 does not have.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The excerpt is "a function of the captured stream bytes, the request's crate and target directories and the caller's home directory alone", but the home directory is not an FR-017 input: `KaniExecutionRequest` carries the harness, installation and crate/target directories only (Inputs, lines 69-75). Two implementers will read it from `$HOME` or from the passwd entry; a `$HOME`-based one makes AC-35's home case depend on process-global state that parallel tests must mutate, so the AC's determinism claim cannot be tested deterministically. Add the home directory to the request (or name its exact source) in Inputs. | spec/kani/functional/FR-017-kani-execution-evidence.md:398, :414-417, :69-75 | missing-requirement |

## Scope examined

Inputs (lines 50-76), the new Outputs bullet (86-88), all subsection bullets (344-428), AC-32 to
AC-36, tests.md rows 44-45 and 99/101. Clean: IDs unique and contiguous; outcome-gating ("every
other outcome and every refusal ... no diagnostics member") is total; the wire form and
serialize-only rule are consistent between body and AC-35; the TC split (AC-32 to AC-35 in TC-027,
AC-36 in TC-043) matches each TC's description.

## Verdict

One medium finding; otherwise structurally consistent.

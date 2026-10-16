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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new input is described as 'the home directory of the launcher's environment', but grouping (FR-017 batch rule and interface-001) keys on option vector, ceilings, launcher, crate and target directory only, so two members with different home_directory values share one launcher process, one environment and one HOME. The batch rule then redacts the shared streams 'with that member's request directories', so the same group output yields different excerpts per member, and at most one of the two homes is the launcher's. Either make home_directory a grouping key or describe it as a redaction input only, not the launcher's home. | spec/kani/functional/FR-017-kani-execution-evidence.md:77-80, :485-487; spec/core/functional/interface-001-codegen-api.md:131 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (fe3a4e1): home_directory is an optional request input, never read from the environment; AC-35 runs under two process HOME values and asserts equal records. |
| FND-002 | fixed | Round 2 (d2784f0): home_directory is now a grouping key (FR-017 batch rule, AC-21, interface-001 :131), and a group's excerpts are redacted with the group's one crate, target and home directory; AC-36 and TC-043 step 11 add the /h/u vs /h/v vs absent process-count case. |

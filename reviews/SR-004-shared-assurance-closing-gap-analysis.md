---
id: SR-004
title: Shared assurance migration closing gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#13 at efb04d3; FR-006 coverage after the adversarial review, and the honest reach of every claim this change makes"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---

# SR-004: Shared assurance migration closing gap analysis

## Summary

SR-002 asked whether each FR-006 criterion had a test that would fail if the criterion stopped
holding. It answered yes for all six, and for FR-006-AC-2 that answer was wrong: the producer
isolation test could not fail for three of the five producers. That is now fixed and re-probed, and
this document restates the coverage with the reach of each claim spelled out rather than asserted.

## Criterion coverage after the fixes

| Criterion | Test | What would make it fail |
| --- | --- | --- |
| FR-006-AC-1 | `tc_008` | A component that does not classify `compatible`; a consumed-artifact digest that moved; a mirror reference; an install line naming a matrix-forbidden version. Both scans are probed by injection, so a scan that matched nothing would itself fail |
| FR-006-AC-2 | `tc_009` ×5 | Any scenario, control or adapter probe not matching; a declared argv absent from `make -n assurance-inputs`; a producer that cannot report anything but pass; a producer invocation by the driver, for any of the four programs, or a coverage export requested by the driver rather than by Quoin; the driver writing into its own input directory by any means at all |
| FR-006-AC-3 | `tc_010` | An impact snapshot that is not the export's digest; an export that does not name every requirement; a matrix row claiming a status its bindings contradict |
| FR-006-AC-4 | `tc_011` | A case whose label its observation does not support; a byte moved; an uncommitted difference; the retained tree falling below its floor; a mutation probe going undetected |
| FR-006-AC-5 | `tc_012` | Any of the twelve states not demonstrated by a case that ran *and* matched; a negative with no positive control |
| FR-006-AC-6 | `tc_013` | A deleted file reappearing; a frozen artifact changing or being deleted; a live schema being dropped or no longer included by the generator; anything executable referencing a frozen artifact; a failure-suppressing Makefile directive; `test` or `msrv` becoming unreachable from `ci` |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-401 | medium | FR-003 and FR-004 have no implementation, no suite, and no proof obligation. 7 of the 38 unbacked coverage rows are theirs. The gap is between specification and code | `spec/functional/FR-003-kani-lowering.md`, `spec/functional/FR-004-vacuity-evidence.md` | correct-requirement-no-evidence |
| FND-402 | medium | The conformance corpus produces 4 of the 9 declared `GenerationErrorCode` values and asserts the declared terminal state of the other 5 without producing them. Those five are produced by the integration tests, which are not transcribed | `examples/generation_conformance.rs` | correct-requirement-no-evidence |
| FND-403 | medium | 38 of 56 coverage rows are unbacked. Every one is a row this change deliberately leaves 🚧 Planned. The number is a census of what has not been reviewed, quoted here with its population rather than as a headline | `spec/test-matrix.md` | wrong-requirement |
| FND-404 | low | No ix-flow decision event exists, so the receipt reads `incomplete` with `decision_missing`. Only the repository owner can create one | `assurance/change-assurance.json` | wrong-requirement |
| FND-405 | low | The chain's measurement floor for `codegen.upstream-identity/v1` cannot fire against anything the producer emits, because `outcome == "pass"` implies `agreeingSources == 3` by construction. It is a check on a hand-written document, which is what it is for, and the docstring says so | `scripts/assurance_chain.py` | wrong-requirement |
| FND-406 | low | The retained floor is a floor and not a digest census, so a retained file could be replaced by a different file of the same count. Git history and pull-request review remain the integrity boundary for retained bytes, and the compatibility view asks Git rather than implying a stronger claim | `scripts/legacy_evidence_view.py` | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition |
| --- | --- |
| SR-002 GAP-001 / FND-401 | **ACCEPTED**. Recorded in the suite registry, the pins and an open unknown. Issues #2 and #5 own it |
| SR-002 GAP-002 / FND-402 | **ACCEPTED**. The census floor is set at the count the corpus actually reaches, so a reduction is caught; widening it is corpus work |
| SR-002 GAP-003 / FND-403 | **ACCEPTED**, with the population stated everywhere the number appears |
| SR-002 GAP-004 / FND-404 | **ACCEPTED**. The correct answer |
| FND-405 | **ACCEPTED**. The reach is stated in the code rather than the claim being widened |
| FND-406 | **ACCEPTED**. Stated rather than closed; the byte-level claim belongs to Git and is asked of Git |

## What this change does not claim

It claims that this repository's own tools produced the structured results named by
`assurance/change-assurance.json` at `efb04d3`, that each attestation states the verdict those bytes
carry, that Quoin retained exactly those bytes, and that the retained evidence under `evidence/` was
read without being modified.

It claims nothing about the correctness of the code generator beyond leaving it unchanged, promotes
no semantic TestMatrix row, and confers no qualification, certification or accreditation.

## Verdict

**APPROVED** at `efb04d3`.

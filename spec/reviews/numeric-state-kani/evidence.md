---
id: SR-018
title: "Evidence review of numeric and state Kani lowering"
type: SpecReview
analysis: evidence
scope: "FR-003-AC-1 through FR-003-AC-8, TC-003/005/007/014, TM-001 and SUITE-008"
review_set: subset
---

## Summary

The deterministic advisor was invoked but Quoin 0.23.1 could not recognize the installed Quire
0.31.0 version string, so it produced no recommendations. Reviewer judgment grounded in Quire's
obligation/property exports and the installed method catalog confirms the authored Test methods and
the Analysis-kind SUITE-008 plan while retaining the advisor failure as an evidence limitation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `quoin advise --json` exits 2 with “could not determine the quire CLI version” even though `quire --version` reports 0.31.0, above the stated 0.21.0 floor. The required deterministic recommendation step is unavailable, so the method disposition below is explicitly reviewer judgment rather than catalog advice. | FR-003-AC-1..FR-003-AC-8; Quoin 0.23.1; Quire 0.31.0 |

## Method disposition

Quire exports all eight FR-003 obligations with authored method `Test`. Its property classifier marks
AC-1, AC-6, and AC-7 as universal and the remaining cases as concrete examples. The installed catalog
maps universal properties to property-based testing and stable output to golden testing; its formal
analysis methods provide solver evidence but do not replace the observable contract, compilation,
refusal, parity, and identity assertions in TC-003/005/007/014. Therefore the requirement methods
remain `Test`, while SUITE-008 correctly records `Analysis` evidence for the actual cargo-kani runs.

SUITE-008 pins cargo-kani 0.67.0, target directory, exact harness/options, executable digest, schemas,
typed ABI/domains, proof success, proof failure, printed concrete playback, and independent oracle
checks. Its planned 0, 1000, -1, and 1001 cases cover both model endpoints and adjacent outside
controls. TM-001 truthfully retains every numeric/state row as Planned; current Boolean test symbols
do not discharge TC-014. Native `runtime::execute` replay remains downstream IT-010 evidence.

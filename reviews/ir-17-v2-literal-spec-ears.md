---
id: SR-15429
title: "IR-17 CG 524 literal construction ears review"
type: SpecReview
analysis: ears-conformance
scope: "CG #524 FR-014 added V2 clause literal behavior and AC-53; TC-024 V2 literal construction subsection; unchanged FR-015/kernel contracts as context"
review_set: subset
---
# SR-15429: IR-17 V2 literal specification ears review

## Summary

Reviewed the exact frozen spec prerequisite for general canonical signed-i64 literal constructor selection. No defect was found in the changed specification under this method; implementation and real proof qualification remain future work.

## Verdict

**PASS — no EARS defect in the new requirement-bearing statements.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

The deterministic engine was run over the full spec tree; all three emitted advisories are on unchanged lines, including the inherited FR-014 line 269 Before trigger. No new statement has an engine diagnostic. Semantic review covered all five new behavior bullets, including sentences within bullets. When emitting a literal and When the eligible literal is minimum are generation-time events, not continuous runtime states. The ineligible literal branch uses the unwanted-condition If ... then pattern. The remaining preservation and exclusivity obligations have concrete actors/subjects and measurable responses.

The full-spelling round trip and if-and-only-if branch eliminate vague canonical/fit interpretations. Retain parse with typed mapping, preserve tree/domain/prefix/counters/stops and leave separate emitters/policies unchanged are testable, with exact TC controls. Multiple related invariants in a bullet do not introduce different event timing or conflicting actors. TC prose and AC tables are not reclassified as free-standing EARS requirement statements by this lens. Inherited EARS debt is disclosed, not treated as a new finding or silently declared fixed.

## Review boundary

Independent reviewer for CG #524, exact frozen head `7da9951d4169ed8b6e1ef001917678b412bfe6f2`, base `be884685379c35ac503cce94d49c498780e313d1`. The entire product diff is 87 inserted lines in FR-014 and TC-024 only. No previous text was deleted; no source, tests, kernel API, dependencies, lock, FR-015 or gate changed. The head and base were read from Git and the published PR head was checked independently.

This is a spec prerequisite review. It approves no implementation, actual profile verdict, kernel representation change, cap increase or proof credit. The one-cover #513 precondition control is distinct from the four-assertion IR-917 profile. The original OOM and held IR-917/IR-948 policy remain unresolved.

## Assurance Context

No applicable repository AssuranceProfile or required review_selection was found in the reviewed spec tree. Profile context and active exceptions for this bounded change: none. Missing context: no implementation or new executable test exists in this spec-only PR, and no runtime or proof measurement is available from this review. Existing unrelated assurance debt is outside the delta; it is not waived.

## Checks and limitations

Actual `quire validate --scope <frozen-worktree> "spec/**/*.md" --summary` exit 0; `git diff --check be884685...HEAD` exit 0. Quire 0.36.2 / engine 0.50.2; Quoin 0.28.4. The entire-spec validation has three inherited grammar diagnostics: FR-017 line 267 unclassifiable/missing-subject and FR-014 line 269 non-canonical-trigger. Those lines and their context are unchanged in the diff. No diagnostic points into the added behavior.

Actual computed `quire matrix --scope <frozen-worktree> --strict --format json` exits 1. New FR-014-AC-53 is untagged and explicitly planned; this spec-only review does not present it as executed coverage or give implementation acceptance. The matrix also contains inherited untagged rows. The subsequent implementation must add meaningful executable bindings and pass its applicable gates. No production code, Cargo, build, namespace, Kani/CBMC, aggregate or engine was run.

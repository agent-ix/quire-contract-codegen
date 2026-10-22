---
id: SR-15427
title: "IR-17 CG 524 literal construction base review"
type: SpecReview
analysis: base
scope: "CG #524 FR-014 added V2 clause literal behavior and AC-53; TC-024 V2 literal construction subsection; unchanged FR-015/kernel contracts as context"
review_set: subset
---
# SR-15427: IR-17 V2 literal specification base review

## Summary

Reviewed the exact frozen spec prerequisite for general canonical signed-i64 literal constructor selection. No defect was found in the changed specification under this method; implementation and real proof qualification remain future work.

## Verdict

**PASS — the changed specification is concrete and preserves the proof/owner boundary.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

The new behavior has a complete canonical spelling input, deterministic iff constructor decision, explicit fallback, typed error, MIN expression rule, scope exclusion and preservation constraints. AC-53 was absent at base and is newly allocated once; it states observable generation and complete evaluation obligations with method Test. TC-024 explicitly cites AC-53 and supplies valid, invalid, boundary, state-transition/denial and provenance/embedding procedures. Local relative links to FR-014 and TC-025 resolve. Existing FR-014 meanings remain unchanged; FR-015 is not edited.

The implementation-independent relation is: if complete signed-i64 parse yields v and canonical display(v) equals the entire original spelling, the kernel canonical parser denotes v, and public From<i64>(v) denotes the same mathematical value. Neither constructor takes a semantic Meter. Owner ordering/arithmetic must therefore retain their existing mathematical/charge behavior by the kernel refinement contract, with the independently prepared differential and all denial prefixes specified as executable checks. Host parsing does not replace reader admission or make a rejected spelling admitted. Canonical direct MIN and a positive 2^63 child under negate have distinct handling.

The bounded normative addition contains no new digest/pin/receipt catalog or source-scan requirement. Existing unrelated criteria are context only, not re-approved wholesale. Every new behavioral obligation is covered by the matching TC subsection; actual test binding is deliberately future implementation, not claimed evidence.

## Review boundary

Independent reviewer for CG #524, exact frozen head `7da9951d4169ed8b6e1ef001917678b412bfe6f2`, base `be884685379c35ac503cce94d49c498780e313d1`. The entire product diff is 87 inserted lines in FR-014 and TC-024 only. No previous text was deleted; no source, tests, kernel API, dependencies, lock, FR-015 or gate changed. The head and base were read from Git and the published PR head was checked independently.

This is a spec prerequisite review. It approves no implementation, actual profile verdict, kernel representation change, cap increase or proof credit. The one-cover #513 precondition control is distinct from the four-assertion IR-917 profile. The original OOM and held IR-917/IR-948 policy remain unresolved.

## Assurance Context

No applicable repository AssuranceProfile or required review_selection was found in the reviewed spec tree. Profile context and active exceptions for this bounded change: none. Missing context: no implementation or new executable test exists in this spec-only PR, and no runtime or proof measurement is available from this review. Existing unrelated assurance debt is outside the delta; it is not waived.

## Checks and limitations

Actual `quire validate --scope <frozen-worktree> "spec/**/*.md" --summary` exit 0; `git diff --check be884685...HEAD` exit 0. Quire 0.36.2 / engine 0.50.2; Quoin 0.28.4. The entire-spec validation has three inherited grammar diagnostics: FR-017 line 267 unclassifiable/missing-subject and FR-014 line 269 non-canonical-trigger. Those lines and their context are unchanged in the diff. No diagnostic points into the added behavior.

Actual computed `quire matrix --scope <frozen-worktree> --strict --format json` exits 1. New FR-014-AC-53 is untagged and explicitly planned; this spec-only review does not present it as executed coverage or give implementation acceptance. The matrix also contains inherited untagged rows. The subsequent implementation must add meaningful executable bindings and pass its applicable gates. No production code, Cargo, build, namespace, Kani/CBMC, aggregate or engine was run.

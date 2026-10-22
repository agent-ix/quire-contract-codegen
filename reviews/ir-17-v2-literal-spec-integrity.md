---
id: SR-15428
title: "IR-17 CG 524 literal construction integrity review"
type: SpecReview
analysis: integrity
scope: "CG #524 FR-014 added V2 clause literal behavior and AC-53; TC-024 V2 literal construction subsection; unchanged FR-015/kernel contracts as context"
review_set: subset
---
# SR-15428: IR-17 V2 literal specification integrity review

## Summary

Reviewed the exact frozen spec prerequisite for general canonical signed-i64 literal constructor selection. No defect was found in the changed specification under this method; implementation and real proof qualification remain future work.

## Verdict

**PASS — internally consistent constructor refinement with explicit preserved owners.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

Traceability for the delta: StR-001 traceable multi-backend generation -> existing FR-014 satisfies relationship -> FR-014-AC-53 -> TC-024 verifies relationship/new named subsection -> Test. No new US, FR/NFR object or relationship edge is introduced. FR-014-AC-38 owns the V2 clause route; its existing AC-3/4/5/10/37/39 remain in force. FR-015-AC-20/40 remains the existing byte-identical newly generated oracle embedding contract; its proof domains and acceptance settings are unchanged.

Exact fit and canonical spelling are independent of claim, domain, operation, backend and cfg, avoiding hidden fixture selection. All ineligible spellings retain parse and InvalidConstant; upstream malformed-input refusal and defensive seam testing are explicitly separate. The iff rule and specific MIN/negate example agree and do not create a second interpretation. Full values outside i64/i128 and genuine continuation are retained. Arithmetic and integer constructor ownership remain with the kernel; scalar bound emission is expressly excluded. The added reader-preservation sentence asserts no new admission API or new reader implementation.

Failure-domain checks: errors remain typed and no stopped result is promoted to completion; expression, clause/node identities and order remain intact; short-circuit topology is not folded; no new effects/caches, external CLI, lookup tie-break, pagination, auth or concurrent API are introduced. TC comparisons against independent parse-based owner inputs, exact boundary denials and no-domain-clipping obligations make the intended refinement falsifiable. This is one constructor-selection behavior with its related invariants, not a second general evaluator architecture.

## Review boundary

Independent reviewer for CG #524, exact frozen head `7da9951d4169ed8b6e1ef001917678b412bfe6f2`, base `be884685379c35ac503cce94d49c498780e313d1`. The entire product diff is 87 inserted lines in FR-014 and TC-024 only. No previous text was deleted; no source, tests, kernel API, dependencies, lock, FR-015 or gate changed. The head and base were read from Git and the published PR head was checked independently.

This is a spec prerequisite review. It approves no implementation, actual profile verdict, kernel representation change, cap increase or proof credit. The one-cover #513 precondition control is distinct from the four-assertion IR-917 profile. The original OOM and held IR-917/IR-948 policy remain unresolved.

## Assurance Context

No applicable repository AssuranceProfile or required review_selection was found in the reviewed spec tree. Profile context and active exceptions for this bounded change: none. Missing context: no implementation or new executable test exists in this spec-only PR, and no runtime or proof measurement is available from this review. Existing unrelated assurance debt is outside the delta; it is not waived.

## Checks and limitations

Actual `quire validate --scope <frozen-worktree> "spec/**/*.md" --summary` exit 0; `git diff --check be884685...HEAD` exit 0. Quire 0.36.2 / engine 0.50.2; Quoin 0.28.4. The entire-spec validation has three inherited grammar diagnostics: FR-017 line 267 unclassifiable/missing-subject and FR-014 line 269 non-canonical-trigger. Those lines and their context are unchanged in the diff. No diagnostic points into the added behavior.

Actual computed `quire matrix --scope <frozen-worktree> --strict --format json` exits 1. New FR-014-AC-53 is untagged and explicitly planned; this spec-only review does not present it as executed coverage or give implementation acceptance. The matrix also contains inherited untagged rows. The subsequent implementation must add meaningful executable bindings and pass its applicable gates. No production code, Cargo, build, namespace, Kani/CBMC, aggregate or engine was run.

---
id: "SR-12184"
title: "CG PR 421 spec review (criterion strength): FR-015-AC-112 to FR-015-AC-117"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858); spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step 4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112 to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md; checked against src/ and tests/it/ at origin/main 0e0f92e"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-12184: CG PR 421 spec review (criterion strength): FR-015-AC-112 to FR-015-AC-117

## Summary

Ticket: IR-858. Whether each new criterion can fail, judged by reading the spec text and the code on main.
Jev was not invoked; this is a manual criterion-strength pass.

- FR-015-AC-113: strong. A cover placed anywhere but last fails it, and so does a renderer that keeps family
  order for the cover.
- FR-015-AC-115: strong as an equality (see SR-12180 FND-002 and SR-12182 FND-002 for its evidence).
- FR-015-AC-117: falsifiable, but its controls are misnamed (SR-12182 FND-001).

Units examined: FR-015-AC-112, FR-015-AC-113, FR-015-AC-114, FR-015-AC-115, FR-015-AC-116, FR-015-AC-117, AD-004-H-1, AD-004-H-2, AD-004-H-3, AD-004-H-4, AD-004-R-1, AD-004-P-1, AD-004-P-2, AD-004-P-3, AD-004-L-4, AD-004-import-rule, AD-004-step-4b, TC-025-step-42, TC-025-step-43, TC-025-step-44, TC-025-step-45, TC-025-step-46, TC-025-step-47, tests.md-FR-015-AC-112-117, tests.md-TC-025-row. Context only: FR-015-AC-7, FR-015-AC-58, AD-004-L-3.

## Verdict

AC-112 misses invariants H-3 states, AC-116's render clause is ill-defined, and AC-114 carries universal
clauses no test can fail. Fix FND-001 and FND-002; FND-003 is wording.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-112 and TC-025 step 42 leave out invariants that H-3 and H-2 state: binding names must be valid Rust identifiers (H-3 item 3), messages must be valid (H-3 item 6, undefined per SR-12181 FND-008), and an ExprText must not contain a kani macro invocation (H-2). The constructor can drop any of them with no test failing. Add each to the AC and to step 42. | spec/kani/functional/FR-015-bounded-kani-obligations.md:737, spec/assurance/AD-004-cg-crate-layout.md:488-489, spec/assurance/AD-004-cg-crate-layout.md:493, spec/kani/matrix/TC-025-bounded-kani-obligations.md:417-421 |
| FND-002 | medium | FR-015-AC-116's render.rs clause ('no operation name, oracle symbol or the word scalar in a string literal') has no defined token set. Oracle symbols are generated per claim, and operation names such as 'sub' occur inside ordinary words ('subject'). Two implementers would write different checks. Removing the clause per SR-12180 FND-001 resolves this too. | spec/kani/functional/FR-015-bounded-kani-obligations.md:741, spec/kani/matrix/TC-025-bounded-kani-obligations.md:433-435 |
| FND-003 | low | FR-015-AC-114 says the output 'depends on no map or set iteration order, clock, environment or address'. Those universal clauses are not something a test can fail beyond the concrete checks (repeated calls, a second process, reordered inputs differ). Keep the concrete checks in the AC and move the rest to R-1 item 4 as design rationale. | spec/kani/functional/FR-015-bounded-kani-obligations.md:739, spec/assurance/AD-004-cg-crate-layout.md:527-530 |

## Dispositions

Round 1, checked at 15d7cad (fix 77f5178..15d7cad).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-112 adds identifier validity, the message and label rule, and the ExprText form and macro rules, and step 42 tests each one. |
| FND-002 | fixed | The undefined token-scan clause is gone. AC-116 is behavioural: the source is the fixed structure plus what the spec supplied. |
| FND-003 | fixed | AC-114 keeps only falsifiable checks; the purity statement is now R-1.4 design rationale. |

---
id: "SR-650"
title: "CG PR 208 spec review: FR-015-AC-37 scalar harness native relation"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a03a883409e9fa9e7a2567571b5736164c95f54b; spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/test-matrix.md, spec/test/complete-v1/TC-025-bounded-kani-obligations.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-650: CG PR 208 spec review

## Summary

Ticket: IR-458. PR: agent-ix/quire-contract-codegen#208 at a03a883, rebased on main 94ab14d. The
fix round added spec text for SR-648 FND-001, so this review covers it: FR-015-AC-37, its
test-matrix row, the TC-025 summary row, and TC-025 procedure step 9.

## Method

I read the new AC against the rendered harness and the tests that cite it, and checked the AC id
for collisions on main. I checked that the matrix and TC-025 entries agree with the AC and with
each other, and that `make spec` (`quire validate`) passes at the head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean.

- **FR-015-AC-37 is accurate.** Its first sentence matches the rendered text byte for byte: the
  `Completed`, `Refused` and `_ => false` arms over `exact: i128`. Each clause can fail. The default
  lane pins each arm and all four native expressions, and I showed every one of those pins is
  load-bearing by mutating it. The second sentence is existential ("an oracle whose arithmetic is
  mutated is falsified"). The kani-lane `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic`
  discharges it, and that test now asserts the failing check is the `sound` assertion.
- **The AC packs two behaviours**, the asserted shape and the backend verdict. That matches how the
  rest of FR-015's ACs are written (AC-19, AC-31), so it is not a finding.
- **No id collision on main.** Main 94ab14d's FR-015 ends at AC-36. The parked IR-459 and IR-464
  branches also add an FR-015-AC-37, so they must renumber when they resume.
- **The matrix agrees.** It has an FR-015-AC-37 → TC-025 ✅ Covered row, and the TC-025 summary row
  lists AC-37, with the TC still 🚧 Planned overall as before. TC-025 step 9 describes both lanes
  and cites the AC.
- `make spec` passed at a03a883 as part of `make ci`.

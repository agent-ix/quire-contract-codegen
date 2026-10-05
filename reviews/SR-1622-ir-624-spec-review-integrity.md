---
id: SR-1622
title: "IR-624 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@68098a543950bc3f3ab97d1c05d8e62f83f774f7; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md"
review_set: subset
---

# SR-1622: IR-624 spec-review/integrity review

## Summary

Ticket: IR-624. PR: quire-contract-codegen#294 at 68098a543950bc3f3ab97d1c05d8e62f83f774f7. Checked consistent interpretation of accessor-only ranges, mixed dispositions, IR admission precedence, and the TC-025/TC-035 status text. One status sentence contradicts the edited AC-30.

## Method

Examined: FR-015-AC-27, FR-015-AC-39, FR-015-AC-59, FR-015-AC-61, FR-015-AC-63, FR-015-AC-66, FR-015-AC-77, FR-015-AC-78, FR-015-AC-81, FR-024-AC-30, FR-024-AC-31, FR-024-AC-32, FR-024-AC-35, TC-025, TC-035, both matrix rows.

## Verdict

**CONDITIONAL**: a low-severity status contradiction needs correction.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-025 Status says FR-024-AC-30 text stays as merged, but this PR changes AC-30 to require an emitted-package harness and no `Twin::aligned`. Correct the status sentence so the review record does not contradict the restated criterion. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:337 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4f63198db87a6f70bbb0666b4a06596046f5e603 |

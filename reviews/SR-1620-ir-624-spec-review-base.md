---
id: SR-1620
title: "IR-624 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@68098a543950bc3f3ab97d1c05d8e62f83f774f7; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md"
review_set: subset
---

# SR-1620: IR-624 spec-review/base review

## Summary

Ticket: IR-624. PR: quire-contract-codegen#294 at 68098a543950bc3f3ab97d1c05d8e62f83f774f7. Checked edited criteria, procedures, and matrix status for ID and cross-reference integrity, testability, and planned coverage. One test procedure still names retired refusal variants.

## Method

Examined: FR-015-AC-27, FR-015-AC-39, FR-015-AC-59, FR-015-AC-61, FR-015-AC-63, FR-015-AC-66, FR-015-AC-77, FR-015-AC-78, FR-015-AC-81, FR-024-AC-30, FR-024-AC-31, FR-024-AC-32, FR-024-AC-35, TC-025, TC-035, both matrix rows.

## Verdict

**FAIL**: a high-severity test instruction is inconsistent with its criterion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TC-025 step 26 still requires constructing every retired `BoundNotResolved` case. FR-015-AC-66 now explicitly excludes those variants and requires the new accessor refusals. The test procedure is impossible to execute after the planned code change and would miss the replacement mapping cases; restate step 26 against the retained variants and causes. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:224 |

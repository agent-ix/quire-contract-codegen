---
id: SR-11640
title: "Base spec review of IR-268 bounded collection claims"
type: SpecReview
analysis: base
scope: "spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-96..102); spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: base
---

## Summary

Reviewed the exact FR-015 and TC-025 candidate diff against the base checklist. No applicable AssuranceProfile or review-selection mode exists under `spec/`. A QSL-emitted bounded `Sequence<IntRange>` `forall` package was admitted by Contract IR at f16d0fd in the focused fixture (1 passed); the current fixture does not exercise `exists` admission. Two scoped findings remain: the field-to-claim binding is not grounded by the fixture, and the changed criteria remain untagged in the computed matrix.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Specify and demonstrate how the admitted function-body collection operand is bound to the selected model field. The preserved fixture independently admits a model field named `items` and a function parameter also named `items`; its `forall` reads the function parameter, and no source evidence ties that parameter to the model field identity. Without this binding, the criterion does not establish that symbolic inputs use the selected field's admitted domain. | FR-015-AC-96, FR-015-AC-99, TC-025 | missing-requirement |
| FND-002 | medium | The scoped `quire matrix --scope <repo> --format tsv` reports FR-015-AC-96..102 as `untagged`. TC-025 describes planned controls, but no test trace tag currently binds those criteria to executable tests; add tags as the implementation tests land before claiming coverage. | FR-015-AC-96..102, TC-025 | correct-requirement-no-evidence |

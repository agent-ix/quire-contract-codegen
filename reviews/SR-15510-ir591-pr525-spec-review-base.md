---
id: SR-15510
title: "CG PR #525 generated-closure corpus base spec review"
type: SpecReview
analysis: base
scope: "CG PR #525 at 8346f8f7: FR-044, TC-058, oracle matrix and spec index"
review_set: subset
---
# SR-15510: CG PR #525 generated-closure corpus base spec review

## Summary

Reviewed the four changed spec documents at head 8346f8f7 against the
requirements checklist, QSL FR-262 and current CG FR-021. The author fixed
the initially broken core FR-043 index link before this review head. The
planned status and CG/QSL evaluator boundary are clear, but three criteria
and test-method details need correction before merge.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Both new AC verification cells say `Test (TC-058)` although the FR artifact contract requires the method alone; AC-2 also mixes behavioral tests with a source inspection of forbidden expectation derivation. Use method-only cells and give the inspection its own criterion or method. | `spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md:63-64`; `spec/oracle/matrix/TC-058-generated-closure-evaluation-corpus.md:31-34` | wrong-requirement |
| FND-002 | medium | The removed-case control requires an “expected census” failure, but neither the FR nor TC names an independent expected application set. A JSONL loader can admit the remaining records as a smaller valid corpus. State that the test compares loaded identities with the live admitted source cases, or otherwise define the missing-case oracle without adding a copied inventory. | `spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md:63-64`; `spec/oracle/matrix/TC-058-generated-closure-evaluation-corpus.md:26-30` | missing-requirement |
| FND-003 | medium | Atomic failure leaves the prior published bundle's fate unspecified. With an existing `cases.jsonl`, a failed replacement can leave stale content that the current “no partial bundle” assertion accepts. State whether the prior complete bundle remains or the target is absent, and test that exact post-failure state. | `spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md:50-53,64`; `spec/oracle/matrix/TC-058-generated-closure-evaluation-corpus.md:31-34` | missing-requirement |

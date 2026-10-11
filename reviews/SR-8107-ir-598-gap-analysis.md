---
id: SR-8107
title: "IR-598 gap-analysis spec review"
type: SpecReview
analysis: gap-analysis
scope: "CG IR-598 four-file Markdown diff, 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9929bfee76c99df96f56daf5add86d754d5b4289"
review_set: subset
---

## Summary

Compared TC-023 traceability prose with the computed Quoin matrix. Planned AC-95 to AC-102 have no tags or run evidence, but the existing blanket statement about AC-82 to AC-95 is inaccurate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-023 says none of AC-82 to AC-95 are bound, while the computed matrix marks FR-015-AC-94 tagged by seven test functions. Correct the sentence to exempt AC-94 or state the exact untagged set. This predates the diff but sits in the edited test-matrix artifact. | spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:20-21; quoin matrix --repo . --json: FR-015-AC-94 |

## Final disposition

Rechecked local fix `a4d0377fa06b57a7027a948a67eb2123911709d7` against reviewed head `9929bfee76c99df96f56daf5add86d754d5b4289`. FND-001 **FIXED**. TC-023 lines 20-24 now identify the seven AC-94 corpus unit-test tags and the exact untagged AC-82 through AC-93 and AC-95 set, matching the computed matrix. It distinguishes trace tags from proof evidence. No new regression in the changed paragraph.

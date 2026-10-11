---
id: SR-8100
title: "IR-598 base spec review"
type: SpecReview
analysis: base
scope: "CG IR-598 four-file Markdown diff, 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9929bfee76c99df96f56daf5add86d754d5b4289"
review_set: subset
---

## Summary

The change correctly keeps FR-015-AC-95 planned until symbolic V2 arithmetic, collection and graph paths have authentic admitted inputs and installed-backend verdicts. One unchanged AD-004 layout-map sentence in the edited artifact still gives QSL-353 alone as the retirement trigger and can cause premature deletion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-004 layout map says the corpus package lowerer is retired after QSL-353 at step 4g, but revised step 4g, FR-015-AC-95 and interface-001 require the complete symbolic V2 replacement and typed graph population input. Amend the map row to the same gate. | spec/assurance/AD-004-cg-crate-layout.md:298, :788-800; spec/kani/functional/FR-015-bounded-kani-obligations.md:699; spec/core/functional/interface-001-codegen-api.md:100 |

## Final disposition

Rechecked local fix `a4d0377fa06b57a7027a948a67eb2123911709d7` against reviewed head `9929bfee76c99df96f56daf5add86d754d5b4289`. FND-001 **FIXED**. AD-004 line 298 now makes step 4g contingent on admitted V2 arithmetic, symbolic collection and graph cases satisfying FR-015-AC-95 through AC-102, typed graph population, and the shared renderer. It explicitly limits QSL-353 to emission-to-admission coverage. This agrees with AD-004 lines 788-800, FR-015-AC-95, and interface-001. No new regression in the changed row.

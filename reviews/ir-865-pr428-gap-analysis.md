---
id: SR-12381
title: "IR-865 gap analysis of CG PR 428"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR 428, 836b298f..eee965cc; changed test evidence and computed matrix"
review_set: subset
---

# IR-865 gap analysis of CG PR 428

## Summary

The computed coverage and matrix are unchanged from the exact PR base to its head. The conversion adds no untagged test, unowned production behavior, stub, or hollow replacement for the existing test evidence.

## Verdict

**PASS for the PR diff** — no new gap or status contradiction. This is a diff-scoped result; it does not claim the repository's existing unbacked criteria are resolved.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new traceability gap in the PR diff (placeholder). | - |

## Coverage

Plan completion: not assessed. Optional semantic review was not requested; the check here is the computed traceability chain and changed source surface.

At base `836b298f` and head `eee965cc`, `quire coverage --format json` reports 499/855 rows backed, 221 unbacked rows, and 0 contradicted statuses. Both have 812 computed matrix criterion rows. After source line numbers are excluded, criterion IDs, statements, statuses, and binders are identical. A separate `quire matrix --format json` run at the head matches the matrix embedded in the coverage report.

The 351 new attributes sit on the same tests that carried the removed doc IDs. The generated agreement crate receives `ix-trace-rs` through its shared scratch manifest, so those attributed agreement cases still compile in that lane. No production source or requirement changed, and no added test body became a stub. Existing unbacked rows are outside this PR's change and remain visible in both reports.

## Final-head recheck

At `54256e7`, the fix commit changes only documentation prose and blank lines. An independent `quire coverage --format json` run at the final head matches the exact PR base: 499/855 backed rows, 221 unbacked, and 0 status lies. The computed matrix still has 812 criteria, with identical statuses and binders after line and column fields are excluded. The 351 attributed test functions retain their exact base ID lists.

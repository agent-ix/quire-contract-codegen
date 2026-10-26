---
id: SR-586010
title: "IR-348 CG #592 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; repository matrix and PR #592 changed code; AD-004 L-8, FR-033-AC-1"
review_set: subset
---
# IR-348 CG #592 gap analysis

## Summary

Ticket: IR-348. Static Quire matrix and reverse ownership were inspected at `d44a9420690969e743f04fa8099991c54a484174`. The changed replay lookup is owned by AD-004 L-8 and FR-033-AC-1; the unsupported-operator arm has FR-014-AC-2 ownership. The diff adds no new production behavior, stub or unowned code. The new source guard is untagged, consistent with the existing AD-004 source guards; it does not establish crate-wide L-8 completion.

## Verdict

FAIL at repository scope because the computed matrix has 322 untagged criteria and 16 tagged only by ignored tests. These gaps predate CG #592; the changed slice introduces no new trace gap. FND-001 records a representative inherited untagged criterion, not a request to expand this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The computed matrix contains 322 untagged criteria, including FR-015-AC-41; this is inherited repository assurance debt. | spec/kani/functional/FR-015-bounded-kani-obligations.md:789 |

## Coverage

Plan completion: not assessed. `quire matrix --scope . --format tsv` produced 951 criterion rows: 560 tagged, 322 untagged, 53 method-without-symbol and 16 tagged-by-ignored-test. FR-033-AC-1 and FR-014-AC-2 are tagged in the computed matrix, including tests of the changed composite builder path. The two changed files contain no acceptance-criterion tag or spec edit, so the repository gaps are inherited. No new source stub or copied implementation appeared. Optional semantic review was not requested; the Opus code review did inspect the changed path's semantics.

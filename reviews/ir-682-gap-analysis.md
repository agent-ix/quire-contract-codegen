---
id: SR-2983
title: "IR-682 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #320 head (commit subject: Specify build-specific native workspace conformance analysis); spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
---

# SR-2983: IR-682 gap-analysis review

## Summary

Ticket: IR-682. PR: quire-contract-codegen#320 is spec-only. Plan completion: not assessed. The
computed matrix adds 10 Analysis rows (FR-034-AC-41 to FR-034-AC-50). Each one computes
`method-without-symbol`, and no test or source symbol is tagged to them. All prior rows are
unchanged. No code is added, so no code is left without an owning requirement. The pending IR-639
source names the "supported-build native-storage measurement" in a comment
(caller_bootstrap.rs), and this PR now gives that measurement an owner. No stub or tautological
test was introduced.

## Method

Planless audit. Ran `quire matrix --format tsv` on base and head, diffed the rows, and counted
statuses. Ran `quire matrix --strict` on both (both exit 1). Grepped the pending IR-639 code
branch for `caller_run_buffers` and stack-size constants, to check that the new requirement has
code to own.

## Verdict

**PASS WITH ONE LOW FINDING.** Coverage is not worse and no test claims the new criteria. One
coverage-inflation risk is recorded.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The 10 new PLANNED/UNRUN Analysis criteria compute `method-without-symbol`, which `quire matrix --strict` counts as passing. Strict coverage therefore rises by 10 criteria with zero executed evidence: only the statement text says UNRUN. The PR body reports this correctly, but a strict-matrix consumer cannot tell these rows from completed Analysis. | spec/kani/functional/FR-034-caller-death-ownership.md:1476-1485 | correct-requirement-no-evidence |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1227-1230 and TC-049:915-916 say that a computed method-without-symbol row is not executed evidence and that completion is zero until a receipt exists. The tool status itself is unchanged. The matrix goes from 578 to 588 rows with the same strict failure set. |

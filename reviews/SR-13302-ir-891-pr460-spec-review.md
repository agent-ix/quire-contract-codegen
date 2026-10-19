---
id: SR-13302
title: "IR-891 PR 460 spec review: core matrix and test-index status rows for NFR-002 and TC-001"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@619b84e2e959b68fc3f6f1a81624ab6930447c98 vs origin/main 58f44780; spec/core/matrix/tests.md, spec/tests.md"
---

# SR-13302: IR-891 PR 460 spec review

## Summary

Ticket: IR-891 (item 1) and IR-897. The PR's spec diff is three status cells. The NFR-002 TC-001
row moves from `🚧 Planned` to `✅ Covered` with the note "header identity and SPDX checks span
emitted `.rs` artifacts of every family in NFR-002 Scope". The TC-001 index row moves to
`🚧 Partial`, with NFR-002 covered and FR-005-AC-2 and NFR-001-AC-1 still planned. The Core row of the
`spec/tests.md` index is updated to match. No requirement, AC or relationship changed, so the
EARS, integrity, object and dependency sub-analyses have nothing in scope and were not run.
`make spec` passes.

The TC-001 `Partial` split is correct: FR-005-AC-2 and NFR-001-AC-1 have no new tests in this
PR. The three rows agree with each other.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The NFR-002 row says `✅ Covered` and claims the checks span every family in NFR-002 Scope. At the head, five emitter subject paths that TC-001 names have no test that can fail (SR-13301 FND-001; SR-13300 FND-002 to FND-005), and one NFR-002 binder is on a test with no header assertion (SR-13300 FND-006). The Covered status overstates the evidence. Fix: keep the row `🚧 Partial` and name the uncovered paths until those tests land, or land them in this PR and keep `✅ Covered`. | spec/core/matrix/tests.md:34, spec/tests.md:15 |

## Verdict

**FAIL as written.** The row structure and the TC-001 Partial split are right. The NFR-002
Covered status must wait until the coverage gaps reported in SR-13300 and SR-13301 are closed.

## Dispositions

Disposition pass 1, at head 627790b83bae15eb440a920bbdee27214b8d21e5.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The coverage gaps were closed in this PR (SR-13301 FND-001 fixed), so the unchanged `✅ Covered` NFR-002 row is now backed by tests |

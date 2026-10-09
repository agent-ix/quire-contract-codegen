---
id: SR-3201
title: "IR-694 spec review (integrity): FR-034-AC-94, TC-049 queued-claim checks and trace rows"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen PR #327, branch spec/ir694-queued-claim-cleanup (frozen head named in the Linear marker, one commit over main); spec/kani/functional/FR-034-caller-death-ownership.md (new Behavior section and AC-94 row), spec/kani/matrix/TC-049-caller-death-ownership.md (allocation row, procedure section, Expected Results row), spec/kani/matrix/tests.md (FR-034 coverage row, TC-049 criterion row); quire matrix --format tsv on main and on the PR tree; context: open branch spec/ir689-stage2-observation (AC-78..AC-93)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3201: IR-694 spec review (integrity)

## Summary

Ticket: IR-694. This pass checks internal consistency, id uniqueness, traceability completeness
and the matrix delta.

- The computed Test Matrix grows from 615 to 616 criterion records. A diff of the two TSV outputs
  shows exactly one added record, FR-034-AC-94, computed `untagged`. All 615 prior records are
  byte-identical.
- AC-94 is unique across every remote branch. The gap AC-78..AC-93 is deliberate: those ids belong
  to the open IR-689 branch, and the reservation comment on the ticket records this.
- AC-94 is present in all five places it must be: the FR-034 AC table, the TC-049 evidence
  allocation table, the TC-049 procedure, TC-049 Expected Results, and both tests.md rows.
- Every new row is PLANNED/UNRUN and claims no test credit.
- No 7+ hex string, local path or date appears in any added line.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-94 is compound. It bundles at least five independently failing properties in one criterion carrying both Test and Analysis: preservation of the negative, no phase/I-right authority, same-cursor authentication, the original cutoff, and whole-chain settlement with CleanupUnconfirmed on damaged delivery. The neighbouring IR-687 criteria AC-57..AC-77 are atomic, one property each. The matrix cannot record one clause as partial. Low, because the ticket mandates ONE criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1836 |
| FND-002 | low | The diff adds two stray blank lines, giving a double blank line in each file: FR-034 between the AC-77 receipt paragraph and "Private retention", and TC-049 before the new section heading. | spec/kani/functional/FR-034-caller-death-ownership.md:398-399; spec/kani/matrix/TC-049-caller-death-ownership.md:1131-1132 |

## Verdict

**Integrity is clean apart from two low findings.** The traceability and matrix delta are exactly
as claimed: 615 to 616 records, all prior records unchanged, AC-94 PLANNED/UNRUN and untagged.

Merge-order note, not a defect: the open IR-689 branch edits the same two tests.md rows and appends
AC-78..AC-93 to the FR-034 AC table. Whichever merges second needs a textual rebase that keeps
AC-78..AC-93 ahead of AC-94.

## Dispositions

Replacement Codex disposition, round 1, on the frozen published PR head. Reviewer run 3911c05e-0b3e-4a4f-bac7-379bbc166b22. The original findings above are unchanged.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | AC-94 is now one pending-claim custody/disposal transaction. The TC tests its conditions independently and expressly denies partial Test or matrix completion; the all-or-nothing criterion has no ambiguous partial status. |
| FND-002 | fixed | The correction removes both extra blank lines. |

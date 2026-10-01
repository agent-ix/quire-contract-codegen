---
id: "SR-765"
title: "CG PR 225 spec review (integrity): unique review ids and matching filename prefixes"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@5d37b92e874c9ea2fa9aff9603e1010f12fee4de; reviews/**, plan/PLAN-001-codegen-v01/tasks/Task-005-backends.md"
relationships:
  - target: ix://agent-ix/quire-contract-ir/ADR-0056
    type: references
---

# SR-765: CG PR 225 spec review (integrity)

## Summary

Ticket: IR-487. PR: agent-ix/quire-contract-codegen#225 at 5d37b92, base 224ca6e (origin/main,
equal to the merge base). Records-only change: 16 review files renamed or re-identified, and 3
citations updated. This review applies spec-review with the integrity sub-analysis. EARS and
the other sub-analyses do not apply: no requirement text changed.

Measured on a detached worktree at the reviewed sha:

- **Unique ids.** reviews/ holds 99 files, each with exactly one frontmatter id, and no id is
  declared twice. Every `SR-NNN-` filename prefix equals its file's id. The 23 files without an
  SR prefix (dated `2026-09-*` and `ir-*`) all carry SR ids. No REV id remains.
- **Content untouched.** `git diff -M origin/main...HEAD` shows 16 renames at 98-99% similarity.
  Each changes only the `id:` line, plus the H1 in SR-806 and SR-807, the only two whose H1
  carried the id. The only other edits are the three citations. No renamed file mentions its
  old id anywhere in its body.
- **Citations.** I grepped the whole repo, not only spec/ and plan/, for each old id
  (REV-015, REV-018..REV-022, SR-008..SR-021) and each old filename. What remains:
  - plan/PLAN-001-codegen-v01/log.md:16,23 and Task-009:59 cite SR-014..SR-017. Those ids
    still belong to the dated 2026-09-12 code-review and gap-analysis files they meant, so the
    citations are now unambiguous.
  - The dated files cite each other as SR-016 and SR-018. Both ids are unchanged.
  - SR-041:218,245 mention SR-016/SR-017 as matrix history. Those are the kept dated ids.
  - SR-658:54,81 describe the old duplicates as history. A review record, rightly left as
    written.
  - No markdown link anywhere targets a renamed path.
  - Outside this repo, the only old path seen is in quire-rs
    reports/2026-09-20-jev-lens-baselines-m8.tsv:226-227. That is a dated baseline snapshot,
    so it counts as history.
- **Inferred citation.** 2026-09-12-numeric-strategy-core-gap-analysis:42 now reads "SR-802
  through SR-805". This is correct. The cited set is "base, failure-domain, integrity, and
  scope-boundary", which matches the four numeric-strategies files exactly. Those files were
  added in the same commit (b3af1c6) as the gap analysis, and the gap analysis covers the same
  slice (FR-008..FR-013, NFR-004). The other SR-010..SR-013 set (numeric-state-oracle) is
  integrity, evidence, risk-complexity and scope-boundary, which does not match.
- **Linear provenance.** I pulled all 163 Linear comments whose body contains
  `repo=agent-ix/quire-contract-codegen`, which is the complete result set. Their reviewer
  markers span SR-033..SR-741, so none carries any id this PR changes. A body search for
  `id=SR-00`, `id=SR-01`, `id=SR-02`, `id=SR-8`, `id=SR-75` and `id=SR-76` found no
  quire-contract-codegen marker. REV-015 and REV-018..REV-022 appear only in IR-321's
  narrative comments, which describe the duplicates. (IR-425 cites quire-contract-ir's own,
  unrelated REV-015.) No renumbered file had posted-comment provenance.
- **New block collides with nothing.** No origin branch other than this PR's declares
  SR-800..SR-815. No open PR (#209, #222, #223, #224) touches reviews/ or plan/. No Linear
  marker uses SR-8xx.
- **Gate.** `make spec` exits 0 at head. It reports the same 3 grammar warnings as origin/main
  (FR-017:159 once, FR-014:278 twice), with no new diagnostics.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | The six REV-0xx files (REV-015, REV-018..REV-022) were re-identified as SR-800..SR-805, although their ids were never duplicated. Only their filename prefixes were wrong. Renaming the files to `REV-0xx-<slug>.md` would have fixed the prefix mismatch without changing six unique ids, and without editing the Task-005 and SR-803 citations. The quire-contract-ir repo already uses `reviews/REV-015-...` names. ADR-0056 Identifiers rule 6 leaves review identifiers to the repo, so either choice is valid. The chosen one gives one SR family and records the mapping in the PR body, but it changes ids that did not need to change. A judgement call, not blocking. | reviews/SR-800-vacuity-recovery-gap-analysis.md:2, reviews/SR-805-numeric-strategies-scope-boundary.md:2 |

## Verdict

Mergeable. The PR fixes what IR-487 asks for. Ids are unique, prefixes match, and no stale
citation or link remains. Content outside the id lines is untouched, the gate is unchanged, and
no Linear provenance is lost.

There is one note on ADR-0056 rule 5, "a collision is renumbered in the later pull request".
For SR-014, the renumbered SR-014-numeric-state-oracle-ears-conformance (e0be330, Sep 12) is
older than the kept dated SR-014 code review (b3af1c6, Sep 13). Rule 6 exempts reviews from
the identifier rules, and keeping the plan-log-cited id avoids rewriting citations, so this is
not a finding. The remaining duplicate pairs were each added in a single commit, so "later"
does not decide them.

FND-001 is a style judgement; merging as is is acceptable.

## Dispositions

| FND | Outcome | sha/reason |
|---|---|---|
| FND-001 | fixed | ad53660: the six files are now REV-015/018/019/020/021/022-<slug>.md, with their REV ids restored. The Task-005 citation is reverted to REV-015, the REV-020 citation to REV-022, and the core gap analysis citation to "REV-019 through REV-022". Re-verified at ad53660 (rebased on 123e3ab): 101 files with unique ids, every prefix equal to its id, and no stale SR-800..805 or old-prefix reference outside history. `make spec` exits 0 with 0 warnings. |

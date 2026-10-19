---
id: SR-13292
title: "IR-891 CG PR 458 spec-review integrity: matrix consistency and overlap with PR 457"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@242c506c617a078d26d748c3fac375f2056f5931 vs origin/main e45b9d2f; spec/core/matrix/TC-001-reproducibility.md, spec/core/matrix/tests.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/tests.md, spec/tests.md, spec/replay/functional/FR-016-witness-native-replay.md"
review_set: subset
---

## Summary

Ticket: IR-891. TC ids, AC ids and Traces To columns are unchanged: no id reuse or renumbering.
NFR-002-AC-1/AC-2 and FR-016-AC-6 stay Planned in the owning matrix rows, the TC index rows and
`spec/tests.md`, and `quoin matrix --json` shows all three untagged with no run evidence, which
agrees. Titles of TC-001 and TC-026 match between the TC files and the index rows.

Overlap with PR #457 (IR-890, not reviewed here): `git merge-tree` of the two heads conflicts in
`spec/replay/matrix/TC-026-witness-native-replay.md`, `spec/replay/matrix/tests.md` and
`spec/tests.md`; `FR-016` auto-merges.

## Examined criteria

- spec/core/matrix/tests.md:34, :57 (examined)
- spec/replay/matrix/tests.md:14, :39 (examined)
- spec/tests.md Core and Replay rows (examined)
- TC-001 front matter and title (examined)
- TC-026 front matter, title, procedure, expected and status text (examined)
- FR-016 "five replay results", FR-016:64, :130 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Conflicts with PR #457. Textual: both rewrite the TC-026 procedure sentence (:24-30), expected-results sentence (:52-58) and status sentence (:135-136), the replay matrix TC-026/TC-035 and FR-016/FR-024 rows, and the spec/tests.md Replay row. Semantic: this PR keeps "a witness whose native replay is unavailable", "out-of-domain, mismatch and unavailable results" and "the unavailable result for an executor fault (AC-12)", which #457 retires (Fault becomes a typed replay error mapped to `Failed`). The "five replay results" count (FR-016:64, AC-8, TC-026:49) drops to four under #457, and this PR's missing-limit refusal sits outside that set unstated. Fix: whichever lands second rebases and keeps both changes, removing the unavailable outcome from this PR's rewritten sentences and stating the count. | spec/replay/matrix/TC-026-witness-native-replay.md:24-30, :52-58, :135-136; spec/replay/matrix/tests.md:14; spec/tests.md:19 |

## Verdict

**CHANGES REQUESTED (sequencing).** Matrix and id integrity are clean; reconcile with PR #457 when
the second of the two merges.

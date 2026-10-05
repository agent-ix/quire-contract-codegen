---
id: "SR-1505"
title: "CG PR 277 spec review (base checklist): FR-015 BoundNotResolved sentence, AC-66 and AC-67 status flips, tests.md split and TC-025 note"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@37a7e1e21162ed1af6031fa03dd0464ad4da62e7; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md (diff origin/main...HEAD, merge base e526390)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-1505: CG PR 277 spec review (base checklist)

## Summary

Ticket: IR-461 (code PR 1 of 2). PR: agent-ix/quire-contract-codegen#277 at 37a7e1e, diffed
against origin/main e526390. The PR edits three spec files:

1. FR-015:341-344. The `BoundNotResolved` sentence changes from "the field, and the member's
   `value.target` node when it has one" to a `cause` (`BoundNotResolvedCause`) over five
   grounds, each with the `value.target` node when the member has one. This changes text merged
   by #275. I judged it on its merits. The old payload could not tell an unbounded type from a
   bound that is not an `integer_range` without reading the graph again, and table rows 326-327
   key on exactly that split. The edit makes the sentence implementable, keeps both rows, and
   changes no AC, disposition or reason. Row 327 says "names the field and the bound node". The
   new sentence's "when it has one" covers the absent-member and non-reference grounds, which
   have no node. The edit is consistent.
2. FR-015-AC-66 and AC-67 drop `PLANNED (IR-461)`. Tagged tests exist for both (SR-1504
   measured the coverage), so the flip is truthful. The AC text is otherwise byte-identical.
   AC-59 to AC-65 and AC-68 keep `PLANNED`.
3. tests.md splits the FR-015-AC-59 to AC-68 row into a Planned row (AC-59 to AC-65, AC-68) and
   a Covered row (AC-66, AC-67). TC-025:229 records that the first change backs steps 26 and 27.

Checks: `quire validate` passes in `make ci` with no new warning (the warnings at FR-017:152
pre-date this PR). The EARS statements at FR-015:395-398 are unchanged. No requirement,
criterion or verification method was added or removed.

## Verdict

PASS for the spec edits as edits. The one wording defect in the new tests.md Covered row, which
claims the engine's `MalformedClause` for a non-identifier graph field name while only the read
path is tested, is recorded once in SR-1504 FND-001 and not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

---
id: SR-581004
title: "Integrity review of IR-1007 CG PR 581 (FR-021 delivery status re-point)"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@ee51b2590f4e2909fc121fbeda082348a64fd37d; spec/assurance/AD-004-cg-crate-layout.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md"
---

## Summary

Ticket: IR-1007. PR: quire-contract-codegen#581, frozen head ee51b2590f4e2909fc121fbeda082348a64fd37d.
Reviewer model: Claude Opus 5.5 (`claude-opus-5-5`). Method: spec-review/integrity. The three
edited files agree with each other on the new status: the QSL facade is delivered, and the
CG consumer is Planned. `quire validate` passes on all four files. But the authored matrix
index `spec/oracle/matrix/tests.md` was not updated. Five of its rows still say these
criteria await IR-1007 and the facade, which contradicts FR-021 and TC-031 at the same head.

## Examined scope

- AD-004:361-375, FR-021:43-48, :81-83, :341-376, :426-431, :460-468, TC-031:16-21, :89, :95,
  :139-140, :146-148, :293-295: examined. The status is consistent: every file says FR-485 is
  delivered by merged #1483 and the IR-595 consumer is Planned.
- Stale references repo-wide: examined with `git grep -E "IR-1007|#1483"`. Outside
  `reviews/` (historical SR records, out of scope), the only remaining IR-1007 gate text is in
  `spec/oracle/matrix/tests.md`.
- spec/oracle/matrix/tests.md row 41 (FR-021-AC-1..3, AC-5..14, AC-17): examined. It says
  "🚧 Planned pending FR-485 through IR-1007 and the IR-595 CG consumer".
- spec/oracle/matrix/tests.md row 42 (FR-021-AC-4): examined. It says "the FR-485 owner path
  is PLANNED pending IR-1007 and IR-595".
- spec/oracle/matrix/tests.md row 50 (FR-021-AC-35..37): examined. It says "🚧 Planned
  (IR-1007/IR-595); ... await the facade and CG consumer".
- spec/oracle/matrix/tests.md row 51 (FR-021-AC-32): examined. It says "PLANNED pending
  IR-1007 and IR-595".
- spec/oracle/matrix/tests.md row 103 (TC-031 summary): examined. It says "AC-2, AC-4, AC-5
  corpus, AC-9 vectors, AC-17 runtime, AC-32 and AC-35 to AC-37 await IR-1007/IR-595".
- `quire validate --scope .` on the three changed files plus tests.md: examined. It passes,
  with only module-configuration warnings.

## Verdict

CHANGES REQUESTED (one medium). The PR's own three files are internally consistent, but the
matrix index still reports the old gate. Anyone reading coverage status from `tests.md`, which
is the matrix index CLAUDE.md names, sees FR-485 as undelivered.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `spec/oracle/matrix/tests.md` was not updated with the three files. Rows 41, 42, 50, 51 and 103 still gate FR-021-AC-1..5, AC-8, AC-9, AC-14, AC-17, AC-32 and AC-35..37 on "IR-1007" and say they "await the facade". At the same head, FR-021, AD-004 and TC-031 say the QSL FR-485 facade is delivered by merged #1483 and only the CG consumer is pending. Recommended fix: in those five rows, replace "IR-1007/IR-595", "pending IR-1007 and IR-595" and "pending FR-485 through IR-1007 and the IR-595 CG consumer" with the FR-021 wording ("pending the IR-595 CG consumer; the QSL FR-485 facade is delivered"). In row 50, drop "await the facade and". Keep every row Planned. Whatever owner is chosen for SR-581002 FND-001 applies here too. | spec/oracle/matrix/tests.md:41; spec/oracle/matrix/tests.md:42; spec/oracle/matrix/tests.md:50; spec/oracle/matrix/tests.md:51; spec/oracle/matrix/tests.md:103 |

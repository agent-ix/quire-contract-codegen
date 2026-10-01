---
id: "SR-658"
title: "CG PR 213 spec review (integrity): subsystem restructure, ids and bodies unchanged"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@b17b2b171fc52500c03710239a7cba5abc0e2f1f; spec/**, reviews/SR-0{08..21}-numeric-state-*.md, reviews/SR-645-ir-318-rust-review.md"
relationships:
  - target: ix://agent-ix/quire-contract-ir/ADR-0056
    type: references
---

# SR-658: CG PR 213 spec review (integrity)

## Summary

Ticket: IR-321. PR: agent-ix/quire-contract-codegen#213 at b17b2b1, base baab597.
This review applies spec-review with the integrity sub-analysis to a mechanical restructure.
EARS is not re-run on the moved text, because the requirement text is unchanged: `make spec`
reports the same 3 grammar warnings before and after.

Measured with scratch commands outside the repo, against `git archive` trees of baab597 and
b17b2b1:

- **Frontmatter ids.** spec/ (minus the old spec/reviews/) has 64 ids at base and 71 at head.
  None is missing and none is duplicated. The seven new ids are TM-002 to TM-008.
- **Acceptance criteria.** There are 263 defined AC ids at base and 263 at head: the same set,
  each defined once.
- **Mentioned ids.** Every FR, NFR, StR, AD, ADR, SUR, TM, TC and interface id mentioned at
  base is still mentioned at head. The only new mentions are ADR-0056 and TM-002 to TM-008.
- **Bodies.** All 30 files whose bytes changed differ from base only in markdown link targets:
  25 FR/NFR files and 4 ADRs, plus `spec/index.md`, which became `spec/spec.md`. The
  normalised body compare shows no other change. All 14 spec/reviews files and every TC file
  are R100.
- **Matrix rows.** All 110 table rows of `spec/test-matrix.md` (headers included) are present
  verbatim across the seven subsystem matrices. Each data row appears exactly once. The only
  repeats are table headers. The only new rows are the 8 rows of `spec/tests.md`. All of the
  old matrix's prose notes moved verbatim to the oracle, routed and strategy matrices.
- **Links.** 188 relative links in spec/, reviews/ and plan/ resolve at head (178 at base).
  Links in README, CLAUDE.md and AGENTS.md also resolve.
- **`make spec`.** It exits 0 before and after, with the same 3 warnings (FR-014:278 twice and
  FR-017:137).
- **`quire coverage`.** It reports 206/302 rows backed before and after, 365/366/371 rust
  bound/tagged/candidates, and 112 diagnostic lines, identical apart from paths. The old
  matrix's 23/29 splits into 4+1+5+3+1+2+7 = 23/29.
- **SR-645.** `reviews/SR-645-ir-318-rust-review.md` is byte-identical to the audit source
  file. Its id SR-645 is unique in the repo.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-005 sits in core, but its frontmatter `depends_on` FR-002 (strategy), FR-015 (kani) and FR-004 (evidence). ADR-0056 rule 4 says "A `core` requirement depends on no other subsystem's requirement". FR-005 (library/CLI publication and cross-backend parity) is an integration requirement over the other subsystems, not a shared primitive. Options: give it (with TC-001, TC-002 and TC-007 and the `publication` module) its own subsystem, or record the exception against the ADR | spec/core/functional/FR-005-cli-conformance.md:6-11 |
| FND-002 | low | The "Requirements Architecture" area table in spec.md still groups requirements across the new subsystems. Its "Backend adapter" row mixes FR-026 (routed) with FR-029 and FR-030 (kani). Its "Tri-state harnesses, vacuity and publication" row spans strategy, evidence and core. A reader now meets two different groupings in one document; the Subsystem Registry should be the one grouping | spec/spec.md:58-66 |
| FND-003 | low | `spec/core/matrix/suites.md` (SUR-001, SuiteRegistry) sits in a `matrix/` directory. ADR-0056 says a matrix directory holds `tests.md` and the TC-### files it declares. The suite registry serves every subsystem's suites. ADR-0056 has no slot for a SuiteRegistry, so this needs either an ADR note or a spec-root location | spec/core/matrix/suites.md:1-5 |
| FND-004 | low | Pre-existing, made visible by the renames. reviews/ now holds two files per filename prefix for SR-008 to SR-013. The older SR-008 to SR-013 files carry frontmatter ids REV-015 and REV-018 to REV-022, so their prefixes are wrong. Frontmatter ids SR-010, SR-011 and SR-014 to SR-021 are each declared by two files: the same 10 duplicates as at baab597, where one copy was under spec/reviews/. Nothing breaks: quire validate does not check duplicate ids, and nothing resolves SR ids by path. But the plan log's mentions of "SR-014" to "SR-017" are ambiguous. Needs a ticket to renumber the later reviews (ADR-0056 Identifiers rule 5) | reviews/SR-010-numeric-state-oracle-integrity.md:2, reviews/SR-008-vacuity-recovery-gap-analysis.md:2 |

## Verdict

The id-preservation integrity is exact: no id is lost, duplicated or changed, the requirement
bodies are byte-identical modulo links, the matrix row sets are identical, and every link
resolves. The layout follows ADR-0056: kind directories only where they hold files, TC files
beside their declaring matrix, TM-001 kept, spec/reviews/ removed, and the registry and the
directories in agreement. Two cross-subsystem references are legitimate and remain unchanged:
TC-033 (routed) traces FR-015-AC-15 to AC-18 (kani), and StR-001's coverage rows in core cite
other subsystems' cases.

## Parked branches (report only)

These parked branches will hit the following moves when they rebase. Git rename detection
carries the FR and TC edits. Every matrix edit must be re-applied by hand, because
`spec/test-matrix.md` is deleted.

- `feat/ir-459-frame-envelope-in-src` and `feat/ir-459-parameter-domains-package`:
  - FR-015 moves to spec/kani/functional/.
  - TC-025 moves to spec/kani/matrix/.
  - Their matrix rows (FR-015 AC-26..36 and TC-025) go to `spec/kani/matrix/tests.md`.
- `feat/ir-277-structured-kani-verdicts`:
  - FR-017, FR-029 and FR-030 move to spec/kani/functional/.
  - TC-027, TC-040 and TC-041 move to spec/kani/matrix/.
  - interface-001 moves to spec/core/functional/.
  - AD-001 stays where it is.
  - Its FR-017, FR-029, FR-030, TC-040 and TC-041 rows go to `spec/kani/matrix/tests.md`.
- `fix/ir-464-nonvacuity-cover`:
  - FR-015 moves to spec/kani/functional/.
  - Its FR-015-AC-37 and TC-023 rows go to `spec/kani/matrix/tests.md`.
  - Its TC-007 row goes to `spec/core/matrix/tests.md`.

---
id: SR-047
title: "Spec review — quire-contract-codegen PR #193 drop dangling PGM-01 citations"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@aa35f7f28c8650725dfb805fd3e24ba1dd68f203; spec/index.md, spec/stakeholder/StR-001-traceable-generation.md, plan/PLAN-001-codegen-v01/tasks/Task-003-dependency-reconciliation.md, planning/pgm-01-reconciliation.md (deleted), planning/release-decision.md (deleted)"
review_set: subset
---

## Summary

Ticket: none. The PR branch (`chore/drop-pgm01-citations`) and title carry no ticket id, and no
Linear issue is attached to PR #193. Base is `origin/main` 9e6e2b7.

The PR removes the `depends_on` and `relationships` edges to `ix://agent-ix/quire-contract-ir/PGM-01`
from `spec/index.md`, the StR-001 `## Dependencies` section that only cited PGM-01, and the
Task-003 `relationships` edge. It deletes REV-003 (`planning/pgm-01-reconciliation.md`, a
pin/digest/merged-SHA reconciliation record) and REV-004 (`planning/release-decision.md`, an open
release-decision record). It replaces the index References entry with ISO/IEC/IEEE 29148.

Checks made at the reviewed sha:

- `PGM-01`, `pgm01`, `REV-003`, `REV-004`, `release-decision` and `pgm-01-reconciliation` searched
  across every tracked file. No hit remains in `spec/`, `src/`, `tests/`, `README.md`, `AGENTS.md`,
  `CLAUDE.md` or the Makefile. The remaining hits are prose in historical records under `planning/`
  (REV-002, REV-005, SR-001 foundation gap analysis), `reviews/` (SR-001, SR-005, SR-006) and the
  bodies of done tasks Task-002 and Task-003. None is a frontmatter relationship edge. The one
  REV-004 mention (planning/foundation-gap-analysis.md:28) is a table cell, not an edge.
- No `ix://` target anywhere in the tree points at PGM-01, REV-003 or REV-004.
- Nothing live depends on the deleted files. The release decision itself is still owned by
  Task-007 (human-owned, not_started), and spec prose that names "the human release decision"
  (README.md:37, interface-001:294, AD-001:216) names the concept, not REV-004.
- `schemas/pgm01-derivation-evidence-envelope-v1.schema.json` no longer exists on main, and `src/`
  has no `pgm` reference. The PR description's "follow-up" about `src/oracle.rs` `include_bytes!`
  of that schema is stale (untrusted PR text; measured here, it is false). Nothing to follow up.
- `quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'`
  (the `make spec` globs, run with quire 0.33.0 directly because the Makefile's hardcoded
  `$(TRUSTED_HOME)/.npm-global/bin/quire` path is not used here) exits 0 at both origin/main and
  the PR head. The output is identical, 10 lines each: module-loading notices plus two
  pre-existing EARS warnings on FR-014:278. No new failure.

On the References line: the ISO spec skeleton (`spec-artifacts-iso` v0.19.0,
`skeletons/spec.md:74-79`) lists ISO/IEC/IEEE 29148 as its first References entry, the
MasterRequirements schema requires a `references` section, and this index already declares
`standards_alignment: [iso-iec-ieee-29148]` on main. The StR uses 29148's
need/rationale/validation-criteria structure. The citation is accurate, not invented.

## Verdict

**PASS**. Every dangling edge to PGM-01 is removed; no live reference to PGM-01, REV-003 or
REV-004 remains in `spec/` or live docs; both deletions are ceremony-only records with no live
dependent; validation is unchanged against main. The References line is correct and is the minimal
content: keep it as written. The skeleton's other two suggested entries (the repository README, and
the specs of upstream components named in `relationships`) are optional. The index no longer has
frontmatter `relationships`, and its upstream links to quire-contract-ir, quire-contract-runtime
and quire-spec-language are carried per-FR, so adding them to References would be new content,
not a fix.

Task-003's `## Current State` (line 18) still says "PGM-01, runtime, and IR are pinned to merged
revisions". Task-003 is `status: done`; that is historical record prose, allowed by the owner rule,
and not listed as a finding.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low | No findings (placeholder) | - |

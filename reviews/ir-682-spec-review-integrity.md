---
id: SR-2982
title: "IR-682 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #320 head (commit subject: Specify build-specific native workspace conformance analysis); spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2982: IR-682 spec-review/integrity review

## Summary

Ticket: IR-682. PR: quire-contract-codegen#320. Structural checks:

- `quire validate` (quire 0.36.1, engine 0.50.1) exits 0 on both changed files, with seven
  environment notices (one `semantic.inline-data-schema`, five `DuplicateArchetype`, one
  `DuplicateInverseEdge`).
- The computed matrix goes from 572 to 582 criterion rows. Exactly 10 rows are added
  (FR-034-AC-41 to FR-034-AC-50, all `method-without-symbol`), and every prior row is
  byte-identical.
- `quire matrix --strict` exits 1 on both base and head, with the same 244 non-passing rows
  (233 untagged, 11 tagged-by-ignored-test).
- Every TC-049 procedure step maps to the new criteria: step 1 to AC-41, step 2 to AC-42,
  step 3 to AC-43/44/45, step 4 to AC-45/46/47, step 5 to AC-48, step 6 to AC-49/50.
- The new text does not repeat the existing fixture-build lines (526-540). It refers to them as
  "the matched normal-artifact delivery rules above".

## Method

Ran `quire validate --scope . <file>` on each changed document, and `quire matrix --format tsv` and
`--strict` on detached base and head worktrees, then diffed the TSV. Compared TC-049's
per-criterion tables, the slice allocation table and Expected Results, against their existing
rows for FR-034-AC-31 to FR-034-AC-40. Checked that each referenced definition has an owner in
the base spec.

## Verdict

**NOT CLEAN.** The matrix and validation are intact, but TC-049's traceability tables were not
extended, and one reference has no owner in the specification.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-049 adds a procedure section for FR-034-AC-41 to FR-034-AC-50 but no rows in its two per-criterion tables, although every criterion from FR-034-AC-31 to FR-034-AC-40 has a row in both. The tables are the slice-1/slice-2 evidence allocation (the IR-655 staging, lines 74-106) and Expected Results (lines 947-970: authority, required observation, regression caught). As a result, the new criteria have no slice assignment within the single lifecycle CODE PR and no stated regression-caught column. | spec/kani/matrix/TC-049-caller-death-ownership.md:74-106,947-970,902-945 | missing-requirement |
| FND-002 | low | "The Linux little-endian aarch64/riscv64 native-policy implementations" is a definite reference to implementations that the base specification never defines; they exist only in pending IR-639 code (backend_policy.rs cfg arms). FR-034 elsewhere names only native x86_64 policy and "other audited native policies". | spec/kani/functional/FR-034-caller-death-ownership.md:1220-1222 | wrong-requirement |
| FND-003 | low | Line 1198 says "The guardian cannot determine this proof's availability". However, `caller_run_buffers` is derived and charged by trusted caller C (lines 1158-1160), not by the guardian. Naming the wrong role blurs which process the gate concerns. | spec/kani/functional/FR-034-caller-death-ownership.md:1198 | wrong-requirement |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | TC-049 now has AC-41..50 rows in both the slice-1/slice-2 table (lines 107-116) and Expected Results (lines 990-999). |
| FND-002 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1218-1221 now say "Other audited native policies permitted by the existing contract remain candidates" and add no implementation claim. |
| FND-003 | fixed in commit "Clarify supported-build native accounting proof and charge ownership" | FR-034:1196-1197 now say "Neither C nor O determines proof availability at runtime". |

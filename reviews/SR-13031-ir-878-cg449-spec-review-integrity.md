---
id: SR-13031
title: "IR-878 CG PR 449 integrity review: matrix row, index and coverage delta"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@f69874b0a8d63bf6f2384656961ee15c105de642 vs origin/main 4fb3402; spec/oracle/matrix/tests.md, spec/tests.md, spec/spec.md, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md"
review_set: subset
---

## Summary

Ticket: IR-878. This review checked the matrix structure, the cross-references and the coverage delta. Both `quire validate --scope .` over the two changed files and `make spec` exit 0, with no warnings on FR-042. The `depends_on` edge `ix://agent-ix/quire-exact/FR-369` resolves.

`quire coverage --scope . --strict` reports 226 unbacked rows and 0 contradicted statuses on origin/main, and 227 unbacked rows and 0 contradicted at HEAD. The only additions are the FR-042-AC-1 verification row (FR-042 line 31), which is unbacked as expected, and its obligation. Both runs exit 1 because unbacked rows were already present.

The new tests.md row at line 89 sits after a prose paragraph, outside the Functional Requirement Coverage table, so coverage never reads it. As a probe, I placed the same row inside the table in the throwaway worktree, then reverted it. It was then read as an `unbacked-row FR-042 functional-coverage`, and the `🚧 PLANNED:` wording raised no `undeclared-status`, so the vocabulary accepts that wording.

## Examined criteria

- `spec/oracle/matrix/tests.md:89`: `| FR-042 | FR-042-AC-1 | — | 🚧 PLANNED: generated-oracle producer test; no TC or trace tag is credited by this spec allocation |`
- `spec/tests.md` Oracle index row: `| Oracle | FR-014, FR-018, FR-021, FR-031 | [oracle](./oracle/matrix/tests.md) | ... |`
- FR-042 frontmatter relationship and Dependencies links (clean).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-042 matrix row comes after the prose paragraph that follows the Functional Requirement Coverage table, so it is a stray pipe line, not a table row. `quire coverage` does not read it at HEAD (no FR-042 functional-coverage row is emitted), and the PLANNED status is never vocabulary-checked. Moved inside the table, it is read and accepted. The same defect appears in RT #132 (SR-13021 FND-001) | spec/oracle/matrix/tests.md:89 |
| FND-002 | low | The spec/tests.md index row for Oracle still lists "FR-014, FR-018, FR-021, FR-031", and spec/spec.md's capability table has no FR-042 entry, so the new requirement cannot be found from the index. FR-035 is also missing from that index row, but that gap predates this PR | spec/tests.md:16 |

## Verdict

**FAIL (changes requested)**: move the FR-042 row into the Functional Requirement Coverage table, after the FR-035 row at line 60 with no blank line in between. Add FR-042 to the spec/tests.md Oracle row. Then re-run `quire coverage --strict` to confirm the oracle matrix emits the FR-042 functional-coverage row.

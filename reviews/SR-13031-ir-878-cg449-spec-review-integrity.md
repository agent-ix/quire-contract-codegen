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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new matrix row reads "generated-oracle producer and QSL consumer tests follow the IR-497 typed-payload cutover" under IR-709, a CG-scoped ticket. That credits QSL consumer tests to the CG row, contradicting FR-042 Behavior ("its preservation and matching work is tracked at that owner") and TC-057 step 3 ("The QSL replay consumer is verified at its owner") | spec/oracle/matrix/tests.md:61 |
| FND-004 | low | The new TC-057 has no row in the oracle Test Case Summary table, while every other oracle TC (TC-024, TC-029, TC-031, TC-044, TC-050) has one with its Traces To criteria. So `quire coverage` emits no traces-to record for TC-057 | spec/oracle/matrix/tests.md:90-103 |

## Dispositions

Disposition pass 1, against head 5d0c8f3c2f5731a7044e9ed4d30fef294683616f. `quire coverage --scope . --strict` goes from 226 unbacked and 0 contradicted on origin/main to 228 unbacked and 0 contradicted. The additions are the FR-042 functional-coverage row (tests.md:61, targets FR-042 and TC-057), the FR-042-AC-1 verification row and its obligation. There is no new undeclared-status, and nothing is credited as backed. `make spec` is clean.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-042 row is now inside the Functional Requirement Coverage table at line 61, right after FR-035, and coverage reads it as `unbacked-row FR-042 functional-coverage` |
| FND-002 | fixed | The spec/tests.md Oracle row now lists FR-014, FR-018, FR-021, FR-031, FR-035, FR-042, and the spec.md Complete-V1 oracles row names FR-042 |

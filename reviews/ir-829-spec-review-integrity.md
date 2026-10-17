---
id: SR-11968
title: "IR-829 integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@ae138524cf6c4d0eeeb78589999f47d109f4acb3; spec/core/matrix/TC-007-cross-backend-parity.md, spec/core/matrix/TC-002-compile-atomic.md, spec/core/matrix/tests.md, spec/core/functional/FR-005-cli-conformance.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-007
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
---

## Summary

Ticket: IR-829. Checked the new TC-007 Differential Disposition Register, the TC-002 procedure edit and the matrix rows for consistency with FR-005 and with the test tree.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| TC-007 Differential Disposition Register | examined | No row is recorded at this revision because no TC-007 corpus run has been executed; the first run's differences are the first rows. |
| TC-002 Test Procedure | examined | Reject each noncanonical artifact path form (absolute, `..`, `.` segment, empty segment, trailing separator, backslash) and interior-dot aliases at bundle construction, ... with `UnsafeArtifactPath` and no destination I/O. |
| TM-001 FR-005 rows | examined | FR-005-AC-3 TC-007 Planned; FR-005-AC-6 through FR-005-AC-9 TC-002 Covered |
| TM-001 Test Case Summary TC-002, TC-007 | examined | TC-002 traces FR-005-AC-1, AC-5..9, NFR-001-AC-2/3; TC-007 traces FR-005-AC-3, AC-4 |
| TM-001 FR-039 rows | examined | FR-039-AC-5 split into its own Planned row |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The register justifies its empty table with "no TC-007 corpus run has been executed". The test tree holds four TC-007-tagged tests in `tests/it/kani_generation.rs`, including `generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus` and `generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus`, which run corpus slices across backends and assert agreement. The stated reason is false; the true one is narrower (no full four-backend differential run produces a differing-case list, and no test joins differences against the register). | spec/core/matrix/TC-007-cross-backend-parity.md:29-30, tests/it/kani_generation.rs:523-527, tests/it/kani_generation.rs:864-867, tests/it/kani_generation.rs:1551-1553, tests/it/kani_generation.rs:1625-1628 |

## Verdict

CHANGES REQUESTED (one medium). Consistent and recorded clean: TC-007's register text, FR-005-AC-3 and the FR-005 Behavior bullet state the same three failure forms; the matrix keeps AC-3 Planned with an honest note; the TC-002 and TC-007 summary rows list the moved and appended criteria; the TC-002 procedure's path forms match AC-6 and the test. The `✅ Covered` row for AC-6..9 beside TC-002's `🚧 Planned` summary status is governed by the matrix's own precedence note and is not a contradiction (but see SR-11966 FND-001 on AC-8's coverage). The FR-039-AC-5 split is accurate: `quire matrix` reports it untagged.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The register now says TC-007-tagged tests run corpus slices and assert agreement, but no full differential run produces a list of differing cases and no check joins that list against the register. |

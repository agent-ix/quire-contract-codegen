---
id: SR-002
title: Shared assurance migration gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#13; FR-006 acceptance criteria against the tests that claim them, and the TM-001 rows this change moves"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---

# SR-002: Shared assurance migration gap analysis

## Summary

The question is whether every FR-006 acceptance criterion has a test that would fail if the criterion
stopped holding, and whether every TM-001 row this change promotes is backed by a real tracking tag
rather than by a claim.

## Criterion coverage

| Criterion | Test | Would it fail if the criterion stopped holding? |
| --- | --- | --- |
| FR-006-AC-1 | `tc_008_every_shared_pin_is_classified_by_the_packaged_matrix` | Yes, and both scans are separately probed by injection: a pins document carrying a mirror reference, and a matrix naming a version that really is written down in this tree. A scan that cannot find a version that is present is a scan that would not have found the one that was. |
| FR-006-AC-2 | `tc_009_*` (four tests) | Yes. The chain must match every scenario, control and probe; every declared argv must appear in `make -n assurance-inputs`; the producers are probed in their failure direction; and the execution boundary is asserted with three PATH runs, one of which is a control that requires the chain to *fail*. |
| FR-006-AC-3 | `tc_010_the_sealed_records_impact_snapshot_is_the_quire_export` | Yes. The digest is recomputed independently, the export must name every requirement, and `status_lies` must be empty. An empty object has a digest too, so the digest alone is not accepted. |
| FR-006-AC-4 | `tc_011_retained_evidence_is_read_through_the_shared_mapping_without_moving_a_byte` | Yes, and the six mutation probes each remove one load-bearing check and require the census to notice. |
| FR-006-AC-5 | `tc_012_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls` | Yes. Only cases that ran *and* matched count; a scenario demonstrating no outcome carries a null rather than borrowing a label. |
| FR-006-AC-6 | `tc_013_no_local_evidence_framework_remains_and_the_frozen_schemas_bind_nothing` | Yes, and the `ci` graph is asked of Make rather than read out of the file, which is what makes deleting a prerequisite visible. |

## Rows this change promotes, and rows it does not

TM-001 gains six rows, TC-008 through TC-013, all `✅ Covered`. They are backed by
`tests/shared_assurance.rs`, whose `/// Trace:` comments are what Quire's census reads. The Quire
export reports `status_lies: 0`, and `tc_010` gates on that field, so a row claiming a status its own
bindings contradict fails a test rather than printing an advisory.

Every FR-001 through FR-005, NFR and StR row stays `🚧 Planned`. That is deliberate and it is the
answer to the obvious question about a migration that adds covered rows to a matrix of planned ones:
this change reviewed the shared-assurance intake path and nothing else, and promoting a semantic
generation row would be claiming a review that did not happen.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-201 | medium | FR-003 and FR-004 have no implementation, no suite, and no proof obligation. The gap is between the specification and the code, not between the code and its tests, and closing it is issues #2 and #5 rather than this migration | `spec/evidence/suites.md`, `assurance/change-assurance.json` | correct-requirement-no-evidence |
| FND-202 | medium | The conformance corpus produces four of the nine declared `GenerationErrorCode` values and asserts the declared terminal state of the other five without producing them. The census row's floor is set at the count the corpus actually reaches, so a reduction is caught; widening it is corpus work | `examples/generation_conformance.rs` | correct-requirement-no-evidence |
| FND-203 | medium | `quire coverage --strict` returns 0 while reporting contradicted statuses, so the engine does not gate what `tc_010` now gates locally through the export's `status_lies` field | `spec/test-matrix.md`, `agent-ix/quire-contract-ir#21` | wrong-requirement |
| FND-204 | low | No ix-flow decision event exists, so the verification receipt reads `incomplete` with `decision_missing`. That is the correct answer: only the repository owner can create one | `assurance/change-assurance.json` | wrong-requirement |
| FND-205 | low | 21 coverage rows are unbacked. Every one is an FR-001 through FR-005, NFR or StR row that this change deliberately leaves `🚧 Planned`, so the number is a census of what has not been reviewed rather than a defect introduced here | `spec/test-matrix.md` | wrong-requirement |

## Gaps

| # | Gap | Disposition |
| --- | --- | --- |
| GAP-001 | FR-003 and FR-004 have no implementation, no suite, and no proof obligation. The gap is between the specification and the code, not between the code and its tests. | Recorded in `spec/evidence/suites.md`, `assurance/pins.json` and `UNKNOWN-kani-and-vacuity-are-specified-not-implemented`. Not closed here; issues #2 and #5 own it. |
| GAP-002 | The conformance corpus covers four of the nine declared `GenerationErrorCode` values by producing them, and the remaining five only by asserting their declared terminal state. `NameCollision`, `UnsupportedDependency`, `UnsupportedObligations`, `ResourceLimitExceeded`, `InvalidGeneratedSyntax` and `SerializationFailed` are produced by the integration tests, not by the producer. | Accepted. The census row's floor is set at the count the corpus actually reaches, so a reduction is caught; widening it is corpus work, not migration work. |
| GAP-003 | `quire coverage --strict` returns 0 while reporting contradicted statuses, so the engine does not gate what `tc_010` now gates locally. | Deferred upstream; recorded as `UNKNOWN-coverage-status-column-unchecked`. |
| GAP-004 | No ix-flow decision event exists, so the verification receipt reads `incomplete`. | Correct and expected. Only the repository owner can create one. |

## Verdict

**CONDITIONAL.** Carried to SR-004 after the independent adversarial review.

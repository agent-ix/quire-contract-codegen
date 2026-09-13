---
id: Task-009
title: "Bound strategy admission, runner, and consumer bundle"
type: Task
status: done
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/Task-004
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/Task-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-012
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-013
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-017
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-020
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-021
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-022
    type: verifies
---
# Task-009: Bound strategy admission, runner, and consumer bundle

## Scope

With codegen #4's bounded-integer oracle grammar landed in PR #29, lower admitted `BoundPackage` clauses into
the strategy core, execute generated cases against the embedded oracle through runtime verdict
accounting, and emit the complete consumer bundle with its Quoin proof attestation.

## Subtasks

- [x] Rebase on the merged codegen #4 implementation and reconcile matrix additions as unions.
- [x] Implement FR-008 admission and ordered structured refusals.
- [x] Implement the FR-011 proptest and census runners, exact counters, rates, and conclusions.
- [x] Count all FR-012 shrink replays in `attempted`.
- [x] Implement the FR-013 case identity surface, bundle assembly, and proof attestation.
- [x] Run TC-017 through TC-022, Rust 1.75, Rust review, gap analysis, and the full local gate.

## Deliverables

- Public `generate_bound_strategy` operation and complete generated bundle.
- Requirement-tagged TC-017, TC-020, TC-021-AC-4, and TC-022 evidence.
- Fully backed FR-008 through FR-013 and NFR-004 matrix rows.

## Notes

- Task-004's codegen #4 implementation landed as PR #29 at `e0be330`; this branch consumes that
  merged revision and retains the matrix changes from both work streams.
- Closing Rust review SR-016 and gap analysis SR-017 found and repaired incomplete TC-017,
  TC-020, and TC-022 controls before approving all 34 slice criteria.

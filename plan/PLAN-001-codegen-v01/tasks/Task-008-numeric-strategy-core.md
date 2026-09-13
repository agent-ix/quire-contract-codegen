---
id: Task-008
title: "Constructive numeric strategy core"
type: Task
status: done
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-012
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-018
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-019
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-021
    type: verifies
---
# Task-008: Constructive numeric strategy core

## Scope

Implement the IR-independent relation model, exact satisfying/violating/broad population
construction, relation-preserving value trees, and deterministic boundary census that can be built
before the bounded-integer oracle grammar lands.

## Subtasks

- [x] Implement exact literal and correlated value sets using checked wide arithmetic.
- [x] Render constructive proptest strategies with no filter, reject, assume, or discard fallback.
- [x] Prevent `Broad` shrinking from crossing its initially drawn relation side.
- [x] Emit the in-domain, out-of-domain, and unrepresentable-edge census arrays.
- [x] Verify the six operators across small domains and `i64::MIN..=i64::MAX`.
- [x] Complete Rust review, gap analysis, pinned-toolchain gates, and matrix reconciliation.

## Deliverables

- `src/bound_strategy/{relation,population,census}.rs`.
- Requirement-tagged TC-018, TC-019, and the value-tree portion of TC-021.
- A review-ready partial implementation that does not claim the bound bundle or runner exists.

## Notes

- FR-012-AC-4 and NFR-004-AC-1 remain in Task-009 because they require the bound runner.
- The public relation constructors make the FR-008 primary-read invariant unrepresentable: a
  two-read relation always uses the authored left operand as primary.

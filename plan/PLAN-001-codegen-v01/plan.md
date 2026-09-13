---
id: PLAN-001
title: "Contract codegen v0.1 implementation and release preparation"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-012
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-013
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: references
---
# PLAN-001: Contract codegen v0.1 implementation and release preparation

## Scope

Specify, reconcile, implement, and verify deterministic code generation from the authoritative
contract IR into runtime-backed oracles, harnesses, proofs, vacuity maps, and Quoin proof attestations.

## Dependency Graph

```text
Task-001 -> Task-002 -> Task-003 -> Task-004 -> Task-005 -> Task-006 -> Task-007
                         ^
              shared ProofAttestationV1 + accepted IR/runtime revisions

Task-008 -------------------------------> Task-009
                                             ^
Task-004 bounded-integer oracle admission ---+
```

## Task File Mapping

| Task | Scope | Status |
|---|---|---|
| [Task-001](./tasks/Task-001-foundation-spec.md) | Foundation specification and assurance | done |
| [Task-002](./tasks/Task-002-foundation-evidence.md) | Foundation evidence and gap review | done |
| [Task-003](./tasks/Task-003-dependency-reconciliation.md) | Upstream dependency reconciliation | done |
| [Task-004](./tasks/Task-004-oracles.md) | Deterministic oracles and attestations | in_progress |
| [Task-005](./tasks/Task-005-backends.md) | Harness, proptest, Kani, and vacuity backends | in_progress |
| [Task-006](./tasks/Task-006-parity.md) | CLI, golden, differential, and parity closure | in_progress |
| [Task-007](./tasks/Task-007-human-release.md) | Human source-release decision | not_started |
| [Task-008](./tasks/Task-008-numeric-strategy-core.md) | Constructive populations, shrinking, and boundary census | done |
| [Task-009](./tasks/Task-009-bound-strategy-integration.md) | Bound admission, runner, consumer bundle, and attestation | blocked |

## Numeric/state strategy plan delta

### Requirements summary

- [ ] **FR-008**: Admit one bound integer relation through the numeric oracle grammar.
- [ ] **FR-009**: Construct satisfying, violating, and broad populations without filtering.
- [ ] **FR-010**: Emit deterministic in-domain, out-of-domain, and unrepresentable-edge censuses.
- [ ] **FR-011**: Run generated cases through the embedded oracle and runtime verdict accounting.
- [ ] **FR-012**: Keep every protocol-valid shrink candidate on its original side and count replays.
- [ ] **FR-013**: Emit the consumer case surface and Quoin proof attestation.
- [ ] **NFR-004**: Prove zero rejection overhead and the 20-case census ceiling.

### Dependency edges

- `FR-008 -> FR-009, FR-010`: package admission supplies the relation, shared domain, clause
  identity, and generated-oracle dependency identifiers.
- `FR-009 + FR-010 + FR-008 -> FR-011`: the runner consumes constructive cases, the finite census,
  and the embedded generated oracle.
- `FR-009 + FR-011 -> FR-012`: shrink accounting requires the bound runner; relation-preserving
  value trees can be verified independently first.
- `FR-008 + FR-009 + FR-010 + FR-011 -> FR-013`: the final bundle combines the case surface,
  populations, census, runner, and one proof attestation.
- `NFR-004` constrains the FR-009 through FR-012 generation and execution paths.

### Test plan

- **TC-017** verifies admission ordering, domain derivation, refusal identity, and refusal spans.
- **TC-018** verifies exact constructive populations, empty-side refusals, extreme domains, and
  deterministic rendering.
- **TC-019** verifies exact boundary censuses, unrepresentable edges, refusal behavior, and the
  20-case maximum.
- **TC-020** verifies oracle conformance, runtime verdict accounting, rates, and the census runner.
- **TC-021** verifies protocol-valid shrinking and runner replay accounting.
- **TC-022** verifies warning-free consumer compilation, package/read identity, regeneration, and
  sealing through the real Quoin CLI.

### Quality gates

- **Core gate (Task-008):** TC-018 and TC-019 pass on stable and Rust 1.75, TC-021's value-tree
  subset passes, the specification validates, and the full local repository gate has no
  change-caused failure.
- **Integration gate (Task-009):** codegen #4 is merged, TC-017 through TC-022 pass with all matrix
  rows backed, Rust review and gap analysis have no unresolved blocking finding, and `make ci`
  passes with the pinned toolchain.

## Coordination Rule

Task-003's dependency gate is complete. `main` now carries the shared-assurance migration, the
deterministic oracle slice, the issue #3 harness/proptest remediation (PR #22), atomic publication
(PR #26), the bounded vacuity observation primitives (PR #23) and the bounded Kani slice (PR #25),
pinned against IR `04eb6f849c03be23177d373549c6c272551f957d` and runtime
`8a4d02b9ff4633cf6d02fd8bdf6ee1b11ad76354`.

That IR revision binds executable expressions through the public API, so Task-006's
serialized-package generation is no longer blocked on the binding itself; what remains is the
serialized CLI surface and cross-backend parity. Task-004 and Task-005 remain in progress until
their semantic acceptance criteria and current-head review findings close. This integration
promotes no planned coverage or parity row. Automation must not complete Task-007.

The numeric/state strategy core in Task-008 may land independently. Task-009 must not copy or edit
the uncommitted codegen #4 worktree; it resumes only after Task-004's bounded-integer oracle grammar
lands on `main`. At that point its matrix conflicts are resolved as unions and the bound bundle is
built in the order FR-008, FR-011, FR-013.

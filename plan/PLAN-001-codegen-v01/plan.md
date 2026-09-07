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

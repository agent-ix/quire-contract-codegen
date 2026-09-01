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
contract IR into runtime-backed oracles, harnesses, proofs, vacuity maps, and derivation evidence.

## Dependency Graph

```text
Task-001 -> Task-002 -> Task-003 -> Task-004 -> Task-005 -> Task-006 -> Task-007
                         ^
              PGM-01 + IR #10 + runtime #5
```

## Task File Mapping

| Task | Scope | Status |
|---|---|---|
| [Task-001](./tasks/Task-001-foundation-spec.md) | Foundation specification and assurance | done |
| [Task-002](./tasks/Task-002-foundation-evidence.md) | Foundation evidence and gap review | done |
| [Task-003](./tasks/Task-003-dependency-reconciliation.md) | Upstream dependency reconciliation | done |
| [Task-004](./tasks/Task-004-oracles.md) | Deterministic oracles and manifests | in_progress |
| [Task-005](./tasks/Task-005-backends.md) | Harness, proptest, Kani, and vacuity backends | in_progress |
| [Task-006](./tasks/Task-006-parity.md) | CLI, golden, differential, and parity closure | not_started |
| [Task-007](./tasks/Task-007-human-release.md) | Human source-release decision | not_started |

## Coordination Rule

Task-004 implementation exists in stacked draft PR #10. Task-003's dependency gate is complete:
the branch is reconciled to accepted IR PR #19 merge `5c49ebfd1c87415f74420ad047392bd03b1bd202`.
PR #10 remains draft until Task-004's own acceptance criteria and review findings are closed.
Automation must not complete Task-007.

Kani issue #2 proceeds as a parallel draft from the exact PR #10 oracle head while harness issue #3
is reviewed separately. Neither backend draft promotes Task-004 or the semantic TestMatrix.

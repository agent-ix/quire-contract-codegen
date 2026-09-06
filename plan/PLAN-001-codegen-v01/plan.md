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
              shared ProofAttestationV1 + accepted IR/runtime revisions
```

## Task File Mapping

| Task | Scope | Status |
|---|---|---|
| [Task-001](./tasks/Task-001-foundation-spec.md) | Foundation specification and assurance | done |
| [Task-002](./tasks/Task-002-foundation-evidence.md) | Foundation evidence and gap review | done |
| [Task-003](./tasks/Task-003-dependency-reconciliation.md) | Upstream dependency reconciliation | done |
| [Task-004](./tasks/Task-004-oracles.md) | Deterministic oracles and manifests | in_progress |
| [Task-005](./tasks/Task-005-backends.md) | Harness, proptest, Kani, and vacuity backends | in_progress |
| [Task-006](./tasks/Task-006-parity.md) | CLI, golden, differential, and parity closure | in_progress |
| [Task-007](./tasks/Task-007-human-release.md) | Human source-release decision | not_started |

## Coordination Rule

Task-003's historical dependency gate is complete for IR PR #19 merge
`5c49ebfd1c87415f74420ad047392bd03b1bd202`. The local integration candidate combines reviewed
PR #22 head `fae8e4216216397ef6f5ec40a2ea3cb60ededcc2` and PR #26 head
`490fde7d11ae92637bc631c8f9206946dc376406`, preserving their separate source branches and shared
ProofAttestationV1 boundary. This is not a claim that either candidate is merged upstream.
Task-004 and Task-005 remain in progress until semantic criteria and current-head review findings
close. Task-006 includes the locally verified atomic publication slice; serialized-package
generation awaits the accepted IR-owned executable binding and explicit pin reconciliation.
Neither integration nor source generation promotes planned coverage/parity rows. Automation must
not complete Task-007.

---
id: Task-004
title: "Deterministic oracles and proof attestations"
type: Task
status: in_progress
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: references
---
# Task-004: Deterministic oracles and proof attestations

## Scope

Complete issue #4's numeric/state slice against the exact pinned IR revision: extend deterministic
per-clause Rust Boolean oracles from the merged Boolean grammar to obligation-free bounded-integer
comparisons over direct input and current/pre/post state observations. Preserve typed `bool`/`i64`
dependency signatures, source maps, and one Quoin ProofAttestationV1 body per output.

## Subtasks

- Specify and review the supported integer/state grammar and undefined-result refusal boundary.
- Add requirement-tagged differential, compilation, determinism and adverse-case tests before the
  implementation.
- Render integer literals, direct integer observations and all six comparison operators.
- Retain the exact rejected IR source span for every unsupported node or obligation.
- Keep arithmetic, negation, definedness obligations, indirect dependencies and object/graph reads
  fail-closed until their versioned semantics exist.
- Verify locally with the repository gates using the dedicated `target-codex-backends` Cargo target
  directory; do not dispatch hosted CI.

## Guard

Task-003 is complete. `main` pins IR revision
`04eb6f849c03be23177d373549c6c272551f957d` and runtime revision
`8a4d02b9ff4633cf6d02fd8bdf6ee1b11ad76354`. The undefined-result ruling permits only Boolean-root
comparison clauses with no definedness obligations in this slice. Task-004 remains in progress until
the numeric/state acceptance criteria, exact-head Rust review and gap analysis are closed.

---
id: Task-006
title: "CLI golden differential and parity closure"
type: Task
status: in_progress
track: C
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-001
    type: references
---
# Task-006: CLI, golden, differential, and parity closure

## Scope

Ship deterministic bundle publication and a library-first CLI, then close reproducibility,
golden-corpus, differential, and cross-backend parity evidence after Tasks 004 and 005 complete.

## Current State

<<<<<<< HEAD
The bounded generation-conformance example exists, but atomic publication, the CLI surface, golden
bundle publication, and full cross-backend parity closure have not started.
=======
The bounded publisher validates and path-sorts complete artifact sets, stages them beside the target,
and replaces only a directory whose contents match its writable marker. Fault injection covers every artifact
and marker staging point plus both swap boundaries. Modified, unmarked, extra-entry, and symlinked
destinations are refused without editing adjacent developer-owned files.

## Open Gates

- PR #26 remediation adds failed-rollback and ownership-I/O injections, explicit destination-state
  assertions, and interior-dot path refusal. The local marker proves content consistency rather
  than authenticated ownership. The publisher has not yet consumed actual generator bundles.

- Portable directory replacement is rollback-atomic for pre-commit failures, not process-crash atomic
  between the old-directory and staged-directory renames. A cleanup failure after commit reports that
  the new destination is published rather than claiming it stayed unchanged.
- File contents are synchronized, but directory entries are not; power-loss durability is outside
  the current guarantee even after a successful return. Ownership reads and traversal still need
  independently enforced disk-side resource bounds.
- The accepted serialized `ContractPackage<ReferenceBody>` carries clause metadata but does not bind
  executable `TypedExpression` values, and the IR wire decoder is private. A truthful
  serialized-package CLI requires an upstream IR design/API decision rather than a codegen-local wire
  format.
- Golden publication and complete executable/proptest/Kani/coverage parity remain to be reconciled.
>>>>>>> origin/main

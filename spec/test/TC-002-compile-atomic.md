---
id: TC-002
title: "Compile generated code and publish atomically"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: verifies
---
# TC-002: Compile generated code and publish atomically

## Description

Verify generated source compiles against only the runtime/customer types and publication is atomic.

## Test Procedure

Compile the supported Boolean grammar in an isolated `rustc` fixture against only the pinned runtime
and compare every truth assignment with an independent evaluator. Construct a validated artifact
bundle, inject a failure before every staged artifact and marker write and at both swap boundaries,
and compare the destination plus an adjacent developer-owned file before and after each run. Attempt
replacement after modifying, adding to, or symlinking the owned boundary and require refusal.
Inject a replacement failure followed by failed rollback; inspect the complete backup and staged
bundles and require `unknown`. Inject ownership-inspection/read failures and require `io_failed`
with `unchanged`, distinct from a missing marker/artifact. Reject interior-dot artifact aliases
before publication, including a bundle containing both `a/b` and `a/./b`.

## Expected Results

Valid Boolean oracles compile without generator/IR dependencies and match the independent evaluator.
Every injected pre-commit failure whose rollback succeeds restores the prior generated boundary,
reports `unchanged`, leaves no staging residue, and does not modify the adjacent developer-owned
file. Failed rollback reports `unknown`, leaves the destination absent, and preserves the complete
prior bundle in its backup sibling and the complete staged replacement for recovery.
A post-commit cleanup failure reports the
destination as published, leaves a complete new bundle, and exposes backup residue rather than
claiming rollback. Unmarked, modified, extra-entry, and symlinked destinations are never replaced.
Portable replacement is not process-crash atomic between its two directory renames and does not
guarantee persistence after power loss. A matching local marker establishes consistency, not
authenticated authorship or a destination-specific ownership grant.

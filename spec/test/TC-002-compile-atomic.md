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

## Expected Results

Valid Boolean oracles compile without generator/IR dependencies and match the independent evaluator.
Every injected pre-commit failure restores the prior generated boundary, leaves no staging residue,
and does not modify the adjacent developer-owned file. A post-commit cleanup failure reports the
destination as published, leaves a complete new bundle, and exposes backup residue rather than
claiming rollback. Unmarked, modified, extra-entry, and symlinked destinations are never replaced.
Portable replacement is not process-crash atomic between its two directory renames.

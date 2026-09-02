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
and compare every truth assignment with an independent evaluator. When atomic publication is added,
inject failures at each staged write and compare destination and developer-owned tree digests before
and after each run.

## Expected Results

Valid Boolean oracles compile without generator/IR dependencies and match the independent evaluator.
The row remains planned until publication failures also leave no partial bundle and modify no
developer-owned file.

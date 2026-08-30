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

Compile representative bundles in isolated Cargo fixtures, inject failures at each staged write, and
compare destination and developer-owned tree digests before and after each run.

## Expected Results

Valid bundles compile without generator/IR dependencies; failures publish no partial bundle and modify
no developer-owned file.

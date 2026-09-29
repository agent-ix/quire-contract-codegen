---
id: TC-028
title: "Verify interface-001's declared API surface matches the generator"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: verifies
---
# TC-028: Verify interface-001's declared API surface matches the generator

## Description

Verify that `spec/interface/interface-001-codegen-api.md` describes the crate as it actually is:
its `## Features` table lists exactly the operations its contract declares, and the terminal-state
vocabulary it declares is exactly what a `GenerationDiagnostic` emits.

## Test Procedure

1. Parse the contract's `operations` entries and its `## Features` table, and compare the two
   sets.
2. Serialize every `GenerationTerminalState` in `GenerationTerminalState::ALL` to its serde wire
   form, check each equals the variant's `label()`, and compare the set with
   `diagnostics.terminal_states`.

## Expected Results

1. The Features table lists each contract operation exactly once and nothing else.
2. Each variant's wire form equals its `label()`, and the six wire forms are exactly
   `diagnostics.terminal_states`.

## Implementation

`tests/it/interface_001.rs`.

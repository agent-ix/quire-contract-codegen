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
every operation it declares implemented is exported under the name it gives, every operation it
declares `status: planned` is absent, and the terminal-state vocabulary it declares is complete.

## Test Procedure

Walk `src/lib.rs` and every `pub mod` it reaches and collect each public function under its
shortest public path. Compare that set with the contract's non-planned `operations` entries in
both directions, and confirm none of the `status: planned` entries is exposed. Match every
`GenerationTerminalState` variant against `diagnostics.terminal_states`.

## Expected Results

The public function set equals the declared non-planned operations exactly, so a new public
function with no contract entry, whether a root `pub fn`, a root re-export or a function in a
`pub mod`, and a declared entry the crate no longer exposes each fail; no planned operation is
exposed; and the six `GenerationTerminalState` variants are exactly
`diagnostics.terminal_states`. Each `match` used to enumerate a Rust enum is exhaustive, so an added
variant this test does not name fails the build rather than passing silently.

## Implementation

`tests/it/interface_001.rs`.

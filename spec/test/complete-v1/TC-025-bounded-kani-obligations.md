---
id: TC-025
title: "Verify separate bounded Kani obligations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-025: Verify separate bounded Kani obligations

## Description

Verify that complete-V1 contracts produce one pinned, bounded harness per
obligation kind and refuse non-finite obligations.

## Test Procedure

Generate harnesses for a contract with a precondition, postcondition,
invariant and frame condition over bounded scalar domains, and for a contract
with an unbounded domain, one with unsatisfiable bounds, and one over a
caller-declared oracle operation. Inspect harness identities, bounds, every pin
and assumptions, and run the pinned Kani backend on the bounded harnesses.

## Expected Results

Four distinct harnesses carry their bounds and backend pin; no assumption
excludes an undefined, refused or incomplete outcome; the unbounded, the
unsatisfiable and the caller-declared obligations are refused with no harness.

## Implementation

`tests/kani_obligations.rs`. The default lane negotiates every item before any
harness is exposed and checks separate harnesses, IR-derived bounds, pins,
assumptions and every refusal. `make kani` runs the ignored lane serially under
a host lock: it asserts the observed Kani, launcher, driver, CBMC and toolchain
pins, verifies the precondition, postcondition and invariant harnesses, falsifies
a seeded postcondition defect with a concrete counterexample, and refuses a
drifted driver digest before running.

## Blocked

- Frame harnesses and every V2 scalar harness: V2 scalar obligations carry only a
  caller-declared operation identity, so they are refused until
  agent-ix/quire-specification#76 lands.
- Model and graph bounds: refused as blocked until
  agent-ix/quire-spec-language#120 lands.

---
id: "SR-039"
title: "IR-21 spec review: FR-016 AC-9, FR-023, TC-034, interface-001, matrix, index"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@d2987efc51b1d34aabda0dbd909663e9b884feef; spec/functional/complete-v1/FR-016-witness-native-replay.md, spec/functional/complete-v1/FR-023-claimed-module-proof-gate.md, spec/test/complete-v1/TC-026-witness-native-replay.md, spec/test/complete-v1/TC-034-claimed-module-proof-gate.md, spec/interface/interface-001-codegen-api.md, spec/test-matrix.md, spec/index.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-023
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
---

# SR-039: IR-21 spec review

## Summary

Ticket: IR-21. PR: agent-ix/quire-contract-codegen#184, head `d2987ef`. This reviews the spec edits:
the new FR-016 Behavior paragraph and AC-9, the new FR-023 and TC-034, three new interface-001
operations, and the test-matrix and index rows. It covers EARS form, integrity, spec-to-code
agreement and cross-reference consistency. `make spec` reports no validation error in any of these
files. Its exit 2 comes from AP-001 and MP-001, which fail the same way on `origin/main`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-016's new Behavior paragraph makes "the generator" name the proved package, the selected function and "the proving run's limits" in the request. The code does not: `replay_falsification` takes a caller-built `ReplayRequestWire` (interface-001 says so), and the spine passes UNLIMITED limits and a hand-mirrored twin's package id. | spec/functional/complete-v1/FR-016-witness-native-replay.md:68-79, spec/interface/interface-001-codegen-api.md:173-176 |
| FND-002 | low | FR-016-AC-9 is compound. It holds three separately testable behaviours in one row: reproduced/violation, inconclusive with both verdicts named, and an unbound value refused before any call. | spec/functional/complete-v1/FR-016-witness-native-replay.md:93 |
| FND-003 | low | FR-016 Dependencies calls `qsl-replay` "the one QSL edge". Cargo.toml now also has `qsl-foundation` and `quire-exact` as dev-deps. Say that these are test-only edges, and why. | spec/functional/complete-v1/FR-016-witness-native-replay.md:97, Cargo.toml:40-44 |
| FND-004 | low | Editorial. TC-034 Status reads "Steps 1 is". The test-matrix TC-026 row says "AC-8 is backed" twice. The TC-034 row sits before TC-033. FR-023 Behavior prescribes the test's procedure ("The spine's test injects one inside each claimed module"), which belongs in TC-034. | spec/test/complete-v1/TC-034-claimed-module-proof-gate.md:41, spec/test-matrix.md:343, spec/test-matrix.md:348, spec/functional/complete-v1/FR-023-claimed-module-proof-gate.md:55-57 |

## Clean

- FR-023's description is an EARS event statement ("When a caller applies ... the code generator
  shall report"), and its unwanted-behaviour bullets use "If ... then ... shall". AC-1 to AC-4 are
  atomic and testable, and match `claimed_module_gate`.
- The interface-001 operations list and the declaration-order feature table both gained the same
  three entries, and TC-028 (`interface_001`) passes.
- The index paragraph and matrix rows for FR-023/TC-034 agree with each other.

## Verdict

One MEDIUM mismatch between spec and code (FND-001). Either narrow FR-016's paragraph to what the
adapter does, or move request construction into the generator. The rest is minor.

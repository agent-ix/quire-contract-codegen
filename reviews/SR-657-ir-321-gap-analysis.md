---
id: "SR-657"
title: "CG PR 213 gap analysis: ticket acceptance and Subsystem Registry module ownership"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@b17b2b171fc52500c03710239a7cba5abc0e2f1f; spec/spec.md, spec/tests.md, spec/*/matrix/tests.md, src/**"
relationships:
  - target: ix://agent-ix/quire-contract-ir/ADR-0056
    type: references
---

# SR-657: CG PR 213 gap analysis

## Summary

Ticket: IR-321. PR: agent-ix/quire-contract-codegen#213 at b17b2b1, base baab597.
Plan completion: not assessed.

I checked the ticket's acceptance list against the diff:

- the subsystems oracle, strategy, kani, routed, evidence and replay
- complete-v1/ and strategies/ folded in
- one matrix per subsystem plus a root tests.md
- a Subsystem Registry in the root spec.md
- git mv plus link fixes
- no relocation map and no id-set script
- `make spec` no worse than before

I also checked the registry's module ownership against the source.
`quire-contract-ir:ADR-0056` Decision rules 3 and 4 are the standard.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The registry gives `bound_coverage` to Strategy, but the module implements FR-004, an Evidence requirement. `src/lib.rs:47` says "Implements: FR-004 (complete domain observations ...)". `src/bound_coverage.rs:128` traces TC-006 and FR-004-AC-3/5/7/9. The module imports `vacuity::normalize_path`. ADR-0056 rule 3 assigns a module to the subsystem whose requirements it implements. Move `bound_coverage` to the Evidence row | spec/spec.md:87, spec/spec.md:92 |
| FND-002 | medium | The registry gives `generation` to Routed. `src/generation.rs` is the shared generation-result vocabulary "every oracle generator returns": `ClaimDisposition`, `ClaimMap`, `OracleGenerationError` and `UpstreamBlocker`. Its users are exact_scalar, composite_equality and exact_function (Oracle) and bound_strategy (Strategy). Neither capability nor routed_generation imports it. Per ADR-0056 rule 4 it belongs to Oracle, or to Core because two subsystems build on it. The ticket's "routed (capability + generation)" most likely means routed_generation, which the row already lists | spec/spec.md:90, src/generation.rs:1-8 |
| FND-003 | low | `definedness_arithmetic` sits in the Oracle row, but its two siblings `bounded_collections` and `finite_reference_graphs` sit in Kani. All three are "admission for generated backends" over `quire_contract_ir::kani` lowering (`lower_checked_arithmetic`, `lower_query`, `lower_reaches`) with `KaniProfile`/`DispatchIndex`, and the same interface-001 operation group. Splitting the family across two subsystems makes ownership unclear | spec/spec.md:88, src/definedness_arithmetic.rs:1-12 |
| FND-004 | low | `oracle`, the Boolean-clause lowering core, is listed under Oracle. It is imported by harness and strategy (Strategy), by kani, kani_obligations and kani_execution (Kani), and by the oracle generators. ADR-0056 rule 4 places a primitive that two or more subsystems build on in Core. Judgment call: either move it or say in the Role cell that Oracle owns the shared lowering core | spec/spec.md:86, spec/spec.md:88 |

## Verdict

Acceptance is met apart from the registry rows above:

- Seven subsystem directories exist: core, strategy, oracle, kani, routed, replay and evidence.
  Core goes beyond the ticket and is justified by ADR-0056 rule 4 (but see SR-658 FND-001 on
  FR-005).
- complete-v1/ and strategies/ are gone.
- Each subsystem has one `matrix/tests.md` (TM-001 kept by core, TM-002 to TM-007 new), and
  `spec/tests.md` is TM-008, typed TestMatrixIndex.
- `spec/spec.md` carries the registry with the ADR's exact columns. The registry names every
  `mod` in `src/lib.rs` plus `lib` exactly once (28 entries; diffed mechanically).
- The diff is 73 renames (59 spec, 14 review), 9 added files (8 matrices or index plus SR-645)
  and 1 deletion (`spec/test-matrix.md`, split up). No relocation map and no script were added.
- `make spec` and `quire coverage` are unchanged; see SR-658.

Placement of the requirements themselves checks out:

- Strategy: FR-002 (harness, strategy).
- Evidence: FR-004.
- Oracle: FR-014, FR-018, FR-021.
- Kani: FR-015, FR-017, FR-025, FR-028, FR-029, FR-030. FR-029 and FR-030 are the Kani run
  outcome maps, so the kani_execution side is right, although ADR-002 discusses them under the
  adapter.
- Routed: FR-019, FR-022, FR-026.
- Replay: FR-016 and FR-024 (kani_witness_join, spine_replay, frame_replay).

Every TC file sits beside the matrix that declares it.

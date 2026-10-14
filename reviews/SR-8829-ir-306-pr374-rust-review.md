---
id: SR-8829
title: "Rust review — IR-306 nested integer operands"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #374 Rust patch series on 9675c0d..7d6ca0cd596fab63e2bfddd666bc34615744fc78; nested scalar inference, Kani identity propagation, routed refusal mapping and Rust tests"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: references
---

## Summary

Read-only Rust review of PR #374 at exact head `ff62ddcab4fd1d2708172f910466ba4da4d2d50f`, including the nested interval inference implementation, Kani identity propagation, routed refusal mapping, and changed tests. The production inference path is explicitly bounded and the round-5 `resource_kind` rename is consistently reflected in its constructors and test patterns. Two test-quality findings remain: one new private unit test is untraced, and the AC42 source assertion can accept an unrelated `limit_kind` occurrence by proximity.

## Verdict

**CONDITIONAL** — the new unit test should carry its criterion trace, and the AC42 guard should tie each permitted occurrence to its actual enum variant or conversion arm.

## Examined scope

- `Cargo.toml`, `Cargo.lock`: direct `num-bigint` dependency and lockfile update.
- `src/oracle/scalar/mod.rs`: bounded recursive range inference, typed refusals, parameter-bound interaction, literal handling and private tests.
- `src/kani/generate/scalar.rs`: inferred range use in harness operands and persisted identity construction.
- `src/kani/identity.rs`: serialized inferred operand identity fields.
- `src/kani/generate/negotiate.rs`: routing of operand-range refusal.
- `src/routed/generate.rs`: routed identity test fixture update.
- `src/lib.rs`: public exports.
- `tests/it/exact_scalar_generation.rs`: AC42 source guard and scalar-generation coverage.
- `tests/it/routed_generation.rs`: traced QSL and routed refusal cases.

## Rust review

The interval walk bounds recursive application depth, node work, and endpoint bit width. Arithmetic uses `BigInt`; Kani conversion to `i64` remains checked. The refusal field rename from `limit_kind` to `resource_kind` is applied to constructors, unit assertions, and routed assertions. No production panic, unsafe, unchecked wire conversion, unbounded recursion, or incomplete public documentation was found in the reviewed patch.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The new private inference test has no `Trace:` tag. The matrix cannot bind its missing-reference, cycle, unsupported-operation and bad-literal assertions to FR-014-AC-46/TC-024; admitted packages reject malformed bodies, so routed tests cannot replace this direct seam evidence. | src/oracle/scalar/mod.rs:4394-4397 | correct-requirement-no-evidence |
| FND-002 | medium | The AC42 source guard permits `limit_kind` by proximity instead of checking that it is part of the permitted variant or conversion. Add a new refusal variant with a `limit_kind` field within twelve lines after `LoweringLimitUnrecognised`; `rposition` finds the permitted name and the guard passes despite the FR-014-AC-42 violation. | tests/it/exact_scalar_generation.rs:3426-3434 | correct-requirement-no-evidence |

## Coverage

Source and test changes were read directly; `git diff --check` returned no output. No builds, tests, Clippy or other gates were run under the dispatch instruction. Separate coder gate receipts were not independently rerun.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The direct inference unit test now carries `Trace: FR-014-AC-46, TC-024`. |
| FND-002 | fixed | The AC42 guard checks the exact typed refusal declaration/conversion and asserts the permitted total `limit_kind` occurrence count, so an additional refusal use fails the assertion. |
| FND-001 | fixed | Rebase confirmation at 1250c6e: the trace tag remains directly above the TC-024 nested inference test (`src/oracle/scalar/mod.rs:4394-4398`). |
| FND-002 | fixed | Rebase confirmation at 1250c6e: the compact typed-field/conversion assertions and exact occurrence counts remain in place (`tests/it/exact_scalar_generation.rs:3420-3478`). |

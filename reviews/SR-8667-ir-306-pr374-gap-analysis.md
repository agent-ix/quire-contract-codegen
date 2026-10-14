---
id: SR-8667
title: "Gap analysis — IR-306 nested integer operands"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #374 diff 9bcfe616f1e1a03ba61512734dd44e3368257288..4f90588264624847b36486248720818bcd22c226; disposition delta from e598a42aa35e48066dd1fb55a28e9fac0050e996; rebase confirmation 9675c0d..06c0d6d788447f522ce06495db08246b3b670828; AC42 assertion delta 06c0d6d788447f522ce06495db08246b3b670828..dbd9d7997a461b2bd27a4b1c282a5ef9aa725495; resource_kind rename delta dbd9d7997a461b2bd27a4b1c282a5ef9aa725495..ff62ddcab4fd1d2708172f910466ba4da4d2d50f; FR-014-AC-42/45/46, FR-015-AC-103/104, TC-024, TC-033"
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

Audited only the IR-306 source and test diff against FR-014-AC-45/46 and FR-015-AC-103/104, using the computed Test Matrix rows for those criteria. The supported QSL cases and resource-ceiling cases have traced behavioral evidence; nested malformed/unsupported/type-mismatch refusal behavior is only asserted at a private inference helper and lacks traced end-to-end evidence for no oracle/harness and sibling preservation.

## Verdict

**CONDITIONAL** — one acceptance-evidence gap remains in the nested refusal cases.

## Coverage

- Scope limited to PR #374's ten changed files and the four specified criteria plus TC-024/TC-033 steps 20a/20b/19; no repo-wide reverse-gap or stub baseline was produced.
- `quoin matrix --repo . --json` exited 0. Filtered rows: FR-014-AC-45, FR-014-AC-46, FR-015-AC-103 and FR-015-AC-104 are all statically tagged. The relevant binders include the routed support test and routed limit test. The private `tc_024_nested_inference_reports_missing_cycle_unsupported_and_bad_literals` unit test has no `Trace:` tag, so it is not a matrix binder.
- Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Nested inference refusal variants lack traced generation/routing evidence for no emitted artifact and sibling isolation. | src/oracle/scalar/mod.rs:4397; tests/it/routed_generation.rs:1501 |

## New findings (disposition pass 4)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The updated AC42 source assertion permits an operand-range `limit_kind` name, while FR-014-AC-42 still restricts that name to `LoweringLimitUnrecognised`. | tests/it/exact_scalar_generation.rs:3426-3434; spec/oracle/functional/FR-014-exact-scalar-oracles.md:462 |

### FND-002 detail

The delta allows a `limit_kind` occurrence when the nearest preceding token among several variants is within twelve lines. In particular, `OperandRangeLimitExceeded` now permits it. The current FR-014-AC-42 text says each file names `limit_kind` only as the field of its own `LoweringLimitUnrecognised` refusal and its conversion. The scalar source declares `OperandRangeLimitExceeded { limit_kind: OperandRangeLimitKind, ... }` (`src/oracle/scalar/mod.rs:501-509`), so the revised assertion accepts source that the criterion's final sentence does not. Update the criterion to allow the separate operand-range limit field, or keep the assertion aligned with the current criterion and resolve the source/spec mismatch.
## Dispositions

| ID | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | Routed integration assertions cover nested `OperandUnsupported` and `OperandTypeMismatch` as Unsupported with no harness, then compare the healthy sibling's record, harness, and claim against isolated generation. `InvalidBody` is intentionally tested at the inference/lowering seam because `CheckedPackageV2::read` does not admit malformed bodies: the pinned reader's `validate` calls `flat_wire::check` and `validate_graph`/`validate_body` before returning `Admitted` (quire-contract-ir 6e45fd5, `checked_package/v2/mod.rs:831-865,1823-1839`); its lowering record docs state an admitted package never yields `InvalidBody` (`checked_package/v2/lower.rs:118-127`). The changed test documents this boundary at `tests/it/routed_generation.rs:1095-1099`; the malformed reference assertion remains exercised at `src/oracle/scalar/mod.rs:4423-4435`. |
| FND-001 | fixed | Round 3 rebase confirmation: `git range-diff 9bcfe616f1e1a03ba61512734dd44e3368257288..4f90588264624847b36486248720818bcd22c226 9675c0d..06c0d6d788447f522ce06495db08246b3b670828` mapped all three commits 1:1 (`=`, no patch changes), so the round-2 routed refusal coverage and InvalidBody admission boundary are unchanged at the exact pushed head. |
| FND-002 | still-open | The test delta broadens what AC42 permits without updating the criterion's explicit only-`LoweringLimitUnrecognised` rule. Reconcile the criterion and assertion before treating this evidence as conformant. |
| FND-002 | fixed | The refusal field is now `resource_kind: OperandRangeLimitKind`; the AC42 test again permits `limit_kind` only near `LoweringLimitUnrecognised`/`LimitUnrecognised`. The routed depth, endpoint-bit and node-limit assertions use `resource_kind` too (`tests/it/routed_generation.rs:1664-1714`). |
| FND-001 | fixed | Rebase confirmation at 1250c6e: routed unsupported nested cases still assert no harness and preserve byte-identical healthy sibling output; `InvalidBody` remains deliberately covered at the inference seam because checked-package admission rejects malformed bodies (`tests/it/routed_generation.rs:1095-1099,1501-1599`; `src/oracle/scalar/mod.rs:4423-4435`). |
| FND-002 | fixed | Rebase confirmation at 1250c6e: the AC42 guard still limits each generator file to its typed lowering refusal/conversion occurrences; the unrelated fixture identity helpers do not change that guard (`tests/it/exact_scalar_generation.rs:3404-3478`). |

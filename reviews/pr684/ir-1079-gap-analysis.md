---
id: SR-2026103044
title: "Gap analysis of IR-1079 source-bound claim identity"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1079; spec/core/functional/FR-039-contract-package-consumption.md, changed src/ and tests/; FR-039-AC-1/2/9/10/11"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: references
---

## Summary

Compared the changed FR-039 claim-identity implementation and tests with the computed Quire matrix. AC-11, which specifies the V2 public identity for scalar, equality, and function claims, has no trace binder; this PR's new function assertion also lacks the AC-11 tag.

## Verdict

**FAIL** — FR-039-AC-11 is untagged in the computed matrix, so its Test verification is not established.

## Examined scope

- FR-039-AC-1, AC-2, AC-9, AC-10, and AC-11: examined; AC-3, AC-4, and AC-5: context only.
- Changed source and tests across core, Kani, oracle, replay, and routed code: examined for reverse ownership, stubs, and trace bindings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-039-AC-11 has no test binder for the new V2 public claim identity contract | spec/core/functional/FR-039-contract-package-consumption.md:240 |

## Coverage

Plan completion: not assessed

`quire matrix --scope . --format json` reports FR-039-AC-1, AC-2, AC-9, and AC-10 tagged, but AC-11 untagged with zero binders. The new `genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys` test is tagged AC-7 and AC-8 and asserts function claim identity without claiming AC-11. The inherited AC-4 untagged state is outside this PR's changed contract; its QSL dependency is recorded in the FR-039 spec. The three focused module failures are detailed in SR-2026103043 and prevent credit for their affected assertions.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At fecd5d6482ac19fba212dc701eb9751c4757ffb0, the computed matrix tags AC-11 to four tests covering scalar, function and equality claim/map identities; the focused module passes. |

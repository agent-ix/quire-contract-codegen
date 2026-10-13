---
id: SR-001
title: "IR-295 PR354 fixture-correction code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@7dea2fc08cd4d0a1b70fd181d733ca0c130293f1; src/routed/contract_package_tests.rs and existing source helpers src/oracle/function/mod.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Reviewed the single-file delta from `6ae3a61b1b89108c44ab85d8d6fe2925722804c` to `7dea2fc08cd4d0a1b70fd181d733ca0c130293f1`, including its fixture corrections and existing source helpers. The updated inputs now use actual source argument references and the declaration-bound addition body, but the delta introduces duplicate JSON reference/binding parsing helpers in the test module.

## Verdict

**FAIL** — a high-severity duplication finding remains in the changed test support code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | Test fixture reimplements checked reference and binding resolution beside existing production helpers. | src/routed/contract_package_tests.rs:98-165; src/oracle/function/mod.rs:1586-1610 |

The new `referenced_node_id` repeats `reference_node_id`'s JSON target digest/domain lookup, and `source_function_body` repeats `binding_value`'s body-member lookup. The production helpers live in `src/oracle/function/mod.rs`; both were searched before recording this finding. If the source-shape rules change, one implementation can be updated while the other silently drifts, leaving the test fixture to resolve references/bindings differently from production. Reuse a shared parser/helper rather than maintaining a second implementation.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| `src/routed/contract_package_tests.rs:98-118` | examined | New `referenced_node_id` extracts target digest/domain, searches the checked graph, and clones the matching id. |
| `src/routed/contract_package_tests.rs:140-166` | examined | New `source_function_body` scans the function's members for the `body` value and resolves its target. |
| `src/oracle/function/mod.rs:1586-1610` | context_only | Existing `binding_value` and `reference_node_id` perform the corresponding body-member and reference-target parsing. |
| `src/routed/contract_package_tests.rs:1095-1186, 1220-1270` | examined | The two adjusted fixtures now pass source call references and select the function declaration's bound body root. |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | bf8bd7c410b97b1b17f996acd2e0ec85227619ec |
| FND-004 | fixed | 1c6d371810c78b858986757e0012c9eda4588bd9 |

FND-004 disposition: the agreement module header no longer lists AC-7. The criterion remains attributed to its flat schedule test in `exact_function_generation.rs`.

## Rust Review

The production code change is a narrow mapping through the existing typed-family helper; the test changes introduce no unsafe code or production panic surface. FND-004 is a low-severity trace documentation issue. Formatting, lint, and focused test results were supplied by the coder and were not rerun independently; full CI and Kani were not run.

## Delta review — 55a3ebb4a363864dc233cffd12f9371325f6a362

Reviewed only the five-path delta from `bf8bd7c410b97b1b17f996acd2e0ec85227619ec` to `55a3ebb4a363864dc233cffd12f9371325f6a362`: `src/oracle/function/mod.rs`, `tests/exact_function_support/agreement_cases.rs`, `tests/exact_function_support/package.rs`, `tests/it/exact_function_agreement.rs`, and `tests/it/exact_function_generation.rs`. The typed Model/Relation and State/Temporal/Protocol refusal mapping follows the existing `unsupported_family` authority. The ordinary fixture corpus uses distinct synthetic application records; source-bound identity, declaration-body selection and ordered call arguments remain in the dedicated checked fixture. The AC-11 test remains explicitly classifier-only for mismatched Model/State request roots and gives no credit for source-bound Model/State application. Removing the old `MAX_CALL_DEPTH` case agrees with FR-021-AC-7, which now specifies the admitted 100K-node flat SSA schedule and does not require Contract Runtime depth behavior. FND-003 remains fixed at `bf8bd7c410b97b1b17f996acd2e0ec85227619ec`. The module-level header in `tests/it/exact_function_agreement.rs` still lists AC-7 after the runtime-depth test was removed, although the flat schedule test in `exact_function_generation.rs` still covers AC-7. This is FND-004, a low-severity documentation/trace attribution issue with no coverage regression. No tests, full gates or Kani were run independently.

### FND-004

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Agreement test module header still attributes AC-7 after its AC-7 case was removed. | tests/it/exact_function_agreement.rs:1 |

The module header lists AC-7, but this file no longer contains an AC-7 test after removal of the obsolete `MAX_CALL_DEPTH` case. AC-7 is still exercised by `tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack` in `tests/it/exact_function_generation.rs`; remove AC-7 from this file-level list to keep the trace description accurate.

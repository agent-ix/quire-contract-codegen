---
id: SR-8663
title: "IR-550 PR #372 diff-scoped gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@a877339ebc41b5e37c0c449d50fbefa285273fb4; PR #372 diff; FR-014-AC-44, FR-018-AC-32, FR-021-AC-33; changed oracle sources and integration tests"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-018", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

This is a diff-scoped reconciliation of the three criteria touched by IR-550 PR #372, using the computed Quire matrix and inspecting the changed tests/source. The implementation is honestly marked partial: the published fairness fixture is absent, and the attempted candidate was refused by the reader; the PR does not claim to close IR-550.

## Verdict

**FAIL** — FR-018-AC-32 remains untagged and incomplete, and the newly added fixture tests cannot run in the repository's default CI environment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | New QSpec-backed tests panic in default CI because `QSPEC_DIR` is not supplied | tests/it/exact_scalar_generation.rs:2380 |
| FND-002 | high | FR-018-AC-32 is still untagged; its new direct-case test lacks trace binding and required sibling/output assertions | FR-018-AC-32; tests/it/exact_scalar_generation.rs:2458 |
| FND-003 | medium | FR-021-AC-33's direct formula/fairness/case branch is not yet demonstrated by the new AC-33 test | tests/it/exact_function_generation.rs:1828-1870 |

## Coverage

- Reconciliation: `quire matrix --scope . --format json` (Quire 0.36.2, engine 0.50.2); the computed matrix was interpreted for the three changed criteria only, not as a repository-wide delta claim.
- Plan completion: not assessed
- Criteria in reviewed subset: tagged 2 · untagged 1 · tagged-by-ignored-test 0 · method-without-symbol 0
- Evidence: no run evidence read
- Untraced behaviors / stubs: no new unowned behavior or stubs found in the changed production code
- Semantic review: skipped (not requested)

FR-014-AC-44 has tagged direct temporal-formula and direct case scalar tests, but the fairness case and same-healthy-sibling byte comparison remain open. FR-018-AC-32 has no trace-bound test: its new equality case test checks the typed refusal only. FR-021-AC-33 is tagged by the declared-body-root test, which verifies the separate call identity and a generated healthy sibling; it does not directly request a formula, fairness, or case root as the call-node id. These gaps are consistent with the PR's explicit partial label and its recorded IR-774 fairness blocker; they do not support closing IR-550.

## Finding Detail

### FND-001 — high

The direct formula and case tests call `qspec_package()`, which panics at `std::env::var_os("QSPEC_DIR").expect(...)` when unset. The checked-in CI workflow runs `cargo test` without a QSpec checkout or `QSPEC_DIR`, so these tests are not runnable in the default gate.

### FND-002 — high

Quire's computed matrix marks FR-018-AC-32 `untagged`. The only new equality test for the criterion, `tc_029_ac32_direct_control_case_is_unsupported`, has no `Trace: FR-018-AC-32` comment or `#[trace]` attribute. It asserts a typed refusal and causative id but omits the requested claim id, no-generation result, and healthy-sibling byte identity required by the criterion.

### FND-003 — medium

The tagged `tc_031_ac33_call_claim_retains_identity_and_uses_declared_body_refusal` tests an IR-admitted call whose separately declared function body is temporal. It does not cover AC-33's distinct direct-call-node cases (formula, fairness, case). The test is useful evidence for the declared-body clause, while the remaining direct-root AC-33 cases must stay open as the PR itself states.

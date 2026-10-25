---
id: SR-35022
title: "gap-analysis of IR-349 CG scalar consumer (quire-contract-codegen PR 568)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@b4f398d7; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/matrix/TC-024-exact-scalar-oracles.md; spec/assurance/AD-001-codegen-architecture.md; src/core/profile.rs; src/oracle/scalar/render.rs; tests/exact_scalar_support/agreement.rs; tests/exact_scalar_support/boolean_agreement_cases.rs; tests/it/exact_scalar_generation.rs; tests/it/exact_scalar_agreement.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
---

# Gap analysis of the IR-349 CG scalar consumer (PR 568)

## Summary

Ticket: IR-349. Planless. Plan completion: not assessed. Frozen head b4f398d7 against origin/main dd59aa14.

Examined:

- FR-014-AC-8 (`runtime_boolean_outcome_has_the_kernel_type`, TC-024): still tagged, passes, and now calls `quire_contract_runtime::scalar`.
- FR-014-AC-35 (`tc_024_boolean_and_integer_equality_nodes_generate_confirmed_runtime_oracles`): still tagged, passes, and asserts the real emitted `scalar_owner::evaluate_boolean_short_circuit(` text.
- FR-014-AC-37 (`tc_024_v2_boolean_and_equality_oracles_agree_with_direct_runtime`, using `boolean_agreement_cases.rs` through the `rt_side` re-export): passes. It compiles and runs the generated crate against direct runtime calls.
- FR-014-AC-6 and FR-014-AC-9 (`tc_024_generated_scalar_crate_agrees_with_direct_runtime`): passes.

No spec, matrix or AC text is in the diff, so no criterion is narrowed and no `#[trace]` tag moved.

Repo-wide check for references to the 11 moved items: code, tests and emitted source are fully repointed (see SR-35021). Two spec-prose references were not repointed (below).

Out of diff, for the lead, not counted as findings of this PR: FR-022 (line 207) says each Kani harness embeds a `use quire_contract_runtime::exact as rt;` line. ADR-003 (line 28) says the oracle calls `quire_contract_runtime::exact`. Both were already stale on main: the generated scalar oracle aliases `quire_exact as rt`. Neither names a moved item.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-014 still names the connective as `exact::evaluate_boolean_short_circuit`, in the Description and in the normative Behavior bullet ("the generator shall ... generate an oracle that calls `exact::evaluate_boolean` or, where the right operand may stop, `exact::evaluate_boolean_short_circuit`"). After RT 95a27602 the function exists only at `quire_contract_runtime::scalar`, and the generator emits `scalar_owner::` for it (`use quire_contract_runtime::scalar as scalar_owner;`). In the same sentence, `exact::evaluate_boolean` refers to `quire_exact`, which never owned the short-circuit function. So the shall-statement names a path that no longer exists, and the brief requires spec references to the moved items to be repointed. Repoint both lines to the runtime `scalar` module. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:38-41, spec/oracle/functional/FR-014-exact-scalar-oracles.md:476-478 |
| FND-002 | low | The AD-001 CG to RT seam row still lists only "Generated calls into `quire_contract_runtime::exact`". Generated scalar oracles now also call `quire_contract_runtime::scalar` (the lazy Boolean connective). Part of the row was already stale: kernel scalar operations come from `quire_exact`. Add `scalar` to the seam. | spec/assurance/AD-001-codegen-architecture.md:214 |

## Verdict

AC coverage is preserved. Every examined AC keeps its tagged evidence and passes, and no criterion is narrowed. Two spec-prose references to the old `exact` path for the moved connective remain: one medium in FR-014's normative Behavior bullet, and one low in the AD-001 seam row. Both are Markdown-only fixes inside this PR.

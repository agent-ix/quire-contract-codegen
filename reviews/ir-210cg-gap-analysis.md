---
id: SR-12081
title: "IR-210 CG slice 1 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@52f9a65c2e9e31f8726096837ad4ff684b9c74cf; tests/exact_scalar_support/agreement_cases.rs, tests/it/bound_strategy_generation.rs, tests/it/capability_settlement.rs, tests/it/composite_equality_generation.rs, tests/it/exact_function_generation.rs, tests/it/exact_scalar_generation.rs, tests/it/harness_generation.rs, tests/it/oracle_generation.rs, tests/it/strategy_generation.rs, spec/**"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: references
---

## Summary

Ticket: IR-210 (CG slice 1, non-Kani), PR quire-contract-codegen#414. This is a planless gap analysis, so plan completion was not assessed. All measurements used quire 0.36.2 (engine 0.50.2). Base is origin/main 9446ca9, checked out in a detached scratch worktree. Head is 52f9a65.

`quire coverage --strict`:

- **Base:** 221 unbacked rows, 0 contradicted statuses, exit 1.
- **Head:** 221 unbacked rows, 0 contradicted statuses, exit 1.
- The `unbacked_rows` sets in the JSON are identical. The strict outputs differ only in three `oracle-resembles-implementation` suspicion line numbers (2566 to 2567 and so on), caused by the one-line insertion in `exact_scalar_generation.rs`.

`quire coverage --json` totals:

- `backed` went from 496 to 499 of 855 minted targets. The three targets that flipped from unbacked to backed are FR-002-AC-1, FR-002-AC-2 and FR-002-AC-5. The `spec/strategy/functional/FR-002-tristate-proptest.md` group went from 2/18 to 5/18. No target flipped from backed to unbacked.
- By target kind:
  - acceptance-criterion: 420 to 423 backed.
  - nfr-acceptance-criterion: 29/36 on both.
  - test-case: 40/49 on both.
  - stakeholder-validation-criterion: 0/4 on both, so no StR row was lost.
  - constraint: 7/8 on both.
  - suite: 0/6 on both.
- The `criteria` section is identical on base and head.
- `unmatched_tags` is 119 on both, and the sets are identical.

Per-criterion binder sets from `coverage_matrix`, compared by (path, symbol):

- Added: FR-002-AC-1 +1, AC-2 +1, AC-5 +1, AC-6 +2, FR-021-AC-33 +1, NFR-001-AC-1 +1, NFR-002-AC-3 +5.
- Removed: none.

The binding census went from 877 to 876 tagged, from 865 to 864 bound, and from 629 to 628 self-named. All three drops come from the renamed, deliberately untraced `integer_add_refuses_a_result_leaving_its_declared_domain`.

`quire trace --file` was run on all 9 files on base and head, comparing (kind, id, symbol):

- **Gained:** exactly the 15 added claims listed above. Every one has the `rust-trace-line` form.
- **Lost:** exactly one, TC-026 on the renamed function through `rust-test-name-id`. That is intended.
- **Form changes:** every other touched claim kept its id and symbol and changed form. The TC-00N, TC-029, TC-030 and TC-031 claims moved from `rust-test-name-id` or `rust-comment-id` to `rust-trace-line`. The bound_strategy_generation continuation-line ids (TC-044, FR-031-AC-10, FR-031-AC-18, FR-011-AC-5, FR-012-AC-4, FR-013-AC-1) moved from `rust-comment-id` to `rust-trace-line`.

`quire trace --id TC-026`:

- **Base:** 46 `rust-trace-line` claims and 2 `rust-test-name-id` claims.
- **Head:** 46 `rust-trace-line` claims and 1 `rust-test-name-id` claim.
- TC-026 stays backed, and none of its Trace-line claims changed.

Pre-existing test failures, checked against origin/main with the same `QSPEC_DIR=/home/peter/dev/worktrees/qspec-gates` (2fb2473) and one shared target dir, running `cargo test --locked --test it -- tc_031_ac33 tc_024_ac44 tc_031_ac32`:

- Base and head both pass 4 and fail 3: `tc_031_ac33_direct_formula_call_node_is_unsupported`, `tc_031_ac33_direct_fairness_call_node_is_unsupported` and `tc_024_ac44_direct_temporal_formula_is_unsupported`.
- The panic sites and messages are identical: `InvalidSemanticGraph` at `/semantic_graph/nodes/14/body` and `/nodes/15/body`, and "unmodified authoritative QSpec fixture must admit". This is QSpec fixture skew that predates this PR, not something it caused.
- `tc_031_ac32_direct_control_case_is_unsupported`, whose doc line this PR changed, passes on both.
- The coder's 32 lib failures (host user namespaces) were not re-run, because this PR does not touch `src/`.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-002-AC-1 | examined | Rejected inputs remain distinct from passing cases in generated reports. |
| FR-002-AC-2 | examined | Boundary strategies exercise immediately inside and outside supported constraints. |
| FR-002-AC-5 | examined | The generated campaign runner owns execution, explicit-discard accounting, and conclusion, returns a typed terminal result, and rejects campaigns below either configured coverage floor or above the explicit-discard ceiling; ... |
| FR-002-AC-6 | examined | Integer and generated Boolean harness campaign cases bind exact values to executable accepted/rejected expectations; enum cases execute an all-admitted membership expectation. |
| FR-021-AC-33 | examined | With an IR-admitted temporal/formula, temporal/fairness or expression/case node used directly as an item's call-node id, the claim-map entry retains that id and is Refused { refusal: ExactFunctionRefusal::Unsupported } ... |
| NFR-001-AC-1 | examined | Identical inputs and configuration produce byte-identical bundles. |
| NFR-002-AC-3 | examined | Unsupported and inconclusive states remain explicit rather than silently succeeding. |
| TC-026 | examined | Verify witness decoding and native replay |
| StR-* stakeholder validation criteria | examined | 4 minted rows, 0 backed on base and head |
| spec/strategy/functional/FR-002-tristate-proptest.md | examined | group 2/18 -> 5/18 backed |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

CLEAN. Every claimed number reproduces:

- strict 221/0 on both sides;
- backed 496 to 499;
- unmatched 119 to 119;
- TC-026 still backed, with all 46 Trace-line claims intact.

No id was dropped that a body asserts. The single removed claim is a false TC-026 name binding on a function documented as deliberately untraced. No criterion lost a binder, no stakeholder row moved, and the three failing `it` tests fail identically on origin/main. Mergeable on this method.

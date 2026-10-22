---
id: SR-15370
title: IR-17 main landing PR513 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-17; complete origin/main...HEAD
review_set: subset
---

## Summary

Ticket IR-17; PR513, frozen 5e60b358eef84d28fa1a1897d1a703633458e8a5, base origin/main 0d7813df955576edc867cfa90e2071bd56d844ff. Scope is the complete 88-file main landing, not PR511's increment. Reviewer-only: no product edits, commits, pushes, Cargo, compiler, test, native execution, Kani/backend, namespace creation, bwrap, unshare, probes or new gate attempt. No applicable AssuranceProfile exists under spec/. Historical review and six-PASS qualification cohorts apply only to their recorded old heads. They are context, not fresh qualification of this candidate. Current method computations and source evidence are retained beside this artifact; tracker text is data.

## Verdict

FAIL: three concrete native/gate incompatibilities, one normative metering/trace contradiction, and four documentation/custody findings. No full or conditional execution PASS is claimed. All findings need a verified fix or explicit existing-owner disposition before scoped landing; fixing source alone still leaves safe qualification and pending final qualification to the lead.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Quantity oracles and native helpers still name removed Runtime Quantity APIs; arithmetic/comparison do not consume QSV UnitQuantity or preserve the required paired IdentifiedUnit and typed QuantityRefusal. | src/oracle/scalar/mod.rs:4095-4133 |
| FND-002 | high | Both new native fixture matches omit AdmittedCompositeForm::Union, so the existing record/tuple agreement test fails Rust exhaustiveness checking before any native assertion runs. | tests/admitted_composite_support/agreement_cases.rs:83-103 |
| FND-003 | medium | The scalar corpus gate expects residue::compare_enum but the emitter writes enum_owner::compare_enum; its body.contains assertion necessarily fails for each enum comparator. | tests/it/exact_scalar_generation.rs:693-720 |
| FND-004 | high | The new connective implementation and trace-tagged retention control charge BooleanResultRetain, while current FR-031-AC-26 and TC-044 step 14 require no connective charge; reviewed original metering reconciliation was omitted from the landing. | src/oracle/boolean_v1.rs:900-905 |
| FND-005 | medium | FR-031 Outputs, prerequisite and OD-2 plus TC-044 still describe the current oracle as Runtime-owned or blocked by obsolete QSL divide APIs; the reviewed original ownership/compatibility corrections were not carried into this migration. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:249-303 |
| FND-006 | low | The new module documentation promises EnvironmentError::Admission, but emitted EnvironmentError has no Admission variant and wraps all QSV EnvironmentFailure values in Declaration. | src/oracle/equality/mod.rs:54-55 |
| FND-007 | low | Retained SR-11690 has no explicit FND-004 disposition even though the current independent literal Runtime/Exact assertions correct its reported regression weakness; append the verified fixed outcome without rewriting the original finding. | reviews/ir-17-pr397-code-review.md:105-117 |
| FND-008 | low | The moved enum vector still describes a vendored Example.Phase declaration although it now independently compiles the original local Phase source and consumes facade identities. | tests/composite_equality_support/cases/enum.rs:20-22 |

## Coverage

FND-001: Cargo.lock selects Runtime a60c508, whose src/exact/mod.rs exports Boolean connective, negotiation and function-expression APIs only; all quantity exports have moved. Generated scalar QuantityComparison/Conversion/Arithmetic still import Runtime as residue and call its deleted Quantity/QuantityUnit/QuantityOperation APIs. tests/exact_scalar_support/agreement.rs and agreement_cases.rs import the same deleted names. Merely renaming the module is insufficient: the locked QSV quantity.rs exposes UnitQuantity inputs and evaluate_quantity_unit -> (Outcome<kernel Quantity>, IdentifiedUnit), with QuantityRefusal::IllTyped/Identity. Current main's FR014 AC48/49/52 requires those carriers. Repair the genuine QSV/kernel integration and independent native fixtures without fabricated identities or weakening original vectors. This is existing scalar quantity functionality, separate from equality's required Unsupported AC18/38.

FND-002: tests/it/admitted_composite_equality.rs:92-100 unconditionally registers tests/admitted_composite_support/agreement_cases.rs in the generated integration crate for both direct/transitive sources. Its matches at 83 and 97 omit the public three-variant facade's Union. Rust checks all variants regardless of the fixture's runtime Record/Tuple choices. An explicit honest fixture-only Union refusal is sufficient if native Union remains out of scope; silently mapping Union to an existing shape is not. This finding is source-proven, not a compiler run.

FND-003: scalar/mod.rs:4093 emits enum_owner::compare_enum and finish() inserts the QSV enum alias. exact_scalar_generation.rs:696 expects residue::compare_enum, then checks the individual generated function body at 720. Fix the stale assertion to the public owner; do not alter the corpus to evade it.

FND-004: completed_boolean calls retain_boolean at boolean_v1.rs:900-905; outcome_short_connective, outcome_total_connective and BooleanNot invoke it, and the new retention test requires counts 1/2/3 plus denial. Current FR031 AC26 instead says none charging any point; TC044 step14 and OD6 repeat that contract. Reviewed original 9938 carried the correct completed-result retention criterion and explicit closed OD6; those owning text changes were lost when only the implementation was carried. Restore/reconcile the approved owning metering contract and its procedure, keeping child-stop and left-decided outcomes precise; do not remove correct retains merely to satisfy stale prose. correct_ac is null in the trace record because no current unchanged criterion authorizes this native connective charge; the existing AC26 needs the owning reconciliation, not a fabricated new ID.

FND-005: current FR031 still describes old QSL divide/QuotientRemainder incompatibility, Runtime-owned current carriers, and open OD2 despite current locked source and emitted quire_exact. The old SR11691 fixed compatibility result cannot be adopted while the corrected statements are missing. Restore only the true owning compatibility/Outputs/decision corrections from reviewed 9938, retaining PLANNED and held proof status. FND-006 is a local doc-only correction: emitted EnvironmentError::Declaration contains the entire typed EnvironmentFailure; there is no Admission variant. FND-007 is custody-only: the real literal two-package/two-repository test at scalar.rs:1719-1741 asserts production membership, separately resolves both locks and checks each build-lock identity; deleting Exact now fails the independent assertion. Preserve that substantive fix and append its explicit original SR11690/FND004 fixed disposition. FND-008 corrects obsolete fixture provenance prose only; the current source, member identities, values, bounds and assertions must stay unchanged.

The rest of the original migration was inspected across all changed source/test paths: direct kernel imports and closed-enum matches; QSV declaration/equality and enum index use; typed checked-invariant causes and first-stop propagation; quotient-only divide; independent native agreement and per-charge denials; public-only QSL nominal capability intake; complete enum membership; item/package/descriptor/index binding; bounds, charged traversal and cycle refusal; original-context recompile, lock/package/whole-IR authentication; complete ten-entry v3 snapshot identity and pre/post execution custody; immutable receipt; dev-only observer example over the canonical adapter with independent root/observer locks; --lib proof target preserves original options; routed test orchestration remains outside core/Kani test support. No owner kernel, facade, schema or compatibility implementation was copied. The new public context accessor has real native callers/tests. No unsafe, warning suppression or CI weakening was introduced.

The original fifteen authentic source programs and native vector/domain subtree and docs are byte-identical to 4179; all case files were inspected. Makefile, scripts, workflows, current-main frame/scoped-delete and lowerer behavior have zero incremental delta. Original protected cases and targets were never touched. Current frame Quantity -> NonRangeType and equality Quantity Unsupported AC18/38 remain; no UnitTable, positive equality Quantity or native Union proof is claimed. Source quantities in current main's scalar AC52 are a separate required existing owner integration, not permission to enable equality.

Qualification remains UNRUN on this head; execution was held during this source-only review. The latest user instruction now authorizes the existing namespace-based CG tests on Titan for the later final qualification after all findings are fixed and reviewed, under the canonical shared lock and 16G/swap0 limits. No fmt, lint, no-run compile, native agreement, real healthy/falsified/OracleStop/teardown backend execution or sealed paired receipt was run. Historical only-codegen/native/compiler logs and earlier six-control PASS are provenance-limited support, never fresh gate results. No final aggregate or namespace-backed production-runner result is available yet; the lifted namespace hold is authorization for later execution, not a passing result. No host/security/account change or synthetic substitute is proposed.

## Examined scope

```yaml
clean: false
scope:
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: '# This file is automatically @generated by Cargo.

    # It is not intended for manual editing.'
- id: Cargo.toml
  path: Cargo.toml
  role: examined
  excerpt: '[package]

    name = "quire-contract-codegen"'
- id: docs/qualification/ir17-host-cases.md
  path: docs/qualification/ir17-host-cases.md
  role: examined
  excerpt: '# IR17 portable host cases

    '
- id: docs/qualification/ir17-host-commands.json
  path: docs/qualification/ir17-host-commands.json
  role: examined
  excerpt: "{\n  \"status\": \"Host-only command inventory; execution results are\
    \ reported separately. Not an independent all-case QSL sweep.\","
- id: reviews/ir-17-gap-analysis.md
  path: reviews/ir-17-gap-analysis.md
  role: examined
  excerpt: '---

    id: SR-11691'
- id: reviews/ir-17-pr397-code-review.md
  path: reviews/ir-17-pr397-code-review.md
  role: examined
  excerpt: '---

    id: SR-11690'
- id: reviews/ir-17-union-pr510-code-review.md
  path: reviews/ir-17-union-pr510-code-review.md
  role: examined
  excerpt: '---

    id: SR-15353'
- id: reviews/ir-17-union-pr510-spec-review-base.md
  path: reviews/ir-17-union-pr510-spec-review-base.md
  role: examined
  excerpt: '---

    id: SR-15354'
- id: reviews/ir-17-union-pr510-spec-review-dependency.md
  path: reviews/ir-17-union-pr510-spec-review-dependency.md
  role: examined
  excerpt: '---

    id: SR-15359'
- id: reviews/ir-17-union-pr510-spec-review-ears-conformance.md
  path: reviews/ir-17-union-pr510-spec-review-ears-conformance.md
  role: examined
  excerpt: '---

    id: SR-15355'
- id: reviews/ir-17-union-pr510-spec-review-evidence.md
  path: reviews/ir-17-union-pr510-spec-review-evidence.md
  role: examined
  excerpt: '---

    id: SR-15357'
- id: reviews/ir-17-union-pr510-spec-review-failure-domain.md
  path: reviews/ir-17-union-pr510-spec-review-failure-domain.md
  role: examined
  excerpt: '---

    id: SR-15358'
- id: reviews/ir-17-union-pr510-spec-review-integrity.md
  path: reviews/ir-17-union-pr510-spec-review-integrity.md
  role: examined
  excerpt: '---

    id: SR-15356'
- id: reviews/ir-17-union-pr511-code-review.md
  path: reviews/ir-17-union-pr511-code-review.md
  role: examined
  excerpt: '---

    id: SR-15360'
- id: reviews/ir-17-union-pr511-gap-analysis.md
  path: reviews/ir-17-union-pr511-gap-analysis.md
  role: examined
  excerpt: '---

    id: SR-15361'
- id: spec/oracle/functional/FR-018-composite-equality-oracles.md
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '---

    id: FR-018'
- id: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '---

    id: TC-029'
- id: src/core/profile.rs
  path: src/core/profile.rs
  role: examined
  excerpt: '//! The version profile: spellings this build emits (AD-004 `core/profile`,
    L-11).

    //!'
- id: src/kani/abi.rs
  path: src/kani/abi.rs
  role: examined
  excerpt: '//! The Kani subject ABI vocabulary: binding roles, primitive types, integer
    bounds, the solver

    //! choice and the adapter option vector.'
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: examined
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim
    and rendering its Kani

    //! harness (FR-015).'
- id: src/kani/generate/spec.rs
  path: src/kani/generate/spec.rs
  role: examined
  excerpt: '//! Validated, private harness descriptions (FR-015).

    '
- id: src/kani/identity.rs
  path: src/kani/identity.rs
  role: examined
  excerpt: '//! The harness and identity record types of a generated Kani obligation.

    //!'
- id: src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  role: examined
  excerpt: '//! Execution of generated Kani obligation harnesses, singly or as a batch
    (FR-017).

    //!'
- id: src/kani/run/scalar.rs
  path: src/kani/run/scalar.rs
  role: examined
  excerpt: '//! Paired proof and native execution of one retained routed scalar artifact
    (FR-017).

    '
- id: src/kani/test_support.rs
  path: src/kani/test_support.rs
  role: examined
  excerpt: '//! Test helpers shared by the tests of more than one `kani/` file.

    '
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '//! Deterministic Rust, property-test, proof, and evidence generation
    from Quire contracts.

    '
- id: src/oracle/boolean_v1.rs
  path: src/oracle/boolean_v1.rs
  role: examined
  excerpt: '//! Deterministic, fail-closed lowering for Boolean clauses over Boolean
    and integer values.

    '
- id: src/oracle/claim.rs
  path: src/oracle/claim.rs
  role: examined
  excerpt: '//! The generation-result and claim vocabulary every oracle generator
    returns.

    //!'
- id: src/oracle/clause.rs
  path: src/oracle/clause.rs
  role: examined
  excerpt: '//! V2 state-clause condition oracles over admitted checked packages.

    '
- id: src/oracle/equality/admitted.rs
  path: src/oracle/equality/admitted.rs
  role: examined
  excerpt: '//! Consume the public replay facade''s checker-issued nominal identities.

    '
- id: src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: '//! Exact composite/structural equality oracle generation from one admitted

    //! ContractPackage V1 (FR-018, FR-039).'
- id: src/oracle/equality/resolution.rs
  path: src/oracle/equality/resolution.rs
  role: examined
  excerpt: '//! Bounded, iterative reconstruction of equality operand types.

    '
- id: src/oracle/equality/union_tests.rs
  path: src/oracle/equality/union_tests.rs
  role: examined
  excerpt: '//! Host controls for the actual generator''s admitted Union closure and
    seals.

    '
- id: src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  role: examined
  excerpt: '//! Exact complete-V1 function-application oracle generation from one
    admitted

    //! ContractPackage V1 (FR-021, FR-039).'
- id: src/oracle/operator_catalog.rs
  path: src/oracle/operator_catalog.rs
  role: examined
  excerpt: '//! Closed typed identities consumed by admitted V2 generators.

    //!'
- id: src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  role: examined
  excerpt: '//! Exact complete-V1 scalar oracle generation from CheckedPackage V2
    (FR-014).

    //!'
- id: src/replay/composite_builder.rs
  path: src/replay/composite_builder.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    // Copyright (C) 2026 Agent-IX'
- id: src/routed/contract_package_tests.rs
  path: src/routed/contract_package_tests.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    // Copyright (C) 2026 Agent-IX'
- id: src/routed/mod.rs
  path: src/routed/mod.rs
  role: examined
  excerpt: '//! The routed subsystem: capability settlement and generation for routed
    items (AD-004, FR-019,

    //! FR-022, FR-026).'
- id: tests/admitted_composite_support/agreement_cases.rs
  path: tests/admitted_composite_support/agreement_cases.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Generated composite oracle versus independent QSV, with genuine source identities.'
- id: tests/admitted_composite_support/fixture.rs
  path: tests/admitted_composite_support/fixture.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Authentic record and tuple sources compiled only through the public replay
    root.'
- id: tests/admitted_enum_support/agreement_cases.rs
  path: tests/admitted_enum_support/agreement_cases.rs
  role: examined
  excerpt: '//! Actual generated enum oracles versus direct QSV, both using real admitted
    identities.

    #[path = "fixture.rs"]'
- id: tests/admitted_enum_support/fixture.rs
  path: tests/admitted_enum_support/fixture.rs
  role: examined
  excerpt: '//! Authentic public-facade enum fixture, shared by generator and generated
    native tests.

    '
- id: tests/admitted_enum_support/native_values.rs
  path: tests/admitted_enum_support/native_values.rs
  role: examined
  excerpt: '//! Native agreement values from genuine owner declarations and retained
    source selection.

    use super::fixture::imported;'
- id: tests/admitted_integer_support/agreement_cases.rs
  path: tests/admitted_integer_support/agreement_cases.rs
  role: examined
  excerpt: '//! Proposed genuine-source regression. No source conversion node is claimed.

    mod generated {'
- id: tests/composite_equality_support/agreement.rs
  path: tests/composite_equality_support/agreement.rs
  role: examined
  excerpt: '//! Agreement support for composite equality (FR-018-AC-2, AC-8, AC-9,

    //! AC-11): direct Contract Runtime execution and generated oracles.'
- id: tests/composite_equality_support/agreement_cases.rs
  path: tests/composite_equality_support/agreement_cases.rs
  role: examined
  excerpt: '//! FR-018-AC-2, AC-8 (the `check_type` guard half), AC-9 and AC-11 (the

    //! complementary-outcomes half): generated composite-equality oracles agree'
- id: tests/composite_equality_support/authentic_native.rs
  path: tests/composite_equality_support/authentic_native.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Each legacy agreement vector retains its own genuine source/context and original
    scalar domains.'
- id: tests/composite_equality_support/cases/collection.rs
  path: tests/composite_equality_support/cases/collection.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv.rs
  path: tests/composite_equality_support/cases/conv.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv_charge.rs
  path: tests/composite_equality_support/cases/conv_charge.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv_dec_dec.rs
  path: tests/composite_equality_support/cases/conv_dec_dec.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv_dec_int.rs
  path: tests/composite_equality_support/cases/conv_dec_int.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv_dec_rat.rs
  path: tests/composite_equality_support/cases/conv_dec_rat.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv_rat_int.rs
  path: tests/composite_equality_support/cases/conv_rat_int.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/conv_rat_rat.rs
  path: tests/composite_equality_support/cases/conv_rat_rat.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/enum.rs
  path: tests/composite_equality_support/cases/enum.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/nested.rs
  path: tests/composite_equality_support/cases/nested.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/option.rs
  path: tests/composite_equality_support/cases/option.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/record.rs
  path: tests/composite_equality_support/cases/record.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/recursive.rs
  path: tests/composite_equality_support/cases/recursive.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/text.rs
  path: tests/composite_equality_support/cases/text.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/cases/tuple.rs
  path: tests/composite_equality_support/cases/tuple.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Original numeric domains and adverse vectors, with independently admitted
    source identities.'
- id: tests/composite_equality_support/codes.rs
  path: tests/composite_equality_support/codes.rs
  role: examined
  excerpt: '//! The composite/structural corpus''s fixture codes and readable node
    keys.

    //!'
- id: tests/exact_function_support/agreement_cases.rs
  path: tests/exact_function_support/agreement_cases.rs
  role: examined
  excerpt: '//! FR-021-AC-2, AC-4, AC-5, AC-9, AC-17: generated function-application

    //! oracles agree with an independently assembled direct Contract Runtime'
- id: tests/exact_scalar_support/agreement.rs
  path: tests/exact_scalar_support/agreement.rs
  role: examined
  excerpt: '//! Agreement support: direct Contract Runtime execution and generated

    //! oracles.'
- id: tests/exact_scalar_support/agreement_cases.rs
  path: tests/exact_scalar_support/agreement_cases.rs
  role: examined
  excerpt: '//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime

    //! execution agree on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193'
- id: tests/exact_scalar_support/boolean_agreement_cases.rs
  path: tests/exact_scalar_support/boolean_agreement_cases.rs
  role: examined
  excerpt: '//! FR-014-AC-37: admitted V2 Boolean and scalar equality oracles against
    direct Runtime calls.

    '
- id: tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: examined
  excerpt: '//! Admitted CheckedPackage V2 fixtures for exact scalar generation.

    //!'
- id: tests/it/admitted_composite_equality.rs
  path: tests/it/admitted_composite_equality.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    //! Authentic source-backed record/tuple generation and native owner agreement.'
- id: tests/it/admitted_enum_equality.rs
  path: tests/it/admitted_enum_equality.rs
  role: examined
  excerpt: '//! Native generated-ABI acceptance uses genuine source compilation, never
    synthetic keys.

    use ix_trace_rs::trace;'
- id: tests/it/capability_settlement.rs
  path: tests/it/capability_settlement.rs
  role: examined
  excerpt: '//! FR-019 capability settlement: one `negotiate_*` arm over a closed
    backend kind.

    //!'
- id: tests/it/composite_equality_agreement.rs
  path: tests/it/composite_equality_agreement.rs
  role: examined
  excerpt: '//! FR-018-AC-2, AC-8 (the `check_type` guard half), AC-9 and AC-11 (the

    //! complementary-outcomes half): generated composite-equality oracles agree'
- id: tests/it/composite_equality_generation.rs
  path: tests/it/composite_equality_generation.rs
  role: examined
  excerpt: '//! FR-018: composite/structural equality oracle generation over admitted

    //! CheckedPackage V2 input.'
- id: tests/it/composite_original_eq.rs
  path: tests/it/composite_original_eq.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    // Copyright (C) 2026 Agent-IX'
- id: tests/it/exact_function_agreement.rs
  path: tests/it/exact_function_agreement.rs
  role: examined
  excerpt: '//! FR-021-AC-2, AC-4, AC-5, AC-9, AC-17: generated function-application

    //! oracles agree with an independently assembled direct Contract Runtime'
- id: tests/it/exact_function_generation.rs
  path: tests/it/exact_function_generation.rs
  role: examined
  excerpt: '//! FR-021: function-application oracle generation over admitted CheckedPackage

    //! V2 input.'
- id: tests/it/exact_scalar_agreement.rs
  path: tests/it/exact_scalar_agreement.rs
  role: examined
  excerpt: '//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime

    //! execution agree on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193'
- id: tests/it/exact_scalar_generation.rs
  path: tests/it/exact_scalar_generation.rs
  role: examined
  excerpt: '//! FR-014: exact complete-V1 scalar oracle generation over admitted

    //! CheckedPackage V2 input.'
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: '//! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and
    Contract IR FR-036

    //! backend negotiation.'
- id: tests/it/kani_obligations_state_frame.rs
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: '//! FR-015 (IR-412): one `state_clause` postcondition over one integer
    field becomes an

    //! operation-contract Kani harness and a frame-effect Kani harness, both scoped
    to the clause''s'
- id: tests/it/kani_scalar_native_digest.rs
  path: tests/it/kani_scalar_native_digest.rs
  role: examined
  excerpt: '//! Independent JCS/SHA-256 oracle for the scalar proof-content string-only
    shape.

    pub(crate) fn independent_digest(entries: &[(String, Vec<u8>)]) -> [u8; 32] {'
- id: tests/it/kani_scalar_native_fixture.rs
  path: tests/it/kani_scalar_native_fixture.rs
  role: examined
  excerpt: '//! Shared admitted scalar routing fixture for the integration and private
    library controls.

    use super::{'
- id: tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  role: examined
  excerpt: '//! Private orchestration controls for scalar-native cleanup after real
    execution.

    #[path = "../exact_scalar_support/package.rs"]'
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs`
    directory as one

    //! test target automatically (no `[[test]]` entry needed), so every former top-level
    `tests/*.rs`'
- id: tests/it/oracle_arithmetic.rs
  path: tests/it/oracle_arithmetic.rs
  role: examined
  excerpt: '//! FR-031: the Boolean oracle''s integer arithmetic and comparison take
    the runtime''s meaning.

    //!'
- id: tests/it/routed_generation.rs
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: '//! FR-022 routed generation: `generate_routed` runs the routed kind''s
    generation arm keyed by the

    //! driver''s request index, and takes the routed backend and kind as given.'
- id: tests/it/scalar_playback.rs
  path: tests/it/scalar_playback.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later

    // Copyright (C) 2026 Agent-IX'
- id: FR-014-AC-1
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition,
    and a refused item contributes no generated function while its siblings are generated
    unchanged.
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison, quantity arithmetic/comparison and exact quantity
    conversion (FR-014-AC-48) descriptors for admitted checked-package expression
    nodes each generate a function calling the matching direct owner operator described
    above. Entry-point dispositions for a `quire.op.quantity.convert` node: derivation
    and generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and capabilit'
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: FR-014-AC-35
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer
    `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate
    an `ir_confirmed` oracle; each connective's oracle calls `evaluate_boolean` or
    `evaluate_boolean_short_circuit`, and each `eq`/`ne` oracle calls `check_equality`
    and `CheckedEquality::evaluate`, each returning `Outcome<bool>`.
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A differential corpus covering every node FR-014-AC-35 names compiles
    against the runtime alone, and each oracle''s outcome, admitted charges and consumed
    counters equal direct runtime execution. This criterion claims no agreement with
    the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle
    against QSL agreement), and FR-014-AC-47 keeps CG''s artifacts from claiming it.'
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed`
    oracle, described by the public operation `quantity_conversion` (`ExactScalarOperation::QuantityConversion`,
    a unit-free variant: the target unit is a call-time argument `unit: &IdentifiedUnit`,
    and the generator does not check it against the `type_argument`, which the node
    carries only as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with
    no checked bound, and the claim-map entry records the `type_argument` type node
    id (FR-01'
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When QSV refuses a `quire.op.quantity.convert` with `IllTyped` before
    any charge, the oracle stops as `ill_typed` with zero admitted charges and no
    change to any consumed counter, equal to direct `convert_quantity` execution (Contract
    Runtime FR-007-AC-21): `1 m` to `s` (U03-style `m` to `s`), U09 (a base dimension
    of the same declaration and shape as `L` under a different node key), `1 N_m`
    to `J` (U22) and compound `4 m^2` to `m`. An affine source or target converts
    and is not refused as `AffineUnitArithmetic`; only arithmetic on an affine unit
    is refused (FR-014-AC-6, U05).'
- id: FR-014-AC-52
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Every generated quantity operand is a QSV `UnitQuantity<''_>` resolved
    through the kernel quantity''s own id by `UnitTable::resolve` or `UnitScope::resolve`,
    with no CG-created unit ids or unrelated metadata pairing. Comparison calls QSV
    `compare_quantity` and preserves its typed `IllTyped`. Arithmetic calls QSV `evaluate_quantity_unit`
    and returns `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>` with the
    exact owner-provided identified result unit on every `Ok`, including stopped outcomes,
    and preserves both `QuantityRefusal::IllTyped` and `QuantityRefusal::Identity`
    as distinct typed '
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete
    emitted oracle package and one in-range concrete operand are passed to the paired
    runner. One real Kani run reports `Verified`, and the runner invokes the public
    oracle from the same staged library source at that operand with the request's
    exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed
    runtime result of that invocation. A Kani verdict alone or a result reconstructed
    by the caller is insufficient.
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The runner measures one canonical proof-content identity over the exact
    paths and bytes of every staged execution input: the complete generated package,
    retained harness, canonical native invocation adapter, observer manifest and both
    independently generated dependency locks. This identity binds the content actually
    proved and executed; it is not a tool-version or file-version tracking identity,
    and no separate identity is added for any staged item. The Kani and native invocations
    cannot write the staged source tree. A changed, replaced or additional staged
    file before or after either invocati'
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The public receipt exposes the measured identity, scalar identity, ordered
    operands, exact supplied and meter-used `ScalarLimits`, verified Kani evidence
    and actual typed native outcome through read-only access. Callers cannot construct,
    deserialize or mutate a receipt, set its identity or outcome, or turn any non-verified,
    refused, inconclusive, faulted or malformed run into a receipt.
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts,
    in UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted
    record `kani-obligations/<module>.json`, `observer/Cargo.lock`, `observer/Cargo.toml`,
    `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs` (harness), and
    `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts,
    harness and record compare byte-identically under the rename map. Removing, renaming,
    reordering, duplicating or changing one byte of any entry changes the digest.
    Non-UTF-8 runner-re'
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The receipt exposes read-only the ordered staged `(path, bytes)` entries
    the digest was built from, including both manifests, both independently generated
    locks, the source map and the canonical `src/bin/ir791_native.rs`. All ten entries
    remain measured, sealed and compared across both execution boundaries. Recomputing
    the digest from only those exposed entries equals the receipt's `DigestRecord`.
    A recomputation that substitutes one byte in the package, harness or harness record
    (which a driver holding different bytes would do) differs from the receipt's digest.
    The exposed entries are exactl
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged.
    The isolated private observer uses resolver 2 and its sole example compiles the
    single canonical adapter in place through `../src/bin/ir791_native.rs`, with runtime
    `exact` and kernel `test-support` exposed only by dev-dependencies; no wrapper,
    copied source, symlink, normal/build logging edge or CLI feature override is allowed.
    Native invocation explicitly names `observer/Cargo.toml` and the example; the
    real production proof retains its harness, ordered options, claims, covers, bounds
    and backend gates. A control re
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved with the existing
    bounded offline generation/update workflow, without copying a parent/other lock
    or substituting a cached resolution. Each resolves both owning runtime and exact
    identities to the build's authoritative lock identities; the independent observer
    regression asserts both literal package names and repository identities independently
    of the renderer's list. Both manifests, locks and all other AC-46 entries are
    sealed and compared as a complete path/byte set before proof, after proof and
    after native execution; mutation/replacemen
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus
    `CheckedEquality::evaluate` invoked directly on an independently constructed environment,
    operands and fresh `Meter`. This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authentic
    duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration
    and cause() equal to None for the Assembly refusal, produces no successful AdmittedContext
    and is not passed to public CG generation. Private CG declaration-admission defenses
    retain the exact DuplicateMember name/cause and unchanged healthy sibling separately,
    without public-acc
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for
    the TC-029 step 1 package (every generated item) contains zero occurrences of
    the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(` or `.unwrap_or_default(`
    anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`,
    and each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`,
    `rebuild_text` and `rebuild_cardinality` turns a failed constructor or parse into
    exactly the mat
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant {
    .. })`'
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds`
    follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element
    reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]` from that
    bound node's named `min` and `max`; changing the semantic type to a non-sequence,
    dropping or duplicating either bound, or making either value non-integer refuses
    only that item with a typed cause and emits no code, never silently yielding an
    unbounded collection. PLANNED (IR-643).
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original
    dependencies and admitted request limits has equal compiled/context/original package
    identities and exact full recompiled/original CheckedPackageV2 equality before
    its context is exposed. An explicitly supplied different context package refuses
    generation and emitted environment construction before classification or lookup,
    including primitive-only requests, publishes no prepared value, and cannot evaluate
    with a completed Boolean or meter charge.
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons
    over all declared cases, including a case absent from every selected literal and
    an enum parameter with no member literals. Exact shape variants and canonical
    positions come from genuine facade/QSV values. Same-spelled cases from distinct
    declarations retain distinct identities; matching and mismatching declaration
    comparisons preserve the owning equality disposition.
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Unknown declaration, unknown case and a directly or transitively imported
    declaration produce the original typed Unknown/Foreign details at consumed and
    generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader
    resource limits, and internal Fault remain Package, Resource and Fault at the
    source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit,
    with no partial context or successful prepared wrapper. The current ImportedContextUnsupported
    guard remains until original admitted dependency packages are retained.
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and
    contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only
    the prepared wrapper's private genuine index reaches check_equality. A private
    environment-only fault fixture still satisfies AC-8, every original charge-stop
    case still satisfies AC-9, and a wrong item/descriptor prepared wrapper refuses
    before charging. Record/tuple/recursive/collection cases remain required by AC-2
    and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved
    migration work, not as accepted Unsupporte
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Equality's generated manifest and native scratch consumers use the one
    profile authority's direct exact/QSV/replay dependencies, while ordinary Boolean/integer
    FR-014 scalar proof manifests and oracle bodies acquire no replay source-compilation/context
    leg. Native generated enum acceptance is reported separately from original TC025/TC027
    proof qualification; neither result substitutes for the other.
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nominal primitive and nested Option/Collection closures; their
    index is empty only after complete zero-enum resolution. Existing admitted-IR
    conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion")
    and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext,
    no symbols for the refused item and unchanged independent siblings. This does
    not claim matching QSL source emission or source authentication for those IR fixtures.
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext
    naming that declaration. Cover left and right source operands, left and right
    conversion targets from primitive sources, nested Option and Collection enum types,
    and eventual record/tuple field and recursive closures through their genuine owner
    capability. No key, default/partial index, omitted nominal declaration or Unsupported
    substitute is produced. Retain the existing charged-entry/cycle/error precedence
    and unaffected sibling results; the separate missing record/tuple owner lookup
    remains outstanding.'
- id: FR-018-AC-40
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Some(wrong-package context) fails the whole standalone call after existing
    setting/accounting checks and before item classification, for primitive-only,
    nominal and empty requests. Emitted context-free and nominal constructors both
    reject the same supplied mismatch before reconstruction. None and Some(wrong)
    have distinct outcomes; no failure publishes partial artifacts or a prepared wrapper.
- id: FR-018-AC-41
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A reached enum still reconstructs every owner-admitted case, including
    unused cases and an enum parameter with no member literal, using the supplied
    genuine keys and canonical positions. Failed nominal admission/lookup/traversal
    cannot yield an empty-index success. A successfully resolved zero-enum closure
    may use an empty index; merely inspecting an outer primitive/Option/Collection
    tag does not establish zero enums.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both check_type guards, same-index
    check_equality and evaluate. Private wrong-binding, present-context mismatch,
    missing nominal context and environment-only faults refuse without charge. The
    native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR
    comparison and cannot use None to bypass them. No scalar proof manifest/body acquires
    replay compilation.
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left
    parameter names its integer_range and the right parameter names canonical base
    Integer. Direct source/target reconstruction preserves Int[0,9] versus unbounded
    Integer despite the sibling range: a caller descriptor converting the left to
    Integer compares with the unbounded right under the original equality rule, including
    right values outside [0,9]; an unbounded Integer source cannot acquire a narrowing
    conversion by borrowing that sibling interval. Claim/native descriptor checks
    remain independent of generated output. If so'
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Genuine Union equality generation preserves the facade declaration key,
    settled node, authoritative member bindings and declared member/payload order
    across a nested Union closure, using the specified lookup and verified sealing
    in both generator and emitted preparation. In-process owner-API controls admit
    values of every member, including record and nested-Union payloads; distinct same-shaped
    unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind
    lookup, name/form disagreement and seal faults retain their typed causes, no successful
    prepared wrapper and no '
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native oracle holds no `quire_contract_runtime::operators`
    path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`;
    a short-circuit connective is an inline `match` over the left operand's `rt::Outcome`
    that returns the first stop and does not evaluate the right operand when the left
    decides, and a total connective evaluates both operands in order and returns the
    first stop, none charging any point and none calling `evaluate_boolean`; in a
    generated crate the outcomes of FR-031-AC
- id: FR-031
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: "oracles do (the `quire_exact` types are the planned ones, contingent as\
    \ \"The kernel the native\n  oracle calls\" states; the oracle IR-596 implemented\
    \ uses Contract Runtime's)."
- id: SR-11690/FND-004
  path: reviews/ir-17-pr397-code-review.md
  role: examined
  excerpt: The new lock-pin test reads the same `LOCKED_DEPENDENCIES` constant that
    `generate_lockfile` reads, then repeats the pin commands itself. Deleting `quire-exact`
    from that constant still passes the test, so FND-002 could regress without any
    test failing
findings:
- fnd: FND-001
  severity: high
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: FR-014-AC-52
  related_ids:
  - FR-014-AC-48
  - FR-014-AC-49
  - FR-014-AC-2
  path: src/oracle/scalar/mod.rs
  lines: 4095-4133
  excerpt: 'Every generated quantity operand is a QSV `UnitQuantity<''_>` resolved
    through the kernel quantity''s own id by `UnitTable::resolve` or `UnitScope::resolve`,
    with no CG-created unit ids or unrelated metadata pairing. Comparison calls QSV
    `compare_quantity` and preserves its typed `IllTyped`. Arithmetic calls QSV `evaluate_quantity_unit`
    and returns `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>` with the
    exact owner-provided identified result unit on every `Ok`, including stopped outcomes,
    and preserves both `QuantityRefusal::IllTyped` and `QuantityRefusal::Identity`
    as distinct typed '
  finding: Quantity oracles and native helpers still name removed Runtime Quantity
    APIs; arithmetic/comparison do not consume QSV UnitQuantity or preserve the required
    paired IdentifiedUnit and typed QuantityRefusal.
- fnd: FND-002
  severity: high
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: FR-018-AC-2
  related_ids:
  - FR-018-AC-8
  - FR-018-AC-39
  path: tests/admitted_composite_support/agreement_cases.rs
  lines: 83-103
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus
    `CheckedEquality::evaluate` invoked directly on an independently constructed environment,
    operands and fresh `Meter`. This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
  finding: Both new native fixture matches omit AdmittedCompositeForm::Union, so the
    existing record/tuple agreement test fails Rust exhaustiveness checking before
    any native assertion runs.
- fnd: FND-003
  severity: medium
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: FR-014-AC-2
  related_ids: []
  path: tests/it/exact_scalar_generation.rs
  lines: 693-720
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison, quantity arithmetic/comparison and exact quantity
    conversion (FR-014-AC-48) descriptors for admitted checked-package expression
    nodes each generate a function calling the matching direct owner operator described
    above. Entry-point dispositions for a `quire.op.quantity.convert` node: derivation
    and generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and capabilit'
  finding: The scalar corpus gate expects residue::compare_enum but the emitter writes
    enum_owner::compare_enum; its body.contains assertion necessarily fails for each
    enum comparator.
- fnd: FND-004
  severity: high
  confidence: high
  method: code-review
  check: trace
  artifact_id: FR-031-AC-26
  related_ids:
  - FR-031
  path: src/oracle/boolean_v1.rs
  lines: 900-905
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native oracle holds no `quire_contract_runtime::operators`
    path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`;
    a short-circuit connective is an inline `match` over the left operand's `rt::Outcome`
    that returns the first stop and does not evaluate the right operand when the left
    decides, and a total connective evaluates both operands in order and returns the
    first stop, none charging any point and none calling `evaluate_boolean`; in a
    generated crate the outcomes of FR-031-AC
  finding: The new connective implementation and trace-tagged retention control charge
    BooleanResultRetain, while current FR-031-AC-26 and TC-044 step 14 require no
    connective charge; reviewed original metering reconciliation was omitted from
    the landing.
  test_id: tests/it/oracle_arithmetic.rs::tc_044_native_connectives_retain_only_completed_results
  claimed_ac: FR-031-AC-26
  correct_ac: null
- fnd: FND-005
  severity: medium
  confidence: high
  method: code-review
  check: soundness
  artifact_id: FR-031
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: 249-303
  excerpt: "oracles do (the `quire_exact` types are the planned ones, contingent as\
    \ \"The kernel the native\n  oracle calls\" states; the oracle IR-596 implemented\
    \ uses Contract Runtime's)."
  finding: FR-031 Outputs, prerequisite and OD-2 plus TC-044 still describe the current
    oracle as Runtime-owned or blocked by obsolete QSL divide APIs; the reviewed original
    ownership/compatibility corrections were not carried into this migration.
- fnd: FND-006
  severity: low
  confidence: high
  method: code-review
  check: other
  artifact_id: src/oracle/equality/mod.rs
  related_ids: []
  path: src/oracle/equality/mod.rs
  lines: 54-55
  excerpt: '//! `EnvironmentError::Declaration`; a bounded admission resource stop
    remains

    //! `EnvironmentError::Admission` and is never fabricated as a declaration cause.'
  finding: The new module documentation promises EnvironmentError::Admission, but
    emitted EnvironmentError has no Admission variant and wraps all QSV EnvironmentFailure
    values in Declaration.
- fnd: FND-007
  severity: low
  confidence: high
  method: code-review
  check: other
  artifact_id: SR-11690/FND-004
  related_ids: []
  path: reviews/ir-17-pr397-code-review.md
  lines: 105-117
  excerpt: The new lock-pin test reads the same `LOCKED_DEPENDENCIES` constant that
    `generate_lockfile` reads, then repeats the pin commands itself. Deleting `quire-exact`
    from that constant still passes the test, so FND-002 could regress without any
    test failing
  finding: Retained SR-11690 has no explicit FND-004 disposition even though the current
    independent literal Runtime/Exact assertions correct its reported regression weakness;
    append the verified fixed outcome without rewriting the original finding.
- fnd: FND-008
  severity: low
  confidence: high
  method: code-review
  check: other
  artifact_id: tests/composite_equality_support/cases/enum.rs
  related_ids: []
  path: tests/composite_equality_support/cases/enum.rs
  lines: 20-22
  excerpt: "    // Enum (E_ENUM, `equal`): equal and unequal members of the vendored\n\
    \    // `Example.Phase` declaration. Previously referenced by nothing (module\n\
    \    // doc gap)."
  finding: The moved enum vector still describes a vendored Example.Phase declaration
    although it now independently compiles the original local Phase source and consumes
    facade identities.
bindings:
- test_id: src/oracle/equality/union_tests.rs::union_closure_seals_owner_members_and_admits_each_nested_payload
  ac_id: FR-018-AC-44
  trace: correct
- test_id: src/oracle/equality/union_tests.rs::union_lookup_and_seal_faults_keep_typed_causes_before_equality
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_union_host_generation_closes_nested_owner_declarations
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_union_capability_refusals_preserve_the_healthy_sibling
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_connectives_retain_only_completed_results
  ac_id: FR-031-AC-26
  trace: wrong
```

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

## Dispositions

Round 1, fix head 64fabc7e4c3545f9a16261d72521931656a42d9a. Original Findings and all original review text above are preserved.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Source fixed: generated operands use QSV UnitQuantity, conversion takes IdentifiedUnit and constructs kernel Quantity with unit.id(), comparison uses QSV compare_quantity, and arithmetic returns the complete QSV (Outcome<Quantity>, IdentifiedUnit) through OracleStop::Quantity preserving both QuantityRefusal variants. Native helpers now use public checked graph/table/scope APIs; original vector values/limits and full direct/generated outcome/unit/charge/counter/denial comparisons remain. New chained and stopped paired-unit assertions exist. Execution acceptance remains unrun. |
| FND-002 | fixed | Source fixed: both independent Record/Tuple fixture matches explicitly reject Union with a fixture-specific panic; no Union shape fallback or native Union qualification claim. The existing authentic Record/Tuple assertions and input source are preserved. Generated integration compilation remains a final qualification requirement. |
| FND-003 | fixed | Fixed: the enum comparator expectation now matches enum_owner::compare_enum in each individual generated function body. The author scalar-family host control passed before the final trace-version constraint change; current source retains the corrected assertion. |
| FND-004 | fixed | Fixed: FR031 AC26, Behavior, OD6 and TC044 step14 now require one retain per completed connective including unary and left-decided results, no retain on a child stop, and precise denial behavior. This matches existing completed_boolean and the retention control; its current AC26 binding is correct. PLANNED and unrun execution status remain. |
| FND-005 | fixed | Fixed: Outputs names the actual quire_exact Outcome/Meter without the misleading Runtime parenthetical; prerequisite and OD2 text state the delivered single-kernel compatibility and current QSV/Runtime ownership. Planned behavior and verification are still required; no native/Kani completion is asserted. |
| FND-006 | fixed | Fixed: module documentation names EnvironmentError::Declaration with complete typed QSV EnvironmentFailure, including bounded admission stops; the nonexistent Admission variant is no longer promised. |
| FND-007 | fixed | Fixed custody: committed reviews/ir-17-pr397-code-review.md exactly contains the reviewer-owned validated historical append. Original text and prior rows are preserved, and FND004 explicitly records fixed with source basis and existing original round-2 comment 04e41ac8-b480-47f4-a973-73fe74bb760d. No coder-authored review verdict or new execution claim. |
| FND-008 | fixed | Fixed: moved enum vector prose now identifies the independently compiled original local Phase source. Only the comment changes; member literals, domains, setup and assertions remain. |

## Round 1 scope and qualification

Round 1 source inspection at 64fabc7e only. All eight original source/custody defects are fixed by inspected changes; this is CONDITIONAL source acceptance, with final generated-native execution and broader qualification still pending. No new source defect was found in the fix diff. The user has authorized existing namespace tests, but no Cargo, namespace/backend execution or gate ran in this reviewer pass. The kernel owns the next heavy slot; this review acquired no slot and changed no host/security/account settings.

Supplied author evidence: fix-round-1/compile.log finishes all-targets no-run compilation, and scalar-generation.log has exactly one passing scalar-family host test, both before the final trace constraint change. scalar-native-agreement.log fails fresh dependency resolution at ix-trace-rs =0.1.0 (candidate 0.1.2), before generated test compilation or native assertions. Current root and agreement manifests consistently allow 0.1, which addresses that selection mismatch by source inspection, but there is no successful rerun yet. fmt.log is a formatting action, not an independent fmt-check result. None of these logs proves all current generated-native vectors, paired backend receipts, aggregate gates, or future integrated head.

Quantity source inspection checked owner UnitQuantity resolution, conversion target.id() handoff, complete paired IdentifiedUnit including stops, unflattened IllTyped/Identity errors, subsequent UnitScope::form consumption, original numeric/quantity vectors and denied-charge prefixes. Public QSV owns identities; no QSL947/unit-table facade dependency or copied private identity recipe was added. Equality Quantity Unsupported AC18/38 is untouched. Per-item standalone scalar oracle source keeps quantity-only error variants out of integer proof/observer bodies, preserving the scalar native ABI. The restored Boolean spec agrees with existing retain/stop semantics. Historical SR11690 custody is exact reviewer-owned text.

This is not the final mergeable freeze: AC45 unit metadata and the codex2 port are explicitly scheduled to join later, requiring integrated review and safe final qualification. No aggregate PASS, positive quantity-equality acceptance, native Union acceptance or backend proof is claimed.

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "                    output: OracleOutput::QuantityWithUnit,\n  \
    \                  prelude: Vec::new(),\n                    call: format!(\n\
    \                        \"quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::{operation},\
    \ meter).map_err(OracleStop::Quantity)\"\n                    ),"
- fnd: FND-002
  outcome: fixed
  after_excerpt: '            AdmittedCompositeForm::Union => panic!("the independent
    record/tuple fixture cannot contain a Union"),'
- fnd: FND-003
  outcome: fixed
  after_excerpt: '            format!("enum_owner::compare_enum(rt::ComparisonOperator::{operator:?},"),'
- fnd: FND-004
  outcome: fixed
  after_excerpt: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean`
    exactly once and charges `boolean.result-retain` once, including a result decided
    by the left; a child stop charges no connective result-retain point. Neither rendering
    calls `evaluate_boolean`. In a generated crate the completed Boolean values and
    first-stop outcomes of FR-031-AC-9's cases and Z-3 agree with the prior rendering,
    while completed connective meter counters include their new result-retain charges;
    denial at that point returns `Incomplete` before applying the denied charge.
- fnd: FND-005
  outcome: fixed
  after_excerpt: "- For an expression with at least one arithmetic node, a native\
    \ oracle is a function that returns\n  `quire_exact::Outcome<bool>` and takes\
    \ a trailing `&mut quire_exact::Meter`, as the FR-014\n  oracles do. A Kani bundle\
    \ oracle is a function that\n  returns `bool`, as before."
- fnd: FND-006
  outcome: fixed
  after_excerpt: '//! `EnvironmentError::Declaration`, which preserves the complete
    typed QSV

    //! `EnvironmentFailure`, including bounded admission stops without inventing
    a declaration cause.'
- fnd: FND-007
  outcome: fixed
  after_excerpt: '| FND-004 | fixed | Historical round 2, private Linear comment 04e41ac8-b480-47f4-a973-73fe74bb760d,
    already records FND-004 fixed. Independently rechecked at landing head 5e60b358:
    src/kani/run/scalar.rs:1719-1741 names literal Runtime/Exact package and repository
    pairs, asserts each is present in production LOCKED_DEPENDENCIES, and checks each
    separately generated root/observer lock against the build-lock identity. Removing
    Exact from production now fails the independent membership assertion. This append
    reconciles the missing artifact custod'
- fnd: FND-008
  outcome: fixed
  after_excerpt: "    // Enum (E_ENUM, `equal`): equal and unequal members of the\
    \ original local\n    // `Phase` source declaration, using the independently compiled\
    \ facade identities."
bindings:
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_connectives_retain_only_completed_results
  ac_id: FR-031-AC-26
  trace: correct
```

## Round 2 source and evidence review

Round 2 source review at cd202f1b8fda0c23428c19aa47c40ce3769ff76c covers the entire origin/main...HEAD landing by retaining the original whole-diff review and inspecting all post-64fabc7e changes. Read-only git ls-remote confirms remote main and local origin/main both 0d7813df955576edc867cfa90e2071bd56d844ff. The working tree is clean; no refs were changed. The eight original source/custody findings remain fixed; their original and round-1 disposition rows remain unchanged. Two new test-intent findings and one independently confirmed product finding require another fix round. Source mergeability: FAIL until FND009/FND010/FND011 are resolved, then conditional on final qualification and the independent non-authored review. No product source edits or Cargo/native/namespace/backend/gate execution ran in this reviewer pass.

Production inspection: admit_environment attaches the owner's complete unit_table clone before Union seals, and the emitted recipe does the equivalent after context/package checks. None preserves construction. Direct/nested Quantity Unsupported remains unchanged. Genuine source helper checks owner node lookup versus owner-issued IDs, exact metadata, quantity resolution, unused km and None preservation. Those helper assertions are sound in isolation but do not exercise the production attachment boundary (FND009). The public integration currently checks typed direct/nested refusals, healthy Generated statuses, missing nominal context and wrong-package whole-request precedence, but omits the planned absent-symbol and isolated-source checks (FND010). No positive Quantity equality acceptance is claimed.

QSL lock is 48b47ab99175a80f799b786d6feda1b1ad37cffb; read-only merge-base confirms descendant of required 0b839dd. Named scalar unit fixtures compile authentic public QSL source for the two original owners, preserve unit dimensions/scales/offsets/labels, retain owner-issued node/ID metadata and complete tables, and independently compile enum members. The reserved keyword in becomes source inch while vector label and 127/5000 scale remain unchanged. Arbitrary low-level labels are confined to explicit malformed-topology controls, not nominal fixtures. No private schema/hash recipe or compatible value implementation is introduced. Paired scalar results retain complete IdentifiedUnit across stops and chained operations, typed IllTyped/Identity and charge/counter/denial oracles. HRTB quantity comparison now supports independent operand lifetimes. The existing record/tuple fixture explicitly excludes Union. No native Union acceptance follows from host source tests.

Evidence inspected from fix-round-2: focused-final-summary.json plus each referenced raw log and command JSON confirms the exact named outer native scalar test passed one test, whose runner requires every #[test] in the current source (20 cases), and the exact record/tuple native test passed one test (direct/transitive inside). Each six original host controls matched exactly one test; source iterates all original fifteen fixtures plus mixed. Union closure and public integration each passed two. Metadata helper one and integration two passed, with the coverage limits above. Quantity refusal, source panic scan, scalar-family generation and Boolean retention each passed one. fmt-check, clippy all-targets -D warnings, and spec validation exit zero; spec warnings remain visible. Earlier failed attempts remain in separate logs and are not counted as final PASS. These are author-run focused observations, not reviewer execution and not aggregate proof.

resolved-all-cohorts.json and the three actual root/generated locks retain one-copy outcomes. QSL/QSV/Exact owner revisions align; root Runtime a60c508 and independently fresh generated Runtime 3d2d820 differ because fresh generated crates resolve their public branch constraints. The compatibility trace range permits root 0.1.0 and fresh 0.1.2; neither equality of those Runtime revisions nor a universal sealed cohort is claimed. No lock was seeded into generated scratch crates. Protected source-preservation assertions and unchanged original case declarations were inspected. Main's Quantity -> NonRangeType and equality AC18/38 refusal are unchanged.

FND011 provenance: externally reported by codex2 through the user and parent, independently confirmed here from mark_arithmetic, outcome_bool, plain_node and outcome_short_connective. In a globally metered root, a false holds_arithmetic mark routes an entire Boolean-only subtree through unmetered plain_node, so nested Not/connectives miss the required completed result retain. The outer short-circuit connective then retains once and skips the arithmetic right branch, yielding the wrong stop boundary for result_units=1. Source proof only; no regression execution performed by this reviewer. Preserve original FR031 spec-fix disposition FND004; this is a distinct implementation defect under corrected AC26. The same coder owns the native regression and production fix in this PR.

The AC45 specification lineage is independently reviewed in the nine retained SR15380..15388 artifacts; this pass checks consuming code and test intent, without rewriting their original verdicts. The initial Union amendment reviews SR15372..15377 and PR510 remain intact. Full aggregate make ci, Kani/CBMC/backend acceptance and final retained receipt remain pending. Source inspection/emission fragments never count as execution of the emitted metadata constructor. No new issue or host/security/account change is proposed.

## New findings (disposition pass 2)

| FND | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | medium | The AC45 table-retention test calls attach_owner_units directly rather than the production admit_environment path; removing the production attachment leaves every new AC45 test green, contrary to the required attachment-deletion oracle. | src/oracle/equality/units.rs:64-65 |
| FND-010 | medium | The AC45 mixed Quantity controls only assert a Generated sibling; they never compare its disposition/source to an isolated run or assert absent symbols for refused Quantity items, leaving the required no-symbol and unchanged-sibling branches untested. | tests/it/admitted_quantity_units.rs:73-79,95-133 |
| FND-011 | high | In an arithmetic-containing native root, outcome_bool renders a Boolean-only connective subtree through plain_node inside Completed, bypassing required retains. And(Not(flag),Eq(Add(x,0),x)) with flag=true charges only the outer And; result_units=1 incorrectly completes false instead of stopping Incomplete at its denied second retain. | src/oracle/boolean_v1.rs:1183-1188 |

FND009 failure scenario: delete only the units::attach_owner_units call at equality/mod.rs:1161 and construct TypeEnvironment directly. Quantity refusal occurs independently; supported equality has no quantity metadata dependency, and emitted recipe remains unchanged, so all new tests still satisfy their assertions. Exercise actual admit_environment with authentic context and inspect its retained complete table to close this gap. This scenario is source-reasoned, not an executed mutation.

FND010 failure scenario: mixed generation may corrupt a healthy sibling body while leaving Generated, or emit a dangling oracle for a Refused Quantity item. Current status and generic source-fragment checks do not distinguish either regression. Compare the isolated supported sibling artifact/body and full disposition, and assert refused symbol absence for direct and nested mixed controls, retaining all genuine source identities and refusal precedence. Existing general sibling tests do not discriminate this new metadata/Quantity combination.

## Dispositions round 2

| FND | outcome | reason |
| --- | --- | --- |
| FND-009 | still-open | The AC45 table-retention test calls attach_owner_units directly rather than the production admit_environment path; removing the production attachment leaves every new AC45 test green, contrary to the required attachment-deletion oracle. |
| FND-010 | still-open | The AC45 mixed Quantity controls only assert a Generated sibling; they never compare its disposition/source to an isolated run or assert absent symbols for refused Quantity items, leaving the required no-symbol and unchanged-sibling branches untested. |
| FND-011 | still-open | In an arithmetic-containing native root, outcome_bool renders a Boolean-only connective subtree through plain_node inside Completed, bypassing required retains. And(Not(flag),Eq(Add(x,0),x)) with flag=true charges only the outer And; result_units=1 incorrectly completes false instead of stopping Incomplete at its denied second retain. |

```yaml
dispositions:
- fnd: FND-009
  outcome: still-open
  reason: The AC45 table-retention test calls attach_owner_units directly rather than
    the production admit_environment path; removing the production attachment leaves
    every new AC45 test green, contrary to the required attachment-deletion oracle.
- fnd: FND-010
  outcome: still-open
  reason: The AC45 mixed Quantity controls only assert a Generated sibling; they never
    compare its disposition/source to an isolated run or assert absent symbols for
    refused Quantity items, leaving the required no-symbol and unchanged-sibling branches
    untested.
- fnd: FND-011
  outcome: still-open
  reason: In an arithmetic-containing native root, outcome_bool renders a Boolean-only
    connective subtree through plain_node inside Completed, bypassing required retains.
    And(Not(flag),Eq(Add(x,0),x)) with flag=true charges only the outer And; result_units=1
    incorrectly completes false instead of stopping Incomplete at its denied second
    retain.
review_head: cd202f1b8fda0c23428c19aa47c40ce3769ff76c
scope:
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table.
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: Deleting either attachment must fail its corresponding check.
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean`
    exactly once and charges `boolean.result-retain` once, including a result decided
    by the left; a child stop charges no connective result-retain point.
prior_outcomes_confirmed:
  FND-001: fixed
  FND-002: fixed
  FND-003: fixed
  FND-004: fixed
  FND-005: fixed
  FND-006: fixed
  FND-007: fixed
  FND-008: fixed
new_findings:
- fnd: FND-009
  severity: medium
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-018-AC-45
  related_ids:
  - TC-029
  path: src/oracle/equality/units.rs
  lines: 64-65
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table.
  finding: The AC45 table-retention test calls attach_owner_units directly rather
    than the production admit_environment path; removing the production attachment
    leaves every new AC45 test green, contrary to the required attachment-deletion
    oracle.
- fnd: FND-010
  severity: medium
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-018-AC-45
  related_ids:
  - TC-029
  - FR-018-AC-18
  - FR-018-AC-38
  path: tests/it/admitted_quantity_units.rs
  lines: 73-79,95-133
  excerpt: Wrong-package refusal retains its whole-request precedence; direct and
    nested Quantity items retain their existing typed Unsupported behavior and refusal
    precedence, emit no symbols, and leave independent healthy siblings unchanged.
  finding: The AC45 mixed Quantity controls only assert a Generated sibling; they
    never compare its disposition/source to an isolated run or assert absent symbols
    for refused Quantity items, leaving the required no-symbol and unchanged-sibling
    branches untested.
- fnd: FND-011
  severity: high
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: FR-031-AC-26
  related_ids:
  - TC-044
  path: src/oracle/boolean_v1.rs
  lines: 1183-1188
  excerpt: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean`
    exactly once and charges `boolean.result-retain` once, including a result decided
    by the left; a child stop charges no connective result-retain point.
  finding: In an arithmetic-containing native root, outcome_bool renders a Boolean-only
    connective subtree through plain_node inside Completed, bypassing required retains.
    And(Not(flag),Eq(Add(x,0),x)) with flag=true charges only the outer And; result_units=1
    incorrectly completes false instead of stopping Incomplete at its denied second
    retain.
```

## Round 3 source and evidence review

Round 3 frozen source head 82051284537d9468f03b8a158a3e4f3605309a3f. Independent code/Rust disposition covers every cd202..820512 product/test/documentation/custody delta and retains the complete original whole-landing review scope above. Read-only git ls-remote confirms remote main equals unchanged local origin/main 0d7813df955576edc867cfa90e2071bd56d844ff. Tree is clean; no refs changed. No product edits, Cargo/tests/native execution, namespace/backend work or gates ran in this reviewer pass.

The eight original findings remain fixed with their earlier immutable dispositions. FND009/FND010/FND011 are now fixed by the inspected source and discriminating author-run evidence. FND012 records the independently confirmed original low ownership-prose defect from external SR15389/FND002 at cd202 and its actual correction here; original external findings remain untouched. No additional current defect of any severity was found. All twelve code findings have explicit fixed outcomes; both inherited gap findings retain existing-owner deferred outcomes. Source readiness: PASS. Mergeability: CONDITIONAL on final make ci and canonical Kani/backend qualification, retained evidence/custody, and the external review's own final dispositions. Aggregate/Kani remain UNRUN, so no full product or proof PASS is asserted.

FND009: the new test traverses actual admit_environment instead of directly calling only the attachment helper. It verifies complete original units including unused km, authentic public node lookup, owner-issued IDs, exact quantity metadata/resolution and production context-free None behavior. The retained helper check additionally preserves a populated table under None. The mutation script removes exactly the production attachment, compiles, fails the complete-table assertion with exit101, and restores source in finally; the restored exact one-test command passes. The red was assertion failure, not setup/compilation failure.

FND010: mixed direct and nested Quantity requests retain all original source declarations and typed Unsupported/context precedence. Both call the new isolation helper. All code artifacts excluding claim-map.json equal their isolated surviving siblings byte-for-byte, so private helper/code leakage is also detected. Generated claim comparisons retain all fields except contract_package_identity: public lowering includes selected root sets in that identity, while source-owner identity, node/operation, descriptors, source map, dependencies, schedule and symbols remain equal. This is a justified request identity normalization, not relaxed nominal identity. Syn parses genuine emitted Rust; the exact public entrypoint census matches only surviving environment/oracle names. Both integration tests pass. Quantity remains Unsupported; metadata does not add positive Quantity equality/conversion or native Union acceptance.

FND011: in a globally metered native root, the pure-child connective branch now takes a typed preorder index and recursively renders its child connectives, retaining each completed result. Pure leaves use plain_node with its own index consumption. Compare analysis accepts bounded integers only, so a pure comparison cannot conceal unvisited Boolean connective children. Globally arithmetic-free rendering and ABI stay on the existing path. New genuine generated-native checks exercise Not, short And/Or, total And/Or and implication plus a comparison-leaf control. Every budget zero through the independent expected retain count checks Completed(false) only at full budget and Incomplete(BooleanResultRetain) otherwise; counters and admitted charges equal the budget, and every charge is BooleanResultRetain, proving right arithmetic remains skipped. The old-code command compiled and produced six failing inner tests at budget1 plus one passing leaf control. The fixed exact outer test passes, and its runner invokes the entire freshly generated native crate with no test filter. Source generates seven distinct #[test] items. Existing native retention and first-stop controls pass, as do arithmetic-free byte identity and comparison shape controls. These are current focused observations; final backend proof is still pending.

FND012: the module and generated comment now name the actual owning crates. No operator body changed in scalar/mod.rs. Existing scalar native twenty-case, record/tuple direct/transitive, six original host controls over fifteen authentic fixtures plus mixed, four Union host checks and scalar-family/Quantity refusal evidence in fix-round-2 remain valid focused observations for unchanged behavior; this round does not relabel them as final-head aggregate/Kani proof. Round3 focused-summary.json and individual command JSON/raw logs confirm exact matched tests and successful statuses: metadata1, Quantity integration2, retention1, first-stop1, unmetered bytes1, unmetered comparison1; pure-connective exact outer1 recorded separately. make fmt-check, locked Clippy all-targets -D warnings and make spec exit zero; existing spec warnings remain recorded. The final-head.json identifies this exact source checkpoint. No broad execution was repeated by this reviewer.

Review custody: both committed official round2 artifacts are byte-identical to their validated reviewer-owned disposition2 files. All four committed external nonauthor reviews SR15389..15392 are byte-identical to their supplied originals. Earlier source/authentication/protected fixture evidence and the nine independent AC45 spec reviews SR15380..15388 remain intact. Branch changes since cd202 are confined to the five product/test paths plus six custody documents listed by git diff. Original domains, fifteen fixture sources, twenty protected qualification directories, native execution ownership and Quantity->NonRangeType are untouched. QSL 48b47ab9 cohort and current public UnitTable/QSV ownership remain unchanged. Final gates belong to the parent, with no extra permission required.

## New findings (disposition pass 3)

| FND | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | low | At cd202 the scalar module introduction and emitted SOURCE_HEADER still call quantity/enumeration operators Runtime-owned despite direct QSV quantity/enumeration calls; correct both ownership statements. Independently confirmed external SR15389/FND002, now fixed in this reviewed round. | src/oracle/scalar/mod.rs:43-44,3634-3635 |

## Dispositions round 3

| FND | outcome | reason |
| --- | --- | --- |
| FND-009 | fixed | Fixed: the genuine-owner metadata control now calls production admit_environment(Some(context), ...), inspects its complete original table including unused km and owner-issued IDs, and checks production None plus helper None preservation. The supplied compiled mutation removes exactly the production attachment and fails the table assertion; restored source passes the exact test. No public/source identity is substituted. |
| FND-010 | fixed | Fixed: both direct and nested mixed Quantity request sets compare all non-claim-map artifacts byte-for-byte against isolated surviving siblings, compare every Generated field except the legitimately request-root-dependent Contract IR package identity, and require a parsed public-function symbol census equal to surviving environment/oracle symbols only. Existing typed refusal/context guards remain. Both exact integration tests pass. |
| FND-011 | fixed | Fixed: outcome_bool recursively renders every BooleanNot/connective within a metered root even without arithmetic below that child; only pure leaves keep the fast path. The globally arithmetic-free entry path remains plain. Native regression covers six pure-connective shapes plus comparison-leaf control across every retain budget, checks only BooleanResultRetain charges, child stops and unreachable arithmetic. Old code compiled: six cases fail budget1, leaf control passes; fixed code passes the native crate. Existing retention, first-stop and unmetered compatibility controls also pass. |
| FND-012 | fixed | Fixed: both scalar module introduction and emitted SOURCE_HEADER identify quire-semantic-value as the quantity/enumeration owner and distinguish Exact and Runtime Boolean operators. Actual emitted owner calls are unchanged. This closes independently confirmed external SR15389/FND002; original external finding/custody is preserved. |

```yaml
dispositions:
- fnd: FND-009
  outcome: fixed
  after_excerpt: "        let prepared =\n            super::super::admit_environment(Some(context),\
    \ Vec::new(), &Default::default())\n                .unwrap_or_else(|_| {\n  \
    \                  panic!(\"production preparation lost the admitted owner's units\"\
    )\n                });\n        assert_eq!(prepared.units(), context.unit_table());"
- fnd: FND-010
  outcome: fixed
  after_excerpt: "    assert_eq!(\n        actual_symbols, expected_symbols,\n   \
    \     \"only generated siblings may publish symbols; refused items publish none\"\
    \n    );"
- fnd: FND-011
  outcome: fixed
  after_excerpt: "        let connective = matches!(\n            expression.kind(),\n\
    \            ExpressionKind::BooleanNot { .. } | ExpressionKind::Boolean { ..\
    \ }\n        );\n        if !connective && !self.holds_arithmetic() {"
- fnd: FND-012
  outcome: fixed
  after_excerpt: '//! Generated functions call `quire-exact` scalar operators, `quire-semantic-value`

    //! quantity and enumeration operators, or Runtime Boolean operators with a caller-supplied
    `Meter`. Each

    //! operator owns its charges; no amount is copied into the generated source.'
review_head: 82051284537d9468f03b8a158a3e4f3605309a3f
scope:
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean`
    exactly once and charges `boolean.result-retain` once, including a result decided
    by the left; a child stop charges no connective result-retain point.
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: Deleting either attachment must fail its corresponding check.
- id: SOURCE_HEADER
  path: src/oracle/scalar/mod.rs
  role: examined
  excerpt: '// Each function calls its `quire-exact` operator or a runtime-owned residue

    // operator with the caller''s `Meter`; no charge amount appears in this source.'
prior_outcomes_confirmed:
  FND-001: fixed
  FND-002: fixed
  FND-003: fixed
  FND-004: fixed
  FND-005: fixed
  FND-006: fixed
  FND-007: fixed
  FND-008: fixed
new_findings:
- fnd: FND-012
  severity: low
  confidence: high
  method: code-review
  check: other
  artifact_id: SOURCE_HEADER
  related_ids:
  - FR-014
  path: src/oracle/scalar/mod.rs
  lines: 43-44,3634-3635
  excerpt: '// Each function calls its `quire-exact` operator or a runtime-owned residue

    // operator with the caller''s `Meter`; no charge amount appears in this source.'
  finding: At cd202 the scalar module introduction and emitted SOURCE_HEADER still
    call quantity/enumeration operators Runtime-owned despite direct QSV quantity/enumeration
    calls; correct both ownership statements. Independently confirmed external SR15389/FND002,
    now fixed in this reviewed round.
```

## Round 4 partial source disposition

PARTIAL round4 source review only, frozen head 730d9aba7a502e9f51f55a1c5c473818e9a83453. This is not the completed round4 or a final mergeable verdict. All original twelve code findings remain fixed, and both inherited gap findings retain their existing-owner deferred outcomes. No old finding is reopened or duplicate unowned finding invented for the already owned gate failures. The actual aggregate at 0c4a36b67bb932ade5cfc711146deb2ba23f1bcd failed: 398 library tests passed, four failed, five ignored, make exit2 before integration. That failed gate is retained at final-gates/ci-20261022T024344Z.log and is not replaced by these focused passes. No aggregate retry, Kani/CBMC or reviewer execution occurred here. Current PR acceptance remains pending.

Only src/oracle/equality/mod.rs changed between failed head and this checkpoint. Independent byte comparison confirms everything before its single cfg(test) tests module is identical (SHA256 24b9ce063dd55aab90b5893a91bfaed389d2cef5c4a3fcd6e3e1c717553762c7). No production UnitTable, public API, manifest, lock, source/domain, work limit, stack size or test assertion was changed. No product edits, builds, namespaces, backend operations or git ref mutations were performed by the reviewer.

Cycle fixture fix: the child diagnostic confirms an ordinary failed assertion, MissingAdmittedContext instead of expected TypeResolutionCycle, rather than evidence of a crash. The resolver helper already received the genuine recursive owner, but the item helper passed None. check_nominal_component_item now uses check_nominal_items_with_limits and the same genuine owner. Both Option and Sequence deliberate private graph mutations retain their exact repeated-node assertions, original sibling resolver checks and healthy Integer item assertions. The subprocess wrapper prints exit status, stdout and stderr. Removed unused context-free test helpers are replaced by this explicit nominal component helper, without suppressing warnings. This remains a private inconsistent-input defense, not a claim that malformed identities were publicly admitted. cycles-final.json/log records the exact outer test, one matched pass; source runs both children and requires each to succeed. No stack/shape/domain relaxation was introduced.

Render fixture fix: emit_two now establishes SourceBuilder.expected_package using the genuine admitted source package identity, matching production's required preparation state. It compiles original public QSL source declaring Length/m and a Quantity<m> signature, retains the owner's actual UnitId and complete table, and prepares checked equality through QSV TypeEnvironment. The Quantity variant passes its actual CheckedItem to render_item; the UnsupportedValueType refusal is no longer merely injected. The owner-admitted plan succeeding does not enable public CG Quantity equality: the actual consumer still returns its unchanged Unsupported arm, and production source is byte-identical. No unit identity, compatibility schema or private API is copied. The whole-call generation error remains an explicitly deliberate boundary injection, not claimed public intake. Every original two-item symbol naming, no-code, per-item refusal, healthy sibling and whole-call abort assertion stays intact. render-isolation-owner-final.json/log records the exact one matched test passing. Synthetic transport labels retained in this explicitly private render seam do not acquire compiler provenance from the genuine metadata; no public request acceptance is claimed.

This checkpoint adds no positive Quantity equality/conversion, native Union, or emitted constructor execution claim. The in-process QSV plan and actual renderer are the intended private seam under AC18. UnitTable is used exclusively to supply genuine owner metadata to that check; there is no UnitId derivation, table reconstruction, cross-owner substitution or production refusal change. The unrelated prior metadata and scalar functionality remains as reviewed in rounds1-3.

Recorded author evidence in fix-round-4/partial-summary.json, exact command JSONs and raw logs confirms cycles1 and actual-render-isolation1 pass, make fmt-check zero and locked Clippy all-targets -D warnings zero. Parent-supplied Quire validation is exit0 with existing warnings; this reviewer independently validates the scratch artifacts. git diff --check also passes. No supplied focused result is treated as passing aggregate or as the two remaining large-source controls.

PENDING unchanged controls: the 50,000-record source is 2,027,895 bytes and retains its 512KiB CG stack. The work-bound source is 2,599,335 bytes and retains its original entry, default limit, denied-count and healthy-sibling assertions. Both still fail at source intake under the current QSL public facade's default S1InputBytes1MiB; no successful rerun or acceptance is claimed. Existing QSL960 owns the public StageLimits configuration constructor/re-export. Wait for the real public API, adopt a final merged cohort, configure only finite fixture intake budgets with genuine unchanged sources, and then inspect the two exact passing regressions before a completed review/gate retry. No direct foundation dependency, fabricated wire, guessed API, inference/string escape hatch, or reduced fixture/profile/domain/stack/work target is approved by this partial review. No new issue was filed.

Verdict for the two reviewed independent fixes: PARTIAL source PASS, no additional defect found. Overall source/qualification completion: PENDING on the two upstream-dependent controls and final integrated review. Mergeability is not granted by this partial checkpoint. The prior round3 conditional statement is historical and does not override the known failed aggregate or these explicit pending obligations.

```yaml
dispositions: []
phase: partial
review_head: 730d9aba7a502e9f51f55a1c5c473818e9a83453
scope:
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
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
prior_outcomes_confirmed:
  FND-001: fixed
  FND-002: fixed
  FND-003: fixed
  FND-004: fixed
  FND-005: fixed
  FND-006: fixed
  FND-007: fixed
  FND-008: fixed
  FND-009: fixed
  FND-010: fixed
  FND-011: fixed
  FND-012: fixed
qualification: failed aggregate retained; two focused fixes PASS; two large-source
  controls PENDING existing QSL960; no mergeability granted
```

## Round 5 complete source disposition

Complete Round5 source disposition at published clean head 3de23f5793b2cf4e301de1713cbc70a106bfa434, following the partial round4 record above. Code review including Rust and the full repository gap method completed. No new finding at any severity. All twelve original code findings remain FIXED. No source edits, Cargo/test/native execution, namespace/backend operations, aggregate/Kani gate, commit/push or ref mutation was performed by this reviewer. All earlier findings, still-open rows, fixes and partial verdicts above remain immutable history.

Source verdict: PASS. Mergeability: CONDITIONAL on the parent-owned final make ci and canonical Kani qualification and retaining final official/external dispositions. The actual earlier aggregate failure at 0c4a36b6 remains retained (398 library PASS,4 FAIL,5 ignored; make exit2 before integration); this disposition verifies the source fixes and all four exact regressions on the merged owner cohort, not a passing aggregate rerun. No Kani/CBMC proof or receipt is claimed. The prior partial upstream-API condition is now satisfied; do not continue to call these four regressions unrun. Broader final gate acceptance is still pending.

Baseline reconciliation: fresh remote main and local origin/main both 43d5a6fdc3743835f90b1d6b720cb9195a7db1dd; it is an ancestor of the candidate. The clean merge retains IR604/PR512's stdout-capture tests and Kani lane registration. Makefile/scripts/workflows have zero incremental delta against this current baseline; adding the inherited stdout test to its dedicated Kani lane is not a weakened or new IR17 gate. Current main already supplies FR016AC25 trace bindings, discussed below. Baseline code was not silently fixed or imported into the PR's defect count. Fresh-main ownership/context was inspected, and all migration source/spec/history plus current own authored UnitTable paths remain in the reviewed whole-landing scope.

Merged owner cohort: QSL's eight crates select actual merged 485ccaa3bd289ae94baa74e96a976adce99fd0d8 (PR1197), exposing StageLimits::with_named and ReplayLimits::with_input_bytes through the qsl_replay root. The public implementation validates catalog names, excludes replay/format names from StageLimits, returns typed name refusal and changes no defaults; FR399AC4 explicitly covers real declaration sources requiring all three syntax allowances. No direct foundation dependency, copied wire/schema/private setting enum, guessed public API or string-to-private-type trick is used. Root manifest remains branch main with qsl-replay the sole direct QSL dependency.

The resolved lock advances kernel to b692cc83d84eac33477a7d3d2770bc11f0e0350a and aligns both quire-contract-ir/model to d0737774acfbe64b1019b265952dc4c9720c2f23. QSV remains 1a38c06f39ece40e044a7bef5b868bd478b41c24 and Runtime a60c508f3a2af230aeb51af97bdf549cd5fffa01. Associated Quire/FCD dependency updates are ordinary transitive resolution of that merged owner cohort; no package override or PR pin is committed. Current root lock contains one of each first-party package; supplied one-copy check passes. Independent JSON comparison verifies metadata-before and metadata-after contain identical complete packages arrays AND identical resolve graphs (268 packages), not only selected names. Thus final ir-facade lock alignment changed no actual selected compiled source or feature graph, and the exact expensive checks validly apply across that metadata-only correction.

Large fixture fixes: all changes since the partial checkpoint are private equality test support or dependency/baseline/custody. The production equality prefix before cfg(test) is byte-identical; original record-source generation bytes, 50,000-record/100,001-entry deep target, 65,536 work accounting, both 512KiB stack allocations, integer/domain and source-publication assertions are preserved. Source intake now uses finite actual source length for S1 input and replay bytes, source_bytes+1 for tokens (fixed ASCII grammar plus EOF), and 8*records+32 syntax nodes with checked arithmetic. These allowances configure only authentic fixture compilation; they do not widen CG work/source/depth/stack/proof profiles or production defaults. The independently compiled retained deep source uses the same public finite allowances outside its unchanged 512KiB CG/runtime thread. No synthetic replacement source, ID or result is introduced.

The work fixture already selected an unlimited output allowance to isolate work accounting; its final inspection now calls finish(usize::MAX) instead of reapplying an unrelated default. Default/exact/one-byte-less source-limit checks remain in the separate deep control. The deep provenance assertion now locates the module actually reconstructing Chain0 via its genuine root node lookup, preserving authoritative key sorting instead of assuming that Chain0 sorts into the first module. Root module provenance and every module's item subject are still asserted.

Seeded lock boundary was independently assessed against FR018AC28/30 and FR017AC53/TC027: copying the executing lock is confined to this private flat-compilation/reconstruction fixture, which must reconstruct the same owner-issued package identities. It does not claim independent fresh dependency resolution or the canonical production/observer paired receipt. The latter still requires two independently freshly resolved locks without copying a parent/cached resolution, and its production/observer implementation, literal-pin assertions and regression are unchanged. The local premerge captured fixture lock and root common first-party identities matched; this final merged deep run uses the same seeding recipe and passes real native compilation/reconstruction. No separate fresh-resolution claim is inferred for this seeded fixture. Parent final gates still own current production/observer resolution and receipt acceptance.

The cycle and actual render-isolation fixes reviewed in partial4 remain unchanged. Cycle paths use the genuine recursive owner before the private repeated-node fault, retaining exact Option/Sequence cycle and healthy-sibling assertions; diagnostics retain both streams/status. Render isolation obtains genuine source package/unit metadata, owner-checks Integer/Quantity plans and actually calls render_item to obtain UnsupportedValueType. Whole-call generation failure remains a labelled private injection. This is defensive seam coverage under AC18, not public positive Quantity equality. No production behavior or nominal authorization is inferred from a privately constructed CheckedItem.

Independently reviewed UnitTable scope includes units.rs, actual admit_environment wiring, emitted environment preparation and admitted_quantity_units.rs. All are unchanged from the reviewed fixed round3 source under the now merged owner cohort. Complete original unit_table cloning occurs after guards and before Union sealing; None preserves construction; complete unused metadata/owner identities are asserted through the production preparation path. Mixed direct and nested Quantity controls compare isolated sibling artifacts/dispositions and assert only surviving entrypoint symbols. Only legitimately root-dependent Contract IR request identity is normalized; original owner/descriptor/schedule/source fields are retained. The earlier compiled production-call removal fails the intended table assertion and restores to PASS. Quantity Unsupported AC18/38, frame Quantity->NonRangeType, typed lookup/declaration errors, no-equality-meter-charge failure ordering and owner-key-vs-node Union sealing remain unchanged. No positive Quantity equality/conversion, native Union or emitted metadata-constructor execution is claimed. The complete authored UnitTable slice was reviewed independently here, rather than excluded with the external review's authored slice.

Actual merged-cohort evidence from fix-round-5-merged: work-bound-merged.log, deep-small-stack-merged.log, cycle-merged.log and render-merged.log each name the exact requested test, match one test and finish with one PASS. The deep test performs full generated native compilation plus independent source admission and runtime 512KiB work; the work control retains both default denied-entry and healthy boundary assertions. Earlier token/node allowance failures remain preserved separately. Final JSON durations include surrounding Cargo time; raw work test is 58.27s and deep test202.02s, consistent with observed commands. These are author executions, not reviewer executions. Current merged make fmt-check, locked all-target Clippy -D warnings, make spec and diff/one-copy checks pass; existing catalog/EARS warnings are retained. Older scalar20/native/host/metadata evidence remains provenance-limited to its earlier cohorts. It is neither discarded nor relabelled as execution of all current kernel/cohort paths; the parent aggregate provides that final check.

Historical custody: committed official code/gap files exactly equal validated disposition4-partial copies. Original historical SR11690 FND004 source-literal pin disposition remains intact; all external SR15389..15392 and nine AC45 spec review artifacts remain preserved. No CG spec/plan text changed in this final fixture/dependency round, so previously completed spec lenses/lineage remain applicable without inventing new clean review artifacts. Optional semantic gap analysis remains not opted in; mandatory code/Rust test-oracle review was performed. All targets and evidence are retained; no new issue, host/security/account change or additional permission flow is introduced.

External nonauthor final source review at the same 3de23f57 head now reports PASS with no new finding; its latest four exact reviewer-owned files remain in qualification-logs/review-cg513/reviews for final custody. Private IR17 code comment 2cd8663a-a83e-48cd-b80a-c8012cee52d5 and gap comment 0935e3fc-5b0e-4d76-9d40-746ecb9f1abe record that review. Official UnitTable review above independently covers the authored slice excluded there. Parent retains those validated latest artifacts before final gates.


```yaml
dispositions: []
phase: complete-source-review
review_head: 3de23f5793b2cf4e301de1713cbc70a106bfa434
baseline_head: 43d5a6fdc3743835f90b1d6b720cb9195a7db1dd
scope:
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
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
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
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table. Valid context-free
    None behavior remains unchanged. Wrong-package refusal retains its whole-request
    precedence; direct and nested Quantity items retain their existing typed Unsupported
    behavior and refusal p
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
- id: FR-399-AC-4
  path: /home/peter/.cargo/git/checkouts/quire-spec-language-c986aa40394a5ad7/485ccaa/spec/functional/FR-399-set-a-named-stage-limit-through-the-replay-root.md
  role: context_only
  excerpt: A source of real declarations larger than 1 MiB, with `s1.input_bytes`
    and the reader limit raised, is refused naming the `s1.tokens` ceiling; with `s1.tokens`
    also raised, naming the `s1.nodes` ceiling; and it compiles only with `s1.input_bytes`,
    `s1.tokens`, `s1.nodes` and the reader limit all raised.
prior_outcomes_confirmed:
  FND-001: fixed
  FND-002: fixed
  FND-003: fixed
  FND-004: fixed
  FND-005: fixed
  FND-006: fixed
  FND-007: fixed
  FND-008: fixed
  FND-009: fixed
  FND-010: fixed
  FND-011: fixed
  FND-012: fixed
new_findings: []
qualification: source PASS; four actual merged-cohort regression PASS; parent aggregate
  and canonical Kani pending
```

## Round 6 combined source disposition

Complete official Round6 source review of the ONE combined PR513 candidate 9b2c69f22f8840930fde7a7a29abe1ff1859eab5 against current main724d6bfba77154dde44623655e57d3d7c92d8301. Fresh read-only remote main matches this baseline and it is retained by ancestry; tree is clean. Whole-PR source acceptance retains the complete prior review and inspects all later integration repairs, accepted IR850 source and selective Runtime adoption. Code/Rust PASS; no new current finding of any severity. All twelve earlier source findings remain FIXED. Historical failures, still-open rows, partial scopes and all explicit fixed/deferred outcomes are preserved above; no blanket baseline fixes or new issue is invented.

Mergeability is CONDITIONAL on the parent final aggregate make ci, canonical Kani gate and the separately requested nonauthor lunar lock-hunk review. This official review does not substitute for that independent lock review. Runtimea60 is reviewed only together with its consumer migration in this combined PR; no standalone main Runtime bump is accepted. No reviewer Cargo/compiler/test/native/LLVM/backend/Kani/namespace execution, product/source/lock edits, branch integration, push or ref mutation occurred. Parent owns final custody and one final-head gate sequence. No permission request is introduced.

The earlier def1f3f0 full batch remains failed historical evidence: library402 PASS, integration697 PASS/6 FAIL/39 ignored, before these repairs. It is retained at final-gates/ci-20261022T052217Z.log. The six failing controls shared five root fixture/expectation causes; the intermediate export E0425 and wrong-manifest preflight defect are also retained in their exact evidence/intake records. They are now corrected by inspected source and the actual combined-candidate focused logs, rather than waived or reported as defects still present. Current aggregate/Kani have not yet run, so a full product/qualification PASS is not claimed.

Combined dependency review: versus the accepted source9b01, Cargo.lock changes ONLY the Runtime package entry: its branch-main source advances 3cc88c480d4efe5179fe91acf4634def5d2d82fd to a60c508f3a2af230aeb51af97bdf549cd5fffa01, plus that package's real quire-semantic-value edge. All other lock bytes remain unchanged. Independently parsed package comparison confirms Runtime is the sole changed named package; root lock measures the exact authorized43dd9ef84f43ea430aa0e3408e5a77063a732adda9a3d294b903642b7a0f9582. Against published main724 the additionally visible codegen->QSV root lock edge is already required by this migration's direct Cargo.toml dependency, not an owner revision adjustment or workaround. No broader resolver change, version pin, source patch, foundation dependency, RT exception or copied type bridge is introduced.

The supported ROOT cohort has all eight QSL crates485ccaa3, shared kernel72b25421, QSV1a38c06f, Runtimea60c508f, quire-rsfd1cdd9d, and all three FCD packagesac62202a. Independent parsing verifies all FCD root sources match and there is one authoritative kernel/QSV identity. Actual locked and offline-locked metadata exit0; their full packages arrays and resolve graphs compare identical, independently rechecked. The supplied authoritative IR736 quire check-lock Cargo.lock exits0 with Git lock coherence checked; one-copy and allowlisted selective-lock checks pass. This is a root-cohort fact, not an assertion about any separately resolved generated workspace. The published owner route is unchanged except for the user's explicitly authorized combined Runtime selection.

Known IR962 observation is retained honestly and NOT substituted for a root-cohort verdict. The independent shared agreement runner still resolves fresh public branch-main scratch closures. Captured bounded/function native locks select Runtime3d2d8201, QSL52620956, kernel72b25421 and split FCD extraction/semanticIRac62202a versus schema6e2c98fc. The actual bounded-native owner check-lock exits2 for that split; native test PASS is not evidence of lock coherence. The user's explicit lunar ruling excludes that generated-scratch issue from PR513 acceptance and CG supported ROOT-cohort validation and assigns existing IR962. This scope disposition is recorded rather than ignored, repaired, re-resolved, seeded or relabelled PASS. The runner and captured scratch locks were not changed. No claim is made that these generated native legs executed root Runtimea60/QSL485, or that IR962 is fixed. This is distinct from the canonical production/observer fresh-lock and receipt requirements, whose implementation and source constraints remain unchanged and whose final gate acceptance is still parent-owned.

Six integration repairs independently inspected:

1. Genuine bounded/asymmetric source stays byte-for-byte Int[0,9] versus unbounded Integer with convert<Integer>(a). The stale absence-of-conversion premise is replaced by positive checks of exactly one authentic conversion, target Integer, original bounded parameter node and equality's genuine converted argument. No source substitution, copied IDs or generated-oracle-derived expected bounds. Claim descriptors, source/target order, independent native values including right operands outside[0,9], domain/no-narrowing and charge assertions remain. This repairs both original bounded-native and claim-descriptor failures. Existing spec AC43's erased-conversion limitation is conditional and does not prohibit the genuinely retained path now observed; independent admitted-IR conversion-reference controls are preserved.
2. Function native WrongValueKind Reference control now uses the real public kernel ObjectReference/UniverseId/EffectiveId/ObjectId, preserving opaque test values[6;32], [7;32] and authored object string. It still expects WrongValueKind(parameter0) and no charges, and expressly does not claim DanglingReference coverage. No nominal compiler provenance is invented for these original negative-control bytes.
3. The ACTUAL generated-v2-clause-corpus manifest now names QSV using the shared public SEMANTIC_VALUE_DEPENDENCY_SOURCE. The earlier misplaced dependency in the unrelated order_numbers-only first control was removed, and preflight's sole missing-manifest finding was corrected at33440669. All15 clause expressions, native expected values, source-map/probe checks and bounded inputs remain. No renderer change or suppression replaces the real missing dependency.
4. Routed scalar Quantity conversion expects the actual quantity_owner::convert_quantity call, retaining Generated dispositions, absent harness and OperationNotRendered reason. It is a structural/disposition control and supplies no positive Quantity native execution evidence. Separate scalar Quantity payload/converted-value work is not silently accepted by this check.
5. The LLVM fixture now declares its actual Exact dependency and uses quire_exact as rt in its native test. Whole-file comparison at accepted9b01 showed that isolated import change preserves every input, limit, expected Completed(true), genuine LLVM coverage invocation and evaluation1/consequent0/classification=vacuous assertion. The fixed code does not bypass LLVM or weaken vacuity observations.
6. IR850's existing profile constant is publicly re-exported, with a FR014AC8/TC024 regression asserting the emitted QSV dependency line through that public root binding. Compiler redE0425 before export and one-test green afterward are retained. The real external clause corpus also uses the public constant. No manifest source spelling/owner implementation is duplicated. Function output aliases rt=Runtime exact, exact=kernel and sv=QSV declaration remain; scalar output owner aliases are unchanged. This exact accepted source lineage is independently recorded in validated SR15393/15394 at ir850-export-review/final-readback, now retained by ancestry in this combined candidate.

Actual combined-candidate evidence: final-summary.json and each exact JSON/raw log match bounded-parameter, claim-descriptor, function-native, clause-native, quantity-route and llvm-native; each runs one outer test with zero failures/ignored and exits0. Both native agreement runners require every declared inner assertion, the clause corpus retains its15 expressions, and LLVM executes instrumented native code. Bounded JSON wall158.29s includes build work; raw test80.48s, function native78.51s, clause5.60s, route0.94s and LLVM5.42s agree with the recorded test results. These are fresh combined-candidate author runs, not merely earlier local IR850 lock validation. Root fmt-check, locked all-target Clippy -D warnings, Quire/spec with existing warnings, diff, owner check-lock and metadata checks exit0. The earlier six-control failure remains visible alongside them. No aggregate, Kani/CBMC, proof receipt or all-native Quantity execution is inferred from these focused passes.

The entire authored UnitTable slice remains independently covered by this reviewer: units.rs, real admit_environment wiring, emitted preparation, genuine unused unit/owner-ID assertions, direct/nested Quantity refusals, isolated sibling/source/symbol checks. These files and production equality logic are unchanged since the accepted prior round. Metadata attachment is still post-guard, clone-complete and before Union sealing; None behavior, typed refusal order, no equality charge on preparation failure, key-vs-node roles and owner member seals are unchanged. No positive equality Quantity/conversion, native Union or emitted metadata-constructor execution is claimed. Quantity Unsupported AC18/38 and frame Quantity->NonRangeType remain. Original fifteen genuine sources/domains, six host bodies, protected qualification files, work counts, deep source generator and512KiB stack controls are unchanged by this round. No compatibility layer, fake facade identity or copied owner implementation was added.

Fresh main724 publishes the supported QSL/kernel/FCD cohort and its owner review; these are baseline context, not new consumer defect fixes. Makefile/scripts/workflows have zero incremental change against main. No normative CG spec/plan delta exists beyond the previously reviewed owning amendments, so existing spec/AC45/Union review lineage remains applicable. No applicable AssuranceProfile or new assurance exception is introduced. Optional semantic gap analysis was not opted in; mandatory code/Rust oracle checks and full mechanical repository requirement/test/source reconciliation were performed. Latest committed official code/gap documents match disposition5 byte-for-byte; append-only Round6 records preserve their entire history. Current final artifacts plus IR850 final-readback files must be retained before parent final gates.

```yaml
dispositions: []
new_findings: []
review_head: 9b2c69f22f8840930fde7a7a29abe1ff1859eab5
baseline_head: 724d6bfba77154dde44623655e57d3d7c92d8301
scope:
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts
    of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2`
    clause bodies.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation
    was observed, that has at least one implication and for which no expected consequent
    probe was observed is `vacuous`.
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class
    of FR-004-AC-5 yields a non-success outcome with no invented classification.
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
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descriptor and generates an `ir_confirmed` oracle over its connectives, integer
    comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`.
    When the condition is a reference to an admitted Boolean expression node, the
    oracle uses that node's expression, not the reference as a Boolean variable; an
    inline condition uses its own expression. In both cases the claim and generated
    oracle retain the clause node identity, not
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other quantity operation, as `OperationNotRendered` (Planned,
    IR-269), because the Kani scalar renderer builds no quantity harness; the conversion
    is credited by its oracle only and is uncredited for Kani. The settlement is not
    a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criterion.
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "Each generated item's claim-map entry records the descriptor the request\
    \ supplied \u2014 operator, both operand source types and both conversion targets\
    \ \u2014 and it is equal to the request's, so the independent native run of AC-2\
    \ is driven from the request and never read back out of the generated crate. An\
    \ authentic asymmetric source control preserves the actual bounded-left/unbounded-Integer-right\
    \ source IDs and independent caller target; a source conversion erased from the\
    \ emitted graph is not evidence of a retained conversion-reference branch."
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference
    to a `convert` expression node is read as the type of what it converts, following
    nested conversions to the first node that is not a conversion, and a reference
    to any other node, including a non-`convert` application, as that node''s own
    `semantic_type`. A descriptor whose `source_type` is the read type generates,
    and any other source is refused as an operand-type disagreement at that position,
    reporting the read type as found. A genuine source fixture with no emitted convert
    node exercises its actual parameter source types'
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
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table. Valid context-free
    None behavior remains unchanged. Wrong-package refusal retains its whole-request
    precedence; direct and nested Quantity items retain their existing typed Unsupported
    behavior and refusal p
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The oracle returns the arity, wrong-value-kind, dangling-reference and
    unknown-function input refusals unchanged to the caller before any execution charge;
    the schedule records ordered arguments without inventing a refusal result. That
    the refusals equal QSL-owned ones is outside CG''s boundary (IR-842, FR-014-AC-47).
    PLANNED, blocked on IR-853: once Contract Runtime FR-275 deletes the `exact` port,
    the refusal comparison''s target is decided by IR-853.'
prior_outcomes_confirmed:
  FND-001: fixed
  FND-002: fixed
  FND-003: fixed
  FND-004: fixed
  FND-005: fixed
  FND-006: fixed
  FND-007: fixed
  FND-008: fixed
  FND-009: fixed
  FND-010: fixed
  FND-011: fixed
  FND-012: fixed
gate_issue_outcomes:
- control: bounded parameter + claim descriptor
  state: fixed and exact combined controls PASS
  source: tests/it/admitted_enum_equality.rs genuine conversion provenance; original
    source/domains retained
- control: function native
  state: fixed and exact combined control PASS
  source: tests/exact_function_support/agreement_cases.rs public kernel reference
    carriers; WrongValueKind/no charges retained
- control: clause corpus
  state: fixed and exact combined control PASS
  source: tests/it/kani_obligations_state_frame.rs actual corpus QSV manifest; public
    export available
- control: Quantity route
  state: fixed and exact combined control PASS
  source: tests/it/routed_generation.rs owner-call assertion; no native Quantity execution
    claim
- control: LLVM vacuity
  state: fixed and exact combined control PASS
  source: tests/it/v2_llvm_coverage.rs Exact dependency and native import; vacuity
    oracle unchanged
included_review_lineage:
- SR-15393
- SR-15394
known_out_of_scope:
  issue: IR-962
  actual_check_lock_exit: 2
  scope: fresh native generated scratch FCD split; user/luna excludes from PR513/root
    supported cohort acceptance
  root_owner_check_lock_exit: 0
qualification: source PASS; six exact combined controls PASS; conditional on final
  parent ci/Kani and separate lunar lock review
```

---
id: SR-12900
title: IR818 code code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; spec/, src/, tests/; IR818 PR439
review_set: subset
---

## Summary

Ticket: IR-818. PR: quire-contract-codegen#439. Code review with the Rust lane examined all fourteen changed files, the merged FR-014 contract and TC-024. The production conversion path is consistent with the exact-only, call-time target contract; one medium assertion gap remains.

## Verdict

CONDITIONAL — FND-001 requires denial runs to retain the full meter observation. No production behavior defect was identified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Injected conversion denials compare only outcome and ResultUnits consumption; admitted-charge traces and all other consumed counters are omitted, so a generated Incomplete branch that admits an extra non-ResultUnits charge can pass agreement. | tests/exact_scalar_support/agreement.rs:184; tests/exact_scalar_support/agreement_cases.rs:1468 |

## Finding evidence

The existing agree2 macro calls metered() for each ordinary run and denials() for every injected denied occurrence. metered() records outcome, admitted_charges and every LimitKind consumed counter. denials() records only (point, occurrence, outcome, consumed(ResultUnits)). The new agree_conversion path relies on that narrower projection. If the generated Outcome::Incomplete branch admitted an IntegerBits charge after RT returned, ordinary completed runs and ResultUnits still match, and the new denial comparisons would remain green. Compare each denied execution's outcome, admitted-charge order and all consumed counters; retain the explicit U10 no-partial-result assertion. This is a static assertion-path finding; no mutation execution was claimed.

## Coverage

The public QuantityConversion variant carries no unit. SourceBuilder calls existing RT convert_quantity with QuantityTarget::Exact, adapts completed Exact magnitude with conversion.unit(), preserves typed IllTyped and Undefined/Refused/Incomplete, and charges no adapter work. Future unsupported outcomes stop with InvalidConstant, following existing generated stop policy. The claim records the actual member declaration CheckedNodeId in ordinary and occurrence paths and omits absent fields on unrelated claims; the output is Serialize-only, so an input-deserialization default is not relevant. Identity/exact confirmation uses the common catalog; mode is checked before target form. Decimal/Integer/nonexact seams are honestly module-only, while compound target admission is tested.

Removing the named conversion scan retains generic body/signature/argument refusals and supported function siblings. Type-kind failure is handled per function before bindings, project_function_node refuses unsupported result types, and item_disposition validates unsupported argument types. Quantity routing reaches OperationNotRendered without quantity Kani rendering; the only Kani scalar edits initialize two test literals. Dependencies, CI, Makefiles and upstream API are unchanged.

New vector rows cover U01, both U04 affine conversions, U09b identity, U10, U15 and denial edge, U22 compound sources to m2/cm2 and denied bits, U25 second step, and U26 with denied edge. Full ordinary outcomes/units/meter state are compared; four IllTyped rows also assert a populated meter baseline. The separate final-retain denial checks Incomplete and zero retained result units. Refusal-only scalar/compound targets are not credited as runtime agreement. Tests use real admitted IR/runtime/generated code, with no mock substitution.

Rust review covered typed errors, unsafe/panic additions, deterministic ordering, bounds, public docs, wire output, test placement/traces, borrow/ownership and gate integrity. There is no new unsafe, request-path panic, thread/async boundary or unbounded traversal. The adaptation repeats no conversion algorithm. Repo conventions and portable rust-style were read.

The frozen candidate was reviewed against its assigned main base. Source, dependency, CI and branch contents were not edited. No applicable AssuranceProfile exists under spec/. No Cargo, rustc, rustfmt or Kani was launched: IR858 owns the exclusive heavy slot. Author execution logs were inspected as author evidence, not independent reviewer runs. The author final fmt/lint logs pass; its clean make kani-gate reports 36/36 controls, 797 seconds and Kani 0.68.0. The lead owns independent final aggregate and Kani execution after review custody.

## Examined units

```yaml
scope:
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division, modulo), rational, ordering,
    decimal, IEEE arithmetic/comparison/width conversion, text comparison, enum comparison, quantity arithmetic/comparison
    and exact quantity conversion (FR-014-AC-48) descriptors for admitted checked-package expression nodes each
    generate a function calling the matching runtime `exact` operator. Entry-point dispositions for a `quire.op.quantity.convert`
    node: derivation and generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and capability settlement '
- id: FR-014-AC-5
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map, claims, bounds and operation
    identity of its item, and, for a `quire.op.quantity.convert` item, the `type_argument` type node id (FR-014-AC-48).
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193 vectors, and on these QSpec
    TC-187 vectors, yields outcomes, charges and consumed counters equal to direct runtime execution: U02 (dimension
    normalization through a product and a quotient, and a zero power), U03 (the dimension mismatch), U05 (arithmetic
    and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base dimension of the same declaration and shape
    as `L` under a different node key, refused `ill_typed`), U12 (compound-unit products, quotients and powers,
    with the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `quire.op.quantity.convert` node whose `type_argument` names a quantity unit and whose `operation.mode`
    is `exact` derives and generates an `ir_confirmed` oracle, described by the public operation `quantity_conversion`
    (`ExactScalarOperation::QuantityConversion`, a unit-free variant: the target unit is a call-time argument `unit:
    &QuantityUnit`, and the generator does not check it against the `type_argument`, which the node carries only
    as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with no checked bound, and the claim-map entry
    records the `type_argument` type node id (FR-014-'
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When the runtime refuses a `quire.op.quantity.convert` with `IllTyped` before any charge, the oracle
    stops as `ill_typed` with zero admitted charges and no change to any consumed counter, equal to direct `convert_quantity`
    execution (Contract Runtime FR-007-AC-21): `1 m` to `s` (U03-style `m` to `s`), U09 (a base dimension of the
    same declaration and shape as `L` under a different node key), `1 N_m` to `J` (U22) and compound `4 m^2` to
    `m`. An affine source or target converts and is not refused as `AffineUnitArithmetic`; only arithmetic on an
    affine unit is refused (FR-014-AC-6, U05).'
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar: its result is always a `Quantity`
    in the supplied target unit, and a conversion node whose `operation.mode` is not `exact` is refused with `ClaimDerivationRefusal::ModeNotDerivable`,
    checked first, so a scalar `type_argument` with a non-`exact` mode is `ModeNotDerivable`; one whose mode is
    `exact` and whose `type_argument` is a scalar type (`Decimal` or `Integer`, QSpec TC-187 U07, U08, U16, U17,
    U18 and U29) or a compound unit (U25''s first step; IR-831) is refused with `OperandFormsNotDerivable`, each
    with no descriptor, no'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity
    operation, as `OperationNotRendered` (Planned, IR-269), because the Kani scalar renderer builds no quantity
    harness; the conversion is credited by its oracle only and is uncredited for Kani. The settlement is not a refusal
    by name, and a quantity harness is IR-269's work, not this requirement's. The owning settlement row is TC-030's
    Quantity scalar row, which names `quire.op.quantity.convert` and cites this criterion.
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: examined
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
- id: src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  role: examined
  excerpt: //! Exact complete-V1 function-application oracle generation from one admitted
- id: src/oracle/function/schedule.rs
  path: src/oracle/function/schedule.rs
  role: examined
  excerpt: //! Target-neutral, bounded flat SSA scheduling for FR-021.
- id: src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  role: examined
  excerpt: //! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
- id: src/routed/capability.rs
  path: src/routed/capability.rs
  role: examined
  excerpt: '//! Capability settlement: the one point where a requested claim is settled.'
- id: tests/exact_scalar_support/agreement.rs
  path: tests/exact_scalar_support/agreement.rs
  role: examined
  excerpt: '//! Agreement support: direct Contract Runtime execution and generated'
- id: tests/exact_scalar_support/agreement_cases.rs
  path: tests/exact_scalar_support/agreement_cases.rs
  role: examined
  excerpt: '//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime'
- id: tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
- id: tests/it/capability_settlement.rs
  path: tests/it/capability_settlement.rs
  role: examined
  excerpt: '//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.'
- id: tests/it/exact_scalar_agreement.rs
  path: tests/it/exact_scalar_agreement.rs
  role: examined
  excerpt: '//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime'
- id: tests/it/exact_scalar_generation.rs
  path: tests/it/exact_scalar_generation.rs
  role: examined
  excerpt: '//! FR-014: exact complete-V1 scalar oracle generation over admitted'
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: //! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and Contract IR FR-036
- id: tests/it/routed_generation.rs
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: '//! FR-022 routed generation: `generate_routed` runs the routed kind''s generation arm keyed by the'
bindings:
- test_id: src/oracle/scalar/mod.rs::tests::quantity_conversion_confirmation_requires_identity_and_exact_mode
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::exact_quantity_conversion_vectors_agree
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::quantity_conversion_derives_and_preserves_target_provenance
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::caller_declared_quantity_conversion_is_confirmed
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::quantity_conversion_ill_typed_preserves_meter
  ac_id: FR-014-AC-49
  trace: correct
- test_id: src/oracle/scalar/mod.rs::tests::quantity_conversion_rejects_scalar_targets_after_mode
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::exact_quantity_conversion_vectors_agree
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::admitted_compound_conversion_target_refuses_without_changing_sibling
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::unit_function_signatures_refuse_per_function_and_preserve_supported_siblings
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/it/capability_settlement.rs::a_selected_quantity_conversion_settles_as_operation_not_rendered
  ac_id: FR-014-AC-51
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_quantity_conversion_claim_is_unrendered_with_no_harness
  ac_id: FR-014-AC-51
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-014-AC-51
  trace: correct
```

## Dispositions

Round 1: the denial-observation fix was inspected in the frozen fix candidate. The original finding and initial verdict above are preserved as the review-pass record.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | DeniedRun now retains the original outcome, denied point/occurrence, ordered admitted charges and every LimitKind consumed counter. agree2 compares the complete denied observations. A real-runtime faulty-adapter control preserves the old projection while adding non-result charges after Incomplete, and is rejected by the full comparison. The explicit U10 Incomplete/zero-result-retention check remains. |

## Disposition round 1 evidence

Latest code-review verdict: PASS, with no unresolved finding or new defect identified. Only tests/exact_scalar_support/agreement.rs and agreement_cases.rs changed during this fix round. No production, dependency, CI, specification or gate change occurred. The new should_panic control accepts only the exact agreement-mismatch assertion text; preliminary assertions have different messages, so they cannot substitute for detecting the mutation. Its admitted-charge and all-counter inequalities are measured from real runtime arithmetic after Incomplete, while both the ordinary-run observation and old denial projection are asserted equal.

The author's fix-round-1-agreement.log records both exact scalar and function generated-crate agreement tests passing (2 passed, 0 failed, 0 ignored). The scalar runner counts all declared #[test] cases, requires a positive count and requires that count to equal the child test summary, so the new expected-mismatch control is included. fix-round-1-fmt.log and fix-round-1-lint.log show successful formatting and all-targets Clippy with -D warnings. These are inspected author logs, not independent reviewer execution; no Cargo, rustfmt, rustc or Kani command was launched by the disposition reviewer. The coder report's static-ready paragraph predates this measurement; the actual completed logs and lead's confined-measurement receipt supply the later execution evidence. Final clean-candidate full Kani and aggregate gates remain with the lead after artifact custody.

SR-12901 is unchanged: this test-only fix introduces no new criterion or unresolved trace ID, no ownership change and no production stub. Its recorded whole-repository FAIL and existing baseline routing remain. No gap disposition comment is needed because it had no real finding.

Fixed source excerpt:

```rust
                    DeniedRun {
                        point,
                        occurrence,
                        outcome,
                        admitted_charges: denied.admitted_charges().to_vec(),
                        consumed: LimitKind::ALL.iter().map(|kind| denied.consumed(*kind)).collect(),
                    }
```

---
id: SR-611031
title: Gap analysis of authenticated-view adaptation in CG PR 608
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1079; CG PR 608 delta a5f60818..eee90d17; FR-014, FR-022, checked-graph association evidence"
review_set: subset
---

## Summary

Planless, diff-scoped gap analysis of the IR-1087 adaptation on CG PR 608 at `eee90d173ba1b9ea8a0504191c1ece1625d30178`. Plan completion: not assessed. The new authenticated test path passes focused quantity conversion, but it does not establish behavior for the public plain-V1 reader input, and one published association control fails before CG checks execute.

## Scope

```yaml
scope:
  - {id: IR-1079-AC-2, path: src/core/ir/scalar.rs, role: examined, excerpt: "Adapt every CG call site to the reviewed current typed APIs (public accessors instead of the private CheckedApplicationOperation::get, exhaustive CheckedTerm/CheckedMemberType, ExecutionPoint unification, owner_model, ReplayResult/IllTypedCause). No compatibility layer, no re-exported private accessor, no synthesized causes, no unchecked conversions."}
  - {id: IR-1079-AC-3, path: tests/it/routed_generation.rs, role: examined, excerpt: "Compile-level adaptation only: no new behavior. Focused tagged tests where an adapted site has behavior; do not claim proof coverage from compiler-only changes."}
  - {id: FR-014-AC-51, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "A selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity operation, as `OperationNotRendered` (Planned, IR-269), because the Kani scalar renderer builds no quantity harness; the conversion is credited by its oracle only and is uncredited for Kani."}
  - {id: FR-022-AC-11, path: spec/routed/functional/FR-022-routed-generation.md, role: context_only, excerpt: "An item with no derivable descriptor keeps its typed refusal and no harness, and its siblings' records and harnesses are unaffected."}
  - {id: IR-1079-AC-4, path: src/oracle/checked_graph/tests.rs, role: examined, excerpt: "One independent diff review, findings fixed in the same PR, one final-head gate, no regate for disjoint main moves. Send the exact compatible CG head to IR-1072/#607, IR-1070, IR-1064 and IR-595."}
```

## Analysis

The focused `tc_033_quantity_conversion_has_an_oracle_and_no_harness` test passes only after its helper changes from plain `read` to `read_with_source`. The production `generate_routed` API still accepts `&ContractPackageV1` without source authentication, and `scalar_ir::mode`, `has_law`, and `quantity_conversion_target` convert `NotAdmitted` into missing optional data. This leaves `IR-1079-AC-3` without a passing plain-reader control. The earlier focused log at `83dfdbac` measured the resulting `ModeNotDerivable` refusal before the helper switch. No fixture or test in the delta asserts a typed refusal for that input.

The checked-graph association test has three published fixture paths. Two pass at the head. The `positive-operation-identities.json` path stops in `CheckedPackageV2::read` with `InvalidDerivedShape`, before the equality and association assertions. This is a live gap in the reviewed PR's association evidence even though the failure source is outside CG. No spec, plan, or matrix artifact changed in the delta. Semantic intent was assessed for the touched AC/test/code triples because that is the explicit scope of this review; no repository-wide completion claim is made.

## Verdict

**FAIL.** One public input path remains uncovered and one tagged association control fails before the code under test runs. Plan completion: not assessed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The passing routed quantity control uses authenticated V1 input and no longer tests the plain reader-admitted V1 path accepted by the public API, where `NotAdmitted` silently becomes a missing scalar mode or law | tests/it/routed_generation.rs:214, src/core/ir/scalar.rs:26-48 |
| FND-002 | high | Published association evidence is incomplete: one of three fixture controls is refused by IR before CG association assertions execute | src/oracle/checked_graph/tests.rs:158 |

## Coverage

Plan completion: not assessed. Diff-scoped AC-to-test-to-code review; no repository-wide matrix verdict. Existing trace tags were retained and the passing quantity control remains tagged `FR-014-AC-51`, `FR-014-AC-2`, `FR-022-AC-11`, `FR-015-AC-15`, `TC-033`.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | At `794a921d`, the added tagged test covers an explicit `UnauthenticatedOperationView` error for plain V1 quantity input, so silent loss is removed. It does not prove `IR-1079-AC-3`'s no-new-behavior requirement for public plain reader-admitted inputs. The new guard rejects all reached Kani applications; the pre-existing plain-read integer-ordering routed test fails 0/1 with `UnauthenticatedOperationView` at `src/routed/contract_package_tests.rs:1774` instead of generating output. No successful plain-reader behavior control is preserved. |
| FND-002 | rejected | The observed failure used stale local QSpec fixture blob `822d6450` at `69d75ea1`. The clean current QSpec `f8d63672` fixture blob is `7f4862a2`, and all three unchanged association tests pass at CG `794a921d` with IR `3e74887e`. The finding does not describe a PR evidence gap. |
| FND-001 | still-open | At `e9ee57ac`, the public plain-read quantity route has a passing typed-refusal control and authenticated quantity still generates. The previously failing plain-read integer-ordering route also passes 1/1 after the guard is limited to families that read authenticated operation views. The source comparison covers each `Family` arm, state-clause and field-projection accessors. Yet the reader-admitted plain-V1 quantity route changes from oracle generation to whole-call error, contrary to `IR-1079-AC-3`'s no-new-behavior criterion and `FR-014-AC-51`'s credited-oracle behavior. |

| FND-001 | fixed | The amended IR-1079 AC-3 and merged FR-014-AC-55/FR-022-AC-30 explicitly define the plain-read typed refusal. The PR now tags focused positive and adverse controls for quantity, division, view-independent integer, and lowest-index permutation behavior. |
| FND-003 | still-open | The two new TestMatrix rows claim AC-55 and AC-30 fully Covered, but their cited tests do not assert the source-authenticated provenance or numeric oracle results AC-55 preserves through AC-18/48/50, and the selected-division positive only checks `OperationNotRendered` plus a domain, not the admitted law profile required by AC-30. Existing FR-014-AC-48 through AC-51 remain Planned. Mark these rows partial or add assertions that meet the full clauses. |

| FND-003 | fixed | At `b9406c45`, both TestMatrix rows now say Partially covered and name the unverified numerical runtime/V2 parity clause. Focused assertions additionally check authenticated IrConfirmed provenance, retained quantity member target and exact owner call, and the selected Truncating division law in emitted source. The remaining numerical oracle agreement is explicitly recorded as unverified under the existing Planned FR-014-AC-48 through AC-51 work; it is not credited as proof from this compile adaptation. |

| FND-004 | still-open | At 704a9956, tagged formula and fairness declared-body controls now assert `Unsupported`, but the changed direct fairness plain-read control still asserts QSL-121. Because FR-021-AC-11/33 have no plain-read exception, the TC-031 trace claim remains partly contradicted. |

| FND-004 | fixed | Merged FR-021-AC-41 now defines a whole-call typed refusal for a plain-read temporal-tag `Unsupported` record with `NotAdmitted` root kind. At 3628eea4, the plain direct fairness/formula controls assert that refusal, while authenticated direct and declared-body roots assert `Unsupported`; the previous QSL-121 plain-read assertion is gone. |

| FND-005 | fixed | At ac68c8d6, the temporal function gets a distinct name. Two separate lowering requests place the body and direct clause respectively at driver index 0, and each asserts the fixed index/root under both function-order and item-order permutations. The late arity control separately proves body precedence. |
| FND-006 | fixed | The authored TM-003 TestMatrix now includes FR-021-AC-41 in TC-031's criterion list and a dedicated FR-021 coverage row. That row is explicitly Partially covered and identifies the missing combined public selected-reference fixture, so it does not overclaim the seam-only control. |

## Disposition evidence

Focused final-head evidence: `target/ir1087-ir1098-cohort-794a921d.log` (new public-route typed-error test 1/1, authenticated quantity control 1/1, association 3/3, all-target check, Clippy, fmt, diff check). My additional focused existing test `cargo test --locked --lib admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling` failed 0/1 with `UnauthenticatedOperationView` at `src/routed/contract_package_tests.rs:1774` using clean QSpec `f8d63672`. Plan completion: not assessed.

### Disposition round 2

The exact-head focused log `target/ir1087-discriminating-guard-e9ee57ac.log` records the formerly failing plain-read integer-ordering test 1/1, the plain-read quantity typed-refusal test 1/1, authenticated quantity generation 1/1, Clippy, fmt, and diff check. I inspected every `Family` arm against actual mode, law, member target, state-clause and projection accessor use in the changed routed paths; no view-reading family was omitted. This closes the view-independent regression but does not restore `FR-014-AC-51` behavior on public reader-admitted quantity input. Plan completion: not assessed. No aggregate gate or Kani was run by this review.

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The newly Covered FR-014-AC-55 and FR-022-AC-30 matrix rows credit preservation of authenticated oracle/provenance and admitted division-law behavior, but the cited tests assert only generation/refusal, generic Kani disposition and domain, and request-index ordering; full AC-55/AC-30 coverage is not demonstrated | spec/oracle/matrix/tests.md:17, spec/routed/matrix/tests.md:27, tests/it/routed_generation.rs:2250, tests/it/capability_settlement.rs:2183 |

### Disposition round 3

At `6e868001`, the explicit plain-read exception closes the original public-route gap. I checked the merged FR-014/FR-022 clauses, changed test tags, selected-division fixture, mixed/permutation test, and both TestMatrix status rows. The new matrix finding is limited to coverage credit; the changed code path has focused positive and refusal controls. Full gate and Kani were not run by this review. Plan completion: not assessed.

### Disposition round 4

Reviewed only the `6e868001..b9406c45` delta: two matrix status corrections and focused quantity/division assertions. The matrix no longer overclaims AC-55/AC-30. The authenticated V1 numeric result versus direct owner and V2 oracle remains unverified, as the corrected rows state; that is the pre-existing FR-014-AC-48 agreement slice (IR-818), not part of IR-1079's compile/API adaptation or its one authorized observable refusal. No full gate or Kani was run by this review. Plan completion: not assessed.

## New findings (disposition pass 5)

Plan completion: not assessed. This is a semantic AC-to-test-to-code check of `4a20e01b..ce76e123`, not a repository-wide matrix claim. FR-021's API description accepts a strictly reader-admitted `ContractPackageV1` and states no source-authentication prerequisite. The changed fairness test asserts a QSL-121 blocker for a plain-read direct root, and the existing declared-body-root test asserts the same for an authenticated formula. Both contradict FR-021-AC-11/33. An independent Opus source review reached the same result. No aggregate gate was run here.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | TC-031's direct-root and declared-body-root controls assert QSL-121 for inputs whose required FR-021-AC-33 outcome is Unsupported, so their trace tags give false acceptance credit | tests/it/exact_function_generation.rs:1932, tests/it/exact_function_generation.rs:2503 |

FND-001/002/003 retain their prior dispositions; FND-004 needs a new disposition after the same-PR fix or an explicit unresolved-blocker decision.

### Disposition round 6

The exact `ce76e123..704a9956` delta corrects the declared-body-root assertion and adds a separate fairness-body call control. The direct plain-read fairness assertion still expects QSL-121; FR-021 does not require source authentication for the public `ContractPackageV1` input. Plan completion: not assessed. No aggregate gate was run.

## New findings (disposition pass 7)

Plan completion: not assessed. Reviewed `3628eea4` against merged FR-021-AC-41 and TC-031 step 13. The new typed error closes the former false assertion. An independent Opus source review found the following coverage defects; no aggregate gate was run.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The AC-41 mixed body/direct permutation control uses two declarations with the same name, so Stage 1 marks both `AmbiguousFunctionName`; the body never enters `unauthenticated_bodies`, and the computed `min` expectation cannot prove lowest eligible index across both kinds | tests/it/exact_function_generation.rs:2640 |
| FND-006 | medium | The TC-031 TestMatrix index stops its criterion list at FR-021-AC-40 and has no FR-021-AC-41 coverage/status row, although normative TC-031 step 13 and three tagged tests now cite AC-41 | spec/oracle/matrix/tests.md:108, spec/oracle/matrix/TC-031-function-application-oracles.md:257 |

FND-004 is fixed under merged AC-41; FND-005/006 require same-PR fixes.

### Disposition round 8

Reviewed `3628eea4..ac68c8d6` against merged AC-41 and TC-031 step 13. The mixed permutation test no longer has ambiguous declaration names, and its two root orders use fixed index-0 expectations, one for each eligible winner. The imported-call control uses source compilation and a real admitted V1 closure; it checks the shared preflight seam rather than a combined public temporal-body request. The new TM-003 row accurately marks that evidence partial. Focused controls and Quire validation passed at the exact head; no aggregate gate was run here. Plan completion: not assessed.

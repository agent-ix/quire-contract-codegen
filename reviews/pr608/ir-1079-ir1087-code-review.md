---
id: SR-611030
title: Code and Rust review of authenticated-view adaptation in CG PR 608
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1079; CG PR 608 delta a5f60818..eee90d17; Cargo.lock, src/core/ir, src/kani/generate, src/oracle/checked_graph/tests.rs, src/routed, tests/it"
review_set: subset
---

## Summary

Independent delta review at `eee90d173ba1b9ea8a0504191c1ece1625d30178`. I applied code-review with its Rust review lane, the repository's `CLAUDE.md`, and rust-style. The merged IR `45a0c993` introduces `NotAdmitted` for unavailable operation views. The revised test helpers use IR's `read_with_source` and the focused quantity test passes, but the public routed input still accepts a plain reader-admitted V1 package whose operation views are unavailable.

## Scope

```yaml
scope:
  - {id: IR-1079-AC-1, path: Cargo.lock, role: examined, excerpt: "Record the exact published heads (IR, QSL after #1729 merges, quire-exact, RT with #171 5524482b) and reproduce the compile failures at those heads before editing; one lock revision per dependency, checked by quire check-lock (0.37.0)."}
  - {id: IR-1079-AC-2, path: src/core/ir/scalar.rs, role: examined, excerpt: "Adapt every CG call site to the reviewed current typed APIs (public accessors instead of the private CheckedApplicationOperation::get, exhaustive CheckedTerm/CheckedMemberType, ExecutionPoint unification, owner_model, ReplayResult/IllTypedCause). No compatibility layer, no re-exported private accessor, no synthesized causes, no unchecked conversions."}
  - {id: IR-1079-AC-3, path: tests/it/routed_generation.rs, role: examined, excerpt: "Compile-level adaptation only: no new behavior. Focused tagged tests where an adapted site has behavior; do not claim proof coverage from compiler-only changes."}
  - {id: IR-1079-AC-4, path: tests/it, role: context_only, excerpt: "One independent diff review, findings fixed in the same PR, one final-head gate, no regate for disjoint main moves. Send the exact compatible CG head to IR-1072/#607, IR-1070, IR-1064 and IR-595."}
  - {id: FR-014-AC-51, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "A selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity operation, as `OperationNotRendered` (Planned, IR-269), because the Kani scalar renderer builds no quantity harness; the conversion is credited by its oracle only and is uncredited for Kani."}
```

## Analysis

The code changes compile against IR `45a0c993`. The changed `projection_field`, `state_clause_kind`, `member_declaration`, `mode`, and `law_definitions` calls handle the new `Result` signature and preserve the prior optional shape result when the view exists. IR's `read_with_source` checks source identity, lock, each represented node, and then installs cloned source terms. The two surviving checked-graph association tests pass through that path. The unchanged `ContractPackageWireV1::read` remains a public admitted input path. It leaves views absent, so the changed `.ok()??` and `.ok()` expressions in CG conflate `NotAdmitted` with missing mode, law, or declaration. `generate_routed` receives only `&ContractPackageV1`; it cannot authenticate after the fact. The test helper's switch to `read_with_source` does not cover this public case. Before that switch, the same quantity test failed with `ModeNotDerivable`, as the focused `83dfdbac` log records.

The published QSpec `positive-operation-identities.json` fails IR V2 admission before `published_source_associations_survive_lowering` reaches a CG assertion. At the reviewed head the observed refusal is `InvalidPackage` / `InvalidDerivedShape` at `/semantic_graph/nodes/43/node_id`; 2/3 association tests pass. This is an external fixture/admission blocker to the final gate. The fixture selection and assertions must stay intact until the producer or admission rule is corrected.

No new cross-repo copied file, compatibility layer, unsafe block, or CI gate narrowing occurs in the delta. `cargo check --locked --all-targets`, scoped Clippy with `-D warnings`, and fmt are recorded as passing at the exact head; I inspected the focused test log and did not run a second build. The aggregate gate and Kani were not run by this review.

## Verdict

**FAIL.** `IR-1079-AC-3` remains unproved for plain reader-admitted V1 inputs, and the published association fixture is refused before the test assertions. The authenticated-path adaptation itself is sound where the source V2 package is retained.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The routed quantity test now authenticates its V1 input, while public `generate_routed` still accepts a plain reader-admitted V1 package; `NotAdmitted` is collapsed to absence and quantity generation silently refuses instead of preserving prior behavior | tests/it/routed_generation.rs:214, src/core/ir/scalar.rs:26-48, src/routed/generate.rs:302 |
| FND-002 | high | The association control cannot exercise CG at this head because merged IR refuses the unchanged published QSpec fixture with `InvalidDerivedShape` at `/semantic_graph/nodes/43/node_id` | src/oracle/checked_graph/tests.rs:158 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | At `794a921d`, `generate_routed` now returns a typed `UnauthenticatedOperationView` for plain reader-admitted Kani closures instead of silently losing the quantity claim. This does not preserve prior successful behavior: the guard rejects every reached application, including integer ordering that needs no mode, law, or member view. I reproduced the existing `admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling` test failing 0/1 at `src/routed/contract_package_tests.rs:1774` with `UnauthenticatedOperationView`. The new test proves only that quantity now gets a typed error. The requested no-new-behavior criterion remains unmet. |
| FND-002 | rejected | The refusal was caused by a stale local QSpec checkout at `69d75ea1`, whose `positive-operation-identities.json` blob `822d6450` differs from current QSpec `f8d63672` blob `7f4862a2`. Against a clean detached `f8d63672` checkout, the unchanged association test passes 3/3 at CG `794a921d` and IR `3e74887e`. The PR does not have this fixture defect. |
| FND-001 | still-open | At `e9ee57ac`, the guard checks authenticated views only for scalar families that actually read mode, law, or member target, plus state-clause and field-projection applications. The previously failing plain-read integer-ordering routed test now passes 1/1, and the plain-read quantity test returns the typed refusal 1/1. This resolves the broad regression and silent loss, but it still changes the public plain-reader quantity outcome from generation to whole-call error. `IR-1079-AC-3` says no new behavior and `FR-014-AC-51` requires the conversion to be credited by its oracle. Those obligations remain unmet for a reader-admitted V1 package. |

| FND-001 | fixed | At `6e868001`, merged FR-014-AC-55/FR-022-AC-30 and amended IR-1079 AC-3 explicitly authorize the typed refusal for plain-read view-dependent routes. The guard classifies only view-reading families; authenticated quantity generation and `OperationNotRendered`, authenticated selected division, plain division refusal, view-independent plain integer success, and lowest-index permutation controls are present. |

| FND-003 | still-open | The exact 704a9956 fix preserves plain-read QSL-121. IR a97babf returns `NotAdmitted` for unsupported source kind on a plain V1 read, and FR-021 has no source-authentication prerequisite or plain-read exception. A separate spec-first decision is pending; this finding remains open. |
| FND-004 | fixed | At 704a9956, Stage 1 calls `authenticated_direct_temporal_refusal(package, position, &admitted)` before `admitted_binary_body`; formula and fairness declared-body controls now assert `Unsupported` on the distinct call claim, preserving the first unsupported node/tag. |

| FND-003 | fixed | At merged CG spec #625 and code head 3628eea4, plain V1 temporal-tag unsupported roots with `NotAdmitted` now return typed whole-call `UnauthenticatedRequestedRootKind` under FR-021-AC-41; authenticated direct formula/fairness still yield `Unsupported`. |

| FND-005 | fixed | At ac68c8d6, `item_reaches_temporal_body` calls the shared `check_lowered_binary_body` used by normal item classification before requiring `call` form. This preserves selected-reference, direct-form and node-tag refusals before the body result; later arity/source/type/package checks no longer suppress AC-41. A real source-compiled imported call with selected references passes the focused control. |

## Disposition evidence

The fix delta is `eee90d17..794a921d`, limited to `Cargo.lock`, `src/routed/generate.rs`, and `tests/it/routed_generation.rs`. The IR and model entries share revision `3e74887e`. The focused final-head log `target/ir1087-ir1098-cohort-794a921d.log` records all-target check, the new public-route regression test, authenticated quantity control, association tests 3/3, Clippy `-D warnings`, fmt, and diff check passing. I ran one additional focused existing test with clean QSpec `f8d63672`: `cargo test --locked --lib admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling` failed 0/1 in 0.36 seconds, returning `UnauthenticatedOperationView` for a plain-read integer-ordering route at `src/routed/contract_package_tests.rs:1774`. No aggregate gate or Kani was run.

### Disposition round 2

The subsequent diff `794a921d..e9ee57ac` changes only `src/oracle/scalar/derive.rs` and `src/routed/generate.rs`. I compared the exhaustive `Family` arms with each call to `Derivation::require_mode`, `division_profile`, `conversion_target`, and `operation_confirmed`: `IntegerDivision`, non-negating decimal arithmetic, IEEE arithmetic, IEEE-to-float32, and quantity conversion require a view; integer ordering, other ordering/equality/arithmetic families, IEEE-to-float64, text/enum and quantity comparisons do not read a view. `quire.op.state.clause` and `quire.op.record.project` cover the frame and projection accessor paths. The exact-head focused log `target/ir1087-discriminating-guard-e9ee57ac.log` records integer ordering 1/1, plain-read quantity refusal 1/1, authenticated quantity generation 1/1, Clippy `-D warnings`, fmt, and diff check passing. FND-002 remained rejected; no association change was made. No aggregate gate or Kani was run. The literal no-new-behavior acceptance remains open for public plain-reader quantity inputs.

### Disposition round 3

Reviewed `e9ee57ac..6e868001`, distinguishing merged main spec changes from this PR diff. The selected division fixture now uses `read_with_source`; the plain copy refuses with `UnauthenticatedOperationView { request_index: 0, node_id }`. The mixed route includes a plain integer positive and two permutations of division and conversion, each naming index 1. Quantity positive generation and the existing renderer settlement remain asserted. The merged #620 spec and the amended IR-1079 AC-3 define this one observable exception. I did not rerun a full gate or Kani; the code lead owns the final-head gate.

### Final config review

At CG #608 head `1a50accc47b7509858e1ef77db586afcd58ebd54`, the only change after the review-clear code head `1a01fb1b` is one `deny.toml` license exception for the exact crate name `quire-checked` and `AGPL-3.0-or-later`. `Cargo.lock` resolves `quire-checked` 0.1.0 from first-party `agent-ix/quire-eval` revision `5406acf3e95092bf64775e14e2b1b4df8d33a9ba`. That revision's workspace manifest declares `license = "AGPL-3.0-or-later"`, and `crates/quire-checked/Cargo.toml` inherits `license.workspace = true`. The exception matches the existing per-crate first-party AGPL policy; it does not change the global permissive allow list, source allow list, bans, or advisory controls. No new finding. The code lead owns the final-head gate.

## New findings (disposition pass 5)

Reviewed only `4a20e01b..ce76e123` after the interrupted final gate. The new IR source-kind accessor distinguishes an authenticated direct formula or fairness request from a temporal clause, and the generated `BodyStep` port matches the runtime contract. The following two paths still disagree with FR-021-AC-11/33. An independent Opus read-only diff review reproduced both by source inspection. No aggregate gate was run by this reviewer.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | A plain reader-admitted V1 direct formula/fairness root is still classified as the clause's QSL-121 blocker because `NotAdmitted` falls through to `item_disposition`; the public generator accepts this input and AC-11/33 state no plain-read exception | src/oracle/function/mod.rs:3001, tests/it/exact_function_generation.rs:2503 |
| FND-004 | medium | Stage 1 still classifies an authenticated declared formula/fairness body root through `admitted_binary_body` as QSL-121, so the distinct call item's claim does not carry the body's required `Unsupported` refusal | src/oracle/function/mod.rs:2786, tests/it/exact_function_generation.rs:1932 |

The earlier FND-001 remains fixed. These new findings require a fresh disposition after a same-PR fix.

### Disposition round 6

Reviewed only `ce76e123..704a9956`. The shared helper preserves authenticated source-kind refusal for both Stage 1 declared bodies and Stage 3 direct items. The tagged formula-body test now asserts `Unsupported`, and the source-backed fairness fixture adds a distinct call whose claim inherits that result. Focused AC-33 5/5, Clippy, fmt and diff-check evidence was reported at the exact fix head; I did not run an aggregate gate. Plain V1 reads still lack the required source form.

## New findings (disposition pass 7)

Reviewed only the source/test fix commit `3628eea4` against merged CG spec #625. An independent Opus source review confirmed the eligibility-order defect below; direct temporal item selection and the driver-to-accounting lookup itself were sound. No aggregate gate was run.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | `valid_call_item_for_temporal_body` does not match pre-body item precedence: it allows lowered call closures with selected dependencies (which `admitted_binary_body` refuses first), while rejecting arity/source/type/package conditions that `item_disposition` checks only after returning the declared body's Stage 1 refusal; AC-41 can wrongly replace an earlier item refusal or fail to refuse an eligible body | src/oracle/function/mod.rs:2448, src/oracle/function/mod.rs:2536, src/oracle/function/mod.rs:3390 |

FND-003 is fixed under merged AC-41. FND-005 requires a same-PR fix and independent disposition.

### Disposition round 8

Reviewed only `3628eea4..ac68c8d6` for FND-005. The preflight and ordinary item path now share `check_lowered_binary_body`, so a selected imported call remains `SelectedDependencyUnsupported` before an eligible temporal body can trigger the whole-call refusal. A later arity mismatch still reaches the body result, matching the normal classification order. The source-compiled imported-call unit control passed 1/1 and the focused AC-33 integration group passed 5/5 according to the exact-head evidence. No aggregate gate was run by this reviewer.
